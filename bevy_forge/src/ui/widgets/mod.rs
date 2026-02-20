use bevy::app::Plugin;

pub mod scroll_view;
pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(scroll_view::ScrollViewPlugin);
    }
}
