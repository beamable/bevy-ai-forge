use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

use async_channel::unbounded;
use bevy::prelude::*;
use dashmap::DashMap;
use futures_lite::StreamExt;

use crate::{
    error::NetworkError,
    network_message::NetworkMessage,
    runtime::{run_async, EventworkRuntime},
    AsyncChannel, Connection, ConnectionId, NetworkData, NetworkEvent, NetworkPacket, Runtime,
};

use super::{Network, NetworkProvider};

impl<NP: NetworkProvider> std::fmt::Debug for Network<NP> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Network [{} Connected Clients]",
            self.established_connections.len()
        )
    }
}

impl<NP: NetworkProvider> Network<NP> {
    pub(crate) fn new(_provider: NP) -> Self {
        Self {
            recv_message_map: Arc::new(DashMap::new()),
            established_connections: Arc::new(DashMap::new()),
            new_connections: AsyncChannel::new(),
            disconnected_connections: AsyncChannel::new(),
            error_channel: AsyncChannel::new(),
            server_handle: None,
            connection_tasks: Arc::new(DashMap::new()),
            connection_task_counts: AtomicU32::new(0),
            connection_count: 0,
        }
    }

    /// Returns true if there are any active connections
    #[inline(always)]
    pub fn has_connections(&self) -> bool {
        self.established_connections.len() > 0
    }

    /// Start listening for new clients
    ///
    /// ## Note
    /// If you are already listening for new connections, this will cancel the original listen
    pub fn listen<RT: Runtime>(
        &mut self,
        accept_info: NP::AcceptInfo,
        runtime: &RT,
        network_settings: &NP::NetworkSettings,
    ) -> Result<(), NetworkError> {
        self.stop();

        let new_connections = self.new_connections.sender.clone();
        let error_sender = self.error_channel.sender.clone();
        let settings = network_settings.clone();

        trace!("Started listening");

        self.server_handle = Some(Box::new(run_async(
            async move {
                let accept = NP::accept_loop(accept_info, settings).await;
                match accept {
                    Ok(mut listen_stream) => {
                        while let Some(connection) = listen_stream.next().await {
                            new_connections
                                .send(connection)
                                .await
                                .expect("Connection channel has closed");
                        }
                    }
                    Err(e) => error_sender
                        .send(e)
                        .await
                        .expect("Error channel has closed."),
                }
            },
            runtime,
        )));

        Ok(())
    }

    /// Start async connecting to a remote server.
    pub fn connect<RT: Runtime>(
        &self,
        connect_info: NP::ConnectInfo,
        runtime: &RT,
        network_settings: &NP::NetworkSettings,
    ) {
        debug!("Starting connection");

        let network_error_sender = self.error_channel.sender.clone();
        let connection_event_sender = self.new_connections.sender.clone();
        let settings = network_settings.clone();

        let connection_task_weak = Arc::downgrade(&self.connection_tasks);
        let task_count = self.connection_task_counts.fetch_add(1, Ordering::SeqCst);

        self.connection_tasks.insert(
            task_count,
            Box::new(run_async(
                async move {
                    match NP::connect_task(connect_info, settings).await {
                        Ok(connection) => connection_event_sender
                            .send(connection)
                            .await
                            .expect("Connection channel has closed"),
                        Err(e) => network_error_sender
                            .send(e)
                            .await
                            .expect("Error channel has closed."),
                    };

                    // Remove the connection task from our dictionary of connection tasks
                    connection_task_weak
                        .upgrade()
                        .expect("Network dropped")
                        .remove(&task_count);
                },
                runtime,
            )),
        );
    }

    /// Send a message to a specific client
    pub fn send_message<T: NetworkMessage>(
        &self,
        client_id: ConnectionId,
        message: T,
    ) -> Result<(), NetworkError> {
        let connection = match self.established_connections.get(&client_id) {
            Some(conn) => conn,
            None => return Err(NetworkError::ConnectionNotFound(client_id)),
        };

        let packet = NetworkPacket {
            kind: String::from(T::NAME),
            data: bincode::serialize(&message).map_err(|_| NetworkError::Serialization)?,
        };

        match connection.send_message.try_send(packet) {
            Ok(_) => (),
            Err(err) => {
                error!("There was an error sending a packet: {}", err);
                return Err(NetworkError::ChannelClosed(client_id));
            }
        }

        Ok(())
    }

    /// Broadcast a message to all connected clients
    pub fn broadcast<T: NetworkMessage + Clone>(&self, message: T) {
        let serialized_message = bincode::serialize(&message).expect("Couldn't serialize message!");
        for connection in self.established_connections.iter() {
            let packet = NetworkPacket {
                kind: String::from(T::NAME),
                data: serialized_message.clone(),
            };

            match connection.send_message.try_send(packet) {
                Ok(_) => (),
                Err(err) => {
                    warn!("Could not send to client because: {}", err);
                }
            }
        }
    }

