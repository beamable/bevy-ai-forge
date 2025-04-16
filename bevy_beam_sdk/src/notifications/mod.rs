pub mod inventory;
pub mod stats;

use crate::slot::prelude::BeamableContexts;
use bevy::prelude::*;
use bevy_eventwork::{AppNetworkMessage, NetworkMessage};

pub trait BeamNotification: NetworkMessage {
    fn handle(
        ev: EventReader<bevy_eventwork::NetworkData<Self>>,
        q: Query<BeamableContexts>,
        commands: Commands,
    );
}

pub(crate) fn plugin(app: &mut App) {
    app.add_beam_event_handler::<inventory::InventoryRefreshNotify>();
    app.add_beam_event_handler::<stats::StatsRefreshNotify>();
}

pub trait BeamNotifcationAppHelper {
    fn add_beam_event_handler<T: BeamNotification>(&mut self) -> &mut Self;
}

impl BeamNotifcationAppHelper for App {
    fn add_beam_event_handler<T: BeamNotification>(&mut self) -> &mut Self {
        self.listen_for_message::<T, bevy_eventwork::websockets::WebSocketProvider>()
            .add_systems(FixedUpdate, T::handle)
    }
}
