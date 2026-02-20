use bevy::app::Plugin;

pub mod widgets;

/// Ui plugin
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(widgets::WidgetsPlugin);
    }
}
