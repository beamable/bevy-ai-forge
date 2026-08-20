use beam_autogen_rs::models::InventoryView;
use bevy::{platform::collections::HashMap, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Reflect, Default, Clone)]
#[reflect(Component, Default)]
/// Observable state for an entity's authoritative inventory synchronization.
pub struct BeamInventorySyncState {
    /// Whether an inventory HTTP request is currently active.
    pub request_in_flight: bool,
    /// Whether notifications or callers queued one follow-up refresh.
    pub refresh_queued: bool,
    /// Monotonically increasing successful snapshot revision.
    pub revision: u64,
    /// Safe summary of the most recent refresh error.
    pub last_error: Option<String>,
    pub(crate) active_scopes: Option<Vec<String>>,
    pub(crate) pending_full: bool,
    pub(crate) pending_scopes: Vec<String>,
    pub(crate) request_started_seconds: f64,
}

impl BeamInventorySyncState {
    pub(crate) fn begin_or_queue(
        &mut self,
        scope: Option<String>,
        now_seconds: f64,
    ) -> Option<Option<String>> {
        let scopes = parse_scopes(scope);
        if self.request_in_flight {
            self.refresh_queued = true;
            if scopes.is_none() {
                self.pending_full = true;
                self.pending_scopes.clear();
            } else if !self.pending_full {
                for scope in scopes.unwrap_or_default() {
                    if !self.pending_scopes.contains(&scope) {
                        self.pending_scopes.push(scope);
                    }
                }
            }
            return None;
        }

        self.request_in_flight = true;
        self.request_started_seconds = now_seconds;
        self.active_scopes = scopes.clone();
        Some(scopes.map(|values| values.join(",")))
    }

    pub(crate) fn finish(&mut self) -> Option<Option<String>> {
        self.request_in_flight = false;
        self.active_scopes = None;
        if !self.refresh_queued {
            return None;
        }
        self.refresh_queued = false;
        let next = if self.pending_full {
            None
        } else {
            Some(self.pending_scopes.join(","))
        };
        self.pending_full = false;
        self.pending_scopes.clear();
        Some(next)
    }
}

fn parse_scopes(scope: Option<String>) -> Option<Vec<String>> {
    scope.map(|scope| {
        scope
            .split(',')
            .map(str::trim)
            .filter(|scope| !scope.is_empty())
            .map(ToOwned::to_owned)
            .collect()
    })
}

#[derive(EntityEvent, Debug, Clone, Reflect)]
/// Emitted after an authoritative inventory snapshot has been applied.
pub struct BeamInventoryUpdated {
    /// Slot entity whose inventory changed.
    pub entity: Entity,
    /// Empty for a full snapshot, otherwise the refreshed scopes.
    pub scopes: Vec<String>,
    /// Successful snapshot revision.
    pub revision: u64,
}

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

impl BeamInventory {
    pub(crate) fn apply_snapshot(&mut self, snapshot: BeamInventory, scopes: Option<&[String]>) {
        let Some(scopes) = scopes else {
            *self = snapshot;
            return;
        };

        for scope in scopes {
            if scope == "currency" || scope.starts_with("currency.") {
                self.currencies
                    .retain(|key, _| !is_scope_or_descendant(key, scope));
            }
            if scope == "items" || scope.starts_with("items.") {
                self.items
                    .retain(|key, _| !is_scope_or_descendant(key, scope));
            }
        }
        self.currencies.extend(snapshot.currencies);
        self.items.extend(snapshot.items);
    }
}

fn is_scope_or_descendant(value: &str, scope: &str) -> bool {
    value == scope
        || value
            .strip_prefix(scope)
            .is_some_and(|suffix| suffix.starts_with('.'))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inventory() -> BeamInventory {
        BeamInventory {
            currencies: HashMap::from([
                ("currency.coins".to_owned(), 10),
                ("currency.gems".to_owned(), 3),
            ]),
            items: HashMap::from([
                ("items.swords.iron".to_owned(), vec![Item { id: 1, ..default() }]),
                ("items.shields".to_owned(), vec![Item { id: 2, ..default() }]),
            ]),
        }
    }

    #[test]
    fn full_empty_snapshot_removes_everything() {
        let mut current = inventory();
        current.apply_snapshot(default(), None);
        assert!(current.items.is_empty());
        assert!(current.currencies.is_empty());
    }

    #[test]
    fn scoped_snapshot_removes_stale_descendants_only() {
        let mut current = inventory();
        current.apply_snapshot(default(), Some(&["items.swords".to_owned()]));
        assert!(!current.items.contains_key("items.swords.iron"));
        assert!(current.items.contains_key("items.shields"));
        assert_eq!(current.currencies["currency.coins"], 10);
    }

    #[test]
    fn refreshes_coalesce_to_one_follow_up() {
        let mut state = BeamInventorySyncState::default();
        assert_eq!(state.begin_or_queue(Some("items".into()), 0.0), Some(Some("items".into())));
        assert_eq!(state.begin_or_queue(Some("currency.coins".into()), 0.1), None);
        assert_eq!(state.begin_or_queue(Some("items".into()), 0.2), None);
        let follow_up = state.finish();
        assert_eq!(follow_up, Some(Some("currency.coins,items".into())));
        assert_eq!(state.finish(), None);
    }
}
