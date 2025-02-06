use std::time::Duration;

use beam_microservice::models::SellSwordRequestArgs;
use bevy::prelude::*;
use bevy_beam_sdk::{
    api::BeamableBasicApi,
    context::{BeamContext, BeamInventory, ItemProperty},
};
use bevy_simple_scroll_view::*;

use crate::{
    consts::{self, *},
    game::{
        components::{self, *},
        sound_on_button,
    },
    microservice::MicroserviceSellSword,
};

#[derive(Resource, Reflect, Default)]
pub struct ItemsOnSale(pub Vec<String>);

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(super::MainGameState::Game), setup)
            .add_systems(
                Update,
                (
                    add_currency_text,
                    update_currency_text,
                    update_inventory,
                    update_inventory_on_sell,
                )
                    .chain()
                    .run_if(in_state(super::MainGameState::Game)),
            )
            .init_resource::<ItemsOnSale>();
    }
}

fn update_inventory_on_sell(
    sword_sell_event: EventReader<crate::microservice::SellSwordEventCompleted>,
    mut cmd: Commands,
    ctx: Res<BeamContext>,
) {
    if !sword_sell_event.is_empty() {
        cmd.beam_get_inventory(
            Some("currency.coins,items.AiItemContent".to_owned()),
            ctx.get_gamer_tag().unwrap().to_string(),
        );
    }
}

fn sell_sword_pressed(
    t: Trigger<Pointer<Up>>,
    q: Query<(&SellItemButton, &Parent)>,
    mut cmd: Commands,
    mut on_sale: ResMut<ItemsOnSale>,
) {
    let Ok((sell_item_info, parent)) = q.get(t.entity()) else {
        return;
    };
    cmd.queue(MicroserviceSellSword {
        data: Some(SellSwordRequestArgs {
            item_id: sell_item_info.0.clone(),
        }),
        entity: None,
    });
    on_sale.0.push(sell_item_info.0.clone());
    if let Some(entity_commands) = cmd.get_entity(parent.get()) {
        entity_commands.try_despawn_recursive();
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

    let Ok(root_entity) = query.get_single() else {
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
    let Ok(mut background) = game_bg.get_single_mut() else {
        return;
    };
    *background.0 = ImageNode::new(asset_server.load("gfx/steampunk_bg_forge.png"));
    background.1.right = Val::Px(0.0);
}

fn update_currency_text(
    mut total_query: Query<(&mut Text, &CurrencyText)>,
    inv: Res<BeamInventory>,
) {
    if inv.is_changed() {
        for (mut text, currency) in total_query.iter_mut() {
            let value = inv.currencies.get(&currency.0).unwrap_or(&-1);
            **text = format!("{}", value);
        }
    }
}

fn add_currency_text(
    mut added_query: Query<(&mut Text, &CurrencyText), Added<CurrencyText>>,
    inv: Res<BeamInventory>,
) {
    for (mut text, currency) in added_query.iter_mut() {
        let value = inv.currencies.get(&currency.0).unwrap_or(&-1);
        **text = format!("{}", value);
    }
}

fn on_start_forging_sword_pressed(
    t: Trigger<Pointer<Up>>,
    ctx: Res<BeamContext>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.beam_add_to_inventory(
        vec!["items.AiItemContent.AiSword".into()],
        ctx.get_gamer_tag().unwrap().to_string(),
    );
    commands.spawn((
        AudioPlayer::new(asset_server.load("sfx/blacksmith.ogg".to_owned())),
        PlaybackSettings::DESPAWN,
        SoundEffectPlayer,
    ));
    commands
        .entity(t.entity())
        .insert(components::HiddenUiElement(Timer::new(
            Duration::from_secs(3),
            TimerMode::Once,
        )));
}

fn on_start_forging_shield_pressed(
    t: Trigger<Pointer<Up>>,
    ctx: Res<BeamContext>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.beam_add_to_inventory(
        vec!["items.AiItemContent.AiShield".into()],
        ctx.get_gamer_tag().unwrap().to_string(),
    );
    commands.spawn((
        AudioPlayer::new(asset_server.load("sfx/blacksmith.ogg".to_owned())),
        PlaybackSettings::DESPAWN,
        SoundEffectPlayer,
    ));
    commands
        .entity(t.entity())
        .insert(components::HiddenUiElement(Timer::new(
            Duration::from_secs(3),
            TimerMode::Once,
        )));
}

fn update_inventory(
    inv: Res<BeamInventory>,
    asset_server: Res<AssetServer>,
    inv_container_q: Query<Entity, With<InventoryContainer>>,
    inv_items_q: Query<(Entity, &ItemDisplay), With<ItemDisplay>>,
    mut commands: Commands,
    on_sale: Res<ItemsOnSale>,
) {
    let Ok(container_entity) = inv_container_q.get_single() else {
        return;
    };
    let mut items = Vec::new();
    if let Some(swords) = inv.items.get("items.AiItemContent.AiSword") {
        let mut it = swords.clone();
        for item in it.iter_mut() {
            item.properties.push(ItemProperty {
                name: "type".to_owned(),
                value: "sword".to_owned(),
            })
        }
        items.append(&mut it);
    };
    if let Some(shields) = inv.items.get("items.AiItemContent.AiShield") {
        let mut it = shields.clone();
        for item in it.iter_mut() {
            item.properties.push(ItemProperty {
                name: "type".to_owned(),
                value: "shield".to_owned(),
            })
        }
        items.append(&mut it);
    }
    for (e, item) in inv_items_q.iter() {
        let find = items.iter().position(|i| i.id == item.0);
        if let Some(index) = find {
            items.remove(index);
        } else {
            commands.entity(e).despawn_recursive();
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

                let Some(name) = item.properties.iter().find(|i| &i.name == "name") else {
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
                            flex_direction: FlexDirection::Column,
                            column_gap: Val::Px(10.0),
                            align_self: AlignSelf::Stretch,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(BORDER_COLOR),
                        BorderColor(BORDER_COLOR),
                        ItemDisplay(item.id),
                        Name::new(name.value.clone()),
                    ))
                    .with_children(|container| {
                        let text_style = TextFont {
                            font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
                            font_size: 40.0,
                            ..default()
                        };
                        let text_color = TextColor(consts::MY_ACCENT_COLOR);
                        container.spawn((
                            Text::new(format!("({}) {}", &item_type.value, &name.value)),
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
                        let Some(price) = item.properties.iter().find(|i| &i.name == "price")
                        else {
                            return;
                        };
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
                    });
            });
    }
}
