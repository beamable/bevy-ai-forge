use crate::slot::prelude::BeamSlot;
use beam_autogen_rs::models::TokenResponse;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Reflect, Default, Clone, Component)]
#[reflect(Component)]
#[require(BeamSlot)]
pub struct TokenStorage {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
    pub scopes: Option<Vec<String>>,
}

impl TokenStorage {
    pub fn get_token_response(&self) -> Option<TokenResponse> {
        if self.access_token.is_some() && self.refresh_token.is_some() {
            return Some(TokenResponse {
                access_token: self.access_token.clone(),
                refresh_token: self.refresh_token.clone(),
                expires_in: self.expires_in,
                scopes: self.scopes.clone(),
                challenge_token: None,
                token_type: "".to_string(),
            });
        }
        None
    }
    pub fn from_token_response(token: &TokenResponse) -> TokenStorage {
        TokenStorage {
            access_token: token.access_token.clone(),
            refresh_token: token.refresh_token.clone(),
            expires_in: token.expires_in,
            scopes: token.scopes.clone(),
        }
    }
}
