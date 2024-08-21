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
pub struct ContentMeta {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "visibility")]
    pub visibility: models::ContentVisibility,
}

impl ContentMeta {
    pub fn new(visibility: models::ContentVisibility) -> ContentMeta {
        ContentMeta {
            data: None,
            text: None,
            visibility,
        }
    }
}
