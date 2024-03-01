use crate::consts::{BORDER_COLOR, INTERACTIVE_BG_COLOR};
use crate::utils::despawn_recursive_by_component;
use crate::{consts, game::components::*};
use bevy::prelude::*;
use bevy_button_released_plugin::ButtonReleasedEvent;

pub struct MenuStatePlugin;

impl Plugin for MenuStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(super::MainGameState::Menu), setup)
            .add_systems(
                Update,
                handle_buttons.run_if(in_state(super::MainGameState::Menu)),
            )
            .add_systems(
                OnExit(super::MainGameState::Menu),
                (
                    despawn_recursive_by_component::<MenuButton>,
                    despawn_recursive_by_component::<GameLogoText>,
                ),
            );
    }
}

fn handle_buttons(
    mut reader: EventReader<ButtonReleasedEvent>,
    q: Query<&MenuButton>,
    mut next_state: ResMut<NextState<super::MainGameState>>,
    mut cmd: Commands,
) {
    for event in reader.read() {
        let Ok(button) = q.get(**event) else {
            continue;
        };
        match button {
            MenuButton::StartGame => {
                next_state.set(super::MainGameState::Game);
                cmd.entity(**event).remove::<Interaction>();
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<GameRoot>>,
) {
    let Ok(root_entity) = query.get_single() else {
        return;
    };
    commands.entity(root_entity).with_children(|parent| {
        let text_style = TextStyle {
            font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
            font_size: 40.0,
            color: consts::MY_ACCENT_COLOR,
        };
        parent
            .spawn((
                ButtonBundle {
                    background_color: INTERACTIVE_BG_COLOR.into(),
                    border_color: BORDER_COLOR.into(),
                    style: Style {
                        padding: UiRect::px(15.0, 15.0, 10.0, 15.0),
                        border: UiRect::all(Val::Px(4.0)),
                        margin: UiRect::top(Val::Px(30.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                MenuButton::StartGame,
            ))
            .with_children(|btn| {
                btn.spawn(
                    TextBundle::from_section("Start Game", text_style.clone())
                        .with_text_justify(JustifyText::Center),
                );
            });
    });
}
