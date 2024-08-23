use std::ops::Deref;

use beam_autogen_rs::models::InventoryView;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

use super::api::accounts::{AttachFederatedIdentityCompletedEvent, GetAccountMeCompletedEvent};
use super::api::common::{CreateAnononymousUserCompletedEvent, GetTokenEvent, PostTokenEvent};
use super::api::inventory::InventoryGetCompletedEvent;
use crate::api::BeamableBasicApi;
use crate::config::BeamExternalIdentityConfig;
use beam_autogen_rs::models::*;

#[derive(Serialize, Debug, Deserialize, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct BeamContext {
    pub name: Option<String>,
    pub token: Option<TokenStorage>,
    pub user: Option<UserView>,
    pub gamer_tag: Option<i64>
}

impl BeamContext {
    pub fn get_gamer_tag(&self) -> Option<i64> {
        if let Some(id) = self.gamer_tag {
            return Some(id);
        }
        self.user.as_ref().map(|view| view.id)
    }
}

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

impl From<beam_autogen_rs::models::AccountPlayerView> for UserView {
    fn from(value: beam_autogen_rs::models::AccountPlayerView) -> Self {
        UserView {
            device_ids: value.device_ids,
            email: value.email,
            external: value.external.map(|v| {
                v.iter()
                    .map(|v| ExternalIdentity {
                        provider_namespace: v.provider_namespace.clone(),
                        user_id: v.user_id.clone(),
                        provider_service: v.provider_service.clone(),
                    })
                    .collect()
            }),
            id: value.id,
            language: value.language,
            scopes: value.scopes,
            third_party_app_associations: value.third_party_app_associations,
        }
    }
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

#[derive(Serialize, Debug, Deserialize, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct BeamInventory {
    pub currencies: HashMap<String, i64>,
    pub items: HashMap<String, Vec<Item>>,
}

