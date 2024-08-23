use bevy_beam_sdk::utils::macros::{create_request_with_args, create_request_no_args};
use beam_microservice::apis::uncategorized_api::*;
use beam_microservice::*;
use bevy::prelude::*;
use beam_microservice::models::{SayHiRequestArgs,SellSwordRequestArgs};

create_request_with_args!(
    RequestSayHiTask,
    MicroserviceSayHi,
    SayHiEventCompleted,
    beam_microservice::apis::uncategorized_api::say_hi_post,
    Option<SayHiRequestArgs>,
    String,
    SayHiPostError
);
create_request_with_args!(
    RequestSellSwordTask,
    MicroserviceSellSword,
    SellSwordEventCompleted,
    beam_microservice::apis::uncategorized_api::sell_sword_post,
    Option<SellSwordRequestArgs>,
    bool,
    SellSwordPostError
);
create_request_no_args!(
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
