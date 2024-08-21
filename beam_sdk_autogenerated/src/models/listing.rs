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
pub struct Listing {
    #[serde(rename = "cohortRequirements")]
    pub cohort_requirements: Vec<models::CohortRequirement>,
    #[serde(rename = "offerSymbol")]
    pub offer_symbol: String,
    #[serde(rename = "purchaseLimit", skip_serializing_if = "Option::is_none")]
    pub purchase_limit: Option<i32>,
    #[serde(rename = "price")]
    pub price: Box<models::Price>,
    #[serde(rename = "playerStatRequirements")]
    pub player_stat_requirements: Vec<models::PlayerStatRequirement>,
    #[serde(rename = "buttonText", skip_serializing_if = "Option::is_none")]
    pub button_text: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "scheduleInstancePurchaseLimit", skip_serializing_if = "Option::is_none")]
    pub schedule_instance_purchase_limit: Option<i32>,
    #[serde(rename = "entitlementRequirements")]
    pub entitlement_requirements: Vec<models::EntitlementRequirement>,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "clientData")]
    pub client_data: std::collections::HashMap<String, String>,
    #[serde(rename = "schedule", skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Box<models::Schedule>>,
    #[serde(rename = "activeDurationCoolDownSeconds", skip_serializing_if = "Option::is_none")]
    pub active_duration_cool_down_seconds: Option<i32>,
    #[serde(rename = "activeDurationSeconds", skip_serializing_if = "Option::is_none")]
    pub active_duration_seconds: Option<i32>,
    #[serde(rename = "activeDurationPurchaseLimit", skip_serializing_if = "Option::is_none")]
    pub active_duration_purchase_limit: Option<i32>,
    #[serde(rename = "offerRequirements")]
    pub offer_requirements: Vec<models::OfferRequirement>,
    #[serde(rename = "activePeriod", skip_serializing_if = "Option::is_none")]
    pub active_period: Option<Box<models::Period>>,
}

impl Listing {
    pub fn new(cohort_requirements: Vec<models::CohortRequirement>, offer_symbol: String, price: models::Price, player_stat_requirements: Vec<models::PlayerStatRequirement>, entitlement_requirements: Vec<models::EntitlementRequirement>, symbol: String, client_data: std::collections::HashMap<String, String>, offer_requirements: Vec<models::OfferRequirement>) -> Listing {
        Listing {
            cohort_requirements,
            offer_symbol,
            purchase_limit: None,
            price: Box::new(price),
            player_stat_requirements,
            button_text: None,
            schedule_instance_purchase_limit: None,
            entitlement_requirements,
            symbol,
            client_data,
            schedule: None,
            active_duration_cool_down_seconds: None,
            active_duration_seconds: None,
            active_duration_purchase_limit: None,
            offer_requirements,
            active_period: None,
        }
    }
}

