use crate::beam::utils::macros::{create_request, create_old_request_no_args};
use beam_microservice::apis::uncategorized_api::*;
use beam_microservice::models::*;
use beam_microservice::*;
use bevy::prelude::*;

create_request!(
    RequestSayHiTask,
    MicroserviceSayHi,
    SayHiEventCompleted,
    beam_microservice::apis::uncategorized_api::say_hi_post,
    SayHiRequestArgs,
    String,
    SayHiPostError
);
create_request!(
    RequestSellSwordTask,
    MicroserviceSellSword,
    SellSwordEventCompleted,
    beam_microservice::apis::uncategorized_api::sell_sword_post,
    SellSwordRequestArgs,
    bool,
    SellSwordPostError
);
create_old_request_no_args!(
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
        RequestSayHiTask::register(app);
        RequestSellSwordTask::register(app);
        RequestStartForgingTask::register(app);
    }
}
