use std::ops::Deref;

use bevy::prelude::*;
use serde::Deserialize;

use crate::api::common::RealmsConfigEvent;

#[derive(Deserialize, Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct BeamableConfig {
    pub host: String,
    pub cid: String,
    pub pid: String,
    pub websocket_url: Option<String>,
}

impl BeamableConfig {
    pub fn get_x_beam_scope(&self) -> String {
        format!("{}.{}", self.cid, self.pid)
    }
    pub fn get_websocket_uri(&self) -> String {
        match &self.websocket_url {
            Some(s) => format!("{}/connect", &s),
            None => "wss://api.beamable.com/api/connect".to_owned(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct BeamExternalIdentityConfig {
    pub provider_service: String,
    pub provider_namespace: String,
}

pub fn update_config(
    mut realm_config_response: EventReader<RealmsConfigEvent>,
    config: Option<ResMut<BeamableConfig>>,
    mut next_state: ResMut<NextState<crate::state::BeamableInitStatus>>,
) {
    let Some(mut config) = config else {
        return;
    };
    for event in realm_config_response.read() {
        let Ok(response) = event.deref() else {
            continue;
        };
        info!("{:#?}", response);
        config.websocket_url = response.websocket_config.uri.clone();
        next_state.set(crate::state::BeamableInitStatus::WaitingForCredentials);
    }
}
