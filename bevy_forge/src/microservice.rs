use beam_microservice::apis::uncategorized_api::*;
use beam_microservice::models::{SayHiRequestArgs, SellSwordRequestArgs};
use beam_microservice::*;
use bevy::prelude::*;
use bevy_beam_sdk::prelude::*;

#[derive(Debug, BeamCommand)]
#[beam_command(SayHiEventCompleted, String, apis::Error<SayHiPostError>, say_hi_post)]
pub struct MicroserviceSayHi(pub Option<SayHiRequestArgs>, pub Entity);
#[derive(Debug, BeamCommand)]
#[beam_command(SellSwordEventCompleted, bool, apis::Error<SellSwordPostError>, sell_sword_post)]
pub struct MicroserviceSellSword(pub Option<SellSwordRequestArgs>, pub Entity);
#[derive(Debug, BeamCommand)]
#[beam_command(StartForgingEventCompleted, bool, apis::Error<StartForgingSwordPostError>, start_forging_sword_post)]
pub struct MicroserviceStartForging(pub Entity);
#[derive(Debug, BeamCommand)]
#[beam_command(StartForgingShieldEventCompleted, bool, apis::Error<StartForgingSwordPostError>, start_forging_shield_post)]
pub struct MicroserviceStartForgingShield(pub Entity);

pub struct MicroservicePlugin;

impl Plugin for MicroservicePlugin {
    fn build(&self, app: &mut App) {
        SellSwordEventCompleted::register(app);
        StartForgingEventCompleted::register(app);
        SayHiEventCompleted::register(app);
        StartForgingShieldEventCompleted::register(app);
    }
}
