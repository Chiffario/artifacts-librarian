//! The version of the OpenAPI document: 1.6
//!
//! Generated by: https://openapi-generator.tech

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockedHitsSchema {
    /// The amount of fire hits blocked.
    #[serde(rename = "fire")]
    pub fire: i32,
    /// The amount of earth hits blocked.
    #[serde(rename = "earth")]
    pub earth: i32,
    /// The amount of water hits blocked.
    #[serde(rename = "water")]
    pub water: i32,
    /// The amount of air hits blocked.
    #[serde(rename = "air")]
    pub air: i32,
    /// The amount of total hits blocked.
    #[serde(rename = "total")]
    pub total: i32,
}

impl BlockedHitsSchema {
    pub fn new(fire: i32, earth: i32, water: i32, air: i32, total: i32) -> BlockedHitsSchema {
        BlockedHitsSchema {
            fire,
            earth,
            water,
            air,
            total,
        }
    }
}
