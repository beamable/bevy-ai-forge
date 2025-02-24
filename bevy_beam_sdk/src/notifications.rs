use crate::api::BeamableBasicApi;
use crate::slot::prelude::BeamableContexts;
use bevy::prelude::*;
use bevy_eventwork::websockets::WebSocketProvider;
use bevy_eventwork::{AppNetworkMessage, NetworkMessage};
use serde::{Deserialize, Serialize};

pub(crate) fn plugin(app: &mut App) {
    app.listen_for_message::<InventoryRefreshNotify, WebSocketProvider>();
    app.add_systems(Update, handle_inventory_refresh_notify);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InventoryRefreshNotify {
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
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

fn handle_inventory_refresh_notify(
    mut ev: EventReader<bevy_eventwork::NetworkData<InventoryRefreshNotify>>,
    q: Query<BeamableContexts>,
    mut commands: Commands,
) {
    for e in ev.read() {
        warn!("GOT MESSAGE: {e:?}");
        let scope = e.scopes.as_ref().map(|array| array.join(","));
        for ctx in q.iter() {
            let gamer_tag = ctx.slot.get_gamer_tag().unwrap().to_string();
            commands
                .entity(ctx.entity)
                .beam_get_inventory(scope.clone(), gamer_tag);
        }
    }
}
