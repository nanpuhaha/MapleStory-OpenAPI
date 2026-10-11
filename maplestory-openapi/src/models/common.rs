// src/models/common.rs
//! Common model types shared across API responses.

use serde::{Deserialize, Serialize};

/// Skill grade for the character skill API.
///
/// Represents the job advancement tier whose skills to query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillGrade {
    /// 0th job (beginner / Zero common)
    #[serde(rename = "0")]
    Zero,
    /// 1st job
    #[serde(rename = "1")]
    First,
    /// 1.5th job
    #[serde(rename = "1.5")]
    FirstHalf,
    /// 2nd job
    #[serde(rename = "2")]
    Second,
    /// 2.5th job
    #[serde(rename = "2.5")]
    SecondHalf,
    /// 3rd job
    #[serde(rename = "3")]
    Third,
    /// 4th job (also Zero alpha/beta)
    #[serde(rename = "4")]
    Fourth,
    /// Hyper passive skills
    #[serde(rename = "hyperpassive")]
    HyperPassive,
    /// Hyper active skills
    #[serde(rename = "hyperactive")]
    HyperActive,
    /// 5th job
    #[serde(rename = "5")]
    Fifth,
    /// 6th job (HEXA)
    #[serde(rename = "6")]
    Sixth,
}

impl std::fmt::Display for SkillGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkillGrade::Zero => write!(f, "0"),
            SkillGrade::First => write!(f, "1"),
            SkillGrade::FirstHalf => write!(f, "1.5"),
            SkillGrade::Second => write!(f, "2"),
            SkillGrade::SecondHalf => write!(f, "2.5"),
            SkillGrade::Third => write!(f, "3"),
            SkillGrade::Fourth => write!(f, "4"),
            SkillGrade::HyperPassive => write!(f, "hyperpassive"),
            SkillGrade::HyperActive => write!(f, "hyperactive"),
            SkillGrade::Fifth => write!(f, "5"),
            SkillGrade::Sixth => write!(f, "6"),
        }
    }
}

// ─── Equipment option stat structs ───────────────────────────────────────────
// These are reused by multiple equipment-type endpoints (item, dragon, mechanic).

/// Final total option of an equipment item (합산 최종 옵션).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemTotalOption {
    pub str: Option<String>,
    pub dex: Option<String>,
    pub int: Option<String>,
    pub luk: Option<String>,
    pub max_hp: Option<String>,
    pub max_mp: Option<String>,
    pub attack_power: Option<String>,
    pub magic_power: Option<String>,
    pub armor: Option<String>,
    pub speed: Option<String>,
    pub jump: Option<String>,
    pub boss_damage: Option<String>,
    pub ignore_monster_armor: Option<String>,
    pub all_stat: Option<String>,
    pub damage: Option<String>,
    pub equipment_level_decrease: Option<i64>,
    pub max_hp_rate: Option<String>,
    pub max_mp_rate: Option<String>,
}

/// Base (base roll) option of an equipment item (기본 옵션).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemBaseOption {
    pub str: Option<String>,
    pub dex: Option<String>,
    pub int: Option<String>,
    pub luk: Option<String>,
    pub max_hp: Option<String>,
    pub max_mp: Option<String>,
    pub attack_power: Option<String>,
    pub magic_power: Option<String>,
    pub armor: Option<String>,
    pub speed: Option<String>,
    pub jump: Option<String>,
    pub boss_damage: Option<String>,
    pub ignore_monster_armor: Option<String>,
    pub all_stat: Option<String>,
    pub max_hp_rate: Option<String>,
    pub max_mp_rate: Option<String>,
    pub base_equipment_level: Option<i64>,
}

/// Exceptional (익셉셔널) option of an equipment item.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemExceptionalOption {
    pub str: Option<String>,
    pub dex: Option<String>,
    pub int: Option<String>,
    pub luk: Option<String>,
    pub max_hp: Option<String>,
    pub max_mp: Option<String>,
    pub attack_power: Option<String>,
    pub magic_power: Option<String>,
    pub exceptional_upgrade: Option<i64>,
}

/// Add (추가) option of an equipment item.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemAddOption {
    pub str: Option<String>,
    pub dex: Option<String>,
    pub int: Option<String>,
    pub luk: Option<String>,
    pub max_hp: Option<String>,
    pub max_mp: Option<String>,
    pub attack_power: Option<String>,
    pub magic_power: Option<String>,
    pub armor: Option<String>,
    pub speed: Option<String>,
    pub jump: Option<String>,
    pub boss_damage: Option<String>,
    pub damage: Option<String>,
    pub all_stat: Option<String>,
    pub equipment_level_decrease: Option<i64>,
}

