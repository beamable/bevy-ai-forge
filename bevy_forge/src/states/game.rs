use std::{
    collections::{HashSet, VecDeque},
    time::Duration,
};

use crate::ui::widgets::scroll_view::*;
use crate::{
    consts::{self, *},
    game::{
        components::{self, *},
        sound_on_button,
    },
    microservice::{
        MicroserviceSellSword, MicroserviceStartForging, MicroserviceStartForgingShield,
        SellSwordEventCompleted, StartForgingEventCompleted, StartForgingShieldEventCompleted,
    },
};
use beam_microservice::models::SellSwordRequestArgs;
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;
use bevy_beam_sdk::api::BeamableBasicApi;
use bevy_beam_sdk::{
    slot::prelude::{BeamInventory, BeamInventoryUpdated, BeamableContexts, ItemProperty},
    websocket::BeamConnectionStatus,
};

#[derive(Resource, Default)]
pub struct PendingSales {
    ids: HashSet<i64>,
    order: VecDeque<i64>,
}

#[derive(Resource, Default)]
struct ForgeActivity {
    requests_in_flight: usize,
    processing_count: usize,
    processing_started_seconds: Option<f64>,
    last_error: Option<String>,
}

#[derive(Resource, Default)]
struct FallbackRefresh {
    elapsed_seconds: f32,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(super::MainGameState::Game), setup)
            .add_systems(
                Update,
                (
                    on_inv_changed,
                    update_inventory,
                    update_busy_text,
                    health_based_refresh,
                )
                    .run_if(in_state(super::MainGameState::Game)),
            )
            .add_systems(
                Update,
                update_items_visibility.run_if(resource_changed::<PendingSales>),
            )
            .add_observer(on_currency_text_add)
            .add_observer(on_forge_sword_completed)
            .add_observer(on_forge_shield_completed)
            .add_observer(on_sale_completed)
            .add_observer(on_inventory_authoritative)
            .init_resource::<PendingSales>()
            .init_resource::<ForgeActivity>()
            .init_resource::<FallbackRefresh>();
    }
}

fn sell_sword_pressed(
    t: On<Pointer<Release>>,
    q: Query<&SellItemButton>,
    ctx: Query<BeamableContexts>,
    mut cmd: Commands,
    mut pending_sales: ResMut<PendingSales>,
) {
    let Ok(sell_item_info) = q.get(t.event().event_target()) else {
        return;
    };
    let Ok(ctx) = ctx.single() else {
        return;
    };
    if pending_sales.ids.contains(&**sell_item_info) {
        return;
    }
    cmd.queue(MicroserviceSellSword(
        Some(SellSwordRequestArgs {
            item_id: sell_item_info.to_string(),
        }),
        ctx.entity,
    ));
    pending_sales.ids.insert(**sell_item_info);
    pending_sales.order.push_back(**sell_item_info);
}

fn update_items_visibility(
    pending_sales: Res<PendingSales>,
    mut q: Query<(&mut Node, &ItemDisplay)>,
) {
    for (mut n, item_display) in q.iter_mut() {
        let visible = !pending_sales.ids.contains(&item_display.0);
        n.display = if visible {
            Display::Flex
        } else {
            Display::None
        };
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<GameRoot>>,
    mut game_bg: Query<(&mut ImageNode, &mut Node), With<GameBackground>>,
) {
    let text_style = TextFont {
        font: asset_server
            .load("fonts/coolvetica_condensed_rg.otf")
            .into(),
        font_size: FontSize::Px(50.0),
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
                BorderColor::all(BORDER_COLOR),
                GameplayObject,
                Name::new("InventoryRoot"),
            ))
            .with_child((ScrollableContent::default(), InventoryContainer));
        parent
            .spawn((
                BackgroundColor(FRAME_BG_COLOR),
                BorderColor::all(BORDER_COLOR),
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
                coin.spawn((Text::new(""), text_style.clone(), font_color))
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
                BorderColor::all(BORDER_COLOR),
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
                SwordForgeButton,
            ))
            .observe(on_start_forging_sword_pressed)
            .with_children(|btn| {
                let text_style = TextFont {
                    font: asset_server
                        .load("fonts/coolvetica_condensed_rg.otf")
                        .into(),
                    font_size: FontSize::Px(40.0),
                    ..default()
                };
                btn.spawn((
                    Text::new("Forge Sword!"),
                    text_style.clone(),
                    font_color,
                    TextLayout::justify(Justify::Center),
                ));
            });
        parent
            .spawn((
                BackgroundColor(INTERACTIVE_BG_COLOR),
                BorderColor::all(BORDER_COLOR),
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
                ShieldForgeButton,
            ))
            .observe(on_start_forging_shield_pressed)
            .with_children(|btn| {
                let text_style = TextFont {
                    font: asset_server
                        .load("fonts/coolvetica_condensed_rg.otf")
                        .into(),
                    font_size: FontSize::Px(40.0),
                    ..Default::default()
                };
                btn.spawn((
                    Text::new("Forge Shield!"),
                    text_style.clone(),
                    font_color,
                    TextLayout::justify(Justify::Center),
                ));
            });
        parent.spawn((
            Text::new("Forge ready"),
            text_style.clone().with_font_size(28.0),
            font_color,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(255.0),
                right: Val::Percent(15.0),
                ..default()
            },
            GameplayObject,
            ForgeBusyText,
        ));
    });
    let Ok(mut background) = game_bg.single_mut() else {
        return;
    };
    *background.0 = ImageNode::new(asset_server.load("gfx/steampunk_bg_forge.png"));
    background.1.right = Val::Px(0.0);
}

