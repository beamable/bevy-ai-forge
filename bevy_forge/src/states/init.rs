use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{consts, game::components::GameLogoText, utils::GameArgs};

pub struct InitStatePlugin;

impl Plugin for InitStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(super::MainGameState::Init), setup)
            .add_systems(
                OnEnter(bevy_beam_sdk::state::BeamableInitStatus::WaitingForCredentials),
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
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Name::new("UI Root"),
            crate::game::components::GameRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: Style {
                        width: Val::Percent(66.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    z_index: ZIndex::Global(-1),
                    image: UiImage::new(asset_server.load("gfx/steampunk_bg_robot.png")),
                    ..default()
                })
                .insert(crate::game::components::GameBackground);
            parent
                .spawn(
                    TextBundle::from_section(
                        "Artificial Forge",
                        TextStyle {
                            font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
                            font_size: 150.0,
                            color: consts::MY_ACCENT_COLOR,
                        },
                    )
                    .with_text_justify(JustifyText::Center)
                    .with_style(Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        ..default()
                    }),
                )
                .insert(Name::new("GameLogo"))
                .insert(GameLogoText);
        });
}
