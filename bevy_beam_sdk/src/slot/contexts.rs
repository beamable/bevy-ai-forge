use crate::api::BeamableBasicApi;
use crate::slot::beam_slot::*;
use crate::slot::inventory::BeamInventory;
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
    pub stats: &'static mut BeamStats,
    pub token: Option<&'static mut TokenStorage>,
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
            .observe(save_user_info)
            .observe(handle_get_token)
            .observe(handle_inventory_get)
            .observe(handle_post_token)
            .observe(handle_get_user_info)
            .observe(handle_get_external_user_info)
            .observe(handle_stats_got)
            .beam_play_as_guest(name);
    }

    pub fn context_from(&mut self, slot: BeamSlot, storage: TokenStorage) {
        let target_id = (*slot.gamer_tag).unwrap_or_default().to_string();
        self.commands
            .spawn((slot, storage))
            .observe(save_user_info)
            .observe(handle_get_token)
            .observe(handle_inventory_get)
            .observe(handle_post_token)
            .observe(handle_get_user_info)
            .observe(handle_get_external_user_info)
            .observe(handle_stats_got)
            .beam_get_user_info()
            .beam_get_stats(target_id.clone())
            .beam_get_inventory(None, target_id);
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
                ..Default::default()
            }
            .into()
        } else {
            None
        }
    }
}
