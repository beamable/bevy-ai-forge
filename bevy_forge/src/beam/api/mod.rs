use beam_common::models::TokenRequestWrapper;
use beam_common_accounts::models::AttachExternalIdentityApiRequest;
use bevy::prelude::*;

pub mod accounts;
pub mod common;
pub mod inventory;

pub(crate) trait BeamableBasicApi {
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
