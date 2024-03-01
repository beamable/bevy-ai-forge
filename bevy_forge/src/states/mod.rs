use bevy::prelude::*;
use bevy_button_released_plugin::*;

pub mod game;
pub mod init;
pub mod login_screen;
pub mod menu;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq, Reflect)]
pub enum MainGameState {
    #[default]
    Init,
    LoginScreen,
    Menu,
    Game,
}

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            game::GameStatePlugin,
            init::InitStatePlugin,
            login_screen::LoginScreenStatePlugin,
            menu::MenuStatePlugin,
            ButtonsReleasedPlugin,
            bevy_simple_text_input::TextInputPlugin,
        ))
        .init_state::<MainGameState>();
    }
}
