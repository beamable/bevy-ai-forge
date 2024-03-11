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
pub struct InventoryQueryRequest {
    #[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

impl Default for InventoryQueryRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl InventoryQueryRequest {
    pub fn new() -> InventoryQueryRequest {
        InventoryQueryRequest { scopes: None }
    }
}