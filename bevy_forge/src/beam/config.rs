use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct Config {
    pub host: String,
    pub cid: String,
    pub pid: String,
}

impl Default for Config {
    fn default() -> Self {
        match serde_json::from_str::<Config>(crate::consts::CONFIG_DEFAULTS) {
            Ok(config) => config,
            Err(_) => Self {
                host: Default::default(),
                cid: Default::default(),
                pid: Default::default(),
            },
        }
    }
}

#[derive(Deserialize, Debug, Clone, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct BeamExternalIdentityConfig {
    pub provider_service: String,
    pub provider_namespace: String,
}
