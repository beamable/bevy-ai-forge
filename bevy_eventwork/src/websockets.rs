/// A provider for WebSockets
#[cfg(not(target_arch = "wasm32"))]
pub type WebSocketProvider = native_websocket::NativeWesocketProvider;

/// A provider for WebSockets
#[cfg(target_arch = "wasm32")]
pub type WebSocketProvider = wasm_websocket::WasmWebSocketProvider;

#[cfg(not(target_arch = "wasm32"))]
pub use native_websocket::NetworkSettings;

#[cfg(target_arch = "wasm32")]
pub use wasm_websocket::NetworkSettings;

#[cfg(not(target_arch = "wasm32"))]
pub use async_tungstenite::tungstenite::{protocol::WebSocketConfig, ClientRequestBuilder};

use serde::{Deserialize, Serialize};
/// A struct representing a notification for Beamable.
///
/// This struct is used to deserialize and serialize notifications sent over WebSockets.
/// It contains various fields that provide information about the notification, such as
/// parameters, channel, message key, context, shard, metadata, and the full message.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeamableWasmNotification {
    /// An optional vector of strings representing parameters associated
    ///   with the message. This field is skipped during serialization if it is `None`.
    #[serde(rename = "messageParams", skip_serializing_if = "Option::is_none")]
    pub message_params: Option<Vec<String>>,
    /// An optional string representing the channel through which the message
    ///   is sent. This field is skipped during serialization if it is `None`.
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// An optional string representing a unique key for the message.
    #[serde(rename = "messageKey", skip_serializing_if = "Option::is_none")]
    pub message_key: Option<String>,
    /// An optional string providing additional context for the message.
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// An optional string representing the shard associated with the message.
    #[serde(rename = "shard", skip_serializing_if = "Option::is_none")]
    pub shard: Option<String>,
    /// An optional hashmap containing metadata related to the message, where
    ///   the keys and values are both strings.
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::HashMap<String, String>>,
    ///  An optional string containing the full content of the message.
    #[serde(rename = "messageFull", skip_serializing_if = "Option::is_none")]
    pub message_full: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
mod native_websocket {
    use std::{net::SocketAddr, pin::Pin};

    use crate::websockets::BeamableWasmNotification;
    use async_channel::{Receiver, Sender};
    
    use async_trait::async_trait;
    use async_tungstenite::async_std::ConnectStream;
    
    use async_tungstenite::{
        tungstenite::{protocol::WebSocketConfig, Message},
        WebSocketStream,
    };
    
    use bevy::prelude::{error, info, trace, Deref, DerefMut, Resource};
    use crate::{error::NetworkError, managers::NetworkProvider, NetworkPacket};
    use futures::{
        stream::{SplitSink, SplitStream},
        SinkExt, StreamExt,
    };
    use futures_lite::Stream;

