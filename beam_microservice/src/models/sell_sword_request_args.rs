/*
 * ForgeService
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SellSwordRequestArgs {
    #[serde(rename = "itemId")]
    pub item_id: String,
}

impl SellSwordRequestArgs {
    pub fn new(item_id: String) -> SellSwordRequestArgs {
        SellSwordRequestArgs { item_id }
    }
}