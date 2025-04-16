use crate::slot::prelude::BeamableContexts;
use bevy::prelude::*;
use bevy_beam_sdk_derive::BeamNotify;
use bevy_eventwork::NetworkMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, BeamNotify)]
#[beam_notify("stats.refresh")]
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

impl super::BeamNotification for StatsRefreshNotify {
    fn handle(
        mut ev: EventReader<bevy_eventwork::NetworkData<StatsRefreshNotify>>,
        mut q: Query<BeamableContexts>,
        mut _commands: Commands,
    ) {
        for e in ev.read() {
            let source = Some(*e.source());
            for mut ctx in q
                .iter_mut()
                .filter(|c| c.connection.is_some_and(|cc| cc.id.eq(&source)))
            {
                if let Some(added) = &e.stat_changes.added {
                    for stat in added.iter() {
                        ctx.stats.insert(stat.0.clone(), stat.1.clone());
                    }
                }
                if let Some(modified) = &e.stat_changes.modified {
                    for stat in modified.iter() {
                        ctx.stats.insert(stat.0.clone(), stat.1.clone());
                    }
                }
                if let Some(deleted) = &e.stat_changes.deleted {
                    for stat in deleted.iter() {
                        ctx.stats.remove(&stat.0);
                    }
                }
            }
        }
    }
}
