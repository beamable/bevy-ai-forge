use beam_autogen_rs::apis;
use bevy::prelude::*;

use crate::config::{self, BeamableConfig};
use crate::context::BeamContext;

#[derive(Resource)]
/// Wrapper around the ReqwestClient, that when inserted as a resource will start connection pools towards
/// the hosts, and also allows all the configuration from the ReqwestLibrary such as setting default headers etc
pub struct ReqwestClient {
    pub client: reqwest::Client,
    pub x_beam_scope: String,
    pub access_token: Option<String>,
}

impl ReqwestClient {
    fn default_headers(config: &BeamableConfig) -> reqwest::header::HeaderMap {
        let beam_sdk_version = env!("CARGO_PKG_VERSION");
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert("X-KS-USER-AGENT", "Bevy-0.15".parse().expect(""));
        default_headers.insert("X-KS-BEAM-SDK-VERSION", beam_sdk_version.parse().expect(""));
        default_headers.insert(
            "X-BEAM-SCOPE",
            config
                .get_x_beam_scope()
                .parse()
                .expect("Could not parse beam scope"),
        );
        default_headers
    }

    pub fn beamable_config(&self) -> apis::configuration::Configuration {
        let api_key = Some(apis::configuration::ApiKey {
            key: self.x_beam_scope.clone(),
            prefix: None,
        });
        apis::configuration::Configuration {
            client: self.client.clone(),
            api_key,
            bearer_access_token: self.access_token.clone(),
            user_agent: Some("Bevy-0.15".to_owned()),
            ..Default::default()
        }
    }
}

pub fn build_config(
    beam: Option<Res<BeamContext>>,
    config: Option<Res<BeamableConfig>>,
    mut commands: Commands,
) {
    let Some(config) = config else {
        return;
    };
    let x_beam_scope = config.get_x_beam_scope();
    let mut headers = ReqwestClient::default_headers(&*config);
    let mut access_token = None;
    if let Some(ctx) = beam {
        if let Some(token) = ctx.access_token() {
            access_token = Some(token.clone());
            headers.insert(
                "authorization",
                format!("Bearer {}", &token).parse().expect(""),
            );
        }
    };
    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .expect("");
    commands.insert_resource(ReqwestClient {
        x_beam_scope,
        client,
        access_token,
    });
}

impl std::ops::Deref for ReqwestClient {
    type Target = reqwest::Client;
    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl std::ops::DerefMut for ReqwestClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.client
    }
}

pub struct RequestsPlugin;

impl Plugin for RequestsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (build_config).run_if(resource_added::<config::BeamableConfig>),
        );
        app.add_systems(
            Update,
            (build_config).run_if(resource_exists_and_changed::<BeamContext>),
        );
    }
}
