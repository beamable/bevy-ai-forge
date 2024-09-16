use beam_autogen_rs::models::NotificationRequestData;
use bevy::prelude::*;

use crate::api::BeamableBasicApi;

#[derive(Event, Deref, DerefMut)]
pub struct Notification(pub NotificationRequestData);

pub fn update_inventory(
    mut cmd: Commands,
    mut reader: EventReader<Notification>,
    ctx: Res<crate::context::BeamContext>,
) {
    for event in reader.read() {
        let notification = (**event).clone();
        info!("NOTIFICATION: {:#?}", notification);
        let Some(context) = notification.context else {
            continue;
        };
        if context.eq("inventory.refresh") {
            let query = serde_json::from_str::<beam_autogen_rs::models::InventoryQueryRequest>(
                &notification.message_full.unwrap_or_default(),
            )
            .unwrap_or_default();
            info!("NOTIFICATION query: {:#?}", query);
            let scope = if query.scopes.is_some() {
                Some(query.scopes.unwrap_or_default().join(","))
            } else {
                None
            };
            cmd.beam_get_inventory(scope, ctx.get_gamer_tag().unwrap().to_string());
        }
    }
}
