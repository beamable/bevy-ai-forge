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
pub struct SteamAuthRequest {
    #[serde(rename = "ticket")]
    pub ticket: String,
}

impl SteamAuthRequest {
    pub fn new(ticket: String) -> SteamAuthRequest {
        SteamAuthRequest {
            ticket,
        }
    }
}

