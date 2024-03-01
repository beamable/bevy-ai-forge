/*
 * auth basic
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@beamable.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChallengeSolution {
    #[serde(rename = "challenge_token")]
    pub challenge_token: String,
    #[serde(rename = "solution")]
    pub solution: String,
}

impl ChallengeSolution {
    pub fn new(challenge_token: String, solution: String) -> ChallengeSolution {
        ChallengeSolution {
            challenge_token,
            solution,
        }
    }
}
