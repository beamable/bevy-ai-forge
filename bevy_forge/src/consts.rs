use bevy::color::Color;

pub const GIT_HASH: &str = env!("GIT_HASH");
pub const GIT_DATE: &str = env!("GIT_DATE");
pub const MY_ACCENT_COLOR: Color = Color::srgb(0.569, 0.694, 0.62);
pub const MY_BG_COLOR: Color = Color::srgb(27.0 / 255.0, 26.0 / 255.0, 31.0 / 255.0);
pub const FRAME_BG_COLOR: Color = Color::srgb(53_f32 / 255.0, 45_f32 / 255.0, 55_f32 / 255.0);
pub const BORDER_COLOR: Color = Color::srgb(39_f32 / 255.0, 17_f32 / 255.0, 40_f32 / 255.0);
pub const INTERACTIVE_BG_COLOR: Color = Color::srgb(62.0 / 255.0, 32.0 / 255.0, 64.0 / 255.0);

pub const CONFIG_DEFAULTS: &str = include_str!("../../.beamable/connection-configuration.json");
