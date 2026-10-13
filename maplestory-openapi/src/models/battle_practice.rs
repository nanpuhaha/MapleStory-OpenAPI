// src/models/battle_practice.rs
//! Models for Battle Practice (연무장) tag API endpoints.
//!
//! - `GET /maplestory/v1/battle-practice/replay-id` → [`ReplayId`]
//! - `GET /maplestory/v1/battle-practice/result` → [`BattlePracticeResult`]
//! - `GET /maplestory/v1/battle-practice/skill-timeline` → [`SkillTimeline`]
//! - `GET /maplestory/v1/battle-practice/character-info` → [`BattlePracticeCharacterInfo`]

use serde::{Deserialize, Serialize};

use crate::models::{
    character::{
        AbilityInfo, CashItemEquipmentDetail, CharacterPropensity, CharacterSkill, FinalStatEntry,
        HexaCoreEquipment, HexaStatCore, HyperStatEntry, ItemEquipmentDetail, LinkSkillPresetItem,
        OtherStatEntry, PetiteLunaPetSkill, SetEffectInfo, SymbolEquipmentDetail, VCoreEquipment,
    },
    common::{PetAutoSkill, PetItemEquipment},
    guild::GuildSkill,
    union::{UnionArtifactEffect, UnionChampionBadgeInfo},
};

// ─── Replay ID ───────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/battle-practice/replay-id`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayId {
    /// Replay list
    pub replay_list: Option<Vec<BattlePracticeReplayListItem>>,
}

/// A single replay item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeReplayListItem {
    /// Period number (increments by 1 on measurement reset)
    pub period_no: Option<i64>,
    /// Replay registration timestamp (KST)
    pub register_date: Option<String>,
    /// Replay identifier
    pub replay_id: Option<String>,
}

// ─── Battle Practice Result ──────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/battle-practice/result`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeResult {
    /// Replay registration timestamp (KST)
    pub register_date: Option<String>,
    /// Total measurement duration in ms
    pub total_play_time: Option<i64>,
    /// Total damage inflicted
    pub total_damage: Option<i64>,
    /// Average DPS
    pub total_dps: Option<i64>,
    /// End type (1: auto end, 2: quit, 3: time expired, 9: other)
    pub end_type: Option<String>,
    /// Replay recommendation count
    pub like_count: Option<i64>,
    /// Skill usage statistics
    pub skill_statistic: Option<Vec<SkillStatisticEntry>>,
}

/// A single skill statistic entry in battle practice result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillStatisticEntry {
    /// Skill name
    pub skill_name: Option<String>,
    /// Accumulated damage
    pub damage: Option<i64>,
    /// Damage proportion percentage
    pub damage_percent: Option<String>,
    /// Skill DPS
    pub dps: Option<i64>,
    /// Number of times used
    pub use_count: Option<i64>,
    /// Damage per single use
    pub damage_per_use: Option<i64>,
    /// Hit count
    pub attack_count: Option<i64>,
    /// Maximum damage (single hit)
    pub max_damage: Option<i64>,
    /// Minimum damage (single hit)
    pub min_damage: Option<i64>,
}

// ─── Skill Timeline ──────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/battle-practice/skill-timeline`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTimeline {
    /// Current page number
    pub page_no: Option<i64>,
    /// Total number of pages
    pub total_page_no: Option<i64>,
    /// Skill usage timeline entries
    pub skill_timeline: Option<Vec<SkillTimelineEntry>>,
}

/// A single skill event in the battle practice timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTimelineEntry {
    /// Elapsed time since battle start in ms
    pub elapse_time: Option<i64>,
    /// Used skill name
    pub skill_name: Option<String>,
    /// HEXA skill specificity flag ("0": not specific, "1": enhanced, "2": accent)
    pub hexa_skill_specificity_flag: Option<String>,
    /// Key sequence name
    pub sequence_name: Option<String>,
    /// Key sequence identifier
    pub sequence_key: Option<String>,
}

// ─── Battle Practice Character Info ──────────────────────────────────────────

