use beam_autogen_rs::models::AttachExternalIdentityApiRequest;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{
    api::common::{self, RealmsConfigEvent},
    data::auth::BeamAuth,
};

#[derive(Deserialize, Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct BeamableConfigResource {
    pub host: String,
    pub cid: String,
    pub pid: String,
}

#[derive(Deserialize, Serialize, Debug, Resource, Reflect, Deref)]
#[reflect(Resource)]
pub struct BeamableWebsocketUrl(pub String);

/// Should be in format: "micro_{serviceName}:{key_value}"
#[derive(Deserialize, Serialize, Debug, Resource, Reflect, Deref, Default, DerefMut)]
#[reflect(Resource)]
pub struct RoutingMapKey(pub String);

#[cfg(not(target_arch = "wasm32"))]
const CONNECT_URI: &str = "/connect?send-session-start=true";

#[cfg(target_arch = "wasm32")]
const CONNECT_URI: &str = "/connect";

impl BeamableWebsocketUrl {
    pub fn uri(&self) -> String {
        if self.is_empty() {
            format!("wss://api.beamable.com/api{}", CONNECT_URI)
        } else {
            format!("{}{}", &**self, CONNECT_URI)
        }
    }
}
impl BeamableConfigResource {
    pub fn get_x_beam_scope(&self) -> String {
        format!("{}.{}", self.cid, self.pid)
    }
}

#[derive(Deserialize, Debug, Clone, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct BeamExternalIdentityConfig {
    pub provider_service: String,
    pub provider_namespace: String,
}

impl BeamExternalIdentityConfig {
    pub fn make_attach_request(
        &self,
        token: impl Into<String>,
    ) -> AttachExternalIdentityApiRequest {
        AttachExternalIdentityApiRequest {
            provider_service: self.provider_service.clone(),
            provider_namespace: self.provider_namespace.clone().into(),
            external_token: token.into(),
            challenge_solution: None,
        }
    }

    pub fn auth(&self, token: impl Into<String>) -> BeamAuth {
        BeamAuth::ExternalIdentity {
            provider_service: self.provider_service.clone(),
            provider_namespace: self.provider_namespace.clone(),
            external_token: token.into(),
        }
    }
}

pub fn update_config(
    realm_config_response: Trigger<RealmsConfigEvent>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<crate::state::BeamableInitStatus>>,
) {
    let Ok(response) = realm_config_response.event().deref() else {
        next_state.set(crate::state::BeamableInitStatus::ConfigError);
        return;
    };
    info!("{:#?}", response);
    let url = match &response.websocket_config.uri {
        Some(s) => BeamableWebsocketUrl(s.to_owned()),
        None => BeamableWebsocketUrl(String::new()),
    };
    commands.insert_resource(url);
    next_state.set(crate::state::BeamableInitStatus::FullyInitialized);
}

pub fn get_config_defaults(
    mut commands: Commands,
    config: Res<BeamableConfigResource>,
    mut next_state: ResMut<NextState<crate::state::BeamableInitStatus>>,
) {
    let x_beam_scope = config.get_x_beam_scope();
    commands.queue(common::RealmsConfig(
        beam_autogen_rs::apis::default_api::BasicRealmsClientDefaultsGetParams {
            x_beam_scope,
            x_beam_gamertag: None,
        },
        Entity::PLACEHOLDER,
    ));
    next_state.set(crate::state::BeamableInitStatus::UpdatingConfiguration);
}

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<BeamableConfigResource>()
            .register_type::<BeamExternalIdentityConfig>()
            .register_type::<RoutingMapKey>()
            .init_resource::<RoutingMapKey>()
            .add_observer(update_config)
            .add_systems(
                Update,
                get_config_defaults.run_if(
                    resource_added::<BeamableConfigResource>
                        .or(resource_exists_and_changed::<BeamableConfigResource>),
                ),
            );
    }
}
