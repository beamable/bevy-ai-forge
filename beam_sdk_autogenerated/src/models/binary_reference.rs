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
pub struct BinaryReference {
    #[serde(rename = "uploadMethod")]
    pub upload_method: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "checksum", skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "lastChanged", skip_serializing_if = "Option::is_none")]
    pub last_changed: Option<String>,
    #[serde(rename = "uploadUri")]
    pub upload_uri: String,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "visibility")]
    pub visibility: String,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}

impl BinaryReference {
    pub fn new(upload_method: String, tags: Vec<String>, uri: String, version: String, id: String, upload_uri: String, r#type: Type, visibility: String) -> BinaryReference {
        BinaryReference {
            upload_method,
            tags,
            uri,
            version,
            id,
            checksum: None,
            last_changed: None,
            upload_uri,
            r#type,
            visibility,
            created: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "binary")]
    Binary,
}

impl Default for Type {
    fn default() -> Type {
        Self::Binary
    }
}