/// Response from `GET /maplestory/v1/battle-practice/character-info`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeCharacterInfo {
    /// Basic character information
    pub basic_object: Option<BattlePracticeBasic>,
    /// Stat information
    pub stat_object: Option<BattlePracticeStatObject>,
    /// Hyper stat information
    pub hyper_stat_object: Option<BattlePracticeHyperStatObject>,
    /// Propensity information
    pub propensity_object: Option<CharacterPropensity>,
    /// Ability information
    pub ability_object: Option<BattlePracticeAbilityObject>,
    /// Equipment information
    pub item_object: Option<BattlePracticeItemObject>,
    /// Cash equipment information
    pub cash_item_object: Option<BattlePracticeCashItemObject>,
    /// Pet information
    pub pet_object: Option<BattlePracticePetObject>,
    /// Skill information
    pub skill_object: Option<BattlePracticeSkillObject>,
    /// Link skill information
    pub link_skill_object: Option<BattlePracticeLinkSkillObject>,
    /// V-Matrix information
    pub v_matrix_object: Option<BattlePracticeVMatrixObject>,
    /// HEXA Matrix information
    pub hexa_matrix_object: Option<BattlePracticeHexaMatrixObject>,
    /// Ring reserve skill equipment information
    pub ring_reserve_skill_object: Option<BattlePracticeRingReserveSkillObject>,
    /// Union raider information
    pub union_raider_object: Option<BattlePracticeUnionRaiderObject>,
    /// Union artifact information
    pub union_artifact_object: Option<BattlePracticeUnionArtifactObject>,
    /// Union champion information
    pub union_champion_object: Option<BattlePracticeUnionChampionObject>,
    /// Guild information
    pub guild_object: Option<BattlePracticeGuildObject>,
}

/// Basic character information at battle practice entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeBasic {
    pub character_name: Option<String>,
    pub character_level: Option<i64>,
    pub character_class: Option<String>,
    pub character_class_level: Option<String>,
    pub character_image: Option<String>,
}

/// Stat information at battle practice entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeStatObject {
    pub basic_stat_object: Option<BattlePracticeBasicStat>,
    pub symbol_stat_object: Option<BattlePracticeSymbolStat>,
    pub other_stat_object: Option<BattlePracticeOtherStat>,
}

/// Basic stat list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeBasicStat {
    pub character_stat: Option<Vec<FinalStatEntry>>,
}

/// Symbol stat list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeSymbolStat {
    pub character_symbol: Option<Vec<SymbolEquipmentDetail>>,
}

/// Other stat influence list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeOtherStat {
    pub other_stat: Option<Vec<OtherStatEntry>>,
}

/// Hyper stat list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeHyperStatObject {
    pub hyper_stat: Option<Vec<HyperStatEntry>>,
}

/// Ability info list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeAbilityObject {
    pub ability_info: Option<Vec<AbilityInfo>>,
}

/// Equipment information object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeItemObject {
    pub item_equipment_object: Option<BattlePracticeItemEquipmentList>,
    pub set_effect_object: Option<BattlePracticeSetEffectList>,
}

/// Item equipment slot list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeItemEquipmentList {
    pub item_equipment: Option<Vec<ItemEquipmentDetail>>,
}

/// Set effect item list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeSetEffectList {
    pub set_effect: Option<Vec<SetEffectInfo>>,
}

/// Cash equipment object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeCashItemObject {
    pub cash_item_equipment_base: Option<Vec<CashItemEquipmentDetail>>,
    pub additional_cash_item_equipment_base: Option<Vec<CashItemEquipmentDetail>>,
}

