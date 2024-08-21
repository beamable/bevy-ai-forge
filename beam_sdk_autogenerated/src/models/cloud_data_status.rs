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
pub struct CloudDataStatus {
    #[serde(rename = "ejected")]
    pub ejected: bool,
    #[serde(rename = "processing")]
    pub processing: bool,
    #[serde(rename = "sent")]
    pub sent: bool,
    #[serde(rename = "forSending")]
    pub for_sending: bool,
}

impl CloudDataStatus {
    pub fn new(ejected: bool, processing: bool, sent: bool, for_sending: bool) -> CloudDataStatus {
        CloudDataStatus {
            ejected,
            processing,
            sent,
            for_sending,
        }
    }
}
