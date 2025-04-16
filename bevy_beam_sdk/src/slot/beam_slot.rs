use crate::api::accounts::{AttachFederatedIdentityCompletedEvent, GetAccountMeCompletedEvent};
use crate::api::common::{GetTokenEvent, UserAuthenticationEvent};
use crate::api::inventory::InventoryGetCompletedEvent;
use crate::api::stats::StatsGetEvent;
use crate::api::BeamableBasicApi;
use crate::data::auth::BeamAuth;
use crate::slot::prelude::*;
use bevy::log::error;
use bevy::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Deref;

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub enum UserLoggedIn {
    Success,
    Fail { status_code: u16, value: String },
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub enum UserInfoUpdated {
    Success,
    Fail { status_code: u16, value: String },
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub enum AttachCredential {
    Success,
    AlreadyInUse,
    Fail { status_code: u16, value: String },
}

#[derive(Serialize, Debug, Deserialize, Reflect, Default, Deref)]
pub struct GamerTag(Option<i64>);

#[derive(Serialize, Debug, Deserialize, Component, Reflect, Default)]
#[reflect(Component, Default)]
#[require(BeamInventory, Name(|| Name::new("BeamSlot")), BeamStats, StateScoped::<crate::state::BeamableInitStatus>(|| StateScoped(crate::state::BeamableInitStatus::FullyInitialized)))]
pub struct BeamSlot {
    pub name: Option<String>,
    pub user: Option<UserView>,
    pub gamer_tag: GamerTag,
}

pub fn on_slot_added(ev: Trigger<OnAdd, BeamSlot>, mut commands: Commands) {
    commands
        .entity(ev.entity())
        .observe(save_user_info)
        .observe(handle_get_token)
        .observe(handle_inventory_get)
        .observe(handle_post_token)
        .observe(handle_get_user_info)
        .observe(handle_get_external_user_info)
        .observe(handle_stats_got);
}

impl BeamSlot {
    pub fn get_gamer_tag(&self) -> GamerTag {
        let value: Option<i64> = self
            .gamer_tag
            .or_else(|| self.user.as_ref().map(|view| view.id));
        GamerTag(value)
    }
}

#[derive(Serialize, Debug, Deserialize, Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component, Default)]
pub struct BeamStats(std::collections::HashMap<String, String>);

pub fn save_user_info(ev: Trigger<UserAuthenticationEvent>, mut commands: Commands) {
    let event = ev.event().deref();
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
        trace!("SAVING TOKEN");
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

            commands.entity(ev.entity()).beam_get_user_info();
        }
        Err(_) => {
            let token = ctx.token.expect("Should have token at this point");
            commands
                .entity(ev.entity())
                .beam_new_user(BeamAuth::RefreshToken(token.refresh_token.clone().unwrap()));
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
    trace!("Inventory update: {:#?}", event);
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

pub fn handle_post_token(ev: Trigger<UserAuthenticationEvent>, mut commands: Commands) {
    let event = ev.event();
    trace!("PostTokenEvent: {:#?}", event);
    match &**event {
        Ok(data) => {
            let new_token = TokenStorage::from_token_response(data);
            commands
                .entity(ev.entity())
                .insert(new_token)
                .trigger(UserLoggedIn::Success);
        }
        Err(beam_autogen_rs::apis::Error::ResponseError(e)) => {
            let status_code = e.status.as_u16();
            let value = e.content.clone();
            let error = UserLoggedIn::Fail { status_code, value };
            commands.trigger_targets(error, ev.entity());
        }
        Err(beam_autogen_rs::apis::Error::Reqwest(e)) => {
            let status_code = e.status().map_or(997, |f| f.as_u16());
            let value = e.to_string();
            let error = UserLoggedIn::Fail { status_code, value };
            commands.trigger_targets(error, ev.entity());
        }
        Err(_) => {
            let error = UserLoggedIn::Fail {
                status_code: 997,
                value: "Unknown error".to_owned(),
            };
            commands.trigger_targets(error, ev.entity());
        }
    }
}

pub fn handle_get_external_user_info(
    ev: Trigger<AttachFederatedIdentityCompletedEvent>,
    mut commands: Commands,
) {
    let event = ev.event();
    trace!("AttachFederatedIdentityCompletedEvent: {:#?}", event);
    match &**event {
        Ok(info) => match info.result.as_str() {
            "ok" => {
                commands.trigger_targets(AttachCredential::Success, ev.entity());
                commands.entity(ev.entity()).beam_get_user_info();
            }
            "challenge" => {
                let error = AttachCredential::Fail {
                    status_code: 997,
                    value: "Challenge, not supported yet on rust SDK side.".into(),
                };
                commands.trigger_targets(error, ev.entity());
            }
            r => {
                let error = AttachCredential::Fail {
                    status_code: 997,
                    value: format!("Unsupported value: {}", r),
                };
                commands.trigger_targets(error, ev.entity());
            }
        },
        Err(beam_autogen_rs::apis::Error::ResponseError(e)) => {
            let error = if e.content.contains("ExternalIdentityUnavailable") {
                AttachCredential::AlreadyInUse
            } else {
                let status_code = e.status.as_u16();
                let value = e.content.clone();
                AttachCredential::Fail { status_code, value }
            };
            commands.trigger_targets(error, ev.entity());
        }
        Err(err) => {
            let error = AttachCredential::Fail {
                status_code: 997,
                value: err.to_string(),
            };
            commands.trigger_targets(error, ev.entity());
        }
    }
}

pub fn handle_stats_got(ev: Trigger<StatsGetEvent>, mut q: Query<BeamableContexts>) {
    let event = ev.event();
    trace!("Stats: {:#?}", event);
    let Ok(event) = &**event else {
        return;
    };

    let Ok(mut ctx) = q.get_mut(ev.entity()) else {
        return;
    };
    for (key, value) in event.stats.iter() {
        ctx.stats.insert(key.to_string(), value.to_string());
    }
}

pub fn handle_get_user_info(
    ev: Trigger<GetAccountMeCompletedEvent>,
    mut q: Query<BeamableContexts>,
    mut commands: Commands,
    #[cfg(feature = "websocket")] config: BeamableConfiguration,
) {
    let event = ev.event();
    let Ok(mut ctx) = q.get_mut(ev.entity()) else {
        return;
    };
    match &**event {
        Ok(e) => {
            ctx.slot.user = Some(e.into());

            #[cfg(feature = "websocket")]
            {
                let t = ctx.token.expect("Failed to get token").clone();
                if let Some(connection) = config.websocket_connection(&t) {
                    commands.entity(ev.entity()).insert_if_new(connection);
                }
            }
            commands
                .entity(ev.entity())
                .trigger(UserInfoUpdated::Success)
                .beam_get_stats(
                    Default::default(),
                    crate::data::stats::StatAccessType::Public,
                )
                .beam_get_stats(
                    Default::default(),
                    crate::data::stats::StatAccessType::Private,
                )
                .beam_get_inventory(None);
        }
        Err(beam_autogen_rs::apis::Error::ResponseError(e)) => {
            let status_code = e.status.as_u16();
            let value = e.content.clone();
            commands
                .entity(ev.entity())
                .trigger(UserInfoUpdated::Fail { status_code, value });
        }
        Err(e) => {
            let status_code = 997;
            let value = e.to_string();
            commands
                .entity(ev.entity())
                .trigger(UserInfoUpdated::Fail { status_code, value });
        }
    }
}
