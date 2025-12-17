use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_args::BevyArgsPlugin;
use bevy_beam_sdk::{config::BeamableConfigResource, BeamPlugin};
use debug::DebugPlugin;
use utils::GameArgs;

pub mod consts;
pub mod debug;
pub mod game;
pub mod microservice;
pub mod states;
pub mod utils;

fn main() {
    let config = match serde_json::from_str::<BeamableConfigResource>(consts::CONFIG_DEFAULTS) {
        Ok(config) => config,
        Err(_) => BeamableConfigResource {
            host: Default::default(),
            cid: Default::default(),
            pid: Default::default(),
        },
    };
    App::new()
        .insert_resource(ClearColor(consts::MY_BG_COLOR))
        .insert_resource(bevy_beam_sdk::config::BeamExternalIdentityConfig {
            provider_service: "ForgeService".to_string(),
            provider_namespace: "OpenAI".to_string(),
        })
        .insert_resource(config)
        .add_plugins(bevy_ehttp::prelude::HttpPlugin)
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            meta_check: bevy::asset::AssetMetaCheck::Never,
            ..default()
        }).set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            filter: "warn,wgpu_core=warn,wgpu_hal=warn,mygame=debug,bevy_eventwork=trace,bevy_beam_sdk=trace".into(),
            ..default()
        }))
        .add_plugins((BevyArgsPlugin::<GameArgs>::default(), BeamPlugin))
        .insert_resource(bevy_pkv::PkvStore::new("Beamable", "Ai_Forge"))
        .add_plugins((
            DebugPlugin,
            bevy_simple_scroll_view::ScrollViewPlugin,
            microservice::MicroservicePlugin,
            game::GamePlugin,
            states::GameStatesPlugin,
        ))
        .run();
}