    /// A provider for WebSockets
    #[derive(Default, Debug)]
    pub struct NativeWesocketProvider;

    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    impl NetworkProvider for NativeWesocketProvider {
        type NetworkSettings = NetworkSettings;

        type Socket = WebSocketStream<ConnectStream>;

        type ReadHalf = SplitStream<WebSocketStream<ConnectStream>>;

        type WriteHalf = SplitSink<WebSocketStream<ConnectStream>, Message>;

        type ConnectInfo = async_tungstenite::tungstenite::ClientRequestBuilder;

        type AcceptInfo = SocketAddr;

        type AcceptStream = OwnedIncoming;

        async fn accept_loop(
            _accept_info: Self::AcceptInfo,
            _: Self::NetworkSettings,
        ) -> Result<Self::AcceptStream, NetworkError> {
            Err(NetworkError::Error("No support for servers".into()))
        }

        async fn connect_task(
            connect_info: Self::ConnectInfo,
            _network_settings: Self::NetworkSettings,
        ) -> Result<Self::Socket, NetworkError> {
            info!("Beginning connection");
            let (stream, _response) =
                async_tungstenite::async_std::connect_async_with_tls_connector_and_config(
                    connect_info,
                    Some(Default::default()),
                    Some(*_network_settings),
                )
                    .await
                    .map_err(|error| match error {
                        async_tungstenite::tungstenite::Error::ConnectionClosed => {
                            NetworkError::Error(String::from("Connection closed"))
                        }
                        async_tungstenite::tungstenite::Error::AlreadyClosed => {
                            NetworkError::Error(String::from("Connection was already closed"))
                        }
                        async_tungstenite::tungstenite::Error::Io(io_error) => {
                            NetworkError::Error(format!("Io Error: {}", io_error))
                        }
                        async_tungstenite::tungstenite::Error::Tls(tls_error) => {
                            NetworkError::Error(format!("Tls Error: {}", tls_error))
                        }
                        async_tungstenite::tungstenite::Error::Capacity(cap) => {
                            NetworkError::Error(format!("Capacity Error: {}", cap))
                        }
                        async_tungstenite::tungstenite::Error::Protocol(proto) => {
                            NetworkError::Error(format!("Protocol Error: {}", proto))
                        }
                        async_tungstenite::tungstenite::Error::WriteBufferFull(buf) => {
                            NetworkError::Error(format!("Write Buffer Full Error: {}", buf))
                        }
                        async_tungstenite::tungstenite::Error::Utf8 => {
                            NetworkError::Error("Utf8 Error".to_string())
                        }
                        async_tungstenite::tungstenite::Error::AttackAttempt => {
                            NetworkError::Error("Attack Attempt".to_string())
                        }
                        async_tungstenite::tungstenite::Error::Url(url) => {
                            NetworkError::Error(format!("Url Error: {}", url))
                        }
                        async_tungstenite::tungstenite::Error::Http(http) => {
                            NetworkError::Error(format!("HTTP Error: {:?}", http))
                        }
                        async_tungstenite::tungstenite::Error::HttpFormat(http_format) => {
                            NetworkError::Error(format!("HTTP Format Error: {}", http_format))
                        }
                    })?;
            Ok(stream)
        }

        async fn recv_loop(
            mut read_half: Self::ReadHalf,
            messages: Sender<NetworkPacket>,
            _settings: Self::NetworkSettings,
        ) {
            loop {
                let message = match read_half.next().await {
                    Some(message) => match message {
                        Ok(message) => message,
                        Err(err) => {
                            error!("Error: {err:?}");
                            match err {
                                async_tungstenite::tungstenite::Error::ConnectionClosed
                                | async_tungstenite::tungstenite::Error::AlreadyClosed => {
                                    error!("Connection Closed");
                                    break;
                                }
                                _ => {
                                    error!("Nonfatal error detected: {}", err);
                                    continue;
                                }
                            }
                        }
                    },
                    None => {
                        continue;
                    }
                };

                let packet = match message {
                    Message::Text(t) => {
                        if let Ok(Ok(packet)) = serde_json::from_str::<BeamableWasmNotification>(&t)
                            .map(|s| {
                                // debug!("BeamableWasmNotification: {:?}", &s);
                                let kind = s.context.unwrap_or_default();
                                let data = s
                                    .message_full
                                    .unwrap_or_default()
                                    .as_bytes()
                                    .to_vec();
                                let beam = NetworkPacket { kind, data };
                                if let Ok(Ok(s)) =
                                    serde_json::to_value(&beam).map(serde_json::from_value::<NetworkPacket>)
                                {
                                    Ok(s)
                                } else {
                                    Err(())
                                }
                            })
                        {
                            packet
                        } else {
                            error!("Text Message Received: {t}");
                            continue;
                        }
                    }
                    Message::Binary(binary) => match bincode::deserialize(&binary) {
                        Ok(packet) => packet,
                        Err(err) => {
                            error!("Failed to decode network packet from: {}", err);
                            break;
                        }
                    },
                    Message::Ping(_) => {
                        info!("Ping Message Received");
                        continue;
                    }
                    Message::Pong(_) => {
                        info!("Pong Message Received");
                        continue;
                    }
                    Message::Close(extra) => {
                        error!("Connection Closed: {extra:?}");
                        break;
                    }
                    Message::Frame(_) => todo!(),
                };

                if messages.send(packet).await.is_err() {
                    error!("Failed to send decoded message to eventwork");
                    break;
                }
                info!("Message deserialized and sent to eventwork");
            }
        }

        async fn send_loop(
            mut write_half: Self::WriteHalf,
            messages: Receiver<NetworkPacket>,
            _settings: Self::NetworkSettings,
        ) {
            while let Ok(message) = messages.recv().await {
                let encoded = match bincode::serialize(&message) {
                    Ok(encoded) => encoded,
                    Err(err) => {
                        error!("Could not encode packet {:?}: {}", message, err);
                        continue;
                    }
                };

                trace!("Sending the content of the message!");

                match write_half
                    .send(Message::Binary(encoded))
                    .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        error!("Could not send packet: {:?}: {}", message, err);
                        break;
                    }
                }

                trace!("Succesfully written all!");
            }
        }

        fn split(combined: Self::Socket) -> (Self::ReadHalf, Self::WriteHalf) {
            let (write, read) = combined.split();
            (read, write)
        }
    }

    #[derive(Clone, Debug, Resource, Default, Deref, DerefMut)]
    #[allow(missing_copy_implementations)]
    /// Settings to configure the network, both client and server
    pub struct NetworkSettings(WebSocketConfig);

    /// A special stream for recieving ws connections
    #[allow(clippy::type_complexity)]
    pub struct OwnedIncoming;

    impl Stream for OwnedIncoming {
        type Item = WebSocketStream<ConnectStream>;

        fn poll_next(
            self: Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Option<Self::Item>> {
            std::task::Poll::Pending
        }
    }

    unsafe impl Send for OwnedIncoming {}
}

