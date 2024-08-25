//! The version of the OpenAPI document: 2
//!
//! Generated by: https://openapi-generator.tech

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquipRequestSchema {
    /// Cooldown details.
    #[serde(rename = "cooldown")]
    pub cooldown: Box<models::CooldownSchema>,
    /// Item slot.
    #[serde(rename = "slot")]
    pub slot: Slot,
    /// Item details.
    #[serde(rename = "item")]
    pub item: Box<models::ItemSchema>,
    /// Player details.
    #[serde(rename = "character")]
    pub character: Box<models::CharacterSchema>,
}

impl EquipRequestSchema {
    pub fn new(
        cooldown: models::CooldownSchema,
        slot: Slot,
        item: models::ItemSchema,
        character: models::CharacterSchema,
    ) -> EquipRequestSchema {
        EquipRequestSchema {
            cooldown: Box::new(cooldown),
            slot,
            item: Box::new(item),
            character: Box::new(character),
        }
    }
}
/// Item slot.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Slot {
    #[serde(rename = "weapon")]
    Weapon,
    #[serde(rename = "shield")]
    Shield,
    #[serde(rename = "helmet")]
    Helmet,
    #[serde(rename = "body_armor")]
    BodyArmor,
    #[serde(rename = "leg_armor")]
    LegArmor,
    #[serde(rename = "boots")]
    Boots,
    #[serde(rename = "ring1")]
    Ring1,
    #[serde(rename = "ring2")]
    Ring2,
    #[serde(rename = "amulet")]
    Amulet,
    #[serde(rename = "artifact1")]
    Artifact1,
    #[serde(rename = "artifact2")]
    Artifact2,
    #[serde(rename = "artifact3")]
    Artifact3,
    #[serde(rename = "consumable1")]
    Consumable1,
    #[serde(rename = "consumable2")]
    Consumable2,
}

impl Default for Slot {
    fn default() -> Slot {
        Self::Weapon
    }
}
