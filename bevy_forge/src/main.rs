use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_args::BevyArgsPlugin;
use bevy_beam_sdk::{config::BeamableConfig, BeamPlugin};
use debug::DebugPlugin;
use utils::GameArgs;

pub mod consts;
pub mod debug;
pub mod game;
pub mod microservice;
pub mod states;
pub mod utils;

fn main() {
    let config = match serde_json::from_str::<bevy_beam_sdk::config::BeamableConfig>(
        crate::consts::CONFIG_DEFAULTS,
    ) {
        Ok(config) => config,
        Err(_) => BeamableConfig {
            host: Default::default(),
            cid: Default::default(),
            pid: Default::default(),
            websocket_url: None,
        },
    };
    App::new()
        .insert_resource(ClearColor(consts::MY_BG_COLOR))
        .insert_resource(bevy_beam_sdk::config::BeamExternalIdentityConfig {
            provider_service: "ForgeService".to_string(),
            provider_namespace: "OpenAI".to_string(),
        })
        .insert_resource(config)
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: bevy::asset::AssetMetaCheck::Never,
            ..default()
        }).set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            filter: "debug,wgpu_core=warn,wgpu_hal=warn,mygame=debug,cosmic_text=warn,naga=warn,bevy_eventwork=trace".into(),
            ..default()
        }))
        .add_plugins(BevyArgsPlugin::<GameArgs>::default())
        .insert_resource(bevy_pkv::PkvStore::new("Beamable", "AiForged"))
        // Never attempts to look up meta files. The default meta configuration will be used for each asset.
        .add_plugins((
            DebugPlugin,
            BeamPlugin,
            bevy_simple_scroll_view::ScrollViewPlugin,
            microservice::MicroservicePlugin,
            game::GamePlugin,
            states::GameStatesPlugin,
        ))
        .run();
}