/// Pet equipment and skill info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticePetObject {
    pub pet_activate_flag: Option<String>,
    pub pet_1_name: Option<String>,
    pub pet_1_nickname: Option<String>,
    pub pet_1_icon: Option<String>,
    pub pet_1_description: Option<String>,
    pub pet_1_equipment: Option<PetItemEquipment>,
    pub pet_1_auto_skill: Option<PetAutoSkill>,
    pub pet_1_pet_type: Option<String>,
    pub pet_1_skill: Option<Vec<String>>,
    pub pet_1_date_expire: Option<String>,
    pub pet_2_name: Option<String>,
    pub pet_2_nickname: Option<String>,
    pub pet_2_icon: Option<String>,
    pub pet_2_description: Option<String>,
    pub pet_2_equipment: Option<PetItemEquipment>,
    pub pet_2_auto_skill: Option<PetAutoSkill>,
    pub pet_2_pet_type: Option<String>,
    pub pet_2_skill: Option<Vec<String>>,
    pub pet_2_date_expire: Option<String>,
    pub pet_3_name: Option<String>,
    pub pet_3_nickname: Option<String>,
    pub pet_3_icon: Option<String>,
    pub pet_3_description: Option<String>,
    pub pet_3_equipment: Option<PetItemEquipment>,
    pub pet_3_auto_skill: Option<PetAutoSkill>,
    pub pet_3_pet_type: Option<String>,
    pub pet_3_skill: Option<Vec<String>>,
    pub pet_3_date_expire: Option<String>,
    pub world_share_pet_1_name: Option<String>,
    pub world_share_pet_1_nickname: Option<String>,
    pub world_share_pet_1_icon: Option<String>,
    pub world_share_pet_1_description: Option<String>,
    pub world_share_pet_1_pet_type: Option<String>,
    pub world_share_pet_1_equipment: Option<PetItemEquipment>,
    pub world_share_pet_1_auto_skill: Option<PetAutoSkill>,
    pub world_share_pet_1_skill: Option<Vec<String>>,
    pub world_share_pet_1_date_expire: Option<String>,
    pub world_share_pet_2_name: Option<String>,
    pub world_share_pet_2_nickname: Option<String>,
    pub world_share_pet_2_icon: Option<String>,
    pub world_share_pet_2_description: Option<String>,
    pub world_share_pet_2_pet_type: Option<String>,
    pub world_share_pet_2_equipment: Option<PetItemEquipment>,
    pub world_share_pet_2_auto_skill: Option<PetAutoSkill>,
    pub world_share_pet_2_skill: Option<Vec<String>>,
    pub world_share_pet_2_date_expire: Option<String>,
    pub world_share_pet_3_name: Option<String>,
    pub world_share_pet_3_nickname: Option<String>,
    pub world_share_pet_3_icon: Option<String>,
    pub world_share_pet_3_description: Option<String>,
    pub world_share_pet_3_pet_type: Option<String>,
    pub world_share_pet_3_equipment: Option<PetItemEquipment>,
    pub world_share_pet_3_auto_skill: Option<PetAutoSkill>,
    pub world_share_pet_3_skill: Option<Vec<String>>,
    pub world_share_pet_3_date_expire: Option<String>,
    pub petite_luna_pet_skill: Option<Vec<PetiteLunaPetSkill>>,
}

/// Skill object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeSkillObject {
    pub character_skill: Option<Vec<CharacterSkill>>,
}

/// Link skill object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeLinkSkillObject {
    pub character_link_skill: Option<Vec<LinkSkillPresetItem>>,
    pub character_owned_link_skill: Option<LinkSkillPresetItem>,
}

/// V-Matrix object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeVMatrixObject {
    pub character_v_core_equipment: Option<Vec<VCoreEquipment>>,
}

/// HEXA Matrix object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeHexaMatrixObject {
    pub hexa_core_object: Option<BattlePracticeHexaCoreObject>,
    pub hexa_matrix_stat_object: Option<BattlePracticeHexaMatrixStatObject>,
}

/// HEXA core list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeHexaCoreObject {
    pub hexa_core_equipment: Option<Vec<HexaCoreEquipment>>,
}

/// HEXA matrix stat cores (I, II, III).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeHexaMatrixStatObject {
    pub character_hexa_stat_core_1: Option<Vec<HexaStatCore>>,
    pub character_hexa_stat_core_2: Option<Vec<HexaStatCore>>,
    pub character_hexa_stat_core_3: Option<Vec<HexaStatCore>>,
}

/// Ring reserve skill object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeRingReserveSkillObject {
    pub special_ring_reserve_name: Option<String>,
    pub special_ring_reserve_level: Option<i64>,
    pub special_ring_reserve_icon: Option<String>,
    pub special_ring_reserve_description: Option<String>,
}

/// Union raider object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeUnionRaiderObject {
    pub union_raider_stat: Option<Vec<String>>,
    pub union_occupied_stat: Option<Vec<String>>,
    pub union_state_stat: Option<Vec<String>>,
}

/// Union artifact object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeUnionArtifactObject {
    pub union_artifact_effect: Option<Vec<UnionArtifactEffect>>,
}

/// Union champion object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeUnionChampionObject {
    pub champion_badge_total_info: Option<Vec<UnionChampionBadgeInfo>>,
}

/// Guild skill object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePracticeGuildObject {
    pub guild_skill: Option<Vec<GuildSkill>>,
    pub guild_noblesse_skill: Option<Vec<GuildSkill>>,
}
