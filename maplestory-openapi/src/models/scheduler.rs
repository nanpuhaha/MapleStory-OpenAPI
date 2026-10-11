// src/models/scheduler.rs
//! Models for Scheduler tag API endpoints.
//!
//! - `GET /maplestory/v1/scheduler/character-state` → [`CharacterStateResponse`]

use serde::{Deserialize, Serialize};

/// Response from `GET /maplestory/v1/scheduler/character-state`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterStateResponse {
    /// Reference date `"YYYY-MM-DD"` (KST)
    pub date: Option<String>,
    /// Character name
    pub character_name: Option<String>,
    /// World name
    pub world_name: Option<String>,
    /// Character level
    pub character_level: Option<i64>,
    /// Character class
    pub character_class: Option<String>,
    /// Daily content schedule info
    pub daily_contents: Option<Vec<SchedulerDailyContent>>,
    /// Weekly content schedule info
    pub weekly_contents: Option<Vec<SchedulerWeeklyContent>>,
    /// Boss content schedule info
    pub boss_contents: Option<Vec<SchedulerBossContent>>,
    /// Number of weekly bosses cleared
    pub weekly_boss_clear_count: Option<i64>,
    /// Weekly boss clear limit
    pub weekly_boss_clear_limit_count: Option<i64>,
}

/// Daily content / quest status in scheduler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerDailyContent {
    /// Content or quest name
    pub content_name: Option<String>,
    /// Content type ('contents' or 'quest')
    pub r#type: Option<String>,
    /// Whether registered in in-game scheduler ("true" / "false")
    pub registration_flag: Option<String>,
    /// Current completion count / points
    pub now_count: Option<i64>,
    /// Maximum completion count / points
    pub max_count: Option<i64>,
    /// Quest state ("0": other, "1": in-progress, "2": completed)
    pub quest_state: Option<String>,
}

/// Weekly content / quest status in scheduler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerWeeklyContent {
    /// Content or quest name
    pub content_name: Option<String>,
    /// Content type ('contents' or 'quest')
    pub r#type: Option<String>,
    /// Whether registered in in-game scheduler ("true" / "false")
    pub registration_flag: Option<String>,
    /// Current completion count / points
    pub now_count: Option<i64>,
    /// Maximum completion count / points
    pub max_count: Option<i64>,
    /// Quest state ("0": other, "1": in-progress, "2": completed)
    pub quest_state: Option<String>,
}

/// Boss content status in scheduler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerBossContent {
    /// Boss name
    pub content_name: Option<String>,
    /// Boss difficulty
    pub difficulty: Option<String>,
    /// Reset cycle (일간, 주간, 월간 등)
    pub cycle: Option<String>,
    /// Display order number
    pub list_order_no: Option<i64>,
    /// Whether registered in in-game scheduler ("true" / "false")
    pub registration_flag: Option<String>,
    /// Whether cleared/completed ("true" / "false")
    pub complete_flag: Option<String>,
}
