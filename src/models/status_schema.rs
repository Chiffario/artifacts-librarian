//! The version of the OpenAPI document: 2
//!
//! Generated by: https://openapi-generator.tech

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusSchema {
    /// Server status
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Maximum level.
    #[serde(rename = "max_level")]
    pub max_level: i32,
    /// Characters online.
    #[serde(rename = "characters_online")]
    pub characters_online: i32,
    /// Server time.
    #[serde(rename = "server_time")]
    pub server_time: String,
    /// Server announcements.
    #[serde(rename = "announcements")]
    pub announcements: Vec<models::AnnouncementSchema>,
    /// Last server wipe.
    #[serde(rename = "last_wipe")]
    pub last_wipe: String,
    /// Next server wipe.
    #[serde(rename = "next_wipe")]
    pub next_wipe: String,
}

impl StatusSchema {
    pub fn new(
        status: String,
        max_level: i32,
        characters_online: i32,
        server_time: String,
        announcements: Vec<models::AnnouncementSchema>,
        last_wipe: String,
        next_wipe: String,
    ) -> StatusSchema {
        StatusSchema {
            status,
            version: None,
            max_level,
            characters_online,
            server_time,
            announcements,
            last_wipe,
            next_wipe,
        }
    }
}
