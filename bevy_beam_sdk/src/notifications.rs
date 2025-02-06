use crate::api::BeamableBasicApi;
use crate::context::BeamContext;
use beam_autogen_rs::models::InventoryQueryRequest;
use bevy::prelude::*;
use bevy_eventwork::{AppNetworkMessage, NetworkData, NetworkMessage};
use bevy_eventwork_mod_websockets::WebSocketProvider;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

pub(crate) fn plugin(app: &mut App) {
    app.listen_for_message::<InventoryRefreshNotify, WebSocketProvider>();
    app.add_systems(
        Update,
        handle_inventory_refresh_notify.run_if(resource_exists::<BeamContext>),
    );
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InventoryRefreshNotify {
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

impl NetworkMessage for InventoryRefreshNotify {
    const NAME: &'static str = "inventory.refresh";

    fn deserialize_message<T>(bytes: &[u8]) -> Option<T>  where T : NetworkMessage {
        serde_json::from_slice::<T>(bytes).ok()
    }
}

fn handle_inventory_refresh_notify(
    mut ev: EventReader<NetworkData<InventoryRefreshNotify>>,
    context: Res<BeamContext>,
    mut commands: Commands,
) {
    for e in ev.read() {
        // warn!("GOT MESSAGE: {e:?}");
        let scope = e.scopes.as_ref().map(|array| array.join(","));
        let gamer_tag = context.get_gamer_tag().unwrap().to_string();
        commands.beam_get_inventory(scope, gamer_tag);
    }
}
