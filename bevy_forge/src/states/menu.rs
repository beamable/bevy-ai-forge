use crate::consts::{BORDER_COLOR, INTERACTIVE_BG_COLOR};
use crate::{consts, game::components::*};
use bevy::prelude::*;

pub struct MenuStatePlugin;

impl Plugin for MenuStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(super::MainGameState::Menu), setup);
    }
}

fn on_start_game_pressed(
    t: On<Pointer<Release>>,
    mut next_state: ResMut<NextState<super::MainGameState>>,
    mut cmd: Commands,
) {
    (*next_state).set_if_neq(super::MainGameState::Game);
    cmd.entity(t.event().event_target()).remove::<Interaction>();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<GameRoot>>,
) {
    let Ok(root_entity) = query.single() else {
        return;
    };
    commands.entity(root_entity).with_children(|parent| {
        let text_style = TextFont {
            font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
            font_size: 40.0,
            ..default()
        };
        let text_color = TextColor(consts::MY_ACCENT_COLOR);

        parent
            .spawn((
                BackgroundColor(INTERACTIVE_BG_COLOR),
                BorderColor::all(BORDER_COLOR),
                Button,
                Node {
                    padding: UiRect::px(15.0, 15.0, 10.0, 15.0),
                    border: UiRect::all(Val::Px(4.0)),
                    margin: UiRect::top(Val::Px(30.0)),
                    ..Default::default()
                },
                DespawnOnExit(super::MainGameState::Menu),
            ))
            .observe(on_start_game_pressed)
            .with_children(|btn| {
                btn.spawn((
                    Text::new("Start Game"),
                    TextLayout::new_with_justify(Justify::Center),
                    text_style,
                    text_color,
                ));
            });
    });
}
