//! The version of the OpenAPI document: 2
//!
//! Generated by: https://openapi-generator.tech

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CooldownSchema {
    /// The total seconds of the cooldown.
    #[serde(rename = "total_seconds")]
    pub total_seconds: i32,
    /// The remaining seconds of the cooldown.
    #[serde(rename = "remaining_seconds")]
    pub remaining_seconds: i32,
    /// The start of the cooldown.
    #[serde(rename = "started_at")]
    pub started_at: String,
    /// The expiration of the cooldown.
    #[serde(rename = "expiration")]
    pub expiration: String,
    /// The reason of the cooldown.
    #[serde(rename = "reason")]
    pub reason: Reason,
}

impl CooldownSchema {
    pub fn new(
        total_seconds: i32,
        remaining_seconds: i32,
        started_at: String,
        expiration: String,
        reason: Reason,
    ) -> CooldownSchema {
        CooldownSchema {
            total_seconds,
            remaining_seconds,
            started_at,
            expiration,
            reason,
        }
    }
}
/// The reason of the cooldown.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Reason {
    #[serde(rename = "movement")]
    Movement,
    #[serde(rename = "fight")]
    Fight,
    #[serde(rename = "crafting")]
    Crafting,
    #[serde(rename = "gathering")]
    Gathering,
    #[serde(rename = "buy_ge")]
    BuyGe,
    #[serde(rename = "sell_ge")]
    SellGe,
    #[serde(rename = "delete_item")]
    DeleteItem,
    #[serde(rename = "deposit_bank")]
    DepositBank,
    #[serde(rename = "withdraw_bank")]
    WithdrawBank,
    #[serde(rename = "equip")]
    Equip,
    #[serde(rename = "unequip")]
    Unequip,
    #[serde(rename = "task")]
    Task,
    #[serde(rename = "recycling")]
    Recycling,
}

impl Default for Reason {
    fn default() -> Reason {
        Self::Movement
    }
}
