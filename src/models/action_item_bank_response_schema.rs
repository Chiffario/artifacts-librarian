//! The version of the OpenAPI document: 1.6
//!
//! Generated by: https://openapi-generator.tech

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionItemBankResponseSchema {
    #[serde(rename = "data")]
    pub data: Box<models::BankItemSchema>,
}

impl ActionItemBankResponseSchema {
    pub fn new(data: models::BankItemSchema) -> ActionItemBankResponseSchema {
        ActionItemBankResponseSchema {
            data: Box::new(data),
        }
    }
}
