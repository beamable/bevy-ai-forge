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
pub struct SayHiRequestArgs {
    #[serde(rename = "name")]
    pub name: String,
}

impl SayHiRequestArgs {
    pub fn new(name: String) -> SayHiRequestArgs {
        SayHiRequestArgs { name }
    }
}
