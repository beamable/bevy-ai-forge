/*
 * Auth Actor
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@beamable.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(rename = "version")]
    pub version: i64,
    /// Milliseconds since midnight, January 1, 1970 UTC
    #[serde(rename = "created")]
    pub created: i64,
    #[serde(rename = "stores")]
    pub stores: Vec<models::Store>,
    #[serde(rename = "offerDefinitions")]
    pub offer_definitions: Vec<models::OfferDefinition>,
}

impl Catalog {
    pub fn new(version: i64, created: i64, stores: Vec<models::Store>, offer_definitions: Vec<models::OfferDefinition>) -> Catalog {
        Catalog {
            version,
            created,
            stores,
            offer_definitions,
        }
    }
}
