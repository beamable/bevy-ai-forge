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
pub struct ItemUpdateRequest {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "properties")]
    pub properties: Vec<crate::models::ItemProperty>,
}

impl ItemUpdateRequest {
    pub fn new(
        content_id: String,
        id: i64,
        properties: Vec<crate::models::ItemProperty>,
    ) -> ItemUpdateRequest {
        ItemUpdateRequest {
            content_id,
            id,
            properties,
        }
    }
}
