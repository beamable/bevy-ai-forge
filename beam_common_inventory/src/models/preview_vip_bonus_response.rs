/*
 * inventory object
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@beamable.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreviewVipBonusResponse {
    #[serde(rename = "currencies")]
    pub currencies: Vec<crate::models::CurrencyPreview>,
}

impl PreviewVipBonusResponse {
    pub fn new(currencies: Vec<crate::models::CurrencyPreview>) -> PreviewVipBonusResponse {
        PreviewVipBonusResponse { currencies }
    }
}