    /// Disconnect all clients and stop listening for new ones
    ///
    /// ## Notes
    /// This operation is idempotent and will do nothing if you are not actively listening
    pub fn stop(&mut self) {
        if let Some(mut conn) = self.server_handle.take() {
            conn.abort();
            for conn in self.established_connections.iter() {
                match self.disconnected_connections.sender.try_send(*conn.key()) {
                    Ok(_) => (),
                    Err(err) => warn!("Could not send to client because: {}", err),
                }
            }
            self.established_connections.clear();
            self.recv_message_map.clear();

            while self.new_connections.receiver.try_recv().is_ok() {}
        }
    }

    /// Disconnect a specific client
    pub fn disconnect(&self, conn_id: ConnectionId) -> Result<(), NetworkError> {
        let connection = if let Some(conn) = self.established_connections.remove(&conn_id) {
            conn
        } else {
            return Err(NetworkError::ConnectionNotFound(conn_id));
        };

        connection.1.stop();

        Ok(())
    }

    /// returns list of established connections ids
    pub fn list_connection_ids(&self) -> Vec<ConnectionId> {
        self.established_connections
            .iter()
            .map(|s| *s.key())
            .collect()
    }
}

pub(crate) fn handle_new_incoming_connections<NP: NetworkProvider, RT: Runtime>(
    mut server: ResMut<Network<NP>>,
    runtime: Res<EventworkRuntime<RT>>,
    network_settings: Res<NP::NetworkSettings>,
    mut network_events: EventWriter<NetworkEvent>,
) {
    while let Ok(new_conn) = server.new_connections.receiver.try_recv() {
        let id = server.connection_count;
        let conn_id = ConnectionId { id };
        server.connection_count += 1;

        let (read_half, write_half) = NP::split(new_conn);
        let recv_message_map = server.recv_message_map.clone();
        let read_network_settings = network_settings.clone();
        let write_network_settings = network_settings.clone();
        let disconnected_connections = server.disconnected_connections.sender.clone();

        let (outgoing_tx, outgoing_rx) = unbounded();
        let (incoming_tx, incoming_rx) = unbounded();

        server.established_connections.insert(
                conn_id,
                Connection {
                    receive_task: Box::new(run_async(async move {
                        trace!("Starting listen task for {}", id);
                        NP::recv_loop(read_half, incoming_tx, read_network_settings).await;

                        match disconnected_connections.send(conn_id).await {
                            Ok(_) => (),
                            Err(_) => {
                                error!("Could not send disconnected event, because channel is disconnected");
                            }
                        }
                    }, &runtime.0)),
                    map_receive_task: Box::new(run_async(async move{
                        while let Ok(packet) = incoming_rx.recv().await{
                            match recv_message_map.get_mut(&packet.kind[..]) {
                                Some(mut packets) => packets.push((conn_id, packet.data)),
                                None => {
                                    error!("Could not find existing entries for message kinds: {:?}", packet);
                                }
                            }
                        }
                    }, &runtime.0)),
                    send_task: Box::new(run_async(async move {
                        trace!("Starting send task for {}", id);
                        NP::send_loop(write_half, outgoing_rx, write_network_settings).await;
                    }, &runtime.0)),
                    send_message: outgoing_tx,
                    //addr: new_conn.addr,
                },
            );

        network_events.send(NetworkEvent::Connected(conn_id));
    }

    while let Ok(disconnected_connection) = server.disconnected_connections.receiver.try_recv() {
        server
            .established_connections
            .remove(&disconnected_connection);
        network_events.send(NetworkEvent::Disconnected(disconnected_connection));
    }
}

/// A utility trait on [`App`] to easily register [`NetworkMessage`]s
pub trait AppNetworkMessage {
    /// Register a network message type
    ///
    /// ## Details
    /// This will:
    /// - Add a new event type of [`NetworkData<T>`]
    /// - Register the type for transformation over the wire
    /// - Internal bookkeeping
    fn listen_for_message<T: NetworkMessage, NP: NetworkProvider>(&mut self) -> &mut Self;
}

impl AppNetworkMessage for App {
    fn listen_for_message<T: NetworkMessage, NP: NetworkProvider>(&mut self) -> &mut Self {
        let server = self.world().get_resource::<Network<NP>>().expect("Could not find `Network`. Be sure to include the `ServerPlugin` before listening for server messages.");

        debug!("Registered a new ServerMessage: {}", T::NAME);

        assert!(
            !server.recv_message_map.contains_key(T::NAME),
            "Duplicate registration of ServerMessage: {}",
            T::NAME
        );
        server.recv_message_map.insert(T::NAME, Vec::new());
        self.add_event::<NetworkData<T>>();
        self.add_systems(PreUpdate, register_message::<T, NP>)
    }
}

pub(crate) fn register_message<T, NP: NetworkProvider>(
    net_res: ResMut<Network<NP>>,
    mut events: EventWriter<NetworkData<T>>,
) where
    T: NetworkMessage,
{
    let mut messages = match net_res.recv_message_map.get_mut(T::NAME) {
        Some(messages) => messages,
        None => return,
    };

    events.send_batch(messages.drain(..).filter_map(|(source, msg)| {
        T::deserialize_message(&msg).map(|inner| NetworkData { source, inner })
    }));
}