impl From<InventoryView> for BeamInventory {
    fn from(value: InventoryView) -> Self {
        BeamInventory {
            currencies: value
                .currencies
                .iter()
                .map(|i| (i.id.clone(), i.amount))
                .collect(),
            items: value
                .items
                .iter()
                .map(|item| {
                    (
                        item.id.clone(),
                        item.items
                            .iter()
                            .map(|item| Item {
                                created_at: item.created_at,
                                id: item.id,
                                properties: item
                                    .properties
                                    .iter()
                                    .map(|p| ItemProperty {
                                        name: p.name.clone(),
                                        value: p.value.clone(),
                                    })
                                    .collect(),
                                updated_at: item.updated_at,
                                proxy_id: item.proxy_id.clone(),
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Reflect)]
pub struct Item {
    pub updated_at: Option<i64>,
    pub proxy_id: Option<String>,
    pub id: i64,
    pub properties: Vec<ItemProperty>,
    pub created_at: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Reflect)]
pub struct ItemProperty {
    pub name: String,
    pub value: String,
}

impl BeamContext {
    pub fn get_name(&self) -> &str {
        if let Some(name) = &self.name {
            name.as_str()
        } else {
            "Anonymous"
        }
    }
}

#[derive(Serialize, Debug, Deserialize, Reflect, Default)]
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

pub fn save_user_info(
    mut ev: EventReader<CreateAnononymousUserCompletedEvent>,
    mut next_state: ResMut<NextState<super::state::BeamableInitStatus>>,
    mut beam: ResMut<BeamContext>,
    mut pkv: ResMut<PkvStore>,
) {
    for event in ev.read() {
        let event = event.deref();
        match event {
            Ok(token) => {
                beam.token = Some(TokenStorage::from_token_response(token));
                pkv.set("user_context", &(*beam))
                    .expect("failed to store user");
                next_state.set(super::state::BeamableInitStatus::LoggedIn);
            }
            Err(err) => println!("Failure: {:#?}", err),
        }
    }
}

pub fn update_user_info(
    mut ev: EventReader<PostTokenEvent>,
    beam: Option<ResMut<BeamContext>>,
    mut pkv: ResMut<PkvStore>,
) {
    let Some(mut beam) = beam else {
        return;
    };
    for event in ev.read() {
        let event = event.deref();
        match event {
            Ok(token) => {
                beam.token = Some(TokenStorage::from_token_response(token));
                pkv.set("user_context", &(*beam))
                    .expect("failed to store user");
            }
            Err(err) => println!("Failure: {:#?}", err),
        }
    }
}

pub fn read_context(
    mut commands: Commands,
    mut pkv: ResMut<PkvStore>,
    mut next_state: ResMut<NextState<super::state::BeamableInitStatus>>,
) {
    let context = if let Ok(user) = pkv.get::<BeamContext>("user_context") {
        info!("Welcome back {}", user.get_name());
        next_state.set(super::state::BeamableInitStatus::LoggedIn);
        user
    } else {
        let user = BeamContext {
            name: Some("Andrew".to_string()),
            token: None,
            user: None,
            gamer_tag: None,
        };
        pkv.set("Andrew", &user).expect("failed to store user");
        user
    };
    commands.insert_resource(context);
    commands.init_resource::<BeamInventory>();
}

pub fn handle_inventory_get(
    mut inventory_events: EventReader<InventoryGetCompletedEvent>,
    mut commands: Commands,
) {
    for event in inventory_events.read() {
        info!("Inventory Get: {:#?}", event);
        if let Ok(event) = &**event {
            let inventory = BeamInventory::from((*event).clone());
            commands.insert_resource(inventory);
        }
    }
}

pub fn handle_token_callbacks(
    mut get_token_events: EventReader<GetTokenEvent>,
    mut post_token_events: EventReader<PostTokenEvent>,
    beam: Option<ResMut<BeamContext>>,
    mut commands: Commands,
) {
    let Some(mut beam) = beam else {
        return;
    };
    for event in get_token_events.read() {
        info!("GetTokenEvent: {:#?}", event);
        match &**event {
            Ok(data) => {
                beam.token.as_mut().unwrap().access_token = Some(data.token.clone());
                beam.gamer_tag = data.gamer_tag;
                let target_id = beam.gamer_tag.unwrap().to_string();
                commands.beam_get_inventory(Some("currency.coins,items.AiItemContent".to_owned()), target_id);
                commands.beam_get_user_info();
            }
            Err(_) => {
                let token = beam.token.as_ref().unwrap();
                commands.beam_post_token(token.refresh_token.clone().unwrap());
            }
        }
    }
    for event in post_token_events.read() {
        info!("PostTokenEvent: {:#?}", event);
        match &**event {
            Ok(data) => {
                beam.token = Some(TokenStorage::from_token_response(data));
                let target_id = beam.gamer_tag.unwrap().to_string();
                commands.beam_get_inventory(Some("currency.coins,items.AiItemContent".to_owned()),target_id);
                commands.beam_get_user_info();
            }
            Err(_) => {}
        }
    }
}

pub fn handle_accounts_callbacks(
    mut get_user_event: EventReader<GetAccountMeCompletedEvent>,
    mut attach_third_party_event: EventReader<AttachFederatedIdentityCompletedEvent>,
    external_identity: Option<Res<BeamExternalIdentityConfig>>,
    mut next_state: ResMut<NextState<super::state::BeamableInitStatus>>,
    beam: Option<ResMut<BeamContext>>,
    mut commands: Commands,
) {
    let Some(mut beam) = beam else {
        return;
    };
    for event in get_user_event.read() {
        info!("GetAccountMe: {:#?}", event);
        if let Ok(event) = &**event {
            beam.user = Some(UserView::from((*event).clone()));
            if let Some(ref external) = &external_identity {
                if event.external.is_some() {
                    next_state.set(super::state::BeamableInitStatus::FullyInitialized);
                } else {
                    commands.beam_attach_federated_identity(
                        beam_autogen_rs::models::AttachExternalIdentityApiRequest {
                            provider_service: external.provider_service.clone(),
                            external_token: "".to_owned(),
                            challenge_solution: None,
                            provider_namespace: Some(external.provider_namespace.clone()),
                        },
                    );
                }
            } else {
                next_state.set(super::state::BeamableInitStatus::FullyInitialized);
            }
        }
    }
    for event in attach_third_party_event.read() {
        println!("Attach third party: {:#?}", event);
        if (**event).is_ok() {
            next_state.set(super::state::BeamableInitStatus::FullyInitialized);
        }
    }
}
