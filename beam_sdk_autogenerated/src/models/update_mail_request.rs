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
pub struct UpdateMailRequest {
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "mailId")]
    pub mail_id: i64,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "acceptAttachments", skip_serializing_if = "Option::is_none")]
    pub accept_attachments: Option<bool>,
}

impl UpdateMailRequest {
    pub fn new(mail_id: i64) -> UpdateMailRequest {
        UpdateMailRequest {
            body: None,
            expires: None,
            subject: None,
            state: None,
            mail_id,
            category: None,
            accept_attachments: None,
        }
    }
}

