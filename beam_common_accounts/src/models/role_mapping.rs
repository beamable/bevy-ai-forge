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
pub struct RoleMapping {
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "role")]
    pub role: String,
}

impl RoleMapping {
    pub fn new(project_id: String, role: String) -> RoleMapping {
        RoleMapping { project_id, role }
    }
}
