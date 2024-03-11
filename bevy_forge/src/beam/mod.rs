use crate::requests::RequestsPlugin;
use bevy::prelude::*;

use self::{api::BeamableBasicApi, context::BeamContext};

pub mod api;
pub mod config;
pub mod context;
pub mod state;
pub mod utils;

pub struct BeamPlugin;

impl Plugin for BeamPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<config::Config>()
            .register_type::<config::Config>()
            .register_type::<config::BeamExternalIdentityConfig>()
            .register_type::<context::BeamContext>()
            .register_type::<context::BeamInventory>()
            .init_state::<state::BeamableInitStatus>()
            .add_plugins(RequestsPlugin)
            .add_systems(
                Update,
                context::save_user_info
                    .run_if(in_state(state::BeamableInitStatus::WaitingForCredentials)),
            )
            .add_systems(
                Update,
                (|mut next_state: ResMut<NextState<state::BeamableInitStatus>>| {
                    next_state.set(state::BeamableInitStatus::WaitingForCredentials);
                })
                .run_if(resource_added::<config::Config>),
            )
            .add_systems(
                OnEnter(state::BeamableInitStatus::WaitingForCredentials),
                context::read_context,
            )
            .add_systems(
                OnEnter(state::BeamableInitStatus::LoggedIn),
                |mut cmd: Commands, context: Res<BeamContext>| {
                    if let Some(token) = &context.token {
                        if let Some(refresh) = &token.access_token {
                            cmd.beam_get_token(refresh.clone());
                        }
                    }
                },
            )
            .add_systems(
                Update,
                (
                    context::handle_accounts_callbacks,
                    context::handle_inventory_get,
                    context::update_user_info,
                ),
            );
        api::register_types(app);
    }
}