fn on_currency_text_add(
    t: On<Add, CurrencyText>,
    mut total_query: Query<(&mut Text, &CurrencyText)>,
    q: Query<&BeamInventory>,
) {
    let Ok(inv) = q.single() else {
        return;
    };

    let Ok((mut text, currency)) = total_query.get_mut(t.event().event_target()) else {
        return;
    };
    let value = inv.currencies.get(&currency.0).unwrap_or(&-1);
    **text = format!("{}", value);
}

fn on_inv_changed(
    q: Query<&BeamInventory, Changed<BeamInventory>>,
    mut total_query: Query<(&mut Text, &CurrencyText)>,
    mut activity: ResMut<ForgeActivity>,
    time: Res<Time>,
) {
    let Ok(inv) = q.single() else {
        return;
    };

    for (mut text, currency) in total_query.iter_mut() {
        let value = inv.currencies.get(&currency.0).unwrap_or(&-1);
        **text = format!("{}", value);
    }
    let processing_count = inv
        .items
        .values()
        .flatten()
        .filter(|item| {
            item.properties
                .iter()
                .any(|property| property.name == "forgeStatus" && property.value == "processing")
        })
        .count();
    if processing_count > 0 && activity.processing_count == 0 {
        activity.processing_started_seconds = Some(time.elapsed_secs_f64());
    } else if processing_count == 0 {
        activity.processing_started_seconds = None;
    }
    activity.processing_count = processing_count;
}

fn on_start_forging_sword_pressed(
    t: On<Pointer<Release>>,
    asset_server: Res<AssetServer>,
    q: Query<BeamableContexts>,
    pending: Query<(), With<components::ForgeRequestPending>>,
    mut commands: Commands,
    mut activity: ResMut<ForgeActivity>,
) {
    let Ok(ctx) = q.single() else {
        return;
    };
    if pending.get(t.event().event_target()).is_ok() {
        return;
    }
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
        .entity(t.event().event_target())
        .insert(ForgeRequestPending);
    activity.requests_in_flight += 1;
    activity.last_error = None;
}

fn on_start_forging_shield_pressed(
    t: On<Pointer<Release>>,
    asset_server: Res<AssetServer>,
    q: Query<BeamableContexts>,
    pending: Query<(), With<components::ForgeRequestPending>>,
    mut commands: Commands,
    mut activity: ResMut<ForgeActivity>,
) {
    let Ok(ctx) = q.single() else {
        return;
    };
    if pending.get(t.event().event_target()).is_ok() {
        return;
    }
    commands.queue(MicroserviceStartForgingShield(ctx.entity));
    // commands
    //     .entity(ctx.entity)
    //     .beam_add_to_inventory(vec!["items.AiItemContent.AiShield".into()]);
    commands.spawn((
        AudioPlayer::new(asset_server.load("sfx/blacksmith.ogg".to_owned())),
        PlaybackSettings::DESPAWN,
        SoundEffectPlayer,
    ));
    commands
        .entity(t.event().event_target())
        .insert(ForgeRequestPending);
    activity.requests_in_flight += 1;
    activity.last_error = None;
}

