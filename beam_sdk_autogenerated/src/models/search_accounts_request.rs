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
pub struct SearchAccountsRequest {
    #[serde(rename = "query")]
    pub query: String,
    #[serde(rename = "page")]
    pub page: i32,
    #[serde(rename = "pagesize")]
    pub pagesize: i32,
}

impl SearchAccountsRequest {
    pub fn new(query: String, page: i32, pagesize: i32) -> SearchAccountsRequest {
        SearchAccountsRequest {
            query,
            page,
            pagesize,
        }
    }
}
