use beam_autogen_rs::models::NotificationRequestData;
use bevy::prelude::*;
use reqwest::header::HeaderValue;
use std::net::TcpStream;
use tungstenite::{client::IntoClientRequest, connect, stream::MaybeTlsStream, WebSocket};

use crate::notifications::Notification;

#[derive(Component)]
pub struct WebSocketConnection {
    pub uri: String,
    pub token: String,
    pub scope: String,
}

#[derive(Component, Deref, DerefMut)]
#[component(storage = "SparseSet")]
pub struct WebSocketConnectionTask(
    pub crossbeam_channel::Receiver<Option<WebSocket<MaybeTlsStream<TcpStream>>>>,
);

#[derive(Component, Deref, DerefMut)]
#[component(storage = "SparseSet")]
pub struct WebSocketMessagerTask(pub crossbeam_channel::Receiver<NotificationRequestData>);

pub fn on_create(
    mut commands: Commands,
    q: Query<(Entity, &WebSocketConnection), Added<WebSocketConnection>>,
) {
    for (e, connection) in q.iter() {
        let thread_pool = bevy::tasks::IoTaskPool::get();
        let (tx, task) = crossbeam_channel::bounded(1);
        let uri = connection.uri.clone();
        let scope = connection.scope.clone();
        let token = format!("Bearer {}", &connection.token);
        thread_pool
            .spawn(async move {
                let mut request = uri.into_client_request().expect("Cannot create request");
                request
                    .headers_mut()
                    .append("Authorization", HeaderValue::from_str(&token).expect(""));
                request
                    .headers_mut()
                    .append("X-BEAM-SCOPE", HeaderValue::from_str(&scope).expect(""));
                match connect(request) {
                    Ok((socket, _)) => {
                        tx.send(Some(socket))
                            .expect("Failed to send working socket");
                    }
                    Err(e) => {
                        eprintln!("Failed to establish websocket connection: {}", e);
                        tx.send(None).expect("Failed to send nothing");
                    }
                }
            })
            .detach();
        commands.entity(e).insert(WebSocketConnectionTask(task));
    }
}

pub fn messages_task_handle(
    mut ev: EventWriter<Notification>,
    mut q: Query<&mut WebSocketMessagerTask>,
) {
    for task in q.iter_mut() {
        let Ok(message) = task.0.try_recv() else {
            continue;
        };
        ev.send(Notification(message));
    }
}

pub fn task_handle(
    mut commands: Commands,
    mut next_state: ResMut<NextState<super::state::BeamableInitStatus>>,
    mut q: Query<(Entity, &mut WebSocketConnectionTask)>,
) {
    for (e, task) in q.iter_mut() {
        let Ok(connected) = task.0.try_recv() else {
            continue;
        };
        commands.entity(e).remove::<WebSocketConnectionTask>();
        next_state.set(super::state::BeamableInitStatus::FullyInitialized);
        let Some(mut connected) = connected else {
            continue;
        };
        let thread_pool = bevy::tasks::IoTaskPool::get();
        let (tx, task) = crossbeam_channel::unbounded();
        thread_pool
            .spawn(async move {
                loop {
                    if !connected.can_read() {
                        break;
                    }
                    let Ok(msg) = connected.read() else {
                        continue;
                    };
                    let Ok(message) = msg.to_text() else {
                        continue;
                    };
                    if message.is_empty() {
                        continue;
                    }
                    match serde_json::from_str::<beam_autogen_rs::models::NotificationRequestData>(
                        message,
                    ) {
                        Ok(data) => {
                            tx.send(data).unwrap();
                        }
                        Err(e) => {
                            info!("Connection Message error: {}", e);
                        }
                    };
                }
                let _ = connected.close(None);
            })
            .detach();
        commands.entity(e).insert(WebSocketMessagerTask(task));
    }
}
