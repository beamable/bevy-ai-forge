use crate::api::BeamableBasicApi;
use crate::slot::prelude::BeamableContexts;
use bevy::prelude::*;
use bevy_beam_sdk_derive::BeamNotify;
use bevy_eventwork::NetworkMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, BeamNotify)]
#[beam_notify("inventory.refresh")]
pub struct InventoryRefreshNotify {
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

impl super::BeamNotification for InventoryRefreshNotify {
    fn handle(
        mut ev: EventReader<bevy_eventwork::NetworkData<InventoryRefreshNotify>>,
        q: Query<BeamableContexts>,
        mut commands: Commands,
    ) {
        for e in ev.read() {
            let scope = e.scopes.as_ref().map(|array| array.join(","));
            for ctx in q.iter() {
                commands
                    .entity(ctx.entity)
                    .beam_get_inventory(scope.clone());
            }
        }
    }
}
