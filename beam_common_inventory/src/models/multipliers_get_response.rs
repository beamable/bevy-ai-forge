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
pub struct MultipliersGetResponse {
    #[serde(rename = "multipliers")]
    pub multipliers: Vec<crate::models::VipBonus>,
}

impl MultipliersGetResponse {
    pub fn new(multipliers: Vec<crate::models::VipBonus>) -> MultipliersGetResponse {
        MultipliersGetResponse { multipliers }
    }
}