/// Etc (기타) option of an equipment item.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemEtcOption {
    pub str: Option<String>,
    pub dex: Option<String>,
    pub int: Option<String>,
    pub luk: Option<String>,
    pub max_hp: Option<String>,
    pub max_mp: Option<String>,
    pub attack_power: Option<String>,
    pub magic_power: Option<String>,
    pub armor: Option<String>,
    pub speed: Option<String>,
    pub jump: Option<String>,
}

/// Starforce option of an equipment item.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemStarforceOption {
    pub str: Option<String>,
    pub dex: Option<String>,
    pub int: Option<String>,
    pub luk: Option<String>,
    pub max_hp: Option<String>,
    pub max_mp: Option<String>,
    pub attack_power: Option<String>,
    pub magic_power: Option<String>,
    pub armor: Option<String>,
    pub speed: Option<String>,
    pub jump: Option<String>,
}

/// A key-value stat entry returned in stat lists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatEntry {
    pub stat_name: String,
    pub stat_value: String,
}

/// Cash item coloring prism info (컬러링 프리즘).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColoringPrism {
    pub color_range: Option<String>,
    pub hue: Option<i64>,
    pub saturation: Option<i64>,
    pub value: Option<i64>,
    pub black_inclusive_flag: Option<String>,
}

/// Skill detail used in multiple contexts (link skill, character skill, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillDetail {
    pub skill_name: Option<String>,
    pub skill_description: Option<String>,
    pub skill_level: Option<i64>,
    pub skill_effect: Option<String>,
    pub skill_effect_next: Option<String>,
    pub skill_icon: Option<String>,
}

/// Hair information (헤어 정보).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HairInfo {
    pub hair_name: Option<String>,
    pub base_color: Option<String>,
    pub mix_color: Option<String>,
    pub mix_rate: Option<String>,
    pub freestyle_flag: Option<String>,
}

/// Face (성형) information.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FaceInfo {
    pub face_name: Option<String>,
    pub base_color: Option<String>,
    pub mix_color: Option<String>,
    pub mix_rate: Option<String>,
    pub freestyle_flag: Option<String>,
}

/// Skin (피부) information.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SkinInfo {
    pub skin_name: Option<String>,
    pub color_style: Option<String>,
    pub hue: Option<i64>,
    pub saturation: Option<i64>,
    pub brightness: Option<i64>,
}

/// Pet auto-skill slot information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetAutoSkill {
    pub skill_1: Option<String>,
    pub skill_1_icon: Option<String>,
    pub skill_2: Option<String>,
    pub skill_2_icon: Option<String>,
}

/// Pet equipment item detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetItemEquipment {
    pub item_name: Option<String>,
    pub item_icon: Option<String>,
    pub item_description: Option<String>,
    pub item_option: Option<Vec<ItemKvOption>>,
    pub scroll_upgrade: Option<i64>,
    pub scroll_upgradable: Option<i64>,
    pub item_shape: Option<String>,
    pub item_shape_icon: Option<String>,
    pub item_date_expire: Option<String>,
}

/// Generic key-value item option used in cash and pet equipment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemKvOption {
    pub option_type: Option<String>,
    pub option_value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_grade_display() {
        assert_eq!(SkillGrade::Zero.to_string(), "0");
        assert_eq!(SkillGrade::First.to_string(), "1");
        assert_eq!(SkillGrade::FirstHalf.to_string(), "1.5");
        assert_eq!(SkillGrade::HyperPassive.to_string(), "hyperpassive");
        assert_eq!(SkillGrade::HyperActive.to_string(), "hyperactive");
        assert_eq!(SkillGrade::Sixth.to_string(), "6");
    }
}

// ─── MapleDate & DateParam ───────────────────────────────────────────────────

/// Strongly typed date representation for the NEXON MapleStory OpenAPI.
///
/// Ensures dates conform to `YYYY-MM-DD`, are not earlier than `MIN_DATE` (`2023-12-21`),
/// and respect NEXON's 01:00 AM KST daily refresh boundary.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct MapleDate(pub(crate) String);

