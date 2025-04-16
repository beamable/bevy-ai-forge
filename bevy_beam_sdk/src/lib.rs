use crate::config::ConfigPlugin;
use crate::slot::BeamSlotPlugin;
use bevy::prelude::*;

pub mod api;
pub mod config;
pub mod content;
#[cfg(feature = "websocket")]
mod inspector;
#[cfg(feature = "websocket")]
pub mod notifications;
pub mod requests;
pub mod slot;
pub mod state;

pub mod data;
#[cfg(feature = "websocket")]
pub mod websocket;

pub mod prelude {
    pub use crate::requests::prelude::*;
    pub use crate::BeamPlugin;
    pub use bevy_beam_sdk_derive::{BeamCommand, BeamNotify};
}

/// A `Plugin` providing the systems and components required to make Beamable SDK work.
pub struct BeamPlugin;

impl Plugin for BeamPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<state::BeamableInitStatus>()
            .add_plugins(ConfigPlugin)
            .add_plugins(BeamSlotPlugin);
        #[cfg(feature = "websocket")]
        {
            app.add_plugins(websocket::websocket_plugin);
        }
        #[cfg(feature = "inspector")]
        {
            app.add_plugins(inspector::InspectorPlugin);
        }

        api::register_types(app);
    }
}
