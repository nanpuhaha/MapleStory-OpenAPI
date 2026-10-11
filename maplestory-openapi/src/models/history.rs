// src/models/history.rs
//! Models for History tag API endpoints.
//!
//! - `GET /maplestory/v1/history/starforce` → [`StarforceHistory`]
//! - `GET /maplestory/v1/history/potential` → [`PotentialHistory`]
//! - `GET /maplestory/v1/history/cube` → [`CubeHistory`]
//! - `GET /maplestory/v1/history/soul-potential` → [`SoulPotentialHistory`]

use serde::{Deserialize, Serialize};

// ─── Common Sub-types ─────────────────────────────────────────────────────────

/// A potential option entry (value and grade).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotentialOption {
    /// Option name / description
    pub value: Option<String>,
    /// Option grade
    pub grade: Option<String>,
}

/// Starforce event info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarforceEvent {
    /// Event success rate boost
    pub success_rate: Option<String>,
    /// Event destroy rate decrease
    pub destroy_decrease_rate: Option<String>,
    /// Event cost discount rate
    pub cost_discount_rate: Option<String>,
    /// Plus star value
    pub plus_value: Option<String>,
    /// Target starforce range for event
    pub starforce_event_range: Option<String>,
    /// Recovery cost discount rate
    pub recovery_cost_discount_rate: Option<String>,
}

// ─── Starforce History ───────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/history/starforce`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarforceHistory {
    /// Result count
    pub count: Option<i64>,
    /// Cursor for pagination
    pub next_cursor: Option<String>,
    /// Starforce history list
    pub starforce_history: Option<Vec<StarforceHistoryEntry>>,
}

/// A single starforce enhancement history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarforceHistoryEntry {
    /// History identifier
    pub id: Option<String>,
    /// Enhancement attempt result (성공, 실패(유지), 실패(하락), 파괴 등)
    pub item_upgrade_result: Option<String>,
    /// Starforce count before attempt
    pub before_starforce_count: Option<i64>,
    /// Starforce count after attempt
    pub after_starforce_count: Option<i64>,
    /// Starcatch result (성공, 실패, 미사용)
    pub starcatch_result: Option<String>,
    /// Superior item flag
    pub superior_item_flag: Option<String>,
    /// Destroy defence flag
    pub destroy_defence: Option<String>,
    /// Chance time flag
    pub chance_time: Option<String>,
    /// Event field flag
    pub event_field_flag: Option<String>,
    /// Upgrade item used
    pub upgrade_item: Option<String>,
    /// Protect shield used
    pub protect_shield: Option<String>,
    /// Bonus stat upgrade flag
    pub bonus_stat_upgrade: Option<String>,
    /// Character name
    pub character_name: Option<String>,
    /// World name
    pub world_name: Option<String>,
    /// Target equipment name
    pub target_item: Option<String>,
    /// Enhancement timestamp (KST)
    pub date_create: Option<String>,
    /// Active starforce events
    pub starforce_event_list: Option<Vec<StarforceEvent>>,
}

// ─── Potential History ───────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/history/potential`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotentialHistory {
    /// Result count
    pub count: Option<i64>,
    /// Cursor for pagination
    pub next_cursor: Option<String>,
    /// Potential reset history list
    pub potential_history: Option<Vec<PotentialHistoryEntry>>,
}

