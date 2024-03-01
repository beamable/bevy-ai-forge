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
pub struct FindAccountRequest {
    #[serde(rename = "query")]
    pub query: String,
}

impl FindAccountRequest {
    pub fn new(query: String) -> FindAccountRequest {
        FindAccountRequest { query }
    }
}
