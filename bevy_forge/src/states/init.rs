use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{consts, game::components::GameLogoText, utils::GameArgs};

pub struct InitStatePlugin;

impl Plugin for InitStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(super::MainGameState::Init), setup)
            .add_systems(
                OnEnter(bevy_beam_sdk::state::BeamableInitStatus::FullyInitialized),
                (|mut next_state: ResMut<NextState<super::MainGameState>>| {
                    next_state.set(super::MainGameState::LoginScreen);
                })
                .run_if(in_state(super::MainGameState::Init)),
            );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_args: Res<GameArgs>,
    mut storage: ResMut<PkvStore>,
) {
    if game_args.clear_profile.is_some() && game_args.clear_profile.unwrap() {
        info!("Removing profile");
        storage.clear().unwrap();
    }
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Name::new("UI Root"),
            crate::game::components::GameRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(66.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    BackgroundColor(Color::WHITE),
                    ZIndex(-1),
                    ImageNode::new(asset_server.load("gfx/steampunk_bg_robot.png")),
                ))
                .insert(crate::game::components::GameBackground);
            parent
                .spawn((
                    Text::new("Artificial Forge"),
                    TextLayout::new_with_justify(JustifyText::Center),
                    TextFont {
                        font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
                        font_size: 150.0,
                        ..default()
                    },
                    TextColor(consts::MY_ACCENT_COLOR),
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        ..default()
                    },
                ))
                .insert(Name::new("GameLogo"))
                .insert(GameLogoText);
        });
}
