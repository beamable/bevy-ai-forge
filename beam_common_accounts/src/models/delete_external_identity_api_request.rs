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
pub struct DeleteExternalIdentityApiRequest {
    #[serde(rename = "provider_service")]
    pub provider_service: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "provider_namespace", skip_serializing_if = "Option::is_none")]
    pub provider_namespace: Option<String>,
}

impl DeleteExternalIdentityApiRequest {
    pub fn new(provider_service: String, user_id: String) -> DeleteExternalIdentityApiRequest {
        DeleteExternalIdentityApiRequest {
            provider_service,
            user_id,
            provider_namespace: None,
        }
    }
}
