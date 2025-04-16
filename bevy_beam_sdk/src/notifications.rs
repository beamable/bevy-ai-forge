use std::ops::DerefMut;

use crate::api::BeamableBasicApi;
use crate::slot::prelude::BeamableContexts;
use bevy::prelude::*;
use bevy_eventwork::websockets::WebSocketProvider;
use bevy_eventwork::{AppNetworkMessage, NetworkMessage};
use serde::{Deserialize, Serialize};

pub(crate) fn plugin(app: &mut App) {
    app.add_beam_event_handler::<InventoryRefreshNotify>();
    app.add_beam_event_handler::<StatsRefreshNotify>();
}

pub trait BeamNotifcationAppHelper {
    fn add_beam_event_handler<T: BeamNotification>(&mut self) -> &mut Self;
}

impl BeamNotifcationAppHelper for App {
    fn add_beam_event_handler<T: BeamNotification>(&mut self) -> &mut Self {
        self.listen_for_message::<T, WebSocketProvider>()
            .add_systems(FixedUpdate, T::handle_refresh)
    }
}

pub trait BeamNotification: NetworkMessage {
    fn handle_refresh(
        ev: EventReader<bevy_eventwork::NetworkData<Self>>,
        q: Query<BeamableContexts>,
        commands: Commands,
    );
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InventoryRefreshNotify {
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

impl BeamNotification for InventoryRefreshNotify {
    fn handle_refresh(
        mut ev: EventReader<bevy_eventwork::NetworkData<InventoryRefreshNotify>>,
        q: Query<BeamableContexts>,
        mut commands: Commands,
    ) {
        for e in ev.read() {
            warn!("InventoryRefreshNotify: {e:?}");
            let scope = e.scopes.as_ref().map(|array| array.join(","));
            for ctx in q.iter() {
                commands
                    .entity(ctx.entity)
                    .beam_get_inventory(scope.clone());
            }
        }
    }
}

impl NetworkMessage for InventoryRefreshNotify {
    const NAME: &'static str = "inventory.refresh";

    fn deserialize_message<T>(bytes: &[u8]) -> Option<T>
    where
        T: NetworkMessage,
    {
        serde_json::from_slice::<T>(bytes).ok()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatsRefreshNotify {
    #[serde(rename = "gamerTag")]
    gamer_tag: u64,
    namespace: String,
    #[serde(rename = "statChanges")]
    pub stat_changes: StatChanges,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StatChanges {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<(String, String)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Vec<(String, String)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<Vec<(String, String)>>,
}

impl NetworkMessage for StatsRefreshNotify {
    const NAME: &'static str = "stats.refresh";

    fn deserialize_message<T>(bytes: &[u8]) -> Option<T>
    where
        T: NetworkMessage,
    {
        serde_json::from_slice::<T>(bytes).ok()
    }
}

impl BeamNotification for StatsRefreshNotify {
    fn handle_refresh(
        mut ev: EventReader<bevy_eventwork::NetworkData<StatsRefreshNotify>>,
        mut q: Query<BeamableContexts>,
        mut _commands: Commands,
    ) {
        for e in ev.read() {
            warn!("StatsRefreshNotify: {:?}", &e);
            let source = Some(*e.source());
            for mut ctx in q
                .iter_mut()
                .filter(|c| c.connection.is_some_and(|cc| cc.id.eq(&source)))
            {
                if let Some(added) = &e.stat_changes.added {
                    for stat in added.iter() {
                        ctx.stats.deref_mut().insert(stat.0.clone(), stat.1.clone());
                    }
                }
                if let Some(modified) = &e.stat_changes.modified {
                    for stat in modified.iter() {
                        ctx.stats.deref_mut().insert(stat.0.clone(), stat.1.clone());
                    }
                }
                if let Some(deleted) = &e.stat_changes.deleted {
                    for stat in deleted.iter() {
                        ctx.stats.deref_mut().remove(&stat.0);
                    }
                }
            }
        }
    }
}
