use crate::api::content::GotManifestEvent;
use bevy::prelude::*;

pub fn on_manifest_got(trigger: On<GotManifestEvent>) {
    let Ok(response) = &**trigger.event() else {
        return;
    };
    for item in &response.entries {
        warn!("[ITEM] {}({}): {:#?}", item.content_id, item.uri, item);
    }
}
