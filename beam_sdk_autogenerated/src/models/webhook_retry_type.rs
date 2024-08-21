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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebhookRetryType {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Once")]
    Once,
    #[serde(rename = "ExponentialBackoff")]
    ExponentialBackoff,

}

impl std::fmt::Display for WebhookRetryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Once => write!(f, "Once"),
            Self::ExponentialBackoff => write!(f, "ExponentialBackoff"),
        }
    }
}

impl Default for WebhookRetryType {
    fn default() -> WebhookRetryType {
        Self::None
    }
}

