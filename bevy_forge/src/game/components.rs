use bevy::prelude::*;

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct GameRoot;

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct GameBackground;

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
#[require(DespawnOnExit::<crate::states::MainGameState>(crate::states::MainGameState::Menu))]
pub struct GameLogoText;

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct RequestText;

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
#[require(DespawnOnExit::<crate::states::MainGameState>(crate::states::MainGameState::Game))]
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

#[derive(Debug, Reflect, Component, Default, Clone, Deref)]
#[reflect(Component)]
pub struct SellItemButton(pub i64);

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
#[require(DespawnOnExit::<crate::states::MainGameState>(crate::states::MainGameState::LoginScreen))]
pub struct LoginScreenObject;

#[derive(Clone, Debug, Component, PartialEq, Reflect)]
pub struct HiddenUiElement(pub Timer);

#[derive(Debug, Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct SoundEffectPlayer;
