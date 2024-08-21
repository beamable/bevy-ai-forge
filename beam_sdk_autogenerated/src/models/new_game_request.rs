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
pub struct NewGameRequest {
    #[serde(rename = "gameName")]
    pub game_name: String,
}

impl NewGameRequest {
    pub fn new(game_name: String) -> NewGameRequest {
        NewGameRequest {
            game_name,
        }
    }
}

