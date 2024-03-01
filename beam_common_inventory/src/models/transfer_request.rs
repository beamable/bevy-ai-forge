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
pub struct TransferRequest {
    #[serde(rename = "transaction", skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,
    #[serde(rename = "recipientPlayer")]
    pub recipient_player: i64,
    #[serde(rename = "currencies", skip_serializing_if = "Option::is_none")]
    pub currencies: Option<::std::collections::HashMap<String, i64>>,
}

impl TransferRequest {
    pub fn new(recipient_player: i64) -> TransferRequest {
        TransferRequest {
            transaction: None,
            recipient_player,
            currencies: None,
        }
    }
}
