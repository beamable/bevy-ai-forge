use std::time::Duration;

use crate::{
    consts::{self, *},
    game::{
        components::{self, *},
        sound_on_button,
    },
    microservice::{MicroserviceSellSword, MicroserviceStartForging},
};
use beam_microservice::models::SellSwordRequestArgs;
use bevy::prelude::*;
use bevy_beam_sdk::api::BeamableBasicApi;
use bevy_beam_sdk::slot::prelude::{BeamInventory, BeamableContexts, ItemProperty};
use bevy_simple_scroll_view::*;

#[derive(Resource, Reflect, Default)]
pub struct ItemsOnSale(pub Vec<String>);

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(super::MainGameState::Game), setup)
            .add_systems(
                Update,
                (on_inv_changed, update_inventory).run_if(in_state(super::MainGameState::Game)),
            )
            .add_observer(on_currency_text_add)
            .init_resource::<ItemsOnSale>();

        // #[cfg(target_arch = "wasm32")]
        app.add_systems(
            FixedUpdate,
            call_update_inventory.run_if(in_state(super::MainGameState::Game).and(
                bevy::time::common_conditions::on_timer(Duration::from_secs(10)),
            )),
        );
    }
}

// #[cfg(target_arch = "wasm32")]
fn call_update_inventory(ctx: Query<BeamableContexts>, mut cmd: Commands) {
    for ctx in ctx.iter() {
        cmd.entity(ctx.entity).beam_get_inventory(None);
    }
}

fn sell_sword_pressed(
    t: Trigger<Pointer<Released>>,
    q: Query<(&SellItemButton, &ChildOf)>,
    ctx: Query<BeamableContexts>,
    mut cmd: Commands,
    mut on_sale: ResMut<ItemsOnSale>,
) {
    let Ok((sell_item_info, child_of)) = q.get(t.target()) else {
        return;
    };
    let Ok(ctx) = ctx.single() else {
        return;
    };
    cmd.queue(MicroserviceSellSword(
        Some(SellSwordRequestArgs {
            item_id: sell_item_info.0.clone(),
        }),
        ctx.entity,
    ));
    on_sale.0.push(sell_item_info.0.clone());
    if let Ok(mut entity_commands) = cmd.get_entity(child_of.parent()) {
        entity_commands.despawn();
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<GameRoot>>,
    mut game_bg: Query<(&mut ImageNode, &mut Node), With<GameBackground>>,
) {
    let text_style = TextFont {
        font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
        font_size: 50.0,
        ..Default::default()
    };
    let font_color = TextColor(consts::MY_ACCENT_COLOR);

    let Ok(root_entity) = query.single() else {
        return;
    };

    commands.entity(root_entity).with_children(|parent| {
        parent
            .spawn((
                Node {
                    border: UiRect::all(Val::Px(5.0)),
                    position_type: PositionType::Absolute,
                    flex_direction: bevy::ui::FlexDirection::Column,
                    max_height: Val::Vh(90.0),
                    height: Val::Vh(90.0),
                    left: Val::Px(0.0),
                    width: Val::Percent(40.0),
                    margin: UiRect::all(Val::Px(30.)),
                    ..scroll_view_node()
                },
                ScrollView::default(),
                BackgroundColor(FRAME_BG_COLOR),
                BorderColor(BORDER_COLOR),
                GameplayObject,
                Name::new("InventoryRoot"),
            ))
            .with_child((
                ScrollableContent::default(),
                InventoryContainer,
                Node {
                    margin: UiRect::all(Val::Px(10.0)),
                    padding: UiRect::bottom(Val::Px(50.0)),
                    flex_direction: bevy::ui::FlexDirection::ColumnReverse,
                    ..scroll_content_node()
                },
            ));
        parent
            .spawn((
                BackgroundColor(FRAME_BG_COLOR),
                BorderColor(BORDER_COLOR),
                Node {
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::new(Val::Px(20.0), Val::Px(80.0), Val::Px(0.0), Val::Px(5.0)),
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    right: Val::Px(20.0),
                    flex_direction: bevy::ui::FlexDirection::Row,
                    ..default()
                },
                GameplayObject,
                Name::new("CoinsRoot"),
            ))
            .with_children(|coin| {
                coin.spawn((Text::new(""), text_style, font_color))
                    .insert(CurrencyText("currency.coins".to_owned()));
                coin.spawn((
                    ImageNode::new(asset_server.load("gfx/coins.png")),
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(-25.0),
                        bottom: Val::Px(-23.0),
                        max_height: Val::Px(100.0),
                        max_width: Val::Px(100.0),
                        ..default()
                    },
                ));
            });

        parent
            .spawn((
                BackgroundColor(INTERACTIVE_BG_COLOR),
                BorderColor(BORDER_COLOR),
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(50.0),
                    right: Val::Percent(20.0),
                    min_width: Val::Px(300.0),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::px(15.0, 15.0, 10.0, 15.0),
                    border: UiRect::all(Val::Px(4.0)),
                    margin: UiRect::top(Val::Px(30.0)),
                    ..Default::default()
                },
                GameplayObject,
            ))
            .observe(on_start_forging_sword_pressed)
            .with_children(|btn| {
                let text_style = TextFont {
                    font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
                    font_size: 40.0,
                    ..default()
                };
                btn.spawn((
                    Text::new("Forge Sword!"),
                    text_style.clone(),
                    font_color,
                    TextLayout::new_with_justify(JustifyText::Center),
                ));
            });
        parent
            .spawn((
                BackgroundColor(INTERACTIVE_BG_COLOR),
                BorderColor(BORDER_COLOR),
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(150.0),
                    right: Val::Percent(20.0),
                    min_width: Val::Px(300.0),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::px(15.0, 15.0, 10.0, 15.0),
                    border: UiRect::all(Val::Px(4.0)),
                    margin: UiRect::top(Val::Px(30.0)),
                    ..Default::default()
                },
                GameplayObject,
            ))
            .observe(on_start_forging_shield_pressed)
            .with_children(|btn| {
                let text_style = TextFont {
                    font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
                    font_size: 40.0,
                    ..Default::default()
                };
                btn.spawn((
                    Text::new("Forge Shield!"),
                    text_style.clone(),
                    font_color,
                    TextLayout::new_with_justify(JustifyText::Center),
                ));
            });
    });
    let Ok(mut background) = game_bg.single_mut() else {
        return;
    };
    *background.0 = ImageNode::new(asset_server.load("gfx/steampunk_bg_forge.png"));
    background.1.right = Val::Px(0.0);
}

