use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, Reflect, PartialEq)]
pub enum BeamableInitStatus {
    #[default]
    None,
    UpdatingConfiguration,
    FullyInitialized,
}
