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
pub struct DatabaseMeasurement {
    #[serde(rename = "dataPoints")]
    pub data_points: Vec<models::DataPoint>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "units")]
    pub units: String,
}

impl DatabaseMeasurement {
    pub fn new(data_points: Vec<models::DataPoint>, name: String, units: String) -> DatabaseMeasurement {
        DatabaseMeasurement {
            data_points,
            name,
            units,
        }
    }
}
