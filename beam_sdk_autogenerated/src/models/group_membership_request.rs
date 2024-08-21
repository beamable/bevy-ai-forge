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
pub struct GroupMembershipRequest {
    #[serde(rename = "successor", skip_serializing_if = "Option::is_none")]
    pub successor: Option<i64>,
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
    #[serde(rename = "subGroup", skip_serializing_if = "Option::is_none")]
    pub sub_group: Option<i64>,
    #[serde(rename = "type")]
    pub r#type: models::GroupType,
    #[serde(rename = "group")]
    pub group: i64,
}

impl GroupMembershipRequest {
    pub fn new(r#type: models::GroupType, group: i64) -> GroupMembershipRequest {
        GroupMembershipRequest {
            successor: None,
            score: None,
            sub_group: None,
            r#type,
            group,
        }
    }
}
