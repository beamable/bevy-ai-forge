use beam_autogen_rs::models::NotificationRequestData;
use bevy::prelude::*;
#[cfg(target_family = "wasm")]
use futures_util::StreamExt;
#[cfg(not(target_family = "wasm"))]
use reqwest::header::HeaderValue;
#[cfg(not(target_family = "wasm"))]
use std::net::TcpStream;
#[cfg(not(target_family = "wasm"))]
use tungstenite::{client::IntoClientRequest, connect, stream::MaybeTlsStream, WebSocket};

use crate::notifications::Notification;

#[derive(Component)]
pub struct WebSocketConnection {
    pub uri: String,
    pub token: String,
    pub scope: String,
}

#[cfg(not(target_family = "wasm"))]
#[derive(Component, Deref, DerefMut)]
#[component(storage = "SparseSet")]
pub struct WebSocketConnectionTask(
    pub crossbeam_channel::Receiver<WebSocket<MaybeTlsStream<TcpStream>>>,
);

#[derive(Component, Deref, DerefMut)]
#[component(storage = "SparseSet")]
pub struct WebSocketMessagerTask(pub crossbeam_channel::Receiver<NotificationRequestData>);

#[cfg(target_family = "wasm")]
pub fn on_create(
    mut commands: Commands,
    q: Query<(Entity, &WebSocketConnection), Added<WebSocketConnection>>,
) {
    for (e, connection) in q.iter() {
        let thread_pool = bevy::tasks::IoTaskPool::get();
        let (tx, task) = crossbeam_channel::unbounded();
        let uri = connection.uri.clone();
        thread_pool
            .spawn(async move {
                let ws = tokio_tungstenite_wasm::connect(uri).await.unwrap();
                let (mut _write, mut receiver) = ws.split();
                loop {
                    let Ok(msg) = receiver.next().await.unwrap() else {
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
            })
            .detach();
        commands.entity(e).insert(WebSocketMessagerTask(task));
    }
}

#[cfg(not(target_family = "wasm"))]
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
                // verbose!("Connecting with {:#?}", &request);
                let (socket, _) = connect(request).expect("Can't connect");
                // info!("Response HTTP code: {}", response.status());
                // info!("Response contains the following headers:");
                // for (header, _value) in response.headers() {
                //     info!("* {header}");
                // }
                tx.send(socket)
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

#[cfg(target_family = "wasm")]
pub fn task_handle() {}

#[cfg(not(target_family = "wasm"))]
pub fn task_handle(
    mut commands: Commands,
    mut next_state: ResMut<NextState<super::state::BeamableInitStatus>>,
    mut q: Query<(Entity, &mut WebSocketConnectionTask)>,
) {
    for (e, task) in q.iter_mut() {
        let Ok(mut connected) = task.0.try_recv() else {
            continue;
        };
        next_state.set(super::state::BeamableInitStatus::FullyInitialized);
        commands.entity(e).remove::<WebSocketConnectionTask>();
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