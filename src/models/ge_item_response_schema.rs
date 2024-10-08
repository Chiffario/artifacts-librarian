//! The version of the OpenAPI document: 2
//!
//! Generated by: https://openapi-generator.tech

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeItemResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::GeItemSchema>,
}

impl GeItemResponseSchema {
    pub fn new(data: models::GeItemSchema) -> GeItemResponseSchema {
        GeItemResponseSchema {
            data: Box::new(data),
        }
    }
}
