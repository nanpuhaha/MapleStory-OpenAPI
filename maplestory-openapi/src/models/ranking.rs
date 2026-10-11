// src/models/ranking.rs
//! Models for Ranking tag API endpoints.
//!
//! - `GET /maplestory/v1/ranking/overall` → [`OverallRanking`]
//! - `GET /maplestory/v1/ranking/union` → [`UnionRanking`]
//! - `GET /maplestory/v1/ranking/guild` → [`GuildRanking`]
//! - `GET /maplestory/v1/ranking/dojang` → [`DojangRanking`]
//! - `GET /maplestory/v1/ranking/theseed` → [`TheSeedRanking`]
//! - `GET /maplestory/v1/ranking/achievement` → [`AchievementRanking`]

use serde::{Deserialize, Serialize};

// ─── Overall Ranking ─────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/ranking/overall`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallRanking {
    /// Ranking list
    pub ranking: Option<Vec<OverallRankingEntry>>,
}

/// A single entry in overall ranking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverallRankingEntry {
    /// Ranking update date (KST)
    pub date: Option<String>,
    /// Ranking placement
    pub ranking: Option<i32>,
    /// Character name
    pub character_name: Option<String>,
    /// World name
    pub world_name: Option<String>,
    /// Class name
    pub class_name: Option<String>,
    /// Sub class name
    pub sub_class_name: Option<String>,
    /// Character level
    pub character_level: Option<i32>,
    /// Character EXP
    pub character_exp: Option<i64>,
    /// Character popularity
    pub character_popularity: Option<i32>,
    /// Character guild name
    pub character_guildname: Option<String>,
}

/// Query parameters for `GET /maplestory/v1/ranking/overall`.
#[derive(Debug, Clone, Default)]
pub struct OverallRankingQuery<'a> {
    /// Target date `"YYYY-MM-DD"` (KST)
    pub date: Option<&'a str>,
    /// World name
    pub world_name: Option<&'a str>,
    /// World type (0: 일반, 1: 리부트)
    pub world_type: Option<i32>,
    /// Class / Job name
    pub class: Option<&'a str>,
    /// Character identifier
    pub ocid: Option<&'a str>,
    /// Page number (default: 1)
    pub page: Option<u32>,
}

impl<'a> OverallRankingQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn date(mut self, date: &'a str) -> Self {
        self.date = Some(date);
        self
    }

    pub fn world_name(mut self, world_name: &'a str) -> Self {
        self.world_name = Some(world_name);
        self
    }

    pub fn world_type(mut self, world_type: i32) -> Self {
        self.world_type = Some(world_type);
        self
    }

    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    pub fn ocid(mut self, ocid: &'a str) -> Self {
        self.ocid = Some(ocid);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}

// ─── Union Ranking ───────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/ranking/union`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionRanking {
    /// Ranking list
    pub ranking: Option<Vec<UnionRankingEntry>>,
}

/// A single entry in union ranking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionRankingEntry {
    /// Ranking update date (KST)
    pub date: Option<String>,
    /// Ranking placement
    pub ranking: Option<i32>,
    /// Character name
    pub character_name: Option<String>,
    /// World name
    pub world_name: Option<String>,
    /// Class name
    pub class_name: Option<String>,
    /// Sub class name
    pub sub_class_name: Option<String>,
    /// Union level
    pub union_level: Option<i32>,
    /// Union power
    pub union_power: Option<i64>,
}

/// Query parameters for `GET /maplestory/v1/ranking/union`.
#[derive(Debug, Clone, Default)]
pub struct UnionRankingQuery<'a> {
    /// Target date `"YYYY-MM-DD"` (KST)
    pub date: Option<&'a str>,
    /// World name
    pub world_name: Option<&'a str>,
    /// Character identifier
    pub ocid: Option<&'a str>,
    /// Page number (default: 1)
    pub page: Option<u32>,
}

impl<'a> UnionRankingQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn date(mut self, date: &'a str) -> Self {
        self.date = Some(date);
        self
    }

    pub fn world_name(mut self, world_name: &'a str) -> Self {
        self.world_name = Some(world_name);
        self
    }

    pub fn ocid(mut self, ocid: &'a str) -> Self {
        self.ocid = Some(ocid);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}

// ─── Guild Ranking ───────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/ranking/guild`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildRanking {
    /// Ranking list
    pub ranking: Option<Vec<GuildRankingEntry>>,
}

/// A single entry in guild ranking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildRankingEntry {
    /// Ranking update date (KST)
    pub date: Option<String>,
    /// Ranking placement
    pub ranking: Option<i32>,
    /// Guild name
    pub guild_name: Option<String>,
    /// World name
    pub world_name: Option<String>,
    /// Guild level
    pub guild_level: Option<i32>,
    /// Guild master character name
    pub guild_master_name: Option<String>,
    /// Guild mark image URL or data
    pub guild_mark: Option<String>,
    /// Guild point
    pub guild_point: Option<i64>,
}

/// Query parameters for `GET /maplestory/v1/ranking/guild`.
#[derive(Debug, Clone)]
pub struct GuildRankingQuery<'a> {
    /// Target date `"YYYY-MM-DD"` (KST)
    pub date: Option<&'a str>,
    /// Ranking type (0: 주간 명성치, 1: 플래그 레이스, 2: 지하 수로)
    pub ranking_type: u32,
    /// World name
    pub world_name: Option<&'a str>,
    /// Guild name
    pub guild_name: Option<&'a str>,
    /// Page number (default: 1)
    pub page: Option<u32>,
}