#[cfg(target_arch = "wasm32")]
mod wasm_websocket {
    use core::panic;
    use std::{net::SocketAddr, pin::Pin};

    use async_channel::{Receiver, Sender};
    use async_trait::async_trait;
    use bevy::prelude::{error, info, trace, Deref, DerefMut, Resource};
    use crate::{error::NetworkError, managers::NetworkProvider, NetworkPacket};
    use futures::{
        stream::{SplitSink, SplitStream},
        SinkExt, StreamExt,
    };
    use futures_lite::Stream;
    use send_wrapper::SendWrapper;
    use tokio_tungstenite_wasm::{Message, WebSocketStream};
    use crate::websockets::BeamableWasmNotification;

    /// A provider for WebSockets
    #[derive(Default, Debug)]
    pub struct WasmWebSocketProvider;

    #[async_trait(?Send)]
    impl NetworkProvider for WasmWebSocketProvider {
        type NetworkSettings = NetworkSettings;

        type Socket = SendWrapper<WebSocketStream>;

        type ReadHalf = SendWrapper<SplitStream<WebSocketStream>>;

        type WriteHalf = SendWrapper<SplitSink<WebSocketStream, Message>>;

        type ConnectInfo = url::Url;

        type AcceptInfo = SocketAddr;

        type AcceptStream = OwnedIncoming;

        async fn accept_loop(
            _accept_info: Self::AcceptInfo,
            _: Self::NetworkSettings,
        ) -> Result<Self::AcceptStream, NetworkError> {
            panic!("Can't create servers on WASM");
        }

        async fn connect_task(
            connect_info: Self::ConnectInfo,
            _network_settings: Self::NetworkSettings,
        ) -> Result<Self::Socket, NetworkError> {
            info!("Beginning connection");
            let stream = tokio_tungstenite_wasm::connect(connect_info)
                .await
                .map_err(|error| match error {
                    tokio_tungstenite_wasm::Error::ConnectionClosed => {
                        NetworkError::Error(format!("Connection Closed"))
                    }
                    tokio_tungstenite_wasm::Error::AlreadyClosed => {
                        NetworkError::Error(format!("Connection Already Closed"))
                    }
                    tokio_tungstenite_wasm::Error::Io(err) => {
                        NetworkError::Error(format!("IO Error: {}", err))
                    }
                    tokio_tungstenite_wasm::Error::Tls(err) => {
                        NetworkError::Error(format!("TLS Error: {}", err))
                    }
                    tokio_tungstenite_wasm::Error::Capacity(err) => {
                        NetworkError::Error(format!("Capacity Error: {}", err))
                    }
                    tokio_tungstenite_wasm::Error::Protocol(err) => {
                        NetworkError::Error(format!("Protocol Error: {}", err))
                    }
                    tokio_tungstenite_wasm::Error::WriteBufferFull(err) => {
                        NetworkError::Error(format!("Write Buffer Full: {}", err))
                    }
                    tokio_tungstenite_wasm::Error::Utf8 => {
                        NetworkError::Error(format!("UTF8 Encoding Error"))
                    }
                    tokio_tungstenite_wasm::Error::AttackAttempt => {
                        NetworkError::Error(format!("Attack Attempt Detected"))
                    }
                    tokio_tungstenite_wasm::Error::Url(err) => {
                        NetworkError::Error(format!("Url Error: {}", err))
                    }
                    tokio_tungstenite_wasm::Error::Http(err) => {
                        NetworkError::Error(format!("HTTP Error: {:?}", err))
                    }
                    tokio_tungstenite_wasm::Error::HttpFormat(err) => {
                        NetworkError::Error(format!("HTTP Format Error: {}", err))
                    }
                    tokio_tungstenite_wasm::Error::BlobFormatUnsupported => {
                        NetworkError::Error("Blob Format Unsupported".to_string())
                    }
                    tokio_tungstenite_wasm::Error::UnknownFormat => {
                        NetworkError::Error(format!("Invalid Format"))
                    }
                })?;
            info!("Connected!");
            return Ok(SendWrapper::new(stream));
        }

