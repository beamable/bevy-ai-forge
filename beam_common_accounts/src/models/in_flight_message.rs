/*
 * accounts basic
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@beamable.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InFlightMessage {
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "gamerTag", skip_serializing_if = "Option::is_none")]
    pub gamer_tag: Option<i64>,
    #[serde(
        rename = "limitFailureRetries",
        skip_serializing_if = "Option::is_none"
    )]
    pub limit_failure_retries: Option<bool>,
    #[serde(rename = "shard", skip_serializing_if = "Option::is_none")]
    pub shard: Option<String>,
    #[serde(rename = "service")]
    pub service: String,
    #[serde(rename = "id")]
    pub id: String,
}

impl InFlightMessage {
    pub fn new(
        method: String,
        body: String,
        path: String,
        service: String,
        id: String,
    ) -> InFlightMessage {
        InFlightMessage {
            method,
            body,
            path,
            gamer_tag: None,
            limit_failure_retries: None,
            shard: None,
            service,
            id,
        }
    }
}
