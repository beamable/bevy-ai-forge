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
pub struct BeamablePeriodCommonPeriodApiPeriodInventoryPeriodFederatedItemUpdateRequest {
    #[serde(rename = "contentId")]
    pub content_id: String,
    #[serde(rename = "proxyId")]
    pub proxy_id: String,
    #[serde(rename = "properties")]
    pub properties: Box<
        crate::models::BeamablePeriodCommonPeriodContentPeriodSerializableDictionaryStringToString,
    >,
}

impl BeamablePeriodCommonPeriodApiPeriodInventoryPeriodFederatedItemUpdateRequest {
    pub fn new(
        content_id: String,
        proxy_id: String,
        properties: crate::models::BeamablePeriodCommonPeriodContentPeriodSerializableDictionaryStringToString,
    ) -> BeamablePeriodCommonPeriodApiPeriodInventoryPeriodFederatedItemUpdateRequest {
        BeamablePeriodCommonPeriodApiPeriodInventoryPeriodFederatedItemUpdateRequest {
            content_id,
            proxy_id,
            properties: Box::new(properties),
        }
    }
}