/// A single potential reset history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotentialHistoryEntry {
    /// History identifier
    pub id: Option<String>,
    /// Character name
    pub character_name: Option<String>,
    /// Timestamp (KST)
    pub date_create: Option<String>,
    /// Potential type (잠재능력, 에디셔널 잠재능력)
    pub potential_type: Option<String>,
    /// Reset result
    pub item_upgrade_result: Option<String>,
    /// Miracle time flag
    pub miracle_time_flag: Option<String>,
    /// Equipment part category
    pub item_equipment_part: Option<String>,
    /// Equipment level
    pub item_level: Option<i64>,
    /// Target equipment name
    pub target_item: Option<String>,
    /// Potential grade
    pub potential_option_grade: Option<String>,
    /// Additional potential grade
    pub additional_potential_option_grade: Option<String>,
    /// Upgrade guarantee flag (ceiling reached)
    pub upgrade_guarantee: Option<bool>,
    /// Current accumulated guarantee stack count
    pub upgrade_guarantee_count: Option<i64>,
    /// Potential options before reset
    pub before_potential_option: Option<Vec<PotentialOption>>,
    /// Additional potential options before reset
    pub before_additional_potential_option: Option<Vec<PotentialOption>>,
    /// Potential options after reset
    pub after_potential_option: Option<Vec<PotentialOption>>,
    /// Additional potential options after reset
    pub after_additional_potential_option: Option<Vec<PotentialOption>>,
}

// ─── Cube History ───────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/history/cube`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubeHistory {
    /// Result count
    pub count: Option<i64>,
    /// Cursor for pagination
    pub next_cursor: Option<String>,
    /// Cube usage history list
    pub cube_history: Option<Vec<CubeHistoryEntry>>,
}

/// A single cube/special consumable usage history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubeHistoryEntry {
    /// History identifier
    pub id: Option<String>,
    /// Character name
    pub character_name: Option<String>,
    /// Timestamp (KST)
    pub date_create: Option<String>,
    /// Cube/item type name
    pub cube_type: Option<String>,
    /// Usage result
    pub item_upgrade_result: Option<String>,
    /// Miracle time flag
    pub miracle_time_flag: Option<String>,
    /// Equipment part category
    pub item_equipment_part: Option<String>,
    /// Equipment level
    pub item_level: Option<i64>,
    /// Target equipment name
    pub target_item: Option<String>,
    /// Potential grade
    pub potential_option_grade: Option<String>,
    /// Additional potential grade
    pub additional_potential_option_grade: Option<String>,
    /// Upgrade guarantee flag (ceiling reached)
    pub upgrade_guarantee: Option<bool>,
    /// Current accumulated guarantee stack count
    pub upgrade_guarantee_count: Option<i64>,
    /// Potential options before reset
    pub before_potential_option: Option<Vec<PotentialOption>>,
    /// Additional potential options before reset
    pub before_additional_potential_option: Option<Vec<PotentialOption>>,
    /// Potential options after reset
    pub after_potential_option: Option<Vec<PotentialOption>>,
    /// Additional potential options after reset
    pub after_additional_potential_option: Option<Vec<PotentialOption>>,
}

// ─── Soul Potential History ─────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/history/soul-potential`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulPotentialHistory {
    /// Result count
    pub count: Option<i64>,
    /// Cursor for pagination
    pub next_cursor: Option<String>,
    /// Soul potential history list
    pub soul_potential_history: Option<Vec<SoulPotentialHistoryEntry>>,
}

/// A single soul potential reset history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulPotentialHistoryEntry {
    /// History identifier
    pub id: Option<String>,
    /// Character name
    pub character_name: Option<String>,
    /// Timestamp (KST)
    pub date_create: Option<String>,
    /// Reset result
    pub item_upgrade_result: Option<String>,
    /// Soul potential amplification grade
    pub soul_potential_amplified_grade: Option<i64>,
    /// Target equipment name
    pub target_item: Option<String>,
    /// Soul potential grade
    pub soul_potential_grade: Option<String>,
    /// Upgrade guarantee flag (ceiling reached)
    pub upgrade_guarantee: Option<bool>,
    /// Current accumulated guarantee stack count
    pub upgrade_guarantee_count: Option<i64>,
    /// Soul potential options before reset
    pub before_soul_potential_option: Option<Vec<PotentialOption>>,
    /// Soul potential options after reset
    pub after_soul_potential_option: Option<Vec<PotentialOption>>,
}
