use bevy::{
    prelude::*,
    tasks::{TaskPool, TaskPoolBuilder},
};
use bevy_eventwork::websockets::*;
use bevy_eventwork::{ConnectionId, EventworkRuntime, Network};

pub(crate) fn websocket_plugin(app: &mut App) {
    app.add_plugins(bevy_eventwork::EventworkPlugin::<WebSocketProvider, TaskPool>::default());
    app.insert_resource(EventworkRuntime(
        TaskPoolBuilder::new().num_threads(2).build(),
    ));

    app.add_plugins(crate::notifications::plugin);
    app.init_resource::<NetworkSettings>()
        .register_type::<WebSocketConnection>();
    app.add_systems(Update, (on_create, handle_network_events))
        .add_observer(disconnect_on_removal);
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct WebSocketConnection {
    pub uri: String,
    pub token: String,
    pub scope: String,
    pub id: Option<ConnectionId>,
}

fn disconnect_on_removal(
    _trigger: On<Remove, WebSocketConnection>,
    q: Query<&WebSocketConnection>,
    net: ResMut<Network<WebSocketProvider>>,
) {
    let connections = net.list_connection_ids();
    trace!("DROPPING CONNECTION START {:?}", &connections);
    for con in connections.iter() {
        if q.iter().any(|f| f.id.is_some_and(|f| f.eq(con))) {
            continue;
        }
        trace!("DROPPING CONNECTION {:?}", con);
        let _ = net.disconnect(*con);
    }
}

pub fn on_create(
    q: Query<(Entity, &WebSocketConnection), Added<WebSocketConnection>>,
    net: ResMut<Network<WebSocketProvider>>,
    settings: Res<NetworkSettings>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
) {
    for (_e, connection) in q.iter() {
        let uri = connection.uri.clone();

        #[cfg(not(target_arch = "wasm32"))]
        {
            let token = format!("Bearer {}", &connection.token);
            let scope = connection.scope.clone();
            let build = ClientRequestBuilder::new(uri.try_into().unwrap())
                .with_header("Authorization", token)
                .with_header("X-BEAM-SCOPE", scope);
            trace!("CONNECTING TO {:?}", &build);
            net.connect(build, &task_pool.0, &settings);
        }
        #[cfg(target_arch = "wasm32")]
        {
            let build = format!("{}?access_token={}", uri, connection.token);
            let uri = url::Url::parse(&build).expect("Could not parse url");
            trace!("CONNECTING TO {:?}", &build);
            net.connect(uri, &task_pool.0, &settings);
        }
    }
}

fn handle_network_events(
    mut new_network_events: MessageReader<bevy_eventwork::NetworkEvent>,
    mut q: Query<&mut WebSocketConnection>,
) {
    for event in new_network_events.read() {
        warn!("EVENT {:?}", &event);
        match event {
            bevy_eventwork::NetworkEvent::Connected(connection_id) => {
                for mut conn in q.iter_mut() {
                    if conn.id.is_none() {
                        conn.id = Some(*connection_id);
                    }
                }
            }
            bevy_eventwork::NetworkEvent::Disconnected(connection_id) => {
                if let Some(_item) = q.iter().find(|e| e.id.eq(&Some(*connection_id))) {
                    warn!("WEBSOCKET DISCONNECTED");
                }
            }
            bevy_eventwork::NetworkEvent::Error(network_error) => {
                error!("ERROR {:?}", network_error)
            }
        }
    }
}
