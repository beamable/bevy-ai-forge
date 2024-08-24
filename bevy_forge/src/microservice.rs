use bevy_beam_sdk::utils::macros::beam_request;
use beam_microservice::apis::uncategorized_api::*;
use beam_microservice::*;
use bevy::prelude::*;
use beam_microservice::models::{SayHiRequestArgs,SellSwordRequestArgs};

beam_request!(
    RequestSayHiTask,
    MicroserviceSayHi,
    SayHiEventCompleted,
    beam_microservice::apis::uncategorized_api::say_hi_post,
    String,
    SayHiPostError,
    Option<SayHiRequestArgs>
);
beam_request!(
    RequestSellSwordTask,
    MicroserviceSellSword,
    SellSwordEventCompleted,
    beam_microservice::apis::uncategorized_api::sell_sword_post,
    bool,
    SellSwordPostError,
    Option<SellSwordRequestArgs>
);
beam_request!(
    RequestStartForgingTask,
    MicroserviceStartForging,
    StartForgingEventCompleted,
    beam_microservice::apis::uncategorized_api::start_forging_sword_post,
    bool,
    StartForgingSwordPostError
);

pub struct MicroservicePlugin;

impl Plugin for MicroservicePlugin {
    fn build(&self, app: &mut App) {
        RequestSellSwordTask::register(app);
        RequestStartForgingTask::register(app);
        RequestSayHiTask::register(app);
    }
}
