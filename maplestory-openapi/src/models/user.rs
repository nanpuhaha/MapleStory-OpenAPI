// src/models/user.rs
//! Models for User tag API endpoints.
//!
//! - `GET /maplestory/v1/character/list` → [`CharacterList`]
//! - `GET /maplestory/v1/user/achievement` → [`Achievement`]

use serde::{Deserialize, Serialize};

// ─── Character List ───────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/list`.
///
/// Returns the list of characters belonging to the authenticated account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterList {
    /// Account information
    pub account_list: Option<Vec<AccountInfo>>,
}

/// Account information containing characters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    /// Account identifier
    pub account_id: Option<String>,
    /// Characters belonging to this account
    pub character_list: Option<Vec<CharacterListItem>>,
}

/// A single character entry in the character list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterListItem {
    /// Character name
    pub character_name: Option<String>,
    /// World name
    pub world_name: Option<String>,
    /// Character class
    pub character_class: Option<String>,
    /// Character level
    pub character_level: Option<i64>,
}

// ─── Achievement ─────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/user/achievement`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    /// Query reference date (KST)
    pub date: Option<String>,
    /// Total achievement score
    pub achievement_point: Option<i64>,
    /// Achievement rank (global)
    pub achievement_ranking: Option<i64>,
    /// List of achieved achievements
    pub achievement_list: Option<Vec<AchievementItem>>,
}

/// A single achievement entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementItem {
    /// Achievement title
    pub achievement_title: Option<String>,
    /// Achievement grade
    pub achievement_grade: Option<String>,
    /// Achievement score
    pub achievement_score: Option<i64>,
    /// Whether the achievement is completed
    pub is_completed: Option<bool>,
    /// Date achievement was completed (KST)
    pub date_completed: Option<String>,
}
