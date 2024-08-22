use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct BeamableConfig {
    pub host: String,
    pub cid: String,
    pub pid: String,
}

#[derive(Deserialize, Debug, Clone, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct BeamExternalIdentityConfig {
    pub provider_service: String,
    pub provider_namespace: String,
}
