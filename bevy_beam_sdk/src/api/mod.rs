use beam_autogen_rs::models::AttachExternalIdentityApiRequest;
use beam_autogen_rs::models::InventoryUpdateRequest;
use beam_autogen_rs::models::ItemCreateRequest;
use beam_autogen_rs::models::TokenRequestWrapper;
use bevy::prelude::*;

use crate::data::stats::StatAccessType;
use crate::data::stats::StatDomainType;
use crate::slot::prelude::BeamSlot;
use crate::slot::prelude::TokenStorage;

pub mod accounts;
pub mod common;
pub mod content;
pub mod inventory;
pub mod stats;

#[allow(dead_code)]
pub trait BeamableBasicApi {
    fn beam_new_user(&mut self, wrapper: impl Into<TokenRequestWrapper>) -> &mut Self;
    fn beam_attach_federated_identity(
        &mut self,
        wrapper: AttachExternalIdentityApiRequest,
    ) -> &mut Self;
    fn beam_get_user_info(&mut self) -> &mut Self;
    fn beam_get_token(&mut self, token: String) -> &mut Self;
    fn beam_get_inventory(&mut self, scope: Option<String>) -> &mut Self;
    fn beam_add_to_inventory(&mut self, new_items: Vec<String>) -> &mut Self;
    fn beam_get_stats(&mut self, domain: StatDomainType, access: StatAccessType) -> &mut Self;
}

impl BeamableBasicApi for EntityCommands<'_> {
    fn beam_new_user(&mut self, wrapper: impl Into<TokenRequestWrapper>) -> &mut Self {
        let id = self.id();
        let token_request_wrapper = Some(wrapper.into());
        self.commands().queue(move |world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfigResource>()
                .unwrap()
                .get_x_beam_scope();
            if let Ok(mut entity) = world.get_entity_mut(id) {
                entity.remove::<TokenStorage>();
                #[cfg(feature = "websocket")]
                entity.remove::<crate::websocket::WebSocketConnection>();
            }
            world.commands().queue(common::AuthenticateUser(
                beam_autogen_rs::apis::default_api::BasicAuthTokenPostParams {
                    x_beam_scope,
                    x_beam_gamertag: None,
                    token_request_wrapper,
                },
                id,
            ))
        });
        self
    }
    fn beam_attach_federated_identity(
        &mut self,
        wrapper: AttachExternalIdentityApiRequest,
    ) -> &mut Self {
        let id = self.id();
        self.commands().queue(move |world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfigResource>()
                .unwrap()
                .get_x_beam_scope();
            world.commands().queue(accounts::AttachFederatedIdentity(
                beam_autogen_rs::apis::default_api::BasicAccountsExternalIdentityPostParams {
                    x_beam_scope,
                    x_beam_gamertag: None,
                    attach_external_identity_api_request: Some(wrapper),
                },
                id,
            ))
        });
        self
    }
    fn beam_get_user_info(&mut self) -> &mut Self {
        let id = self.id();
        self.commands().queue(move |world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfigResource>()
                .unwrap()
                .get_x_beam_scope();
            world.commands().queue(accounts::GetAccountMe(
                beam_autogen_rs::apis::default_api::BasicAccountsMeGetParams {
                    x_beam_scope,
                    x_beam_gamertag: None,
                },
                id,
            ))
        });
        self
    }
    fn beam_get_token(&mut self, token: String) -> &mut Self {
        let id = self.id();
        self.commands().queue(move |world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfigResource>()
                .unwrap()
                .get_x_beam_scope();
            world.commands().queue(common::GetToken(
                beam_autogen_rs::apis::default_api::BasicAuthTokenGetParams {
                    x_beam_scope,
                    token,
                    x_beam_gamertag: None,
                },
                id,
            ))
        });
        self
    }
    fn beam_get_inventory(&mut self, scope: Option<String>) -> &mut Self {
        let id = self.id();
        self.commands().queue(move |world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfigResource>()
                .unwrap()
                .get_x_beam_scope();
            let Some(object_id) = world.get_beam_id(id) else {
                return;
            };
            world.commands().queue(inventory::InventoryGet(
                beam_autogen_rs::apis::default_api::ObjectInventoryObjectIdGetParams {
                    x_beam_scope,
                    object_id,
                    x_beam_gamertag: None,
                    scope,
                },
                id,
            ))
        });
        self
    }
    fn beam_add_to_inventory(&mut self, new_items: Vec<String>) -> &mut Self {
        let id = self.id();
        let data = InventoryUpdateRequest {
            new_items: Some(
                new_items
                    .iter()
                    .map(|i| ItemCreateRequest {
                        content_id: i.clone(),
                        properties: vec![],
                    })
                    .collect(),
            ),
            ..default()
        };
        self.commands().queue(move |world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfigResource>()
                .unwrap()
                .get_x_beam_scope();
            let Some(object_id) = world.get_beam_id(id) else {
                return;
            };
            world.commands().queue(inventory::InventoryAdd(
                beam_autogen_rs::apis::default_api::ObjectInventoryObjectIdPutParams {
                    x_beam_scope,
                    object_id,
                    x_beam_gamertag: None,
                    inventory_update_request: Some(data),
                },
                id,
            ))
        });
        self
    }

    fn beam_get_stats(&mut self, domain: StatDomainType, access: StatAccessType) -> &mut Self {
        let id = self.id();
        self.commands().queue(move |world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfigResource>()
                .unwrap()
                .get_x_beam_scope();
            let Some(the_id) = world.get_beam_id(id) else {
                return;
            };
            let object_id = format!("{}.{}.player.{}", domain, access, the_id);
            world.commands().queue(stats::StatsGet(
                beam_autogen_rs::apis::default_api::ObjectStatsObjectIdClientGetParams {
                    stats: None,
                    x_beam_scope,
                    object_id,
                    x_beam_gamertag: None,
                },
                id,
            ))
        });
        self
    }
}

pub fn register_types(app: &mut App) {
    // use crate::requests::prelude::BeamRequestResource;
    common::GetTokenEvent::register(app);
    common::UserAuthenticationEvent::register(app);
    common::RealmsConfigEvent::register(app);
    content::GetManifestEvent::register(app);
    accounts::GetAccountMeCompletedEvent::register(app);
    accounts::AttachFederatedIdentityCompletedEvent::register(app);
    inventory::InventoryGetCompletedEvent::register(app);
    inventory::InventoryAddCompletedEvent::register(app);
    stats::StatsGetEvent::register(app);
}

trait BeamIdHelper {
    fn get_beam_id(self, entity: Entity) -> Option<String>;
}

impl BeamIdHelper for &World {
    fn get_beam_id(self, entity: Entity) -> Option<String> {
        let e = self.get_entity(entity).ok()?;
        let slot = e.get::<BeamSlot>()?;
        slot.get_gamer_tag().map(|e| e.to_string())
    }
}
