use beam_autogen_rs::models::InventoryView;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct BeamInventory {
    pub currencies: HashMap<String, i64>,
    pub items: HashMap<String, Vec<Item>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Reflect)]
pub struct Item {
    pub updated_at: Option<i64>,
    pub proxy_id: Option<String>,
    pub id: i64,
    pub properties: Vec<ItemProperty>,
    pub created_at: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Reflect)]
pub struct ItemProperty {
    pub name: String,
    pub value: String,
}

impl From<InventoryView> for BeamInventory {
    fn from(value: InventoryView) -> Self {
        BeamInventory {
            currencies: value
                .currencies
                .iter()
                .map(|i| (i.id.clone(), i.amount))
                .collect(),
            items: value
                .items
                .iter()
                .map(|item| {
                    (
                        item.id.clone(),
                        item.items
                            .iter()
                            .map(|item| Item {
                                created_at: item.created_at,
                                id: item.id,
                                properties: item
                                    .properties
                                    .iter()
                                    .map(|p| ItemProperty {
                                        name: p.name.clone(),
                                        value: p.value.clone(),
                                    })
                                    .collect(),
                                updated_at: item.updated_at,
                                proxy_id: item.proxy_id.clone(),
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}
