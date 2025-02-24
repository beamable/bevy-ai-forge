use crate::slot::prelude::*;
use bevy::prelude::*;
use bevy_pkv::PkvStore;

mod beam_slot;
mod contexts;
mod inventory;
mod token_storage;
mod user_view;

pub mod prelude {
    pub use crate::slot::beam_slot::{BeamSlot, BeamStats, GamerTag, UserLoggedIn};
    pub use crate::slot::contexts::{BeamableConfiguration, BeamableContexts};
    pub use crate::slot::inventory::{BeamInventory, Item, ItemProperty};
    pub use crate::slot::token_storage::TokenStorage;
    pub use crate::slot::user_view::{ExternalIdentity, UserView};
    pub mod functions {
        pub use crate::slot::beam_slot::{
            handle_get_external_user_info, handle_get_token, handle_get_user_info,
            handle_inventory_get, handle_post_token, handle_stats_got, save_token, save_user_info,
        };
    }
}

pub struct BeamSlotPlugin;

impl Plugin for BeamSlotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            beam_slot::save_token.run_if(
                resource_exists::<PkvStore>
                    .and(in_state(crate::state::BeamableInitStatus::FullyInitialized)),
            ),
        )
        .add_event::<UserLoggedIn>()
        .add_systems(
            OnEnter(crate::state::BeamableInitStatus::FullyInitialized),
            beam_slot::try_read_token.run_if(resource_exists::<PkvStore>),
        )
        .enable_state_scoped_entities::<crate::state::BeamableInitStatus>()
        .register_type::<BeamInventory>()
        .register_type::<BeamStats>()
        .register_type::<BeamSlot>()
        .register_type::<TokenStorage>();
    }
}