fn on_currency_text_add(
    t: Trigger<OnAdd, CurrencyText>,
    mut total_query: Query<(&mut Text, &CurrencyText)>,
    q: Query<&BeamInventory>,
) {
    let Ok(inv) = q.single() else {
        return;
    };

    let Ok((mut text, currency)) = total_query.get_mut(t.target()) else {
        return;
    };
    let value = inv.currencies.get(&currency.0).unwrap_or(&-1);
    **text = format!("{}", value);
}

fn on_inv_changed(
    q: Query<&BeamInventory, Changed<BeamInventory>>,
    mut total_query: Query<(&mut Text, &CurrencyText)>,
) {
    let Ok(inv) = q.single() else {
        return;
    };

    for (mut text, currency) in total_query.iter_mut() {
        let value = inv.currencies.get(&currency.0).unwrap_or(&-1);
        **text = format!("{}", value);
    }
}

fn on_start_forging_sword_pressed(
    t: Trigger<Pointer<Released>>,
    asset_server: Res<AssetServer>,
    q: Query<BeamableContexts>,
    mut commands: Commands,
) {
    let Ok(ctx) = q.single() else {
        return;
    };
    commands.queue(MicroserviceStartForging(ctx.entity));
    // commands
    //     .entity(ctx.entity)
    //     .beam_add_to_inventory(vec!["items.AiItemContent.AiSword".into()]);
    commands.spawn((
        AudioPlayer::new(asset_server.load("sfx/blacksmith.ogg".to_owned())),
        PlaybackSettings::DESPAWN,
        SoundEffectPlayer,
    ));
    commands
        .entity(t.target())
        .insert(components::HiddenUiElement(Timer::new(
            Duration::from_secs(3),
            TimerMode::Once,
        )));
}

fn on_start_forging_shield_pressed(
    t: Trigger<Pointer<Released>>,
    asset_server: Res<AssetServer>,
    q: Query<BeamableContexts>,
    mut commands: Commands,
) {
    let Ok(ctx) = q.single() else {
        return;
    };
    commands
        .entity(ctx.entity)
        .beam_add_to_inventory(vec!["items.AiItemContent.AiShield".into()]);
    commands.spawn((
        AudioPlayer::new(asset_server.load("sfx/blacksmith.ogg".to_owned())),
        PlaybackSettings::DESPAWN,
        SoundEffectPlayer,
    ));
    commands
        .entity(t.target())
        .insert(components::HiddenUiElement(Timer::new(
            Duration::from_secs(3),
            TimerMode::Once,
        )));
}

