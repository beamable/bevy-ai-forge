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
pub struct LocalizedValue {
    #[serde(rename = "language")]
    pub language: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl LocalizedValue {
    pub fn new(language: String, value: String) -> LocalizedValue {
        LocalizedValue {
            language,
            value,
        }
    }
}