fn on_forge_sword_completed(
    event: On<StartForgingEventCompleted>,
    buttons: Query<Entity, With<SwordForgeButton>>,
    mut activity: ResMut<ForgeActivity>,
    mut commands: Commands,
) {
    finish_forge_request(&event, &buttons, &mut activity, &mut commands, "sword");
}

fn on_forge_shield_completed(
    event: On<StartForgingShieldEventCompleted>,
    buttons: Query<Entity, With<ShieldForgeButton>>,
    mut activity: ResMut<ForgeActivity>,
    mut commands: Commands,
) {
    finish_forge_request(&event, &buttons, &mut activity, &mut commands, "shield");
}

fn finish_forge_request<E>(
    event: &On<E>,
    buttons: &Query<Entity, impl QueryFilter>,
    activity: &mut ForgeActivity,
    commands: &mut Commands,
    item_type: &str,
) where
    E: EntityEvent
        + std::ops::Deref<
            Target = Result<
                bool,
                beam_microservice::apis::Error<
                    beam_microservice::apis::uncategorized_api::StartForgingSwordPostError,
                >,
            >,
        >,
{
    for button in buttons.iter() {
        commands.entity(button).remove::<ForgeRequestPending>();
    }
    activity.requests_in_flight = activity.requests_in_flight.saturating_sub(1);
    match &**event.event() {
        Ok(true) => {
            activity.last_error = None;
            commands
                .entity(event.event_target())
                .beam_get_inventory(Some("items,currency.coins".to_owned()));
        }
        Ok(false) => {
            activity.last_error = Some(format!(
                "Could not forge {item_type}. Check your coin balance and try again."
            ));
        }
        Err(_) => {
            activity.last_error = Some(format!(
                "The {item_type} request failed. Check the connection and try again."
            ));
        }
    }
}

fn on_sale_completed(
    event: On<SellSwordEventCompleted>,
    mut pending_sales: ResMut<PendingSales>,
    mut activity: ResMut<ForgeActivity>,
    mut commands: Commands,
) {
    let sold_id = pending_sales.order.pop_front();
    match &**event.event() {
        Ok(true) => {
            activity.last_error = None;
            commands
                .entity(event.event_target())
                .beam_get_inventory(Some("items,currency.coins".to_owned()));
        }
        Ok(false) => {
            if let Some(id) = sold_id {
                pending_sales.ids.remove(&id);
            }
            activity.last_error = Some(
                "That item could not be sold. It is visible again; retry after refreshing."
                    .to_owned(),
            );
        }
        Err(_) => {
            if let Some(id) = sold_id {
                pending_sales.ids.remove(&id);
            }
            activity.last_error =
                Some("Sale failed. Check the connection; the item has been restored.".to_owned());
        }
    }
}

fn on_inventory_authoritative(
    event: On<BeamInventoryUpdated>,
    inventory: Query<&BeamInventory>,
    mut pending_sales: ResMut<PendingSales>,
) {
    let Ok(inventory) = inventory.get(event.event().entity) else {
        return;
    };
    pending_sales.ids.retain(|id| {
        inventory
            .items
            .values()
            .flatten()
            .any(|item| item.id == *id)
    });
    let retained = pending_sales.ids.clone();
    pending_sales.order.retain(|id| retained.contains(id));
}

fn update_busy_text(activity: Res<ForgeActivity>, mut text: Query<&mut Text, With<ForgeBusyText>>) {
    if !activity.is_changed() {
        return;
    }
    let message = if let Some(error) = &activity.last_error {
        error.clone()
    } else if activity.requests_in_flight > 0 || activity.processing_count > 0 {
        format!(
            "Requests: {}  |  Forging: {}",
            activity.requests_in_flight, activity.processing_count
        )
    } else {
        "Forge ready".to_owned()
    };
    for mut text in &mut text {
        **text = message.clone();
    }
}

fn health_based_refresh(
    time: Res<Time>,
    contexts: Query<BeamableContexts>,
    pending_sales: Res<PendingSales>,
    activity: Res<ForgeActivity>,
    mut fallback: ResMut<FallbackRefresh>,
    mut commands: Commands,
) {
    let Ok(context) = contexts.single() else {
        return;
    };
    let connected = context
        .connection
        .is_some_and(|connection| connection.status == BeamConnectionStatus::Connected);
    let pending_work = !pending_sales.ids.is_empty()
        || activity.requests_in_flight > 0
        || activity.processing_count > 0;
    let processing_long = activity.processing_started_seconds.is_some_and(|started| {
        time.elapsed_secs_f64() - started >= Duration::from_secs(30).as_secs_f64()
    });
    let interval = if connected {
        processing_long.then_some(30.0)
    } else if pending_work {
        Some(2.0)
    } else {
        Some(30.0)
    };
    let Some(interval) = interval else {
        fallback.elapsed_seconds = 0.0;
        return;
    };
    fallback.elapsed_seconds += time.delta_secs();
    if fallback.elapsed_seconds >= interval {
        fallback.elapsed_seconds = 0.0;
        commands
            .entity(context.entity)
            .beam_get_inventory(Some("items,currency.coins".to_owned()));
    }
}

