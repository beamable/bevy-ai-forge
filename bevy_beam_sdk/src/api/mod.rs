use beam_autogen_rs::models::AttachExternalIdentityApiRequest;
use beam_autogen_rs::models::InventoryUpdateRequest;
use beam_autogen_rs::models::ItemCreateRequest;
use beam_autogen_rs::models::TokenRequestWrapper;
use bevy::prelude::*;

pub mod accounts;
pub mod common;
pub mod inventory;

#[allow(dead_code)]
pub trait BeamableBasicApi {
    fn beam_play_as_guest<S: Into<std::string::String>>(&mut self, name: Option<S>) -> &mut Self;
    fn beam_new_user(&mut self, wrapper: TokenRequestWrapper) -> &mut Self;
    fn beam_attach_federated_identity(
        &mut self,
        wrapper: AttachExternalIdentityApiRequest,
    ) -> &mut Self;
    fn beam_get_user_info(&mut self) -> &mut Self;
    fn beam_get_token(&mut self, token: String) -> &mut Self;
    fn beam_post_token(&mut self, token: String) -> &mut Self;
    fn beam_get_inventory(&mut self, scope: Option<String>, target_id: String) -> &mut Self;
    fn beam_add_to_inventory(&mut self, new_items: Vec<String>, target_id: String) -> &mut Self;
    fn beam_basic_get_realm_config(&mut self) -> &mut Self;
}

impl BeamableBasicApi for Commands<'_, '_> {
    fn beam_play_as_guest<S: Into<std::string::String>>(&mut self, name: Option<S>) -> &mut Self {
        let mut new_user = beam_autogen_rs::models::TokenRequestWrapper::new("guest".to_string());
        new_user.username = name.map(|username| username.into());
        self.queue(|world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfig>()
                .unwrap()
                .get_x_beam_scope();

            world.commands().queue(common::CreateAnononymousUser {
                data: beam_autogen_rs::apis::default_api::BasicAuthTokenPostParams {
                    x_beam_scope,
                    x_beam_gamertag: None,
                    token_request_wrapper: Some(new_user),
                },
                entity: None,
            })
        });
        self
    }
    fn beam_new_user(&mut self, wrapper: TokenRequestWrapper) -> &mut Self {
        self.queue(|world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfig>()
                .unwrap()
                .get_x_beam_scope();
            world.commands().queue(common::CreateAnononymousUser {
                data: beam_autogen_rs::apis::default_api::BasicAuthTokenPostParams {
                    x_beam_scope,
                    x_beam_gamertag: None,
                    token_request_wrapper: Some(wrapper),
                },
                entity: None,
            })
        });
        self
    }
    fn beam_attach_federated_identity(
        &mut self,
        wrapper: AttachExternalIdentityApiRequest,
    ) -> &mut Self {
        self.queue(|world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfig>()
                .unwrap()
                .get_x_beam_scope();
            world.commands().queue(accounts::AttachFederatedIdentity {
                data: beam_autogen_rs::apis::default_api::BasicAccountsExternalIdentityPostParams {
                    x_beam_scope,
                    x_beam_gamertag: None,
                    attach_external_identity_api_request: Some(wrapper),
                },
                entity: None,
            })
        });
        self
    }
    fn beam_get_user_info(&mut self) -> &mut Self {
        self.queue(|world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfig>()
                .unwrap()
                .get_x_beam_scope();
            world.commands().queue(accounts::GetAccountMe {
                data: beam_autogen_rs::apis::default_api::BasicAccountsMeGetParams {
                    x_beam_scope,
                    x_beam_gamertag: None,
                },
                entity: None,
            })
        });
        self
    }
    fn beam_get_token(&mut self, token: String) -> &mut Self {
        self.queue(|world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfig>()
                .unwrap()
                .get_x_beam_scope();
            world.commands().queue(common::GetToken {
                data: beam_autogen_rs::apis::default_api::BasicAuthTokenGetParams {
                    x_beam_scope,
                    token,
                    x_beam_gamertag: None,
                },
                entity: None,
            })
        });
        self
    }
    fn beam_post_token(&mut self, token: String) -> &mut Self {
        self.queue(|world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfig>()
                .unwrap()
                .get_x_beam_scope();
            world.commands().queue(common::PostToken {
                data: beam_autogen_rs::apis::default_api::BasicAuthTokenPostParams {
                    x_beam_scope,
                    x_beam_gamertag: None,
                    token_request_wrapper: Some(TokenRequestWrapper {
                        refresh_token: Some(token),
                        ..Default::default()
                    }),
                },
                entity: None,
            })
        });
        self
    }
    fn beam_get_inventory(&mut self, scope: Option<String>, target_id: String) -> &mut Self {
        let val = scope.unwrap_or("items".to_owned());
        self.queue(|world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfig>()
                .unwrap()
                .get_x_beam_scope();
            world.commands().queue(inventory::InventoryGet {
                data: beam_autogen_rs::apis::default_api::ObjectInventoryObjectIdGetParams {
                    x_beam_scope,
                    object_id: target_id,
                    x_beam_gamertag: None,
                    scope: Some(val),
                },
                entity: None,
            })
        });
        self
    }
    fn beam_add_to_inventory(&mut self, new_items: Vec<String>, target_id: String) -> &mut Self {
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
            apply_vip_bonus: None,
            currencies: None,
            currency_content_ids: vec![],
            currency_properties: None,
            transaction: None,
            delete_items: None,
            empty: false,
            item_content_ids: vec![],
            update_items: None,
        };
        self.queue(|world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfig>()
                .unwrap()
                .get_x_beam_scope();
            world.commands().queue(inventory::InventoryAdd {
                data: beam_autogen_rs::apis::default_api::ObjectInventoryObjectIdPutParams {
                    x_beam_scope,
                    object_id: target_id,
                    x_beam_gamertag: None,
                    inventory_update_request: Some(data),
                },
                entity: None,
            })
        });
        self
    }
    fn beam_basic_get_realm_config(&mut self) -> &mut Self {
        self.queue(|world: &mut World| {
            let x_beam_scope = world
                .get_resource::<crate::config::BeamableConfig>()
                .unwrap()
                .get_x_beam_scope();
            world.commands().queue(common::RealmsConfig {
                data: beam_autogen_rs::apis::default_api::BasicRealmsClientDefaultsGetParams {
                    x_beam_scope,
                    x_beam_gamertag: None,
                },
                entity: None,
            });
        });
        self
    }
}

pub fn register_types(app: &mut App) {
    common::CreateAnononymousUserTask::register(app);
    common::GetTokenTask::register(app);
    common::PostTokenTask::register(app);
    common::RealmsConfigTask::register(app);
    accounts::GetAccountMeTask::register(app);
    accounts::AttachFederatedIdentityTask::register(app);
    inventory::InventoryGetTask::register(app);
    inventory::InventoryAddTask::register(app);
}
