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
pub struct PromoteRealmRequest {
    #[serde(rename = "sourcePid")]
    pub source_pid: String,
    #[serde(rename = "promotions", skip_serializing_if = "Option::is_none")]
    pub promotions: Option<Vec<String>>,
    #[serde(rename = "contentManifestIds", skip_serializing_if = "Option::is_none")]
    pub content_manifest_ids: Option<Vec<String>>,
}

impl PromoteRealmRequest {
    pub fn new(source_pid: String) -> PromoteRealmRequest {
        PromoteRealmRequest {
            source_pid,
            promotions: None,
            content_manifest_ids: None,
        }
    }
}
