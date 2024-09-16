use beam_autogen_rs::models::NotificationRequestData;
use bevy::{ecs::world::CommandQueue, prelude::*};

use crate::api::BeamableBasicApi;

#[derive(Event, Deref, DerefMut)]
pub struct Notification(pub NotificationRequestData);

pub trait NotifyBeam {
    fn context_id() -> &'static str;
    fn handle_command(
        notification: &NotificationRequestData,
        ctx: &crate::context::BeamContext,
    ) -> CommandQueue;
}

pub fn notification_handle<T: NotifyBeam>(
    mut cmd: Commands,
    mut reader: EventReader<Notification>,
    ctx: Res<crate::context::BeamContext>,
) {
    for event in reader.read() {
        let notification = &event.0;
        info!("NOTIFICATION: {:#?}", notification);
        let Some(context) = &notification.context else {
            continue;
        };
        if context.eq(T::context_id()) {
            let mut command = T::handle_command(&notification, &ctx);
            cmd.append(&mut command);
        }
    }
}

pub struct InventoryRefreshNotify;

impl NotifyBeam for InventoryRefreshNotify {
    fn context_id() -> &'static str {
        "inventory.refresh"
    }

    fn handle_command(
        notification: &NotificationRequestData,
        ctx: &crate::context::BeamContext,
    ) -> CommandQueue {
        let mut command_queue = CommandQueue::default();
        let q = notification.message_full.clone().unwrap_or_default();

        let query = serde_json::from_str::<beam_autogen_rs::models::InventoryQueryRequest>(&q)
            .unwrap_or_default();
        let scope = if query.scopes.is_some() {
            Some(query.scopes.unwrap_or_default().join(","))
        } else {
            None
        };
        let gamer_tag = ctx.get_gamer_tag().unwrap().to_string();

        command_queue.push(move |world: &mut World| {
            world.commands().beam_get_inventory(scope, gamer_tag);
        });

        command_queue
    }
}
