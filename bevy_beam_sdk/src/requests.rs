use bevy::prelude::*;

use crate::config::{self, BeamableConfig};

#[derive(Resource)]
/// Wrapper around the ReqwestClient, that when inserted as a resource will start connection pools towards
/// the hosts, and also allows all the configuration from the ReqwestLibrary such as setting default headers etc
pub struct ReqwestClient(pub reqwest::Client);

impl ReqwestClient {
    fn from_config(config: &BeamableConfig) -> Self {
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert("X-KS-USER-AGENT", "Bevy-0.15".parse().expect(""));
        default_headers.insert(
            "X-BEAM-SCOPE",
            config
                .get_x_beam_scope()
                .parse()
                .expect("Could not parse beam scope"),
        );
        let client = reqwest::ClientBuilder::new()
            .default_headers(default_headers)
            .build()
            .expect("");
        Self(client)
    }
}

impl std::ops::Deref for ReqwestClient {
    type Target = reqwest::Client;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ReqwestClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct RequestsPlugin;

impl Plugin for RequestsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (|mut cmd: Commands, config: Res<config::BeamableConfig>| {
                cmd.insert_resource(ReqwestClient::from_config(&config));
            })
            .run_if(resource_added::<config::BeamableConfig>),
        );
    }
}
