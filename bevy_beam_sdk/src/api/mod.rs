use beam_autogen_rs::models::AttachExternalIdentityApiRequest;
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
    fn beam_get_inventory(&mut self, scope: Option<String>) -> &mut Self;
    fn beam_add_to_inventory(&mut self, new_items: Vec<String>) -> &mut Self;
}

impl<'w, 's> BeamableBasicApi for Commands<'w, 's> {
    fn beam_play_as_guest<S: Into<std::string::String>>(&mut self, name: Option<S>) -> &mut Self {
        let mut new_user = beam_autogen_rs::models::TokenRequestWrapper::new("guest".to_string());
        new_user.username = if name.is_some() {
            Some(name.unwrap().into())
        } else {
            None
        };
        self.add(common::CreateAnononymousUser(new_user));
        self
    }
    fn beam_new_user(&mut self, wrapper: TokenRequestWrapper) -> &mut Self {
        self.add(common::CreateAnononymousUser(wrapper));
        self
    }
    fn beam_attach_federated_identity(
        &mut self,
        wrapper: AttachExternalIdentityApiRequest,
    ) -> &mut Self {
        self.add(accounts::AttachFederatedIdentity(wrapper));
        self
    }
    fn beam_get_user_info(&mut self) -> &mut Self {
        self.add(accounts::GetAccountMe);
        self
    }
    fn beam_get_token(&mut self, token: String) -> &mut Self {
        self.add(common::GetToken(token));
        self
    }
    fn beam_post_token(&mut self, token: String) -> &mut Self {
        self.add(common::PostToken(token));
        self
    }
    fn beam_get_inventory(&mut self, scope: Option<String>) -> &mut Self {
        let val = scope.unwrap_or("items".to_owned());
        self.add(inventory::InventoryGetCommand { scope: val });
        self
    }
    fn beam_add_to_inventory(&mut self, new_items: Vec<String>) -> &mut Self {
        self.add(inventory::InventoryAddCommand { new_items });
        self
    }
}

pub fn register_types(app: &mut App) {
    common::CreateAnononymousUserTask::register(app);
    common::GetTokenTask::register(app);
    common::PostTokenTask::register(app);
    accounts::GetAccountMeTask::register(app);
    accounts::AttachFederatedIdentityTask::register(app);
    inventory::InventoryGet::register(app);
    inventory::InventoryAdd::register(app);
}
