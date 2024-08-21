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
pub struct ListAuditRequest {
    #[serde(rename = "providerid", skip_serializing_if = "Option::is_none")]
    pub providerid: Option<String>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "txid", skip_serializing_if = "Option::is_none")]
    pub txid: Option<i64>,
    #[serde(rename = "player", skip_serializing_if = "Option::is_none")]
    pub player: Option<i64>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl ListAuditRequest {
    pub fn new() -> ListAuditRequest {
        ListAuditRequest {
            providerid: None,
            provider: None,
            state: None,
            txid: None,
            player: None,
            start: None,
            limit: None,
        }
    }
}

