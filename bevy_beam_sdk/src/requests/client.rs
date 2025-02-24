use crate::config::BeamableConfig;
use crate::slot::prelude::*;

#[derive(Clone)]
/// Wrapper around the ReqwestClient, that when inserted as a resource will start connection pools towards
/// the hosts, and also allows all the configuration from the ReqwestLibrary such as setting default headers etc
pub struct ReqwestClient {
    pub client: reqwest::Client,
    pub x_beam_scope: String,
    pub access_token: Option<String>,
    pub gamer_tag: Option<i64>,
}

impl ReqwestClient {
    pub fn with_default_headers(config: &BeamableConfig) -> reqwest::header::HeaderMap {
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

    pub fn new(config: &BeamableConfig, gamer_tag: Option<i64>, token: &TokenStorage) -> Self {
        let x_beam_scope = config.get_x_beam_scope();
        let mut headers = ReqwestClient::with_default_headers(config);
        let mut access_token = None;
        if let Some(token) = &token.access_token {
            headers.insert(
                "authorization",
                format!("Bearer {}", &token).parse().expect(""),
            );
            access_token = Some(token.to_owned().clone());
        }

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("Failed to create a reqwest client");
        Self {
            x_beam_scope,
            client,
            gamer_tag,
            access_token,
        }
    }
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
