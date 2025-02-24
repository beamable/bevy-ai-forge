use bevy::prelude::*;
use bevy_args::{Deserialize, Parser, Serialize};

#[derive(Default, Debug, Resource, Serialize, Deserialize, Parser)]
#[command(about = "app arguments", version, long_about = None)]
pub struct GameArgs {
    #[arg(long)]
    pub clear_profile: Option<bool>,
}