fn update_inventory(
    asset_server: Res<AssetServer>,
    inv_container_q: Query<Entity, With<InventoryContainer>>,
    inv_items_q: Query<(Entity, &ItemDisplay), With<ItemDisplay>>,
    q: Query<BeamableContexts>,
    on_sale: Res<ItemsOnSale>,
    mut commands: Commands,
) {
    let Ok(ctx) = q.single() else {
        println!("NO CONTEXT");
        return;
    };
    let Ok(container_entity) = inv_container_q.single() else {
        println!("NO CONTAINER");
        return;
    };
    let mut items = Vec::new();
    if let Some(swords) = ctx.inventory.items.get("items.AiItemContent.AiSword") {
        let mut it = swords.clone();
        for item in it.iter_mut() {
            item.properties.push(ItemProperty {
                name: "type".to_owned(),
                value: "sword".to_owned(),
            });
            if !item.properties.iter().any(|e| e.name == "name") {
                item.properties.push(ItemProperty {
                    name: "name".to_owned(),
                    value: "Sword".to_owned(),
                });
            }
        }
        items.append(&mut it);
    };
    if let Some(shields) = ctx.inventory.items.get("items.AiItemContent.AiShield") {
        let mut it = shields.clone();
        for item in it.iter_mut() {
            item.properties.push(ItemProperty {
                name: "type".to_owned(),
                value: "shield".to_owned(),
            });
            if !item.properties.iter().any(|e| e.name == "name") {
                item.properties.push(ItemProperty {
                    name: "name".to_owned(),
                    value: "Shield".to_owned(),
                });
            }
        }
        items.append(&mut it);
    }
    for (e, item) in inv_items_q.iter() {
        let find = items.iter().position(|i| i.id == item.0);
        if let Some(index) = find {
            items.remove(index);
        } else {
            commands.entity(e).despawn();
        }
    }
    let binding = ItemProperty {
        name: "type".to_owned(),
        value: "sword".to_owned(),
    };
    for item in items {
        commands
            .entity(container_entity)
            .with_children(|inventory| {
                let Some(proxy_id) = item.proxy_id else {
                    return;
                };
                if on_sale.0.iter().any(|id| id.eq(&proxy_id)) {
                    return;
                }

                let Some(name) = item.properties.iter().find_map(|i| {
                    if &i.name == "name" {
                        Some(i.value.clone())
                    } else {
                        None
                    }
                }) else {
                    return;
                };
                let item_type = item
                    .properties
                    .iter()
                    .find(|i| &i.name == "type")
                    .unwrap_or(&binding);
                inventory
                    .spawn((
                        Node {
                            border: UiRect::all(Val::Px(5.0)),
                            margin: UiRect::all(Val::Px(5.0)),
                            padding: UiRect::all(Val::Px(10.0)),
                            flex_direction: FlexDirection::Row,
                            row_gap: Val::Px(5.0),
                            align_self: AlignSelf::Stretch,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(BORDER_COLOR),
                        BorderColor(BORDER_COLOR),
                        Name::new(name.clone()),
                        ItemDisplay(item.id),
                    ))
                    .with_children(|inv_root| {
                        use bevy::image::{ImageFormat, ImageFormatSetting, ImageLoaderSettings};
                        if let Some(image_url) =
                            item.properties.iter().find(|i| &i.name == "imageUrl")
                        {
                            inv_root.spawn((
                                ImageNode {
                                    image: asset_server.load_with_settings(
                                        image_url.value.clone(),
                                        |settings: &mut ImageLoaderSettings| {
                                            settings.format =
                                                ImageFormatSetting::Format(ImageFormat::Jpeg);
                                        },
                                    ),
                                    // image: asset_server.load(image_url.value.clone()),
                                    ..Default::default()
                                },
                                Name::new(image_url.value.clone()),
                                Node {
                                    width: Val::Px(128.0),
                                    height: Val::Px(128.0),
                                    ..Default::default()
                                },
                            ));
                        }

                        inv_root
                            .spawn((Node {
                                margin: UiRect::all(Val::Px(5.0)),
                                flex_direction: FlexDirection::Column,
                                column_gap: Val::Px(10.0),
                                align_self: AlignSelf::Stretch,
                                align_items: AlignItems::Center,
                                ..default()
                            },))
                            .with_children(|container| {
                                let text_style = TextFont {
                                    font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
                                    font_size: 40.0,
                                    ..default()
                                };
                                let text_color = TextColor(consts::MY_ACCENT_COLOR);
                                container.spawn((
                                    Text::new(format!("({}) {}", &item_type.value, &name)),
                                    text_style.clone(),
                                    text_color,
                                    TextLayout::new_with_justify(JustifyText::Center),
                                ));
                                if let Some(description) =
                                    item.properties.iter().find(|i| &i.name == "description")
                                {
                                    let text_style = text_style.clone().with_font_size(20.0);
                                    container.spawn((
                                        Text::new(&description.value),
                                        text_style.clone(),
                                        text_color,
                                        TextLayout::new_with_justify(JustifyText::Center),
                                    ));
                                }
                                if let Some(price) =
                                    item.properties.iter().find(|i| &i.name == "price")
                                {
                                    container
                                        .spawn((
                                            BackgroundColor(INTERACTIVE_BG_COLOR),
                                            BorderColor(BORDER_COLOR),
                                            Button,
                                            Node {
                                                padding: UiRect::px(15.0, 15.0, 10.0, 15.0),
                                                border: UiRect::all(Val::Px(4.0)),
                                                ..Default::default()
                                            },
                                            SellItemButton(proxy_id.clone()),
                                        ))
                                        .observe(sell_sword_pressed)
                                        .with_children(|btn| {
                                            btn.spawn((
                                                Text::new(format!("Sell it for {}", &price.value)),
                                                text_style.clone(),
                                                text_color,
                                                TextLayout::new_with_justify(JustifyText::Center),
                                            ));
                                        })
                                        .observe(sound_on_button);
                                };
                            });
                    });
            });
    }
}
