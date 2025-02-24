use crate::prelude::*;
use beam_autogen_rs::{
    apis::default_api::{
        object_stats_object_id_client_get, ObjectStatsObjectIdClientGetError,
        ObjectStatsObjectIdClientGetParams,
    },
    models::StatsResponse,
    *,
};
use bevy::prelude::*;

#[derive(Debug, BeamCommand)]
#[beam_command(StatsGetEvent, StatsResponse, apis::Error<ObjectStatsObjectIdClientGetError>, object_stats_object_id_client_get)]
pub struct StatsGet(pub ObjectStatsObjectIdClientGetParams, pub Entity);