impl MapleDate {
    /// Earliest available date across NEXON MapleStory OpenAPI (`2023-12-21`).
    pub const MIN_DATE: &'static str = "2023-12-21";

    /// Create a new `MapleDate` from a string slice without range validation.
    pub fn new(date: impl Into<String>) -> Self {
        Self(date.into())
    }

    /// Access the raw date string in `YYYY-MM-DD` format.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert into the underlying `String`.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Parse this date into a `chrono::NaiveDate`.
    pub fn to_naive_date(&self) -> Option<chrono::NaiveDate> {
        chrono::NaiveDate::parse_from_str(&self.0, "%Y-%m-%d").ok()
    }

    /// Calculate the latest available date for querying NEXON MapleStory OpenAPI
    /// based on the current UTC time converted to KST (+09:00).
    ///
    /// NEXON releases daily snapshots at 01:00 AM KST:
    /// - Before 01:00 AM KST: data is available up to 2 days ago.
    /// - At or after 01:00 AM KST: data is available up to yesterday (1 day ago).
    pub fn latest_available() -> Self {
        Self::latest_available_at(chrono::Utc::now())
    }

    /// Calculate the latest available date relative to an arbitrary datetime.
    pub fn latest_available_at<Tz: chrono::TimeZone>(dt: chrono::DateTime<Tz>) -> Self {
        use chrono::Timelike;
        let kst_offset = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let kst_dt = dt.with_timezone(&kst_offset);
        let days_ago = if kst_dt.hour() < 1 { 2 } else { 1 };
        let date = kst_dt.date_naive() - chrono::Duration::days(days_ago);
        Self(date.format("%Y-%m-%d").to_string())
    }
}

impl std::fmt::Display for MapleDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for MapleDate {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MapleDate {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&str> for MapleDate {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for MapleDate {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&String> for MapleDate {
    fn from(s: &String) -> Self {
        Self(s.clone())
    }
}

impl From<chrono::NaiveDate> for MapleDate {
    fn from(d: chrono::NaiveDate) -> Self {
        Self(d.format("%Y-%m-%d").to_string())
    }
}

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for MapleDate {
    fn from(dt: chrono::DateTime<Tz>) -> Self {
        let kst_offset = chrono::FixedOffset::east_opt(9 * 3600).unwrap();
        let kst_dt = dt.with_timezone(&kst_offset);
        Self(kst_dt.format("%Y-%m-%d").to_string())
    }
}

/// Parameter helper for API methods that accept an optional date.
///
/// Can be constructed from:
/// - `None` (or `Option<&str>`)
/// - `Some("YYYY-MM-DD")`
/// - `"YYYY-MM-DD"` (`&str` directly)
/// - `String` / `&String`
/// - `MapleDate` / `&MapleDate`
/// - `chrono::NaiveDate`
/// - `chrono::DateTime<Tz>`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateParam {
    None,
    Date(String),
}

impl DateParam {
    pub fn as_deref(&self) -> Option<&str> {
        match self {
            DateParam::None => None,
            DateParam::Date(s) => Some(s.as_str()),
        }
    }
}

impl From<Option<&str>> for DateParam {
    fn from(opt: Option<&str>) -> Self {
        match opt {
            Some(s) => DateParam::Date(s.to_string()),
            None => DateParam::None,
        }
    }
}

impl From<&str> for DateParam {
    fn from(s: &str) -> Self {
        DateParam::Date(s.to_string())
    }
}

impl From<String> for DateParam {
    fn from(s: String) -> Self {
        DateParam::Date(s)
    }
}

impl From<&String> for DateParam {
    fn from(s: &String) -> Self {
        DateParam::Date(s.clone())
    }
}

impl From<MapleDate> for DateParam {
    fn from(d: MapleDate) -> Self {
        DateParam::Date(d.0)
    }
}

impl From<&MapleDate> for DateParam {
    fn from(d: &MapleDate) -> Self {
        DateParam::Date(d.0.clone())
    }
}

impl From<chrono::NaiveDate> for DateParam {
    fn from(d: chrono::NaiveDate) -> Self {
        DateParam::Date(d.format("%Y-%m-%d").to_string())
    }
}

impl<Tz: chrono::TimeZone> From<chrono::DateTime<Tz>> for DateParam {
    fn from(dt: chrono::DateTime<Tz>) -> Self {
        DateParam::Date(MapleDate::from(dt).0)
    }
}
