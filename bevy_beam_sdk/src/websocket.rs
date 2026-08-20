use crate::requests::prelude::BeamHttpClient;
use crate::slot::prelude::TokenStorage;
use beam_autogen_rs::apis::auth_api::{
    api_auth_tokens_refresh_token_post, ApiAuthTokensRefreshTokenPostParams,
};
use beam_autogen_rs::models::{AuthResponse, RefreshTokenAuthRequest};
use bevy::{
    prelude::*,
    tasks::{IoTaskPool, TaskPool, TaskPoolBuilder},
};
use bevy_eventwork::websockets::*;
use bevy_eventwork::{ConnectionId, EventworkRuntime, Network};
use std::time::Duration;

pub(crate) fn websocket_plugin(app: &mut App) {
    app.add_plugins(bevy_eventwork::EventworkPlugin::<WebSocketProvider, TaskPool>::default());
    app.insert_resource(EventworkRuntime(
        TaskPoolBuilder::new().num_threads(2).build(),
    ));
    app.add_plugins(crate::notifications::plugin);
    app.init_resource::<NetworkSettings>()
        .init_resource::<PendingCredentialRefreshes>()
        .register_type::<WebSocketConnection>()
        .register_type::<BeamConnectionStatus>();
    app.add_systems(
        Update,
        (
            tick_connection_backoff,
            start_disconnected_connections,
            poll_credential_refreshes,
            handle_network_events,
        )
            .chain(),
    )
    .add_observer(disconnect_on_removal);
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Reflect)]
/// Lifecycle of the notification websocket for a Beamable context.
pub enum BeamConnectionStatus {
    /// Refreshing credentials or establishing the websocket.
    Connecting,
    /// Notification websocket is healthy.
    Connected,
    /// A retry timer is running after a failed attempt.
    BackingOff,
    /// No active connection or retry attempt.
    #[default]
    Disconnected,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct WebSocketConnection {
    pub uri: String,
    #[reflect(ignore)]
    pub token: String,
    #[reflect(ignore)]
    pub refresh_token: Option<String>,
    pub scope: String,
    pub id: Option<ConnectionId>,
    pub status: BeamConnectionStatus,
    #[reflect(ignore)]
    pub(crate) retry_timer: Option<Timer>,
    pub(crate) retry_attempt: u32,
    pub(crate) refresh_before_connect: bool,
}

impl std::fmt::Debug for WebSocketConnection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WebSocketConnection")
            .field("uri", &redact_uri(&self.uri))
            .field("scope", &self.scope)
            .field("id", &self.id)
            .field("status", &self.status)
            .field("retry_attempt", &self.retry_attempt)
            .finish()
    }
}

struct PendingCredentialRefresh {
    entity: Entity,
    receiver: crossbeam_channel::Receiver<Result<AuthResponse, String>>,
}

#[derive(Resource, Default)]
struct PendingCredentialRefreshes(Vec<PendingCredentialRefresh>);

fn disconnect_on_removal(
    _trigger: On<Remove, WebSocketConnection>,
    q: Query<&WebSocketConnection>,
    net: ResMut<Network<WebSocketProvider>>,
) {
    for connection_id in net.list_connection_ids() {
        if q.iter()
            .any(|connection| connection.id == Some(connection_id))
        {
            continue;
        }
        let _ = net.disconnect(connection_id);
    }
}

fn tick_connection_backoff(time: Res<Time>, mut q: Query<&mut WebSocketConnection>) {
    for mut connection in &mut q {
        if connection.status != BeamConnectionStatus::BackingOff {
            continue;
        }
        let Some(timer) = connection.retry_timer.as_mut() else {
            connection.status = BeamConnectionStatus::Disconnected;
            continue;
        };
        timer.tick(time.delta());
        if timer.is_finished() {
            connection.retry_timer = None;
            connection.status = BeamConnectionStatus::Disconnected;
        }
    }
}

