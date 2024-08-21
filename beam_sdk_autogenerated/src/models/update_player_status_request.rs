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
pub struct UpdatePlayerStatusRequest {
    #[serde(rename = "playerId")]
    pub player_id: i64,
    #[serde(rename = "tournamentId")]
    pub tournament_id: String,
    #[serde(rename = "update")]
    pub update: Box<models::PlayerStatusUpdate>,
}

impl UpdatePlayerStatusRequest {
    pub fn new(player_id: i64, tournament_id: String, update: models::PlayerStatusUpdate) -> UpdatePlayerStatusRequest {
        UpdatePlayerStatusRequest {
            player_id,
            tournament_id,
            update: Box::new(update),
        }
    }
}
