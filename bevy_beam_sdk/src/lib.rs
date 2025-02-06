use self::{api::BeamableBasicApi, context::BeamContext};
use bevy::prelude::*;

pub mod api;
pub mod config;
pub mod context;
pub mod notifications;
pub mod requests;
pub mod state;
pub mod utils;

#[cfg(not(target_family = "wasm"))]
pub mod websocket;

/// A `Plugin` providing the bsystems and components required to make Beamable SDK work.
pub struct BeamPlugin;

impl Plugin for BeamPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<config::BeamableConfig>()
            .register_type::<config::BeamExternalIdentityConfig>()
            .register_type::<context::BeamContext>()
            .register_type::<context::BeamInventory>()
            .init_state::<state::BeamableInitStatus>()
            .add_plugins(crate::requests::RequestsPlugin)
            .add_systems(
                Update,
                context::save_user_info
                    .run_if(in_state(state::BeamableInitStatus::WaitingForCredentials)),
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
                .run_if(resource_added::<requests::ReqwestClient>),
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
                Update,
                (
                    context::handle_accounts_callbacks,
                    context::handle_token_callbacks,
                    context::update_user_info,
                    config::update_config,
                ),
            );
        #[cfg(not(target_family = "wasm"))]
        app.add_systems(
            OnEnter(state::BeamableInitStatus::WebsocketConnection),
            |mut cmd: Commands, context: Res<BeamContext>, config: Res<config::BeamableConfig>| {
                if let Some(token) = &context.token {
                    if let Some(access) = &token.access_token {
                        cmd.spawn(websocket::WebSocketConnection {
                            uri: config.get_websocket_uri(),
                            scope: config.get_x_beam_scope(),
                            token: access.clone(),
                            ..Default::default()
                        });
                    }
                }
            },
        );
        app.add_plugins(websocket::websocket_plugin);

        api::register_types(app);
    }
}
