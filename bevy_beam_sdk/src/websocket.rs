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
    app.add_systems(Update, (on_create, handle_network_events));
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct WebSocketConnection {
    pub uri: String,
    pub token: String,
    pub scope: String,
    pub id: Option<ConnectionId>,
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
            dbg!("CONNECTING TO {:?}", &build);
            net.connect(build, &task_pool.0, &settings);
        }
        #[cfg(target_arch = "wasm32")]
        {
            let build = format!("{}?access_token={}", uri, connection.token);
            let uri = url::Url::parse(&build).expect("Could not parse url");
            warn!("CONNECTING TO {:?}", &build);
            net.connect(uri, &task_pool.0, &settings);
        }
    }
}

fn handle_network_events(
    mut new_network_events: EventReader<bevy_eventwork::NetworkEvent>,
    mut q: Query<&mut WebSocketConnection>,
) {
    for event in new_network_events.read() {
        trace!("EVENT {:?}", &event);
        if let bevy_eventwork::NetworkEvent::Connected(connection_id) = event {
            for mut conn in q.iter_mut() {
                conn.id = Some(*connection_id);
            }
        }
    }
}
