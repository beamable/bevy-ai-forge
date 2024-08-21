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
pub struct AnnouncementDto {
    #[serde(rename = "body")]
    pub body: Box<models::LocalizationRef>,
    #[serde(rename = "channel")]
    pub channel: String,
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "gift", skip_serializing_if = "Option::is_none")]
    pub gift: Option<Box<models::PlayerReward>>,
    #[serde(rename = "stat_requirements", skip_serializing_if = "Option::is_none")]
    pub stat_requirements: Option<Vec<models::PlayerStatRequirement>>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "clientData", skip_serializing_if = "Option::is_none")]
    pub client_data: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "title")]
    pub title: Box<models::LocalizationRef>,
    #[serde(rename = "attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<models::AnnouncementAttachment>>,
    #[serde(rename = "summary")]
    pub summary: Box<models::LocalizationRef>,
}

impl AnnouncementDto {
    pub fn new(body: models::LocalizationRef, channel: String, symbol: String, title: models::LocalizationRef, summary: models::LocalizationRef) -> AnnouncementDto {
        AnnouncementDto {
            body: Box::new(body),
            channel,
            start_date: None,
            tags: None,
            gift: None,
            stat_requirements: None,
            symbol,
            client_data: None,
            end_date: None,
            title: Box::new(title),
            attachments: None,
            summary: Box::new(summary),
        }
    }
}

