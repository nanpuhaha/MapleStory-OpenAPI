// src/models/union.rs
//! Models for Union tag API endpoints.
//!
//! - `GET /maplestory/v1/user/union` → [`UserUnion`]
//! - `GET /maplestory/v1/user/union-raider` → [`UserUnionRaider`]
//! - `GET /maplestory/v1/user/union-artifact` → [`UserUnionArtifact`]

use serde::{Deserialize, Serialize};

// ─── Union Basic ─────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/user/union`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUnion {
    /// Query reference date (KST)
    pub date: Option<String>,
    /// Union level
    pub union_level: Option<i64>,
    /// Union grade (e.g. "그랜드 마스터 유니온 4")
    pub union_grade: Option<String>,
    /// Union artifact level
    pub union_artifact_level: Option<i64>,
    /// Union artifact current EXP
    pub union_artifact_exp: Option<i64>,
    /// Union artifact points
    pub union_artifact_point: Option<i64>,
}

// ─── Union Raider ────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/user/union-raider`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUnionRaider {
    /// Query reference date (KST)
    pub date: Option<String>,
    /// Raider character stats
    pub union_raider_stat: Option<Vec<String>>,
    /// Occupied area stats
    pub union_occupied_stat: Option<Vec<String>>,
    /// Placed blocks
    pub union_block: Option<Vec<UnionBlock>>,
    /// Inner board stats
    pub union_inner_stat: Option<Vec<UnionInnerStat>>,
    /// Currently active preset number
    pub use_preset_no: Option<i64>,
    /// Raider preset 1
    pub union_raider_preset_1: Option<UnionRaiderPreset>,
    /// Raider preset 2
    pub union_raider_preset_2: Option<UnionRaiderPreset>,
    /// Raider preset 3
    pub union_raider_preset_3: Option<UnionRaiderPreset>,
    /// Raider preset 4
    pub union_raider_preset_4: Option<UnionRaiderPreset>,
    /// Raider preset 5
    pub union_raider_preset_5: Option<UnionRaiderPreset>,
    /// Union state stats
    pub union_state_stat: Option<Vec<String>>,
    /// State stat presets
    pub union_state_stat_preset: Option<Vec<UnionStateStatPreset>>,
    /// Maximum available points
    pub union_max_point: Option<i64>,
}

/// Union board block placement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionBlock {
    pub block_type: Option<String>,
    pub block_class: Option<String>,
    pub block_level: Option<String>,
    pub block_control_point: Option<BlockPosition>,
    pub block_position: Option<Vec<BlockPosition>>,
}

/// 2D coordinate on the Union board.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockPosition {
    pub x: Option<i64>,
    pub y: Option<i64>,
}

/// Inner board stat field description.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionInnerStat {
    pub stat_field_id: Option<String>,
    pub stat_field_effect: Option<String>,
}

/// A Union raider preset slot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionRaiderPreset {
    pub union_raider_stat: Option<Vec<String>>,
    pub union_occupied_stat: Option<Vec<String>>,
    pub union_inner_stat: Option<Vec<UnionInnerStat>>,
    pub union_block: Option<Vec<UnionBlock>>,
}

/// A state stat preset entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionStateStatPreset {
    pub preset_no: Option<i64>,
    pub union_state_stat: Option<Vec<String>>,
}

// ─── Union Artifact ──────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/user/union-artifact`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUnionArtifact {
    /// Query reference date (KST)
    pub date: Option<String>,
    /// Active artifact effects
    pub union_artifact_effect: Option<Vec<UnionArtifactEffect>>,
    /// Artifact crystals
    pub union_artifact_crystal: Option<Vec<UnionArtifactCrystal>>,
    /// Remaining AP
    pub union_artifact_remain_ap: Option<i64>,
}

/// A single Union artifact effect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionArtifactEffect {
    /// Effect name and value description
    pub name: Option<String>,
    /// Effect level
    pub level: Option<i64>,
}

/// A Union artifact crystal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionArtifactCrystal {
    /// Crystal name (e.g. "크리스탈 : 주황")
    pub name: Option<String>,
    /// Validity flag ("0" = active, "1" = expired)
    pub validity_flag: Option<String>,
    /// Expiration date (KST)
    pub date_expire: Option<String>,
    /// Crystal level
    pub level: Option<i64>,
    /// Option 1 name
    pub crystal_option_name_1: Option<String>,
    /// Option 2 name
    pub crystal_option_name_2: Option<String>,
    /// Option 3 name
    pub crystal_option_name_3: Option<String>,
}
