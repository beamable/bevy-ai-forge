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
pub struct AddTags {
    #[serde(rename = "playerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub player_id: Option<Option<String>>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<models::Tag>>>,
    #[serde(rename = "replace", skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
}

impl AddTags {
    pub fn new() -> AddTags {
        AddTags {
            player_id: None,
            tags: None,
            replace: None,
        }
    }
}

