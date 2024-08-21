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
pub struct CurrencyChange {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "originalAmount", skip_serializing_if = "Option::is_none")]
    pub original_amount: Option<i64>,
}

impl CurrencyChange {
    pub fn new(symbol: String, amount: i64) -> CurrencyChange {
        CurrencyChange {
            symbol,
            amount,
            original_amount: None,
        }
    }
}