fn start_disconnected_connections(
    mut q: Query<(Entity, &mut WebSocketConnection, Option<&TokenStorage>)>,
    mut pending: ResMut<PendingCredentialRefreshes>,
    http: Res<BeamHttpClient>,
    net: ResMut<Network<WebSocketProvider>>,
    settings: Res<NetworkSettings>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
) {
    for (entity, mut connection, token_storage) in &mut q {
        if connection.status != BeamConnectionStatus::Disconnected {
            continue;
        }
        connection.id = None;
        connection.status = BeamConnectionStatus::Connecting;
        if !connection.refresh_before_connect {
            connect_with_access_token(&connection, &net, &settings, &task_pool);
            continue;
        }
        let refresh_token = token_storage
            .and_then(|storage| storage.refresh_token.clone())
            .or_else(|| connection.refresh_token.clone());
        let Some(refresh_token) = refresh_token else {
            connect_with_access_token(&connection, &net, &settings, &task_pool);
            continue;
        };

        let client = http.client_for_scope(&connection.scope);
        let scope = connection.scope.clone();
        let (sender, receiver) = crossbeam_channel::bounded(1);
        IoTaskPool::get()
            .spawn(async move {
                let configuration = beam_autogen_rs::apis::configuration::Configuration {
                    client,
                    user_agent: None,
                    ..Default::default()
                };
                let request = api_auth_tokens_refresh_token_post(
                    &configuration,
                    ApiAuthTokensRefreshTokenPostParams {
                        x_beam_scope: Some(scope),
                        x_beam_gamertag: None,
                        refresh_token_auth_request: Some(RefreshTokenAuthRequest {
                            refresh_token: Some(Some(refresh_token)),
                            ..Default::default()
                        }),
                    },
                );
                #[cfg(target_arch = "wasm32")]
                let result = request.await;
                #[cfg(not(target_arch = "wasm32"))]
                let result = async_compat::Compat::new(request).await;
                let result = result.map_err(|error| safe_auth_error(&error));
                let _ = sender.send(result);
            })
            .detach();
        pending
            .0
            .push(PendingCredentialRefresh { entity, receiver });
    }
}

fn poll_credential_refreshes(
    mut pending: ResMut<PendingCredentialRefreshes>,
    mut q: Query<(&mut WebSocketConnection, Option<&mut TokenStorage>)>,
    net: ResMut<Network<WebSocketProvider>>,
    settings: Res<NetworkSettings>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
) {
    pending.0.retain_mut(|refresh| {
        let result = match refresh.receiver.try_recv() {
            Ok(result) => result,
            Err(crossbeam_channel::TryRecvError::Empty) => return true,
            Err(crossbeam_channel::TryRecvError::Disconnected) => {
                if let Ok((mut connection, _)) = q.get_mut(refresh.entity) {
                    schedule_backoff(&mut connection, "credential refresh task stopped");
                }
                return false;
            }
        };
        let Ok((mut connection, token_storage)) = q.get_mut(refresh.entity) else {
            return false;
        };
        match result {
            Ok(response) => {
                let Some(access_token) = response.access_token.flatten() else {
                    schedule_backoff(&mut connection, "credential response had no access token");
                    return false;
                };
                connection.token = access_token.clone();
                if let Some(refresh_token) = response.refresh_token.flatten() {
                    connection.refresh_token = Some(refresh_token.clone());
                    if let Some(mut storage) = token_storage {
                        storage.access_token = Some(access_token);
                        storage.refresh_token = Some(refresh_token);
                    }
                } else if let Some(mut storage) = token_storage {
                    storage.access_token = Some(access_token);
                }
                connection.refresh_before_connect = false;
                connect_with_access_token(&connection, &net, &settings, &task_pool);
            }
            Err(error) => schedule_backoff(&mut connection, &error),
        }
        false
    });
}

