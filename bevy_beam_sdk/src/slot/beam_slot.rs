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

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component, Default)]
pub(crate) struct BeamSessionHydration {
    completed: bool,
}

#[derive(EntityEvent, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct UserLoggedIn {
    pub entity: Entity,
    pub result: UserLoggedInResult,
}

#[derive(Debug, Clone, Reflect, PartialEq, Eq)]
pub enum UserLoggedInResult {
    Success,
    Fail { status_code: u16, value: String },
}

#[derive(EntityEvent, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct UserInfoUpdated {
    pub entity: Entity,
    pub result: UserInfoUpdatedResult,
}

#[derive(Debug, Clone, Reflect, PartialEq, Eq)]
pub enum UserInfoUpdatedResult {
    Success,
    Fail { status_code: u16, value: String },
}

#[derive(EntityEvent, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct AttachCredential {
    pub entity: Entity,
    pub result: AttachCredentialResult,
}

#[derive(Debug, Clone, Reflect, PartialEq, Eq)]
pub enum AttachCredentialResult {
    Success,
    AlreadyInUse,
    Fail { status_code: u16, value: String },
}

#[derive(Serialize, Debug, Deserialize, Reflect, Default, Deref)]
pub struct GamerTag(Option<i64>);

#[derive(Serialize, Debug, Deserialize, Component, Reflect, Default)]
#[reflect(Component, Default)]
#[require(BeamInventory, BeamInventorySyncState, BeamSessionHydration, Name = Name::new("BeamSlot"), BeamStats, DespawnOnExit::<crate::state::BeamableInitStatus>(crate::state::BeamableInitStatus::FullyInitialized))]
pub struct BeamSlot {
    pub name: Option<String>,
    pub user: Option<UserView>,
    pub gamer_tag: GamerTag,
}

