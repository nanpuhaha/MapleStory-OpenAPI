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

// ─── Guild Profile (종합 길드 프로필) ──────────────────────────────────────────

/// Comprehensive aggregated guild profile.
///
/// Contains basic guild information, Noblesse skill analysis,
/// and optionally detailed basic information and combat power statistics for guild members.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildProfile {
    /// Basic guild information
    pub basic: GuildBasic,
    /// Total Noblesse skill points
    pub total_noblesse_points: i64,
    /// Active Noblesse skills
    pub noblesse_skills: Vec<GuildSkill>,
    /// Detailed basic info of fetched guild members (if requested)
    pub members: Vec<crate::models::character::CharacterBasic>,
    /// Combat powers of fetched active guild members (if requested)
    pub member_combat_powers: Vec<i64>,
}

impl GuildProfile {
    /// Construct a `GuildProfile` from `GuildBasic`, optional member basic profiles, and member combat powers.
    pub fn from_basic(
        basic: GuildBasic,
        members: Vec<crate::models::character::CharacterBasic>,
        member_combat_powers: Vec<i64>,
    ) -> Self {
        let total_noblesse_points = basic.total_noblesse_points();
        let noblesse_skills = basic.guild_noblesse_skill.clone().unwrap_or_default();
        Self {
            basic,
            total_noblesse_points,
            noblesse_skills,
            members,
            member_combat_powers,
        }
    }

    /// Returns the number of members whose character profiles were successfully fetched.
    pub fn fetched_member_count(&self) -> usize {
        self.members.len()
    }

    /// Average level of fetched members.
    pub fn average_member_level(&self) -> Option<f64> {
        let levels: Vec<i64> = self
            .members
            .iter()
            .filter_map(|m| m.character_level)
            .collect();
        if levels.is_empty() {
            None
        } else {
            let sum: i64 = levels.iter().sum();
            Some(sum as f64 / levels.len() as f64)
        }
    }

    /// Total combat power across fetched guild members.
    pub fn total_member_combat_power(&self) -> i64 {
        self.member_combat_powers.iter().sum()
    }

    /// Average combat power across fetched guild members.
    pub fn average_member_combat_power(&self) -> Option<f64> {
        if self.member_combat_powers.is_empty() {
            None
        } else {
            let sum: i64 = self.member_combat_powers.iter().sum();
            Some(sum as f64 / self.member_combat_powers.len() as f64)
        }
    }

    /// Highest combat power among fetched guild members.
    pub fn max_member_combat_power(&self) -> Option<i64> {
        self.member_combat_powers.iter().copied().max()
    }
}