        async fn recv_loop(
            mut read_half: Self::ReadHalf,
            messages: Sender<NetworkPacket>,
            _settings: Self::NetworkSettings,
        ) {
            loop {
                let message = match read_half.next().await {
                    Some(message) => match message {
                        Ok(message) => message,
                        Err(err) => match err {
                            tokio_tungstenite_wasm::Error::ConnectionClosed
                            | tokio_tungstenite_wasm::Error::AlreadyClosed => {
                                error!("Connection Closed");
                                break;
                            }
                            _ => {
                                error!("Nonfatal error detected: {}", err);
                                continue;
                            }
                        },
                    },
                    None => {
                        continue;
                    }
                };

                let packet = match message {
                    Message::Text(t) => {
                        if let Ok(Ok(packet)) = serde_json::from_str::<BeamableWasmNotification>(&t)
                            .map(|s| {
                                let kind = s.context.unwrap_or_default();
                                let data = s
                                    .message_full
                                    .unwrap_or_default()
                                    .as_bytes()
                                    .to_vec();
                                let beam = NetworkPacket { kind, data };
                                if let Ok(Ok(s)) =
                                    serde_json::to_value(&beam).map(|e| serde_json::from_value::<NetworkPacket>(e))
                                {
                                    Ok(s)
                                } else {
                                    Err(())
                                }
                            })
                        {
                            packet
                        } else {
                            error!("Text Message Received: {t}");
                            continue;
                        }
                    }
                    Message::Binary(binary) => match bincode::deserialize(&binary) {
                        Ok(packet) => packet,
                        Err(err) => {
                            error!("Failed to decode network packet from: {}", err);
                            break;
                        }
                    },

                    Message::Close(_) => {
                        error!("Connection Closed");
                        break;
                    }
                };

                if messages.send(packet).await.is_err() {
                    error!("Failed to send decoded message to eventwork");
                    break;
                }
                info!("Message deserialized and sent to eventwork");
            }
        }

        async fn send_loop(
            mut write_half: Self::WriteHalf,
            messages: Receiver<NetworkPacket>,
            _settings: Self::NetworkSettings,
        ) {
            while let Ok(message) = messages.recv().await {
                let encoded = match bincode::serialize(&message) {
                    Ok(encoded) => encoded,
                    Err(err) => {
                        error!("Could not encode packet {:?}: {}", message, err);
                        continue;
                    }
                };

                trace!("Sending the content of the message!");

                match write_half
                    .send(Message::Binary(encoded))
                    .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        error!("Could not send packet: {:?}: {}", message, err);
                        break;
                    }
                }

                trace!("Succesfully written all!");
            }
        }

        fn split(combined: Self::Socket) -> (Self::ReadHalf, Self::WriteHalf) {
            let (write, read) = combined.take().split();
            (SendWrapper::new(read), SendWrapper::new(write))
        }
    }

    #[derive(Clone, Debug, Resource, Deref, DerefMut)]
    #[allow(missing_copy_implementations)]
    /// Settings to configure the network
    ///
    /// Note that on WASM this is currently ignored and defaults are used
    pub struct NetworkSettings {
        max_message_size: usize,
    }

    impl Default for NetworkSettings {
        fn default() -> Self {
            Self {
                max_message_size: 64 << 20,
            }
        }
    }

    /// A dummy struct as WASM is unable to accept connections and act as a server
    pub struct OwnedIncoming;

    impl Stream for OwnedIncoming {
        type Item = SendWrapper<WebSocketStream>;

        fn poll_next(
            self: Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Option<Self::Item>> {
            panic!("WASM does not support servers");
        }
    }
}