pub fn on_slot_added(ev: On<Add, BeamSlot>, mut commands: Commands) {
    commands
        .entity(ev.event_target())
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

pub fn save_user_info(ev: On<UserAuthenticationEvent>, mut commands: Commands) {
    let event = ev.event().deref();
    match event {
        Ok(token) => {
            let access_token = token.access_token.clone().unwrap_or_default();
            let token = TokenStorage::from_token_response(token);
            commands
                .entity(ev.event_target())
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

pub fn try_read_token(
    pkv: Res<PkvStore>,
    mut config: BeamableConfiguration,
    existing: Query<(), With<BeamSlot>>,
) {
    if !existing.is_empty() {
        return;
    }
    let Ok(token) = pkv.get::<TokenStorage>("user_token") else {
        return;
    };
    let Ok(slot) = pkv.get::<BeamSlot>("user_slot") else {
        return;
    };
    config.context_from(slot, token);
}

pub fn handle_get_token(
    ev: On<GetTokenEvent>,
    mut commands: Commands,
    mut q: Query<BeamableContexts>,
) {
    let event = ev.event().deref();
    let Ok(mut ctx) = q.get_mut(ev.event_target()) else {
        return;
    };
    match &event {
        Ok(data) => {
            ctx.token
                .expect("Should have token at this point")
                .access_token = Some(data.token.clone());
            ctx.slot.gamer_tag = GamerTag(data.gamer_tag);

            commands.entity(ev.event_target()).beam_get_user_info();
        }
        Err(_) => {
            let token = ctx.token.expect("Should have token at this point");
            commands
                .entity(ev.event_target())
                .beam_new_user(BeamAuth::RefreshToken(token.refresh_token.clone().unwrap()));
        }
    }
}
pub fn handle_inventory_get(
    ev: On<InventoryGetCompletedEvent>,
    mut q: Query<BeamableContexts>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let event = ev.event().deref();
    let Ok(mut ctx) = q.get_mut(ev.event_target()) else {
        return;
    };
    let scopes = ctx.inventory_sync.active_scopes.clone();
    if let Ok(event) = event {
        let inventory = BeamInventory::from((*event).clone());
        ctx.inventory.apply_snapshot(inventory, scopes.as_deref());
        ctx.inventory_sync.revision += 1;
        ctx.inventory_sync.last_error = None;
        debug!(
            "inventory request completed in {:.0}ms (revision {}, {} scopes)",
            (time.elapsed_secs_f64() - ctx.inventory_sync.request_started_seconds) * 1000.0,
            ctx.inventory_sync.revision,
            scopes.as_ref().map_or(0, Vec::len)
        );
        commands.trigger(BeamInventoryUpdated {
            entity: ctx.entity,
            scopes: scopes.clone().unwrap_or_default(),
            revision: ctx.inventory_sync.revision,
        });
    } else {
        ctx.inventory_sync.last_error = Some("inventory refresh failed".to_owned());
        warn!("inventory request failed; cached inventory was preserved");
    }
    if let Some(scope) = ctx.inventory_sync.finish() {
        commands.entity(ctx.entity).beam_get_inventory(scope);
    }
}

pub fn handle_post_token(ev: On<UserAuthenticationEvent>, mut commands: Commands) {
    let event = ev.event();
    trace!("authentication request completed");
    let entity = ev.event_target();
    match &**event {
        Ok(data) => {
            let new_token = TokenStorage::from_token_response(data);
            commands.entity(entity).insert(new_token);
            commands.trigger(UserLoggedIn {
                entity,
                result: UserLoggedInResult::Success,
            });
        }
        Err(beam_autogen_rs::apis::Error::ResponseError(e)) => {
            let status_code = e.status.as_u16();
            let value = e.content.clone();
            commands.trigger(UserLoggedIn {
                entity,
                result: UserLoggedInResult::Fail { status_code, value },
            });
        }
        Err(beam_autogen_rs::apis::Error::Reqwest(e)) => {
            let status_code = e.status().map_or(997, |f| f.as_u16());
            let value = e.to_string();
            commands.trigger(UserLoggedIn {
                entity,
                result: UserLoggedInResult::Fail { status_code, value },
            });
        }
        Err(_) => {
            commands.trigger(UserLoggedIn {
                entity,
                result: UserLoggedInResult::Fail {
                    status_code: 997,
                    value: "Unknown error".to_owned(),
                },
            });
        }
    }
}

pub fn handle_get_external_user_info(
    ev: On<AttachFederatedIdentityCompletedEvent>,
    mut commands: Commands,
) {
    let event = ev.event();
    trace!("AttachFederatedIdentityCompletedEvent: {:#?}", event);
    let entity = ev.event_target();
    match &**event {
        Ok(info) => match info.result.as_str() {
            "ok" => {
                commands.trigger(AttachCredential {
                    entity,
                    result: AttachCredentialResult::Success,
                });
                commands.entity(entity).beam_get_user_info();
            }
            "challenge" => {
                commands.trigger(AttachCredential {
                    entity,
                    result: AttachCredentialResult::Fail {
                        status_code: 997,
                        value: "Challenge, not supported yet on rust SDK side.".into(),
                    },
                });
            }
            r => {
                commands.trigger(AttachCredential {
                    entity,
                    result: AttachCredentialResult::Fail {
                        status_code: 997,
                        value: format!("Unsupported value: {}", r),
                    },
                });
            }
        },
        Err(beam_autogen_rs::apis::Error::ResponseError(e)) => {
            let result = if e.content.contains("ExternalIdentityUnavailable") {
                AttachCredentialResult::AlreadyInUse
            } else {
                let status_code = e.status.as_u16();
                let value = e.content.clone();
                AttachCredentialResult::Fail { status_code, value }
            };
            commands.trigger(AttachCredential { entity, result });
        }
        Err(err) => {
            commands.trigger(AttachCredential {
                entity,
                result: AttachCredentialResult::Fail {
                    status_code: 997,
                    value: err.to_string(),
                },
            });
        }
    }
}

pub fn handle_stats_got(ev: On<StatsGetEvent>, mut q: Query<BeamableContexts>) {
    let event = ev.event();
    trace!("Stats: {:#?}", event);
    let Ok(event) = &**event else {
        return;
    };

    let Ok(mut ctx) = q.get_mut(ev.event_target()) else {
        return;
    };
    for (key, value) in event.stats.iter() {
        ctx.stats.insert(key.to_string(), value.to_string());
    }
}

pub fn handle_get_user_info(
    ev: On<GetAccountMeCompletedEvent>,
    mut q: Query<BeamableContexts>,
    mut commands: Commands,
    #[cfg(feature = "websocket")] config: BeamableConfiguration,
) {
    let event = ev.event();
    let entity = ev.event_target();
    let Ok(mut ctx) = q.get_mut(entity) else {
        return;
    };
    match &**event {
        Ok(e) => {
            ctx.slot.user = Some(e.into());

            #[cfg(feature = "websocket")]
            {
                let t = ctx.token.expect("Failed to get token").clone();
                if let Some(connection) = config.websocket_connection(&t) {
                    commands.entity(entity).insert_if_new(connection);
                }
            }
            commands.trigger(UserInfoUpdated {
                entity,
                result: UserInfoUpdatedResult::Success,
            });
            if !ctx.hydration.completed {
                ctx.hydration.completed = true;
                commands
                    .entity(entity)
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
        }
        Err(beam_autogen_rs::apis::Error::ResponseError(e)) => {
            let status_code = e.status.as_u16();
            let value = e.content.clone();
            commands.trigger(UserInfoUpdated {
                entity,
                result: UserInfoUpdatedResult::Fail { status_code, value },
            });
        }
        Err(e) => {
            let status_code = 997;
            let value = e.to_string();
            commands.trigger(UserInfoUpdated {
                entity,
                result: UserInfoUpdatedResult::Fail { status_code, value },
            });
        }
    }
}
