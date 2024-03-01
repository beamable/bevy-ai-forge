use bevy::prelude::*;
use bevy_args::{Deserialize, Parser, Serialize};

#[derive(Default, Debug, Resource, Serialize, Deserialize, Parser)]
#[command(about = "app arguments", version, long_about = None)]
pub struct GameArgs {
    #[arg(long)]
    pub clear_profile: Option<bool>,
}

pub fn despawn_recursive_by_component<T: bevy::prelude::Component>(
    q: bevy::prelude::Query<bevy::prelude::Entity, bevy::prelude::With<T>>,
    mut commands: bevy::prelude::Commands,
) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
