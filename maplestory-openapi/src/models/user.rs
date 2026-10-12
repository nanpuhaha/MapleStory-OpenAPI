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

// ─── OUID ─────────────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/ouid` and `GET /maplestory/legacy/ouid`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Account identifier (ouid)
    pub ouid: Option<String>,
}

// ─── Account Summary (종합 계정 요약) ──────────────────────────────────────────

/// Comprehensive aggregated account summary.
///
/// Combines the account's character roster, achievement statistics,
/// and main character Union progression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSummary {
    /// Characters belonging to the account
    pub character_list: CharacterList,
    /// Achievement statistics for the account
    pub achievement: Option<Achievement>,
    /// Union statistics for the main (highest-level) character
    pub union: Option<crate::models::union::UserUnion>,
}

impl AccountSummary {
    /// Construct an `AccountSummary` from its constituent responses.
    pub fn from_parts(
        character_list: CharacterList,
        achievement: Option<Achievement>,
        union: Option<crate::models::union::UserUnion>,
    ) -> Self {
        Self {
            character_list,
            achievement,
            union,
        }
    }

    /// Total count of characters across all worlds and accounts.
    pub fn total_character_count(&self) -> usize {
        self.character_list
            .account_list
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter_map(|acc| acc.character_list.as_deref())
            .map(|list| list.len())
            .sum()
    }

    /// Finds the highest level character in the account roster.
    pub fn highest_level_character(&self) -> Option<&CharacterListItem> {
        self.character_list
            .account_list
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter_map(|acc| acc.character_list.as_deref())
            .flatten()
            .max_by_key(|c| c.character_level.unwrap_or(0))
    }

    /// Total achievement score if available.
    pub fn achievement_points(&self) -> Option<i64> {
        self.achievement.as_ref().and_then(|a| a.achievement_point)
    }

    /// Union level of the main (highest-level) character if available.
    pub fn main_character_union_level(&self) -> Option<i64> {
        self.union.as_ref().and_then(|u| u.union_level)
    }

    /// Union grade of the main (highest-level) character if available.
    pub fn main_character_union_grade(&self) -> Option<&str> {
        self.union.as_ref().and_then(|u| u.union_grade.as_deref())
    }
}
