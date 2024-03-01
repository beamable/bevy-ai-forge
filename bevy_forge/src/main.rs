use beam::BeamPlugin;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_args::BevyArgsPlugin;
use debug::DebugPlugin;
use utils::GameArgs;

pub mod beam;
pub mod consts;
pub mod debug;
pub mod game;
pub mod microservice;
pub mod requests;
pub mod states;
pub mod utils;

fn main() {
    App::new()
        .insert_resource(ClearColor(consts::MY_BG_COLOR))
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(beam::config::BeamExternalIdentityConfig {
            provider_service: "ForgeService".to_string(),
            provider_namespace: "OpenAI".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyArgsPlugin::<GameArgs>::default())
        .insert_resource(bevy_pkv::PkvStore::new("Beamable", "AiForge"))
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
