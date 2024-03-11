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
pub struct AccountAvailableResponse {
    #[serde(rename = "available")]
    pub available: bool,
}

impl AccountAvailableResponse {
    pub fn new(available: bool) -> AccountAvailableResponse {
        AccountAvailableResponse { available }
    }
}