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
pub struct BeamablePeriodCommonPeriodApiPeriodInventoryPeriodItemProperty {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl BeamablePeriodCommonPeriodApiPeriodInventoryPeriodItemProperty {
    pub fn new(
        name: String,
        value: String,
    ) -> BeamablePeriodCommonPeriodApiPeriodInventoryPeriodItemProperty {
        BeamablePeriodCommonPeriodApiPeriodInventoryPeriodItemProperty { name, value }
    }
}
