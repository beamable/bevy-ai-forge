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
pub struct MailQueryResponse {
    #[serde(rename = "unreadCount")]
    pub unread_count: i64,
}

impl MailQueryResponse {
    pub fn new(unread_count: i64) -> MailQueryResponse {
        MailQueryResponse {
            unread_count,
        }
    }
}

