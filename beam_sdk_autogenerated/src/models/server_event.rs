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
pub struct ServerEvent {
    #[serde(rename = "event")]
    pub event: String,
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "toAll")]
    pub to_all: bool,
}

impl ServerEvent {
    pub fn new(event: String, to_all: bool) -> ServerEvent {
        ServerEvent {
            event,
            payload: None,
            to_all,
        }
    }
}

