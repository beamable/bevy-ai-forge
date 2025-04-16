use beam_autogen_rs::models::TokenRequestWrapper;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone, Reflect)]
pub enum BeamAuth {
    #[default]
    Anonymous,
    ExternalIdentity {
        provider_service: String,
        provider_namespace: String,
        external_token: String,
    },
    RefreshToken(String),
    UserWithPassword {
        username: String,
        password: String,
    },
    UserWithPasswordCustomerScoped {
        username: String,
        password: String,
    },
}

impl From<BeamAuth> for TokenRequestWrapper {
    fn from(val: BeamAuth) -> Self {
        match val {
            BeamAuth::Anonymous => TokenRequestWrapper {
                grant_type: "guest".to_owned(),
                ..default()
            },
            BeamAuth::ExternalIdentity {
                provider_service,
                provider_namespace,
                external_token,
            } => TokenRequestWrapper {
                grant_type: "external".into(),
                provider_service: provider_service.into(),
                provider_namespace: provider_namespace.into(),
                external_token: external_token.into(),
                ..default()
            },
            BeamAuth::RefreshToken(refresh_token) => TokenRequestWrapper {
                grant_type: "refresh_token".into(),
                refresh_token: Some(refresh_token),
                ..default()
            },
            BeamAuth::UserWithPassword { username, password } => TokenRequestWrapper {
                grant_type: "password".into(),
                username: username.into(),
                password: password.into(),
                customer_scoped: Some(false),
                ..default()
            },
            BeamAuth::UserWithPasswordCustomerScoped { username, password } => {
                TokenRequestWrapper {
                    grant_type: "password".into(),
                    username: username.into(),
                    password: password.into(),
                    customer_scoped: Some(true),
                    ..default()
                }
            }
        }
    }
}
