/*
 * Auth Actor
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@beamable.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRoomResponse {
    #[serde(rename = "room")]
    pub room: Box<models::RoomInfo>,
}

impl CreateRoomResponse {
    pub fn new(room: models::RoomInfo) -> CreateRoomResponse {
        CreateRoomResponse {
            room: Box::new(room),
        }
    }
}

