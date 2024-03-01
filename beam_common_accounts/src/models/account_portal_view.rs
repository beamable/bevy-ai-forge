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
pub struct AccountPortalView {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "roleString", skip_serializing_if = "Option::is_none")]
    pub role_string: Option<String>,
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "external", skip_serializing_if = "Option::is_none")]
    pub external: Option<Vec<crate::models::ExternalIdentity>>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<crate::models::RoleMapping>>,
    #[serde(rename = "thirdPartyAppAssociations")]
    pub third_party_app_associations: Vec<String>,
}

impl AccountPortalView {
    pub fn new(
        scopes: Vec<String>,
        id: i64,
        third_party_app_associations: Vec<String>,
    ) -> AccountPortalView {
        AccountPortalView {
            email: None,
            role_string: None,
            scopes,
            id,
            external: None,
            language: None,
            roles: None,
            third_party_app_associations,
        }
    }
}
