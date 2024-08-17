//! The version of the OpenAPI document: 1.6
//!
//! Generated by: https://openapi-generator.tech

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeTransactionSchema {
    /// Item code.
    #[serde(rename = "code")]
    pub code: String,
    /// Item quantity.
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// Item price.
    #[serde(rename = "price")]
    pub price: i32,
    /// Total price of the transaction.
    #[serde(rename = "total_price")]
    pub total_price: i32,
}

impl GeTransactionSchema {
    pub fn new(code: String, quantity: i32, price: i32, total_price: i32) -> GeTransactionSchema {
        GeTransactionSchema {
            code,
            quantity,
            price,
            total_price,
        }
    }
}
