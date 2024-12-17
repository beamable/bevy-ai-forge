use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

pub mod components;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<components::GameRoot>()
            .register_type::<components::GameBackground>()
            .register_type::<components::LoginScreenObject>()
            .register_type::<components::LoginScreenButton>()
            .register_type::<components::MenuButton>()
            .register_type::<components::GameplayObject>()
            .register_type::<components::GameplayButton>()
            .register_type::<components::ItemDisplay>()
            .register_type::<components::SellItemButton>()
            .register_type::<components::InventoryContainer>()
            .register_type::<components::CurrencyText>()
            .register_type::<components::GameLogoText>()
            .register_type::<components::SoundEffectPlayer>()
            .register_type::<components::LoadingIndicator>()
            .register_type::<components::RequestText>()
            .add_systems(Update, (hide_items, show_items, timer_update));
    }
}

fn hide_items(mut q: Query<&mut Node, With<components::HiddenUiElement>>) {
    for mut s in q.iter_mut() {
        s.display = Display::None;
    }
}

fn timer_update(
    mut query_timer: Query<(Entity, &mut components::HiddenUiElement)>,
    time: Res<Time>,
    mut cmd: Commands,
) {
    for (e, mut timer) in query_timer.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            cmd.entity(e).remove::<components::HiddenUiElement>();
        }
    }
}

fn show_items(
    mut removed: RemovedComponents<components::HiddenUiElement>,
    mut query: Query<&mut Node>,
) {
    for entity in removed.read() {
        if let Ok(mut style) = query.get_mut(entity) {
            style.display = Display::Flex;
        }
    }
}

pub fn sound_on_button(
    trigger: Trigger<bevy_button_released_plugin::OnButtonReleased>,
    forge_button: Query<Entity, With<components::GameplayButton>>,
    asset_server: Res<AssetServer>,
    other_sounds: Query<Entity, With<components::SoundEffectPlayer>>,
    mut cmd: Commands,
) {
    for e in other_sounds.iter() {
        cmd.entity(e).despawn();
    }
    if let Ok(button_entity) = forge_button.get(trigger.entity()) {
        cmd.spawn((
            AudioPlayer::new(asset_server.load("sfx/blacksmith.ogg".to_owned())),
            PlaybackSettings::DESPAWN,
            components::SoundEffectPlayer,
        ));
        cmd.entity(button_entity)
            .insert(components::HiddenUiElement(Timer::new(
                Duration::from_secs(3),
                TimerMode::Once,
            )));
    } else {
        let mut rng = rand::thread_rng();
        let number = rng.gen::<u32>();
        cmd.spawn((
            AudioPlayer::new(asset_server.load(format!(
                "sfx/interface_{}.ogg",
                number.checked_rem_euclid(3).unwrap() + 1
            ))),
            PlaybackSettings::DESPAWN,
            components::SoundEffectPlayer,
        ));
    }
}
