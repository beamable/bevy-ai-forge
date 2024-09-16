use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, Reflect, PartialEq)]
pub enum BeamableInitStatus {
    #[default]
    None,
    // TODO add health endpoint
    // CheckInternet,
    WaitingForCredentials,
    LoggedIn,
    WebsocketConnection,
    FullyInitialized,
}
