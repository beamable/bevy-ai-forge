use crate::config::BeamableConfigResource;
use crate::slot::prelude::*;
use bevy::prelude::Resource;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Mutex;

#[derive(Resource)]
/// Shared HTTP transport. Clones reuse reqwest's connection pool.
pub struct BeamHttpClient {
    clients: Mutex<std::collections::HashMap<String, reqwest::Client>>,
}

impl Default for BeamHttpClient {
    fn default() -> Self {
        Self {
            clients: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

impl BeamHttpClient {
    fn client(
        &self,
        config: &BeamableConfigResource,
        routing: Option<&crate::config::RoutingMapKey>,
        access_token: Option<&str>,
    ) -> reqwest::Client {
        let route = routing.map(|value| value.to_string()).unwrap_or_default();
        let auth_key = access_token.map_or(0, |token| {
            let mut hasher = DefaultHasher::new();
            token.hash(&mut hasher);
            hasher.finish()
        });
        let client_key = format!("{route}\0auth:{auth_key}");
        let mut clients = self.clients.lock().expect("Beamable HTTP pool poisoned");
        clients
            .entry(client_key)
            .or_insert_with(|| {
                let mut headers = ReqwestClient::with_default_headers(config);
                if !route.is_empty() {
                    headers.insert(
                        "X-BEAM-SERVICE-ROUTING-KEY",
                        route.parse().expect("invalid service routing key"),
                    );
                }
                if let Some(token) = access_token {
                    headers.insert(
                        reqwest::header::AUTHORIZATION,
                        ReqwestClient::authorization_header(token),
                    );
                }
                reqwest::ClientBuilder::new()
                    .default_headers(headers)
                    .build()
                    .expect("failed to create Beamable HTTP client")
            })
            .clone()
    }

    pub(crate) fn client_for_scope(&self, scope: &str) -> reqwest::Client {
        let key = format!("scope:{scope}");
        let mut clients = self.clients.lock().expect("Beamable HTTP pool poisoned");
        clients
            .entry(key)
            .or_insert_with(|| {
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "X-BEAM-SCOPE",
                    scope.parse().expect("invalid Beamable scope"),
                );
                reqwest::ClientBuilder::new()
                    .default_headers(headers)
                    .build()
                    .expect("failed to create Beamable HTTP client")
            })
            .clone()
    }
}

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
    fn authorization_header(token: &str) -> reqwest::header::HeaderValue {
        let mut authorization = format!("Bearer {token}")
            .parse::<reqwest::header::HeaderValue>()
            .expect("invalid Beamable access token");
        authorization.set_sensitive(true);
        authorization
    }

    pub fn with_default_headers(config: &BeamableConfigResource) -> reqwest::header::HeaderMap {
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

    pub fn new(
        http: &BeamHttpClient,
        config: &BeamableConfigResource,
        routing: Option<&crate::config::RoutingMapKey>,
        gamer_tag: Option<i64>,
        token: &TokenStorage,
    ) -> Self {
        let x_beam_scope = config.get_x_beam_scope();
        let mut access_token = None;
        if let Some(token) = &token.access_token {
            access_token = Some(token.to_owned().clone());
        }
        let client = http.client(config, routing, access_token.as_deref());
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

#[cfg(test)]
mod tests {
    use super::ReqwestClient;

    #[test]
    fn authenticated_clients_supply_generator_fallback_headers() {
        let authorization = ReqwestClient::authorization_header("access-token");

        assert_eq!(authorization, "Bearer access-token");
        assert!(authorization.is_sensitive());
    }
}
