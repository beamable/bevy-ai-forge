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
pub struct StartSessionRequest {
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "customParams", skip_serializing_if = "Option::is_none")]
    pub custom_params: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "shard", skip_serializing_if = "Option::is_none")]
    pub shard: Option<String>,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "deviceParams", skip_serializing_if = "Option::is_none")]
    pub device_params: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Box<models::SessionLanguageContext>>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "gamer", skip_serializing_if = "Option::is_none")]
    pub gamer: Option<i64>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
}

impl StartSessionRequest {
    pub fn new() -> StartSessionRequest {
        StartSessionRequest {
            source: None,
            custom_params: None,
            shard: None,
            locale: None,
            device_params: None,
            language: None,
            time: None,
            platform: None,
            gamer: None,
            device: None,
        }
    }
}