impl<'a> GuildRankingQuery<'a> {
    pub fn new(ranking_type: u32) -> Self {
        Self {
            date: None,
            ranking_type,
            world_name: None,
            guild_name: None,
            page: None,
        }
    }

    pub fn date(mut self, date: &'a str) -> Self {
        self.date = Some(date);
        self
    }

    pub fn world_name(mut self, world_name: &'a str) -> Self {
        self.world_name = Some(world_name);
        self
    }

    pub fn guild_name(mut self, guild_name: &'a str) -> Self {
        self.guild_name = Some(guild_name);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}

// ─── Dojang Ranking ──────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/ranking/dojang`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DojangRanking {
    /// Ranking list
    pub ranking: Option<Vec<DojangRankingEntry>>,
}

/// A single entry in dojang ranking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DojangRankingEntry {
    /// Ranking update date (KST)
    pub date: Option<String>,
    /// Ranking placement
    pub ranking: Option<i32>,
    /// Character name
    pub character_name: Option<String>,
    /// World name
    pub world_name: Option<String>,
    /// Class name
    pub class_name: Option<String>,
    /// Sub class name
    pub sub_class_name: Option<String>,
    /// Character level
    pub character_level: Option<i32>,
    /// Dojang floor reached
    pub dojang_floor: Option<i32>,
    /// Dojang clear time record in seconds
    pub dojang_time_record: Option<i32>,
}

/// Query parameters for `GET /maplestory/v1/ranking/dojang`.
#[derive(Debug, Clone)]
pub struct DojangRankingQuery<'a> {
    /// Target date `"YYYY-MM-DD"` (KST)
    pub date: Option<&'a str>,
    /// Difficulty (0: 통달, 1: 입문)
    pub difficulty: u32,
    /// World name
    pub world_name: Option<&'a str>,
    /// Class / Job name
    pub class: Option<&'a str>,
    /// Character identifier
    pub ocid: Option<&'a str>,
    /// Page number (default: 1)
    pub page: Option<u32>,
}

impl<'a> DojangRankingQuery<'a> {
    pub fn new(difficulty: u32) -> Self {
        Self {
            date: None,
            difficulty,
            world_name: None,
            class: None,
            ocid: None,
            page: None,
        }
    }

    pub fn date(mut self, date: &'a str) -> Self {
        self.date = Some(date);
        self
    }

    pub fn world_name(mut self, world_name: &'a str) -> Self {
        self.world_name = Some(world_name);
        self
    }

    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    pub fn ocid(mut self, ocid: &'a str) -> Self {
        self.ocid = Some(ocid);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}

// ─── The Seed Ranking ────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/ranking/theseed`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheSeedRanking {
    /// Ranking list
    pub ranking: Option<Vec<TheSeedRankingEntry>>,
}

/// A single entry in the seed ranking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheSeedRankingEntry {
    /// Ranking update date (KST)
    pub date: Option<String>,
    /// Ranking placement
    pub ranking: Option<i32>,
    /// Character name
    pub character_name: Option<String>,
    /// World name
    pub world_name: Option<String>,
    /// Class name
    pub class_name: Option<String>,
    /// Sub class name
    pub sub_class_name: Option<String>,
    /// Character level
    pub character_level: Option<i32>,
    /// The Seed floor reached
    pub theseed_floor: Option<i32>,
    /// The Seed clear time record in seconds
    pub theseed_time_record: Option<i32>,
}

/// Query parameters for `GET /maplestory/v1/ranking/theseed`.
#[derive(Debug, Clone, Default)]
pub struct TheSeedRankingQuery<'a> {
    /// Target date `"YYYY-MM-DD"` (KST)
    pub date: Option<&'a str>,
    /// World name
    pub world_name: Option<&'a str>,
    /// Character identifier
    pub ocid: Option<&'a str>,
    /// Page number (default: 1)
    pub page: Option<u32>,
}

impl<'a> TheSeedRankingQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn date(mut self, date: &'a str) -> Self {
        self.date = Some(date);
        self
    }

    pub fn world_name(mut self, world_name: &'a str) -> Self {
        self.world_name = Some(world_name);
        self
    }

    pub fn ocid(mut self, ocid: &'a str) -> Self {
        self.ocid = Some(ocid);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}

// ─── Achievement Ranking ─────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/ranking/achievement`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementRanking {
    /// Ranking list
    pub ranking: Option<Vec<AchievementRankingEntry>>,
}

/// A single entry in achievement ranking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementRankingEntry {
    /// Ranking update date (KST)
    pub date: Option<String>,
    /// Ranking placement
    pub ranking: Option<i32>,
    /// Character name
    pub character_name: Option<String>,
    /// World name
    pub world_name: Option<String>,
    /// Class name
    pub class_name: Option<String>,
    /// Sub class name
    pub sub_class_name: Option<String>,
    /// Trophy grade
    pub trophy_grade: Option<String>,
    /// Trophy score
    pub trophy_score: Option<i32>,
}

/// Query parameters for `GET /maplestory/v1/ranking/achievement`.
#[derive(Debug, Clone, Default)]
pub struct AchievementRankingQuery<'a> {
    /// Target date `"YYYY-MM-DD"` (KST)
    pub date: Option<&'a str>,
    /// Character identifier
    pub ocid: Option<&'a str>,
    /// Page number (default: 1)
    pub page: Option<u32>,
}

impl<'a> AchievementRankingQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn date(mut self, date: &'a str) -> Self {
        self.date = Some(date);
        self
    }

    pub fn ocid(mut self, ocid: &'a str) -> Self {
        self.ocid = Some(ocid);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}
