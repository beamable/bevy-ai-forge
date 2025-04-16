use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Reflect, Deserialize)]
pub struct UserView {
    pub email: Option<String>,
    pub device_ids: Vec<String>,
    pub scopes: Vec<String>,
    pub id: i64,
    pub external: Option<Vec<ExternalIdentity>>,
    pub language: Option<String>,
    pub third_party_app_associations: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Reflect, Deserialize)]
pub struct ExternalIdentity {
    #[serde(rename = "providerService")]
    pub provider_service: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "providerNamespace")]
    pub provider_namespace: String,
}

impl From<&beam_autogen_rs::models::AccountPlayerView> for UserView {
    fn from(value: &beam_autogen_rs::models::AccountPlayerView) -> Self {
        UserView {
            device_ids: value.device_ids.clone(),
            email: value.email.clone(),
            external: value.external.clone().map(|v| {
                v.iter()
                    .map(|v| ExternalIdentity {
                        provider_namespace: v.provider_namespace.clone(),
                        user_id: v.user_id.clone(),
                        provider_service: v.provider_service.clone(),
                    })
                    .collect()
            }),
            id: value.id,
            language: value.language.clone(),
            scopes: value.scopes.clone(),
            third_party_app_associations: value.third_party_app_associations.clone(),
        }
    }
}
