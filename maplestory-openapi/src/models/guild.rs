// src/models/guild.rs
//! Models for Guild tag API endpoints.
//!
//! - `GET /maplestory/v1/guild/id` → [`GuildId`]
//! - `GET /maplestory/v1/guild/basic` → [`GuildBasic`]

use serde::{Deserialize, Serialize};

/// Response from `GET /maplestory/v1/guild/id`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildId {
    /// Unique open guild identifier (oguild_id)
    pub oguild_id: Option<String>,
}

/// Response from `GET /maplestory/v1/guild/basic`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildBasic {
    /// Query reference date (KST)
    pub date: Option<String>,
    /// World name
    pub world_name: Option<String>,
    /// Guild name
    pub guild_name: Option<String>,
    /// Guild level
    pub guild_level: Option<i64>,
    /// Guild fame (명성치)
    pub guild_fame: Option<i64>,
    /// Guild points (GP)
    pub guild_point: Option<i64>,
    /// Guild master character name
    pub guild_master_name: Option<String>,
    /// Guild member count
    pub guild_member_count: Option<i64>,
    /// Guild user count (same account characters grouped)
    pub guild_user_count: Option<i64>,
    /// List of guild member character names
    pub guild_member: Option<Vec<String>>,
    /// Guild common skills
    pub guild_skill: Option<Vec<GuildSkill>>,
    /// Guild noblesse skills (노블레스 스킬)
    pub guild_noblesse_skill: Option<Vec<GuildSkill>>,
    /// Guild mark / emblem
    pub guild_mark: Option<String>,
    /// Guild custom mark image
    pub guild_mark_custom: Option<String>,
}

/// A guild skill entry (regular skill or noblesse skill).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildSkill {
    /// Skill name
    pub skill_name: Option<String>,
    /// Skill description
    pub skill_description: Option<String>,
    /// Skill level
    pub skill_level: Option<i64>,
    /// Skill effect
    pub skill_effect: Option<String>,
    /// Skill icon URL
    pub skill_icon: Option<String>,
}

impl GuildBasic {
    /// Sum of all Noblesse skill points (노블레스 스킬 포인트 총합).
    pub fn total_noblesse_points(&self) -> i64 {
        self.guild_noblesse_skill
            .as_ref()
            .map(|skills| skills.iter().filter_map(|s| s.skill_level).sum())
            .unwrap_or(0)
    }

    /// Check if a specific character is in this guild.
    pub fn contains_member(&self, character_name: &str) -> bool {
        self.guild_member
            .as_ref()
            .map(|members| members.iter().any(|m| m == character_name))
            .unwrap_or(false)
    }
}
