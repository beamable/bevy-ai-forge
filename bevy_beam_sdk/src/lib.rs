use bevy::prelude::*;
use config::BeamableConfig;
use websocket::WebSocketConnection;

use self::{api::BeamableBasicApi, context::BeamContext};

pub mod api;
pub mod config;
pub mod context;
pub mod notifications;
pub mod requests;
pub mod state;
pub mod utils;
pub mod websocket;

/// A `Plugin` providing the bsystems and components required to make Beamable SDK work.
pub struct BeamPlugin;

impl Plugin for BeamPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<config::BeamableConfig>()
            .register_type::<config::BeamExternalIdentityConfig>()
            .register_type::<context::BeamContext>()
            .register_type::<context::BeamInventory>()
            .add_event::<notifications::Notification>()
            .init_state::<state::BeamableInitStatus>()
            .add_plugins(crate::requests::RequestsPlugin)
            .add_systems(
                Update,
                context::save_user_info
                    .run_if(in_state(state::BeamableInitStatus::WaitingForCredentials)),
            )
            .add_systems(
                Update,
                notifications::notification_handle::<notifications::InventoryRefreshNotify>
                    .run_if(resource_exists::<context::BeamContext>),
            )
            .add_systems(
                Update,
                context::handle_inventory_get.run_if(resource_exists::<context::BeamInventory>),
            )
            .add_systems(
                Update,
                (|mut cmd: Commands| {
                    cmd.beam_basic_get_realm_config();
                })
                .run_if(resource_added::<config::BeamableConfig>),
            )
            .add_systems(
                OnEnter(state::BeamableInitStatus::WaitingForCredentials),
                context::read_context,
            )
            .add_systems(
                OnEnter(state::BeamableInitStatus::LoggedIn),
                |mut cmd: Commands, context: Res<BeamContext>| {
                    if let Some(token) = &context.token {
                        if let Some(access) = &token.access_token {
                            cmd.beam_get_token(access.clone());
                        }
                    }
                },
            )
            .add_systems(
                OnEnter(state::BeamableInitStatus::WebsocketConnection),
                |mut cmd: Commands, context: Res<BeamContext>, config: Res<BeamableConfig>| {
                    if let Some(token) = &context.token {
                        if let Some(access) = &token.access_token {
                            cmd.spawn(WebSocketConnection {
                                uri: config.get_websocket_uri(),
                                socket: None,
                                scope: config.get_x_beam_scope(),
                                token: access.clone(),
                            });
                        }
                    }
                },
            )
            .add_systems(
                Update,
                (
                    websocket::on_create,
                    websocket::task_handle,
                    websocket::messages_task_handle,
                    context::handle_accounts_callbacks,
                    context::handle_token_callbacks,
                    context::update_user_info,
                    config::update_config,
                ),
            );
        api::register_types(app);
    }
}
