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
pub struct SessionHistoryResponse {
    #[serde(rename = "payments")]
    pub payments: Vec<String>,
    #[serde(rename = "totalPaid")]
    pub total_paid: Vec<models::PaymentTotal>,
    #[serde(rename = "sessions")]
    pub sessions: Vec<String>,
    #[serde(rename = "date")]
    pub date: Box<models::LocalDate>,
    #[serde(rename = "installDate", skip_serializing_if = "Option::is_none")]
    pub install_date: Option<String>,
    #[serde(rename = "daysPlayed")]
    pub days_played: i32,
}

impl SessionHistoryResponse {
    pub fn new(payments: Vec<String>, total_paid: Vec<models::PaymentTotal>, sessions: Vec<String>, date: models::LocalDate, days_played: i32) -> SessionHistoryResponse {
        SessionHistoryResponse {
            payments,
            total_paid,
            sessions,
            date: Box::new(date),
            install_date: None,
            days_played,
        }
    }
}
