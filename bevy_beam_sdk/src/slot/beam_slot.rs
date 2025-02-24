use crate::api::accounts::{AttachFederatedIdentityCompletedEvent, GetAccountMeCompletedEvent};
use crate::api::common::{CreateAnononymousUserCompletedEvent, GetTokenEvent, PostTokenEvent};
use crate::api::inventory::InventoryGetCompletedEvent;
use crate::api::BeamableBasicApi;
use crate::config::BeamExternalIdentityConfig;
use crate::slot::prelude::*;
use bevy::log::{debug, error, info};
use bevy::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Event)]
pub struct UserLoggedIn;

#[derive(Serialize, Debug, Deserialize, Reflect, Default, Deref)]
pub struct GamerTag(Option<i64>);

#[derive(Serialize, Debug, Deserialize, Component, Reflect, Default)]
#[reflect(Component, Default)]
#[require(BeamInventory, StateScoped::<crate::state::BeamableInitStatus>(|| StateScoped(crate::state::BeamableInitStatus::FullyInitialized)))]
pub struct BeamSlot {
    pub name: Option<String>,
    pub user: Option<UserView>,
    pub gamer_tag: GamerTag,
}

impl BeamSlot {
    pub fn get_gamer_tag(&self) -> GamerTag {
        let value: Option<i64> = self
            .gamer_tag
            .or_else(|| self.user.as_ref().map(|view| view.id));
        GamerTag(value)
    }
}

pub fn save_user_info(ev: Trigger<CreateAnononymousUserCompletedEvent>, mut commands: Commands) {
    let event = ev.event().deref();
    // let Ok((_,mut slot,_,mut token)) = beam.q.get(ev.entity()) else {return;};
    // token
    match event {
        Ok(token) => {
            let access_token = token.access_token.clone().unwrap_or_default();
            let token = TokenStorage::from_token_response(token);
            commands
                .entity(ev.entity())
                .insert(token)
                .beam_get_token(access_token);
        }
        Err(err) => error!("Failure: {:#?}", err),
    }
}

pub fn save_token(
    q: Query<(&TokenStorage, &BeamSlot), Changed<TokenStorage>>,
    mut pkv: ResMut<PkvStore>,
) {
    for (token, slot) in q.iter() {
        info!("SAVING TOKEN");
        pkv.set("user_token", &token).expect("Failed to save user");
        pkv.set("user_slot", &slot).expect("Failed to save user");
    }
}

pub fn try_read_token(pkv: Res<PkvStore>, mut config: BeamableConfiguration) {
    let Ok(token) = pkv.get::<TokenStorage>("user_token") else {
        return;
    };
    let Ok(slot) = pkv.get::<BeamSlot>("user_slot") else {
        return;
    };
    config.context_from(slot, token);
}

pub fn handle_get_token(
    ev: Trigger<GetTokenEvent>,
    mut commands: Commands,
    mut q: Query<BeamableContexts>,
) {
    let event = ev.event().deref();
    let Ok(mut ctx) = q.get_mut(ev.entity()) else {
        return;
    };
    match &event {
        Ok(data) => {
            ctx.token
                .expect("Should have token at this point")
                .access_token = Some(data.token.clone());
            ctx.slot.gamer_tag = GamerTag(data.gamer_tag);
            let target_id = data.gamer_tag.unwrap().to_string();

            commands
                .entity(ev.entity())
                .beam_get_inventory(
                    Some("currency.coins,items.AiItemContent".to_owned()),
                    target_id,
                )
                .beam_get_user_info();
        }
        Err(_) => {
            let token = ctx.token.expect("Should have token at this point");
            commands
                .entity(ev.entity())
                .beam_post_token(token.refresh_token.clone().unwrap());
        }
    }
}
pub fn handle_inventory_get(
    ev: Trigger<InventoryGetCompletedEvent>,
    mut q: Query<BeamableContexts>,
) {
    let event = ev.event().deref();
    let Ok(mut ctx) = q.get_mut(ev.entity()) else {
        return;
    };
    debug!("Inventory update: {:#?}", event);
    if let Ok(event) = event {
        let inventory = BeamInventory::from((*event).clone());
        for (currency, amount) in inventory.currencies {
            if let Some(cur) = ctx.inventory.currencies.get_mut(&currency) {
                *cur = amount;
            } else {
                ctx.inventory.currencies.insert(currency, amount);
            }
        }
        for (item, new_items) in inventory.items {
            if let Some(items) = ctx.inventory.items.get_mut(&item) {
                *items = new_items;
            } else {
                ctx.inventory.items.insert(item, new_items);
            }
        }
    }
}

pub fn handle_post_token(
    ev: Trigger<PostTokenEvent>,
    mut q: Query<BeamableContexts>,
    mut commands: Commands,
) {
    let event = ev.event();
    let Ok(ctx) = q.get_mut(ev.entity()) else {
        return;
    };
    info!("PostTokenEvent: {:#?}", event);
    if let Ok(data) = &**event {
        let new_token = TokenStorage::from_token_response(data);
        let target_id = ctx.slot.gamer_tag.unwrap().to_string();
        commands
            .entity(ev.entity())
            .insert(new_token)
            .beam_get_inventory(
                Some("currency.coins,items.AiItemContent".to_owned()),
                target_id,
            )
            .beam_get_user_info();
    }
}

pub fn handle_get_external_user_info(
    ev: Trigger<AttachFederatedIdentityCompletedEvent>,
    mut commands: Commands,
) {
    let event = ev.event();
    info!("AttachFederatedIdentityCompletedEvent: {:#?}", event);
    let Ok(_) = &**event else {
        return;
    };
    commands.entity(ev.entity()).beam_get_user_info();
}

pub fn handle_get_user_info(
    ev: Trigger<GetAccountMeCompletedEvent>,
    mut q: Query<BeamableContexts>,
    mut commands: Commands,
    external_identity: Option<Res<BeamExternalIdentityConfig>>,
    #[cfg(feature = "websocket")] config: BeamableConfiguration,
) {
    let event = ev.event();
    info!("GetAccountMe: {:#?}", event);
    let Ok(event) = &**event else {
        return;
    };
    {
        let Ok(mut ctx) = q.get_mut(ev.entity()) else {
            return;
        };
        ctx.slot.user = Some(UserView::from((*event).clone()));

        #[cfg(feature = "websocket")]
        {
            let t = ctx.token.expect("Failed to get token").clone();
            if let Some(connection) = config.websocket_connection(&t) {
                commands.entity(ev.entity()).insert_if_new(connection);
            }
        }
    }
    if event.external.is_some() {
        commands.trigger(UserLoggedIn);
        commands.trigger_targets(UserLoggedIn, ev.entity());
        return;
    }

    let Some(external) = &external_identity else {
        commands.trigger(UserLoggedIn);
        commands.trigger_targets(UserLoggedIn, ev.entity());
        return;
    };
    let external = beam_autogen_rs::models::AttachExternalIdentityApiRequest {
        provider_service: external.provider_service.clone(),
        external_token: "".to_owned(),
        challenge_solution: None,
        provider_namespace: Some(external.provider_namespace.clone()),
    };
    commands
        .entity(ev.entity())
        .beam_attach_federated_identity(external);
}
