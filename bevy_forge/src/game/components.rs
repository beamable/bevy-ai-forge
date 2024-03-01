use bevy::prelude::*;

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct GameRoot;

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct GameBackground;

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct GameLogoText;

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct RequestText;

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct GameplayObject;

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct CurrencyText(pub String);

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct LoadingIndicator;

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct InventoryContainer;

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct ItemDisplay(pub i64);

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct SellItemButton(pub String);

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct LoginScreenObject;

#[derive(Clone, Debug, Hash, Eq, Component, PartialEq, Reflect)]
pub enum LoginScreenButton {
    PlayAsGuest,
    Login,
}

#[derive(Clone, Debug, Hash, Eq, Component, PartialEq, Reflect)]
pub enum MenuButton {
    StartGame,
}

#[derive(Clone, Debug, Hash, Eq, Component, PartialEq, Reflect)]
pub enum GameplayButton {
    StartForgingSword,
    StartForgingShield,
}

#[derive(Clone, Debug, Component, PartialEq, Reflect)]
pub struct HiddenUiElement(pub Timer);

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct SoundEffectPlayer;
