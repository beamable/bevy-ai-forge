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
pub struct CurrencyView {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "properties")]
    pub properties: Vec<crate::models::CurrencyProperty>,
    #[serde(rename = "proxy", skip_serializing_if = "Option::is_none")]
    pub proxy: Option<Box<crate::models::ArchetypeProxy>>,
}

impl CurrencyView {
    pub fn new(
        id: String,
        amount: i64,
        properties: Vec<crate::models::CurrencyProperty>,
    ) -> CurrencyView {
        CurrencyView {
            id,
            amount,
            properties,
            proxy: None,
        }
    }
}
