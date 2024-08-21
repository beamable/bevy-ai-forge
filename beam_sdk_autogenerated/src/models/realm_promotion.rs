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
pub struct RealmPromotion {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "source")]
    pub source: Box<models::Promotable>,
    #[serde(rename = "destination")]
    pub destination: Box<models::Promotable>,
}

impl RealmPromotion {
    pub fn new(name: String, source: models::Promotable, destination: models::Promotable) -> RealmPromotion {
        RealmPromotion {
            name,
            source: Box::new(source),
            destination: Box::new(destination),
        }
    }
}
