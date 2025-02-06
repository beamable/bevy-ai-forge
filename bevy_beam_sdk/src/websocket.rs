// use beam_autogen_rs::models::NotificationRequestData;
use bevy::{
    prelude::*,
    tasks::{TaskPool, TaskPoolBuilder},
};
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_eventwork::{AppNetworkMessage, ConnectionId, EventworkRuntime, Network, NetworkMessage};
use bevy_eventwork_mod_websockets::{
    ClientRequestBuilder, NetworkSettings, WebSocketConfig, WebSocketProvider,
};

pub(crate) fn websocket_plugin(app: &mut App) {
    app.add_plugins(bevy_eventwork::EventworkPlugin::<
        WebSocketProvider,
        bevy::tasks::TaskPool,
    >::default());
    app.insert_resource(EventworkRuntime(
        TaskPoolBuilder::new().num_threads(2).build(),
    ));

    app.add_plugins(crate::notifications::plugin);
    app.init_resource::<NetworkSettings>();
    app.add_systems(Update, (on_create, handle_network_events));
}

// fn handle_incoming_messages(
//     mut messages: Query<&mut GameChatMessages>,
//     mut new_messages: EventReader<NetworkData<shared::NewChatMessage>>,
// ) {
//     let mut messages = messages.get_single_mut().unwrap();

//     for new_message in new_messages.read() {
//         messages.add(UserMessage::new(&new_message.name, &new_message.message));
//     }
// }

#[derive(Component, Default, Debug)]
pub struct WebSocketConnection {
    pub uri: String,
    pub token: String,
    pub scope: String,
    pub id: Option<ConnectionId>,
}

pub fn on_create(
    // mut commands: Commands,
    q: Query<(Entity, &WebSocketConnection), Added<WebSocketConnection>>,
    net: ResMut<Network<WebSocketProvider>>,
    settings: Res<NetworkSettings>,
    task_pool: Res<EventworkRuntime<TaskPool>>,
) {
    for (_e, connection) in q.iter() {
        let uri = connection.uri.clone();
        let scope = connection.scope.clone();
        let token = format!("Bearer {}", &connection.token);

        let build = ClientRequestBuilder::new(uri.try_into().unwrap())
            .with_header("Authorization", token)
            .with_header("X-BEAM-SCOPE", scope);
        info!("CONNECTING TO {:?}", &build);
        net.connect(build, &task_pool.0, &settings);

        // let thread_pool = bevy::tasks::IoTaskPool::get();
        // let (tx, task) = crossbeam_channel::bounded(1);
        // let scope = connection.scope.clone();
        // let token = format!("Bearer {}", &connection.token);
        // thread_pool
        //     .spawn(async move {
        //         let mut request = uri.into_client_request().expect("Cannot create request");
        //         request
        //             .headers_mut()
        //             .append("Authorization", HeaderValue::from_str(&token).expect(""));
        //         request
        //             .headers_mut()
        //             .append("X-BEAM-SCOPE", HeaderValue::from_str(&scope).expect(""));
        //         match connect(request) {
        //             Ok((socket, _)) => {
        //                 tx.send(Some(socket))
        //                     .expect("Failed to send working socket");
        //             }
        //             Err(e) => {
        //                 eprintln!("Failed to establish websocket connection: {}", e);
        //                 tx.send(None).expect("Failed to send nothing");
        //             }
        //         }
        //     })
        //     .detach();
        // commands.entity(e).insert(WebSocketConnectionTask(task));
    }
}

fn handle_network_events(
    mut new_network_events: EventReader<bevy_eventwork::NetworkEvent>,
    mut next_state: ResMut<NextState<super::state::BeamableInitStatus>>,
    mut q: Query<&mut WebSocketConnection>,
) {
    for event in new_network_events.read() {
        info!("EVENNT {:?}", &event);
        match event {
            bevy_eventwork::NetworkEvent::Connected(connection_id) => {
                for mut conn in q.iter_mut() {
                    conn.id = Some(*connection_id);
                }
                next_state.set(super::state::BeamableInitStatus::FullyInitialized);
            }
            _ => {}
        }
    }
}
// pub fn messages_task_handle(
//     mut ev: EventWriter<Notification>,
//     mut q: Query<&mut WebSocketMessagerTask>,
// ) {
//     for task in q.iter_mut() {
//         let Ok(message) = task.0.try_recv() else {
//             continue;
//         };
//         ev.send(Notification(message));
//     }
// }

// pub fn task_handle(
//     mut commands: Commands,
//     mut next_state: ResMut<NextState<super::state::BeamableInitStatus>>,
//     mut q: Query<(Entity, &mut WebSocketConnectionTask)>,
// ) {
//     for (e, task) in q.iter_mut() {
//         let Ok(connected) = task.0.try_recv() else {
//             continue;
//         };
//         commands.entity(e).remove::<WebSocketConnectionTask>();
//         next_state.set(super::state::BeamableInitStatus::FullyInitialized);
//         let Some(mut connected) = connected else {
//             continue;
//         };
//         let thread_pool = bevy::tasks::IoTaskPool::get();
//         let (tx, task) = crossbeam_channel::unbounded();
//         thread_pool
//             .spawn(async move {
//                 loop {
//                     if !connected.can_read() {
//                         break;
//                     }
//                     let Ok(msg) = connected.read() else {
//                         continue;
//                     };
//                     let Ok(message) = msg.to_text() else {
//                         continue;
//                     };
//                     if message.is_empty() {
//                         continue;
//                     }
//                     match serde_json::from_str::<beam_autogen_rs::models::NotificationRequestData>(
//                         message,
//                     ) {
//                         Ok(data) => {
//                             tx.send(data).unwrap();
//                         }
//                         Err(e) => {
//                             info!("Connection Message error: {}", e);
//                         }
//                     };
//                 }
//                 let _ = connected.close(None);
//             })
//             .detach();
//         commands.entity(e).insert(WebSocketMessagerTask(task));
//     }
// }