fn connect_with_access_token(
    connection: &WebSocketConnection,
    net: &Network<WebSocketProvider>,
    settings: &NetworkSettings,
    task_pool: &EventworkRuntime<TaskPool>,
) {
    debug!(
        "connecting Beamable websocket to {}",
        redact_uri(&connection.uri)
    );
    #[cfg(not(target_arch = "wasm32"))]
    {
        let request = ClientRequestBuilder::new(
            connection
                .uri
                .parse()
                .expect("invalid Beamable websocket URI"),
        )
        .with_header("Authorization", format!("Bearer {}", connection.token))
        .with_header("X-BEAM-SCOPE", connection.scope.clone());
        net.connect(request, &task_pool.0, settings);
    }
    #[cfg(target_arch = "wasm32")]
    {
        let separator = if connection.uri.contains('?') {
            '&'
        } else {
            '?'
        };
        let uri = url::Url::parse(&format!(
            "{}{separator}access_token={}",
            connection.uri, connection.token
        ))
        .expect("invalid Beamable websocket URI");
        net.connect(uri, &task_pool.0, settings);
    }
}

fn handle_network_events(
    mut events: MessageReader<bevy_eventwork::NetworkEvent>,
    mut q: Query<&mut WebSocketConnection>,
) {
    for event in events.read() {
        match event {
            bevy_eventwork::NetworkEvent::Connected(connection_id) => {
                if let Some(mut connection) = q
                    .iter_mut()
                    .find(|connection| connection.status == BeamConnectionStatus::Connecting)
                {
                    connection.id = Some(*connection_id);
                    connection.status = BeamConnectionStatus::Connected;
                    connection.retry_attempt = 0;
                    connection.retry_timer = None;
                    connection.refresh_before_connect = false;
                    info!("Beamable websocket connected");
                }
            }
            bevy_eventwork::NetworkEvent::Disconnected(connection_id) => {
                if let Some(mut connection) = q
                    .iter_mut()
                    .find(|connection| connection.id == Some(*connection_id))
                {
                    connection.id = None;
                    schedule_backoff(&mut connection, "websocket closed");
                }
            }
            bevy_eventwork::NetworkEvent::Error(error) => {
                for mut connection in q
                    .iter_mut()
                    .filter(|connection| connection.status == BeamConnectionStatus::Connecting)
                {
                    let reason = match error {
                        bevy_eventwork::error::NetworkError::Http { status, .. } => {
                            connection.refresh_before_connect = matches!(*status, 401 | 403);
                            format!("websocket handshake returned HTTP {status}")
                        }
                        _ => "websocket transport failed".to_owned(),
                    };
                    connection.id = None;
                    schedule_backoff(&mut connection, &reason);
                }
            }
        }
    }
}

fn schedule_backoff(connection: &mut WebSocketConnection, reason: &str) {
    connection.id = None;
    let exponent = connection.retry_attempt.min(6);
    let base_seconds = (1_u64 << exponent).min(60) as f32;
    let jitter = 0.85 + ((connection.retry_attempt.wrapping_mul(37) % 31) as f32 / 100.0);
    let delay = (base_seconds * jitter).min(60.0);
    connection.retry_attempt = connection.retry_attempt.saturating_add(1);
    connection.retry_timer = Some(Timer::new(Duration::from_secs_f32(delay), TimerMode::Once));
    connection.status = BeamConnectionStatus::BackingOff;
    warn!("{reason}; retrying websocket in {delay:.1}s");
}

fn redact_uri(uri: &str) -> &str {
    uri.split('?').next().unwrap_or("<invalid websocket URI>")
}

fn safe_auth_error<T>(error: &beam_autogen_rs::apis::Error<T>) -> String {
    match error {
        beam_autogen_rs::apis::Error::ResponseError(response) => {
            format!(
                "credential refresh returned HTTP {}",
                response.status.as_u16()
            )
        }
        beam_autogen_rs::apis::Error::Reqwest(_) => {
            "credential refresh transport failed".to_owned()
        }
        _ => "credential refresh failed".to_owned(),
    }
}
