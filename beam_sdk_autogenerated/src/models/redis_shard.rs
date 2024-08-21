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
pub struct RedisShard {
    #[serde(rename = "shardId")]
    pub shard_id: i32,
    #[serde(rename = "masterHost")]
    pub master_host: String,
    #[serde(rename = "slaveHosts")]
    pub slave_hosts: Vec<String>,
}

impl RedisShard {
    pub fn new(shard_id: i32, master_host: String, slave_hosts: Vec<String>) -> RedisShard {
        RedisShard {
            shard_id,
            master_host,
            slave_hosts,
        }
    }
}
