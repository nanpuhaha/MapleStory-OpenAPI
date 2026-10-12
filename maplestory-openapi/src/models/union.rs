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

impl UserUnionArtifact {
    /// Returns true if the artifact currently has active effects.
    ///
    /// If crystals are present, at least one crystal must not be expired.
    pub fn is_active(&self) -> bool {
        if let Some(crystals) = self.union_artifact_crystal.as_deref() {
            if !crystals.is_empty() {
                return crystals.iter().any(|c| !c.is_expired());
            }
        }
        true
    }

    /// Returns an iterator over non-empty, active artifact effects.
    ///
    /// Yields an empty iterator if all artifact crystals are expired.
    pub fn active_effects(&self) -> impl Iterator<Item = &UnionArtifactEffect> {
        let active = self.is_active();
        self.union_artifact_effect
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter(move |e| active && e.name.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false))
    }

    /// Returns all crystals whose validity flag is expired ("1") or expiration timestamp has passed.
    pub fn expired_crystals(&self) -> Vec<&UnionArtifactCrystal> {
        self.union_artifact_crystal
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter(|c| c.is_expired())
            .collect()
    }

    /// Returns all active crystals.
    pub fn active_crystals(&self) -> Vec<&UnionArtifactCrystal> {
        self.union_artifact_crystal
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter(|c| !c.is_expired())
            .collect()
    }

    /// Parse and summarize active bonus numbers across all active artifact effects.
    ///
    /// Correctly parses multi-word stat names and filters out expired artifact effects.
    ///
    /// E.g.
    /// - `"올스탯 150 증가"` -> `{"올스탯": 150.0}`
    /// - `"공격력/마력 30 증가"` -> `{"공격력": 30.0, "마력": 30.0}`
    /// - `"보스 몬스터 공격 시 데미지 15.00% 증가"` -> `{"보스 몬스터 공격 시 데미지": 15.0}`
    pub fn total_artifact_stats(&self) -> std::collections::HashMap<String, f64> {
        let mut totals = std::collections::HashMap::new();

        for effect in self.active_effects() {
            if let Some(desc) = effect.name.as_deref() {
                // Split multiple clauses by comma: e.g. "올스탯 150 증가, 공격력/마력 30 증가"
                for part in desc.split(',') {
                    let clause = part.trim();
                    if clause.is_empty() {
                        continue;
                    }
                    let cleaned = clause.trim_end_matches("증가").trim_end_matches("감소").trim();
                    if let Some(last_space) = cleaned.rfind(' ') {
                        let stat_name = cleaned[..last_space].trim();
                        let num_part = cleaned[last_space + 1..].trim();
                        let num_str = num_part.trim_end_matches('%').trim_end_matches('초');
                        if let Ok(val) = num_str.parse::<f64>() {
                            if stat_name == "공격력/마력" {
                                *totals.entry("공격력".to_string()).or_insert(0.0) += val;
                                *totals.entry("마력".to_string()).or_insert(0.0) += val;
                            } else if !stat_name.is_empty() {
                                *totals.entry(stat_name.to_string()).or_insert(0.0) += val;
                            }
                        }
                    }
                }
            }
        }

        totals
    }

    /// Parse and summarize active bonus numbers across all active artifact effects,
    /// keyed by the strongly-typed [`StatType`](crate::models::character::StatType) domain enum.
    pub fn total_artifact_stats_typed(
        &self,
    ) -> std::collections::HashMap<crate::models::character::StatType, f64> {
        use crate::models::character::StatType;
        let mut typed_totals = std::collections::HashMap::new();
        for (stat_name, val) in self.total_artifact_stats() {
            let st = StatType::from_stat_name(&stat_name).unwrap_or(StatType::Other);
            *typed_totals.entry(st).or_insert(0.0) += val;
        }
        typed_totals
    }
}

impl UnionArtifactCrystal {
    /// Returns true if this crystal is expired according to its `validity_flag` ("1")
    /// or if its `date_expire` timestamp has already passed.
    pub fn is_expired(&self) -> bool {
        if self.validity_flag.as_deref() == Some("1") {
            return true;
        }

        if let Some(date_str) = self.date_expire.as_deref() {
            if let Ok(expire_dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
                if expire_dt.to_utc() < chrono::Utc::now() {
                    return true;
                }
            } else if let Ok(expire_date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                let kst_today = chrono::Utc::now()
                    .with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap())
                    .date_naive();
                if expire_date < kst_today {
                    return true;
                }
            }
        }

        false
    }
}

// ─── Union Champion ──────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/user/union-champion`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUnionChampion {
    /// Query reference date (KST)
    pub date: Option<String>,
    /// Union champion list
    pub union_champion: Option<Vec<UnionChampionInfo>>,
    /// Champion badge total effects
    pub champion_badge_total_info: Option<Vec<UnionChampionBadgeInfo>>,
}

/// A Union champion entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionChampionInfo {
    /// Champion character name
    pub champion_name: Option<String>,
    /// Champion slot index
    pub champion_slot: Option<i64>,
    /// Champion grade
    pub champion_grade: Option<String>,
    /// Champion character class
    pub champion_class: Option<String>,
    /// Champion badge info
    pub champion_badge_info: Option<Vec<UnionChampionBadgeInfo>>,
}

/// Union champion badge effect information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnionChampionBadgeInfo {
    /// Badge stat / effect description
    pub stat: Option<String>,
}
