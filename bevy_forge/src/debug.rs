use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::StateInspectorPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::consts::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, git_info)
            .add_plugins(
                WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F1)),
            )
            .add_plugins(
                StateInspectorPlugin::<bevy_beam_sdk::state::BeamableInitStatus>::default()
                    .run_if(input_toggle_active(false, KeyCode::F1)),
            )
            .add_plugins(
                StateInspectorPlugin::<crate::states::MainGameState>::default()
                    .run_if(input_toggle_active(false, KeyCode::F1)),
            );
    }
}

fn git_info(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    beam_config: Res<bevy_beam_sdk::config::BeamableConfig>,
) {
    commands
        .spawn((
            Text::new(format!(
                "Made with Bevy and Beamable\ngit:{} ({})\nCID.PID:{}.{}",
                GIT_HASH, GIT_DATE, beam_config.cid, beam_config.pid
            )),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 16.0,
                ..default()
            },
            TextColor(MY_ACCENT_COLOR),
            TextLayout::new_with_justify(JustifyText::Right),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(15.0),
                right: Val::Px(15.0),
                ..default()
            },
        ))
        .insert(Name::new("Debug Info UI"));
}
