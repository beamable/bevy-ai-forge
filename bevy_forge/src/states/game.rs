use beam_microservice::models::SellSwordRequestArgs;
use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_button_released_plugin::ButtonReleasedEvent;
use bevy_simple_scroll_view::*;

use crate::{
    beam::{api::BeamableBasicApi, context::BeamInventory, state::BeamableInitStatus},
    consts::{self, *},
    game::components::*,
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
                    handle_buttons,
                    update_inventory,
                    sell_sword_pressed,
                )
                    .chain()
                    .run_if(in_state(super::MainGameState::Game)),
            )
            .init_resource::<ItemsOnSale>()
            .add_systems(
                Update,
                (|mut cmd: Commands, init_status: Res<State<BeamableInitStatus>>| {
                    if !init_status.eq(&BeamableInitStatus::FullyInitialized) {
                        return;
                    }
                    cmd.beam_get_inventory(Some("currency.coins,items.AiItemContent".to_owned()));
                })
                .run_if(on_timer(std::time::Duration::from_secs(3))),
            );
    }
}

fn sell_sword_pressed(
    mut events: EventReader<ButtonReleasedEvent>,
    mut sword_sell_event: EventReader<crate::microservice::SellSwordEventCompleted>,
    q: Query<(Entity, &SellItemButton, &Parent)>,
    mut cmd: Commands,
    mut on_sale: ResMut<ItemsOnSale>,
) {
    for event in events.read() {
        if let Ok(button) = q.get(**event) {
            cmd.add(MicroserviceSellSword(SellSwordRequestArgs {
                item_id: button.1 .0.clone(),
            }));
            cmd.entity(button.0).remove::<Interaction>();
            on_sale.0.push(button.1 .0.clone());
            if let Some(entity_commands) = cmd.get_entity(button.2.get()) {
                entity_commands.despawn_recursive();
            }
        };
    }
    for _ in sword_sell_event.read() {
        cmd.beam_get_inventory(Some("currency.coins,items.AiItemContent".to_owned()));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<GameRoot>>,
    mut game_bg: Query<(&mut UiImage, &mut Style), With<GameBackground>>,
) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
        font_size: 50.0,
        color: consts::MY_ACCENT_COLOR,
    };

    let Ok(root_entity) = query.get_single() else {
        return;
    };

    commands.entity(root_entity).with_children(|parent| {
        parent
            .spawn((
                NodeBundle {
                    style: Style {
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        position_type: PositionType::Absolute,
                        flex_direction: bevy::ui::FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        max_height: Val::Vh(90.0),
                        height: Val::Vh(90.0),
                        left: Val::Px(0.0),
                        width: Val::Percent(40.0),
                        margin: UiRect::all(Val::Px(30.)),
                        ..default()
                    },
                    background_color: FRAME_BG_COLOR.into(),
                    border_color: BORDER_COLOR.into(),
                    ..default()
                },
                GameplayObject,
                Name::new("InventoryRoot"),
            ))
            .with_children(|inventory| {
                inventory
                    .spawn((
                        NodeBundle {
                            style: Style {
                                overflow: Overflow::clip(),
                                max_height: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        },
                        ScrollView::default(),
                        Name::new("InventoryContainerClip"),
                    ))
                    .with_children(|container| {
                        container.spawn((
                            ScrollableContent::default(),
                            InventoryContainer,
                            NodeBundle {
                                style: Style {
                                    padding: UiRect::axes(Val::Px(5.0), Val::Px(20.0)),
                                    flex_direction: bevy::ui::FlexDirection::ColumnReverse,
                                    align_items: AlignItems::Start,
                                    width: Val::Percent(100.0),
                                    ..default()
                                },
                                ..default()
                            },
                        ));
                    });
            });
        parent
            .spawn((
                NodeBundle {
                    style: Style {
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::new(
                            Val::Px(20.0),
                            Val::Px(80.0),
                            Val::Px(0.0),
                            Val::Px(5.0),
                        ),
                        position_type: PositionType::Absolute,
                        top: Val::Px(20.0),
                        right: Val::Px(20.0),
                        flex_direction: bevy::ui::FlexDirection::Row,
                        ..default()
                    },
                    background_color: FRAME_BG_COLOR.into(),
                    border_color: BORDER_COLOR.into(),
                    ..default()
                },
                GameplayObject,
                Name::new("CoinsRoot"),
            ))
            .with_children(|coin| {
                coin.spawn(TextBundle::from_section("", text_style))
                    .insert(CurrencyText("currency.coins".to_owned()));
                coin.spawn(ImageBundle {
                    image: UiImage::new(asset_server.load("gfx/coins.png")),
                    style: Style {
                        position_type: PositionType::Absolute,
                        right: Val::Px(-25.0),
                        bottom: Val::Px(-23.0),
                        max_height: Val::Px(100.0),
                        max_width: Val::Px(100.0),
                        ..default()
                    },
                    ..default()
                });
            });

        parent
            .spawn((
                ButtonBundle {
                    background_color: INTERACTIVE_BG_COLOR.into(),
                    border_color: BORDER_COLOR.into(),
                    style: Style {
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
                    ..Default::default()
                },
                GameplayObject,
                GameplayButton::StartForgingSword,
            ))
            .with_children(|btn| {
                let text_style = TextStyle {
                    font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
                    font_size: 40.0,
                    color: consts::MY_ACCENT_COLOR,
                };
                btn.spawn(
                    TextBundle::from_section("Forge Sword!", text_style.clone())
                        .with_text_justify(JustifyText::Center),
                );
            });
        parent
            .spawn((
                ButtonBundle {
                    background_color: INTERACTIVE_BG_COLOR.into(),
                    border_color: BORDER_COLOR.into(),
                    style: Style {
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
                    ..Default::default()
                },
                GameplayObject,
                GameplayButton::StartForgingShield,
            ))
            .with_children(|btn| {
                let text_style = TextStyle {
                    font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
                    font_size: 40.0,
                    color: consts::MY_ACCENT_COLOR,
                };
                btn.spawn(
                    TextBundle::from_section("Forge Shield!", text_style.clone())
                        .with_text_justify(JustifyText::Center),
                );
            });
    });
    let Ok(mut background) = game_bg.get_single_mut() else {
        return;
    };
    *background.0 = UiImage::new(asset_server.load("gfx/steampunk_bg_forge.png"));
    background.1.right = Val::Px(0.0);
}

fn update_currency_text(
    mut total_query: Query<(&mut Text, &CurrencyText)>,
    inv: Res<BeamInventory>,
) {
    if inv.is_changed() {
        for (mut text, currency) in total_query.iter_mut() {
            let value = inv.currencies.get(&currency.0).unwrap_or(&-1);
            text.sections[0].value = format!("{}", value);
        }
    }
}

fn add_currency_text(
    mut added_query: Query<(&mut Text, &CurrencyText), Added<CurrencyText>>,
    inv: Res<BeamInventory>,
) {
    for (mut text, currency) in added_query.iter_mut() {
        let value = inv.currencies.get(&currency.0).unwrap_or(&-1);
        text.sections[0].value = format!("{}", value);
    }
}

fn handle_buttons(
    mut reader: EventReader<ButtonReleasedEvent>,
    q: Query<&GameplayButton>,
    mut commands: Commands,
) {
    for event in reader.read() {
        let Ok(button) = q.get(**event) else {
            continue;
        };
        match button {
            GameplayButton::StartForgingSword => {
                commands.beam_add_to_inventory(vec!["items.AiItemContent.AiSword".into()]);
            }
            GameplayButton::StartForgingShield => {
                commands.beam_add_to_inventory(vec!["items.AiItemContent.AiShield".into()]);
            }
        }
    }
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
            item.properties.push(crate::beam::context::ItemProperty {
                name: "type".to_owned(),
                value: "sword".to_owned(),
            })
        }
        items.append(&mut it);
    };
    if let Some(shields) = inv.items.get("items.AiItemContent.AiShield") {
        let mut it = shields.clone();
        for item in it.iter_mut() {
            item.properties.push(crate::beam::context::ItemProperty {
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
    let binding = crate::beam::context::ItemProperty {
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
                        NodeBundle {
                            style: Style {
                                border: UiRect::all(Val::Px(5.0)),
                                margin: UiRect::all(Val::Px(5.0)),
                                flex_direction: FlexDirection::Column,
                                column_gap: Val::Px(10.0),
                                align_self: AlignSelf::Stretch,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BORDER_COLOR.into(),
                            border_color: BORDER_COLOR.into(),
                            ..default()
                        },
                        ItemDisplay(item.id),
                        Name::new(name.value.clone()),
                    ))
                    .with_children(|container| {
                        let text_style = TextStyle {
                            font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
                            font_size: 40.0,
                            color: consts::MY_ACCENT_COLOR,
                        };
                        let sections = [
                            TextSection::new(
                                format!("({}) ", &item_type.value),
                                text_style.clone(),
                            ),
                            TextSection::new(&name.value, text_style.clone()),
                        ];
                        container.spawn(
                            TextBundle::from_sections(sections)
                                .with_text_justify(JustifyText::Center),
                        );
                        if let Some(description) =
                            item.properties.iter().find(|i| &i.name == "description")
                        {
                            let text_style = TextStyle {
                                font: asset_server.load("fonts/coolvetica_condensed_rg.otf"),
                                font_size: 20.0,
                                color: consts::MY_ACCENT_COLOR,
                            };
                            let sections =
                                [TextSection::new(&description.value, text_style.clone())];
                            container.spawn(
                                TextBundle::from_sections(sections)
                                    .with_text_justify(JustifyText::Center),
                            );
                        }
                        let Some(price) = item.properties.iter().find(|i| &i.name == "price")
                        else {
                            return;
                        };
                        container
                            .spawn((
                                ButtonBundle {
                                    background_color: INTERACTIVE_BG_COLOR.into(),
                                    border_color: BORDER_COLOR.into(),
                                    style: Style {
                                        padding: UiRect::px(15.0, 15.0, 10.0, 15.0),
                                        border: UiRect::all(Val::Px(4.0)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                SellItemButton(proxy_id.clone()),
                            ))
                            .with_children(|btn| {
                                btn.spawn(
                                    TextBundle::from_section(
                                        format!("Sell it for {}", &price.value),
                                        text_style.clone(),
                                    )
                                    .with_text_justify(JustifyText::Center),
                                );
                            });
                    });
            });
    }
}
