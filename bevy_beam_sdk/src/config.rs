use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct BeamableConfig {
    pub host: String,
    pub cid: String,
    pub pid: String,
}

impl BeamableConfig {
    pub fn get_x_beam_scope(&self) -> String {
        format!("{}.{}", self.cid, self.pid)
    }
    pub fn get_websocket_uri(&self) -> String {
        "wss://socket.beamable.com/connect".to_owned()
    }
}

#[derive(Deserialize, Debug, Clone, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct BeamExternalIdentityConfig {
    pub provider_service: String,
    pub provider_namespace: String,
}
