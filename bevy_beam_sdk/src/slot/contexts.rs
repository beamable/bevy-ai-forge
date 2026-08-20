use crate::api::BeamableBasicApi;
use crate::data::auth::BeamAuth;
use crate::slot::beam_slot::*;
use crate::slot::inventory::BeamInventory;
use crate::slot::inventory::BeamInventorySyncState;
use crate::slot::prelude::*;
#[cfg(feature = "websocket")]
use crate::websocket::WebSocketConnection;
use bevy::ecs::query::QueryData;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use std::fmt::Debug;

#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct BeamableContexts {
    pub entity: Entity,
    pub slot: &'static mut BeamSlot,
    pub inventory: &'static mut BeamInventory,
    pub inventory_sync: &'static mut BeamInventorySyncState,
    pub stats: &'static mut BeamStats,
    pub(crate) hydration: &'static mut crate::slot::beam_slot::BeamSessionHydration,
    pub token: Option<&'static mut TokenStorage>,
    #[cfg(feature = "websocket")]
    pub connection: Option<&'static WebSocketConnection>,
}

#[derive(SystemParam)]
pub struct BeamableConfiguration<'w, 's> {
    pub config: Res<'w, crate::config::BeamableConfigResource>,
    #[cfg(feature = "websocket")]
    pub ws_config: Option<Res<'w, crate::config::BeamableWebsocketUrl>>,
    commands: Commands<'w, 's>,
}

impl BeamableConfiguration<'_, '_> {
    pub fn new_context(&mut self, name: Option<String>) {
        self.commands
            .spawn(BeamSlot {
                name: name.clone(),
                ..default()
            })
            .beam_new_user(BeamAuth::Anonymous);
    }

    pub fn context_from(&mut self, slot: BeamSlot, storage: TokenStorage) {
        let auth = persisted_session_auth(&storage);
        let mut context = self.commands.spawn((slot, storage));
        if let Some(auth) = auth {
            debug!("refreshing persisted Beamable session before account hydration");
            context.beam_new_user(auth);
        } else {
            warn!("persisted Beamable session has no refresh token; using cached access token");
            context.beam_get_user_info();
        }
    }

    #[cfg(feature = "websocket")]
    pub fn websocket_connection(&self, token: &TokenStorage) -> Option<WebSocketConnection> {
        let Some(web_config) = &self.ws_config else {
            return None;
        };
        if let Some(access) = &token.access_token {
            WebSocketConnection {
                uri: web_config.uri(),
                scope: self.config.get_x_beam_scope(),
                token: access.clone(),
                refresh_token: token.refresh_token.clone(),
                ..Default::default()
            }
            .into()
        } else {
            None
        }
    }
}

fn persisted_session_auth(storage: &TokenStorage) -> Option<BeamAuth> {
    storage.refresh_token.clone().map(BeamAuth::RefreshToken)
}

#[cfg(test)]
mod tests {
    use super::persisted_session_auth;
    use crate::{data::auth::BeamAuth, slot::prelude::TokenStorage};

    #[test]
    fn persisted_session_uses_refresh_token() {
        let storage = TokenStorage {
            access_token: Some("stale-access-token".to_owned()),
            refresh_token: Some("refresh-token".to_owned()),
            ..Default::default()
        };

        assert!(matches!(
            persisted_session_auth(&storage),
            Some(BeamAuth::RefreshToken(token)) if token == "refresh-token"
        ));
    }

    #[test]
    fn persisted_session_without_refresh_token_has_no_refresh_auth() {
        let storage = TokenStorage {
            access_token: Some("cached-access-token".to_owned()),
            ..Default::default()
        };

        assert!(persisted_session_auth(&storage).is_none());
    }
}
