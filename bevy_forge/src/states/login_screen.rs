use crate::{
    consts::{self, BORDER_COLOR, FRAME_BG_COLOR, INTERACTIVE_BG_COLOR},
    game::components::{GameRoot, LoadingIndicator, LoginScreenButton, LoginScreenObject},
    utils::despawn_recursive_by_component,
};
use bevy::prelude::*;
use bevy_beam_sdk::{api::BeamableBasicApi, context::BeamContext};
use bevy_button_released_plugin::ButtonReleasedEvent;
use bevy_simple_text_input::{TextInputSettings, TextInputValue};

pub struct LoginScreenStatePlugin;

impl Plugin for LoginScreenStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(super::MainGameState::LoginScreen), setup)
            .add_systems(
                Update,
                (handle_buttons, rotate).run_if(in_state(super::MainGameState::LoginScreen)),
            )
            .add_systems(
                OnExit(super::MainGameState::LoginScreen),
                despawn_recursive_by_component::<LoginScreenObject>,
            )
            .add_systems(
                OnEnter(bevy_beam_sdk::state::BeamableInitStatus::FullyInitialized),
                (|mut next_state: ResMut<NextState<super::MainGameState>>| {
                    next_state.set(super::MainGameState::Menu);
                })
                .run_if(in_state(super::MainGameState::LoginScreen)),
            );
    }
}

fn handle_buttons(
    mut reader: EventReader<ButtonReleasedEvent>,
    mut cmd: Commands,
    q: Query<&LoginScreenButton>,
    text: Query<&TextInputValue, Changed<TextInputValue>>,
    mut beam: ResMut<BeamContext>,
    mut q1: Query<(
        &mut Node,
        Option<&LoginScreenObject>,
        Option<&LoadingIndicator>,
    )>,
) {
    if let Ok(text) = text.get_single() {
        if !text.0.is_empty() {
            beam.name = Some(text.0.clone());
        }
    }
    for event in reader.read() {
        let Ok(button) = q.get(**event) else {
            continue;
        };
        match button {
            LoginScreenButton::PlayAsGuest => {
                // let mut new_user =
                //     beam_autogen_rs::models::TokenRequestWrapper::new("guest".to_string());
                // new_user.username = beam.name.clone();
                // cmd.beam_new_user(new_user);
                cmd.beam_play_as_guest(beam.name.clone());
                cmd.entity(**event).remove::<Interaction>();
                for (mut s, login, loading) in q1.iter_mut() {
                    if loading.is_some() {
                        s.display = Display::Flex;
                    } else if login.is_some() {
                        s.display = Display::None;
                    }
                }
            }
            LoginScreenButton::Login => todo!(),
        }
    }
}

fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<LoadingIndicator>>) {
    let Ok(mut loading) = query.get_single_mut() else {
        return;
    };
    loading.rotate(Quat::from_rotation_z(time.delta_secs() * 1.0));
    loading.scale = Vec3::splat(time.elapsed_secs().sin().abs() * 0.2 + 0.45);
}

fn setup(
    beam: Res<BeamContext>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<GameRoot>>,
) {
    let Ok(root_entity) = query.get_single() else {
        return;
    };
    let show_register_form = beam.token.is_none();

    commands.entity(root_entity).with_children(|parent| {
        parent.spawn((
            ImageNode::new(asset_server.load("gfx/gameIconTransparent.png")),
            Node {
                display: if show_register_form {
                    Display::None
                } else {
                    Display::Flex
                },
                position_type: PositionType::Absolute,
                bottom: Val::Px(50.0),
                ..default()
            },
            LoadingIndicator,
            LoginScreenObject,
        ));
        if !show_register_form {
            return;
        }
        parent
            .spawn((
                Node {
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(25.0)),
                    flex_direction: bevy::ui::FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(FRAME_BG_COLOR),
                BorderColor(BORDER_COLOR),
                LoginScreenObject,
            ))
            .with_children(|parent| {
                let text_style = TextFont {
                    font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
                    font_size: 40.0,
                    ..default()
                };
                let text_color = TextColor(consts::MY_ACCENT_COLOR);
                parent.spawn((
                    Text::new("User Name"),
                    TextLayout::new_with_justify(JustifyText::Center),
                    text_style.clone(),
                    text_color,
                    LoginScreenObject,
                ));
                parent.spawn((
                    Node {
                        width: Val::Px(300.0),
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        margin: UiRect::top(Val::Px(30.0)),
                        ..default()
                    },
                    BackgroundColor(INTERACTIVE_BG_COLOR),
                    BorderColor(BORDER_COLOR),
                    bevy_simple_text_input::TextInput,
                    bevy_simple_text_input::TextInputTextFont(text_style.clone()),
                    bevy_simple_text_input::TextInputTextColor(text_color),
                    bevy_simple_text_input::TextInputPlaceholder {
                        value: "NewUser".to_string(),
                        ..default()
                    },
                    TextInputSettings {
                        retain_on_submit: true,
                        mask_character: None,
                    },
                    LoginScreenObject,
                ));
                parent
                    .spawn((
                        Button,
                        Node {
                            padding: UiRect::px(15.0, 15.0, 10.0, 15.0),
                            border: UiRect::all(Val::Px(4.0)),
                            margin: UiRect::top(Val::Px(30.0)),
                            ..Default::default()
                        },
                        BackgroundColor(INTERACTIVE_BG_COLOR),
                        BorderColor(BORDER_COLOR),
                        LoginScreenButton::PlayAsGuest,
                        LoginScreenObject,
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("Play as guest"),
                            TextLayout::new_with_justify(JustifyText::Center),
                            text_style.clone(),
                            text_color,
                        ));
                    });
            });
    });
}