fn update_inventory(
    asset_server: Res<AssetServer>,
    inv_container_q: Query<Entity, With<InventoryContainer>>,
    inv_items_q: Query<(Entity, &ItemDisplay), With<ItemDisplay>>,
    q: Query<BeamableContexts>,
    pending_sales: Res<PendingSales>,
    mut commands: Commands,
) {
    let ctx = match q.single() {
        Ok(o) => o,
        Err(e) => {
            error!("No context: {}", e.to_string());
            return;
        }
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
    items.retain(|item| {
        !item
            .properties
            .iter()
            .any(|property| property.name == "forgeStatus" && property.value == "processing")
    });
    for (e, item) in inv_items_q.iter() {
        let find = items.iter().position(|i| i.id == item.0);
        // let find = items.iter().position(|i| i.proxy_id.as_ref().is_some_and(|i| i == &item.0));
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
        if item.proxy_id.is_none() {
            continue;
        }
        if pending_sales.ids.contains(&item.id) {
            continue;
        }
        let Some(name) = item.properties.iter().find_map(|i| {
            if &i.name == "name" {
                Some(i.value.clone())
            } else {
                None
            }
        }) else {
            continue;
        };
        commands
            .entity(container_entity)
            .with_children(|inventory| {
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
                        BorderColor::all(BORDER_COLOR),
                        Name::new(name.clone()),
                        ItemDisplay(item.id),
                        // ItemDisplay(proxy_id.clone()),
                    ))
                    .with_children(|inv_root| {
                        use bevy::image::{ImageFormat, ImageFormatSetting, ImageLoaderSettings};
                        if let Some(image_url) =
                            item.properties.iter().find(|i| &i.name == "imageUrl")
                        {
                            inv_root.spawn((
                                ImageNode {
                                    image: asset_server
                                        .load_builder()
                                        .with_settings(|settings: &mut ImageLoaderSettings| {
                                            settings.format =
                                                ImageFormatSetting::Format(ImageFormat::Jpeg);
                                        })
                                        .load(image_url.value.clone()),
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
                                    font: asset_server
                                        .load("fonts/coolvetica_condensed_rg.otf")
                                        .into(),
                                    font_size: FontSize::Px(40.0),
                                    ..default()
                                };
                                let text_color = TextColor(consts::MY_ACCENT_COLOR);
                                container.spawn((
                                    Text::new(format!("({}) {}", item_type.value, name)),
                                    text_style.clone(),
                                    text_color,
                                    TextLayout::justify(Justify::Center),
                                ));
                                if let Some(description) =
                                    item.properties.iter().find(|i| &i.name == "description")
                                {
                                    let text_style = text_style.clone().with_font_size(20.0);
                                    container.spawn((
                                        Text::new(&description.value),
                                        text_style.clone(),
                                        text_color,
                                        TextLayout::justify(Justify::Center),
                                    ));
                                }
                                if let Some(price) =
                                    item.properties.iter().find(|i| &i.name == "price")
                                {
                                    container
                                        .spawn((
                                            BackgroundColor(INTERACTIVE_BG_COLOR),
                                            BorderColor::all(BORDER_COLOR),
                                            Button,
                                            Node {
                                                padding: UiRect::px(15.0, 15.0, 10.0, 15.0),
                                                border: UiRect::all(Val::Px(4.0)),
                                                ..Default::default()
                                            },
                                            SellItemButton(item.id),
                                        ))
                                        .observe(sell_sword_pressed)
                                        .with_children(|btn| {
                                            btn.spawn((
                                                Text::new(format!("Sell it for {}", price.value)),
                                                text_style.clone(),
                                                text_color,
                                                TextLayout::justify(Justify::Center),
                                            ));
                                        })
                                        .observe(sound_on_button);
                                };
                            });
                    });
            });
    }
}
