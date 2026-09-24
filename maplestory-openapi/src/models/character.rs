// src/models/character.rs
//! Models for Character tag API endpoints (23 endpoints).

use serde::{Deserialize, Serialize};

use crate::models::common::{
    ColoringPrism, FaceInfo, HairInfo, ItemAddOption, ItemBaseOption, ItemEtcOption,
    ItemExceptionalOption, ItemKvOption, ItemStarforceOption, ItemTotalOption, PetAutoSkill,
    PetItemEquipment, SkillDetail, SkinInfo,
};

// ─── OCID ────────────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/id`.
///
/// Contains the OCID (Open Character ID) for a character name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterOcid {
    /// Unique open character identifier
    pub ocid: Option<String>,
}

// ─── Basic ───────────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/basic`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterBasic {
    /// Query reference date (KST)
    pub date: Option<String>,
    /// Character name
    pub character_name: Option<String>,
    /// World name
    pub world_name: Option<String>,
    /// Character gender (남, 여)
    pub character_gender: Option<String>,
    /// Character class
    pub character_class: Option<String>,
    /// Class level (job advancement tier)
    pub character_class_level: Option<String>,
    /// Character level
    pub character_level: Option<i64>,
    /// Current EXP
    pub character_exp: Option<i64>,
    /// EXP percentage towards next level
    pub character_exp_rate: Option<String>,
    /// Guild name
    pub character_guild_name: Option<String>,
    /// Character image URL
    pub character_image: Option<String>,
    /// Date of character creation (KST)
    pub character_date_create: Option<String>,
    /// Whether the character is accessible (false during maintenance etc.)
    pub access_flag: Option<String>,
    /// Liberation quest completion status
    pub liberation_quest_clear_flag: Option<String>,
}

// ─── Popularity ──────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/popularity`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPopularity {
    /// Query reference date (KST)
    pub date: Option<String>,
    /// Popularity value
    pub popularity: Option<i64>,
}

// ─── Stat ────────────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/stat`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterStat {
    /// Query reference date (KST)
    pub date: Option<String>,
    /// Character class
    pub character_class: Option<String>,
    /// Final stat list
    pub final_stat: Option<Vec<FinalStatEntry>>,
    /// Remaining AP
    pub remain_ap: Option<i64>,
}

/// A single stat entry in the final stat list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalStatEntry {
    pub stat_name: Option<String>,
    pub stat_value: Option<String>,
}

// ─── Hyper Stat ──────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/hyper-stat`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterHyperStat {
    /// Query reference date (KST)
    pub date: Option<String>,
    /// Character class
    pub character_class: Option<String>,
    /// Currently active preset number
    pub use_preset_no: Option<String>,
    /// Remaining hyper stat points
    pub use_available_hyper_stat: Option<i64>,
    /// Hyper stats for preset 1
    pub hyper_stat_preset_1: Option<Vec<HyperStatEntry>>,
    pub hyper_stat_preset_1_remain_point: Option<i64>,
    /// Hyper stats for preset 2
    pub hyper_stat_preset_2: Option<Vec<HyperStatEntry>>,
    pub hyper_stat_preset_2_remain_point: Option<i64>,
    /// Hyper stats for preset 3
    pub hyper_stat_preset_3: Option<Vec<HyperStatEntry>>,
    pub hyper_stat_preset_3_remain_point: Option<i64>,
}

/// A single hyper stat entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperStatEntry {
    pub stat_type: Option<String>,
    pub stat_point: Option<i64>,
    pub stat_level: Option<i64>,
    pub stat_increase: Option<String>,
}

// ─── Propensity ──────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/propensity`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPropensity {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub charisma_level: Option<i64>,
    pub sensibility_level: Option<i64>,
    pub insight_level: Option<i64>,
    pub willingness_level: Option<i64>,
    pub handicraft_level: Option<i64>,
    pub charm_level: Option<i64>,
}

// ─── Ability ─────────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/ability`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAbility {
    /// Query reference date (KST)
    pub date: Option<String>,
    /// Overall ability grade
    pub ability_grade: Option<String>,
    /// Active ability lines
    pub ability_info: Option<Vec<AbilityInfo>>,
    /// Remaining fame (인지도 포인트)
    pub remain_fame: Option<i64>,
    /// Preset 1
    pub ability_preset_1: Option<AbilityPreset>,
    /// Preset 2
    pub ability_preset_2: Option<AbilityPreset>,
    /// Preset 3
    pub ability_preset_3: Option<AbilityPreset>,
}

/// A single ability line.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityInfo {
    pub ability_no: Option<String>,
    pub ability_grade: Option<String>,
    pub ability_value: Option<String>,
}

/// Ability preset (프리셋).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityPreset {
    pub ability_preset_grade: Option<String>,
    pub ability_info: Option<Vec<AbilityInfo>>,
}

// ─── Item Equipment ───────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/item-equipment`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterItemEquipment {
    /// Query reference date (KST)
    pub date: Option<String>,
    /// Character gender
    pub character_gender: Option<String>,
    /// Character class
    pub character_class: Option<String>,
    /// Preset number currently equipped
    pub preset_no: Option<i64>,
    /// Currently equipped items
    pub item_equipment: Option<Vec<ItemEquipmentDetail>>,
    /// Preset 1
    pub item_equipment_preset_1: Option<Vec<ItemEquipmentDetail>>,
    /// Preset 2
    pub item_equipment_preset_2: Option<Vec<ItemEquipmentDetail>>,
    /// Preset 3
    pub item_equipment_preset_3: Option<Vec<ItemEquipmentDetail>>,
    /// Title equipment info
    pub title: Option<TitleInfo>,
    /// Dragon equipment (Evan only)
    pub dragon_equipment: Option<Vec<SpecialItemEquipment>>,
    /// Mechanic equipment (Mechanic only)
    pub mechanic_equipment: Option<Vec<SpecialItemEquipment>>,
}

/// Full detail of a single equipped item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemEquipmentDetail {
    pub item_equipment_part: Option<String>,
    pub item_equipment_slot: Option<String>,
    pub item_name: Option<String>,
    pub item_icon: Option<String>,
    pub item_description: Option<String>,
    pub item_shape_name: Option<String>,
    pub item_shape_icon: Option<String>,
    pub item_gender: Option<String>,
    pub item_total_option: Option<ItemTotalOption>,
    pub item_base_option: Option<ItemBaseOption>,
    pub potential_option_grade: Option<String>,
    pub additional_potential_option_grade: Option<String>,
    pub potential_option_flag: Option<String>,
    pub additional_potential_option_flag: Option<String>,
    pub potential_option_1: Option<String>,
    pub potential_option_2: Option<String>,
    pub potential_option_3: Option<String>,
    pub additional_potential_option_1: Option<String>,
    pub additional_potential_option_2: Option<String>,
    pub additional_potential_option_3: Option<String>,
    pub equipment_level_increase: Option<i64>,
    pub item_exceptional_option: Option<ItemExceptionalOption>,
    pub item_add_option: Option<ItemAddOption>,
    pub growth_exp: Option<i64>,
    pub growth_level: Option<i64>,
    pub scroll_upgrade: Option<String>,
    pub cuttable_count: Option<String>,
    pub golden_hammer_flag: Option<String>,
    pub scroll_resilience_count: Option<String>,
    pub scroll_upgradable_count: Option<String>,
    pub soul_name: Option<String>,
    pub soul_option: Option<String>,
    pub soul_pad: Option<String>,
    pub soul_mad: Option<String>,
    pub soul_active: Option<String>,
    pub soul_potential_grade: Option<String>,
    pub soul_potential_amplified_grade: Option<i64>,
    pub soul_potential_option_1: Option<String>,
    pub soul_potential_option_2: Option<String>,
    pub soul_potential_option_3: Option<String>,
    pub item_etc_option: Option<ItemEtcOption>,
    pub starforce: Option<String>,
    pub starforce_scroll_flag: Option<String>,
    pub item_starforce_option: Option<ItemStarforceOption>,
    pub special_ring_level: Option<i64>,
    pub date_expire: Option<String>,
    pub freestyle_flag: Option<String>,
}

/// Title equipment detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleInfo {
    pub preset_no: Option<i64>,
    pub title_name: Option<String>,
    pub title_icon: Option<String>,
    pub title_description: Option<String>,
    pub date_expire: Option<String>,
    pub date_option_expire: Option<String>,
    pub title_shape_name: Option<String>,
    pub title_shape_icon: Option<String>,
    pub title_shape_description: Option<String>,
    pub title_preset_1: Option<TitlePreset>,
    pub title_preset_2: Option<TitlePreset>,
    pub title_preset_3: Option<TitlePreset>,
}

/// A title preset entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitlePreset {
    pub title_name: Option<String>,
    pub title_icon: Option<String>,
    pub title_description: Option<String>,
    pub date_expire: Option<String>,
    pub date_option_expire: Option<String>,
    pub title_shape_name: Option<String>,
    pub title_shape_icon: Option<String>,
    pub title_shape_description: Option<String>,
    pub disable_flag: Option<String>,
}

/// Equipment item for Evan dragon / Mechanic (same shape).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialItemEquipment {
    pub item_equipment_part: Option<String>,
    pub equipment_slot: Option<String>,
    pub item_name: Option<String>,
    pub item_icon: Option<String>,
    pub item_description: Option<String>,
    pub item_shape_name: Option<String>,
    pub item_shape_icon: Option<String>,
    pub item_gender: Option<String>,
    pub item_total_option: Option<ItemTotalOption>,
    pub item_base_option: Option<ItemBaseOption>,
    pub equipment_level_increase: Option<i64>,
    pub item_exceptional_option: Option<ItemExceptionalOption>,
    pub item_add_option: Option<ItemAddOption>,
    pub growth_exp: Option<i64>,
    pub growth_level: Option<i64>,
    pub scroll_upgrade: Option<String>,
    pub cuttable_count: Option<String>,
    pub golden_hammer_flag: Option<String>,
    pub scroll_resilience_count: Option<String>,
    pub scroll_upgradable_count: Option<String>,
    pub soul_name: Option<String>,
    pub soul_option: Option<String>,
    pub item_etc_option: Option<ItemEtcOption>,
    pub starforce: Option<String>,
    pub starforce_scroll_flag: Option<String>,
    pub item_starforce_option: Option<ItemStarforceOption>,
    pub special_ring_level: Option<i64>,
    pub date_expire: Option<String>,
    pub freestyle_flag: Option<String>,
}

// ─── Cash Item Equipment ──────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/cashitem-equipment`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterCashItemEquipment {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub character_gender: Option<String>,
    pub character_class: Option<String>,
    pub character_look_mode: Option<String>,
    pub preset_no: Option<i64>,
    /// Currently equipped cash items
    pub cash_item_equipment_base: Option<Vec<CashItemEquipmentDetail>>,
    pub cash_item_equipment_preset_1: Option<Vec<CashItemEquipmentDetail>>,
    pub cash_item_equipment_preset_2: Option<Vec<CashItemEquipmentDetail>>,
    pub cash_item_equipment_preset_3: Option<Vec<CashItemEquipmentDetail>>,
    /// Additional mode items (Zero beta / Angelic Buster dress-up)
    pub additional_cash_item_equipment_base: Option<Vec<CashItemEquipmentDetail>>,
    pub additional_cash_item_equipment_preset_1: Option<Vec<CashItemEquipmentDetail>>,
    pub additional_cash_item_equipment_preset_2: Option<Vec<CashItemEquipmentDetail>>,
    pub additional_cash_item_equipment_preset_3: Option<Vec<CashItemEquipmentDetail>>,
}

/// A single cash item equipment entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashItemEquipmentDetail {
    pub cash_item_equipment_part: Option<String>,
    pub cash_item_equipment_slot: Option<String>,
    pub cash_item_name: Option<String>,
    pub cash_item_icon: Option<String>,
    pub cash_item_description: Option<String>,
    pub cash_item_option: Option<Vec<ItemKvOption>>,
    pub date_expire: Option<String>,
    pub date_option_expire: Option<String>,
    pub cash_item_label: Option<String>,
    pub cash_item_coloring_prism_flag: Option<String>,
    pub cash_item_coloring_prism: Option<ColoringPrism>,
    pub cash_item_effect_prism_flag: Option<String>,
    pub cash_item_effect_prism: Option<ColoringPrism>,
    pub item_gender: Option<String>,
    pub skills: Option<Vec<String>>,
    pub freestyle_flag: Option<String>,
    pub emotion_name: Option<String>,
    pub custom_origin: Option<Vec<CustomOrigin>>,
}

/// Custom origin entry for cash items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomOrigin {
    pub custom_origin_no: Option<i64>,
    pub x: Option<String>,
    pub y: Option<String>,
}

// ─── Symbol Equipment ─────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/symbol-equipment`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSymbolEquipment {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub character_class: Option<String>,
    pub symbol: Option<Vec<SymbolEquipmentDetail>>,
}

/// A single symbol equipment entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolEquipmentDetail {
    pub symbol_name: Option<String>,
    pub symbol_icon: Option<String>,
    pub symbol_description: Option<String>,
    pub symbol_other_effect_description: Option<String>,
    pub symbol_force: Option<String>,
    pub symbol_level: Option<i64>,
    pub symbol_str: Option<String>,
    pub symbol_dex: Option<String>,
    pub symbol_int: Option<String>,
    pub symbol_luk: Option<String>,
    pub symbol_hp: Option<String>,
    pub symbol_drop_rate: Option<String>,
    pub symbol_meso_rate: Option<String>,
    pub symbol_exp_rate: Option<String>,
    pub symbol_growth_count: Option<i64>,
    pub symbol_require_growth_count: Option<i64>,
}

impl CharacterSymbolEquipment {
    /// Calculate total Arcane Force (아케인포스 총합).
    pub fn total_arcane_force(&self) -> i64 {
        self.symbol
            .as_ref()
            .map(|symbols| {
                symbols
                    .iter()
                    .filter(|item| item.symbol_name.as_deref().unwrap_or("").contains("아케인"))
                    .filter_map(|item| item.symbol_force.as_deref())
                    .filter_map(|s| s.parse::<i64>().ok())
                    .sum()
            })
            .unwrap_or(0)
    }

    /// Calculate total Authentic Force (어센틱포스 총합).
    pub fn total_authentic_force(&self) -> i64 {
        self.symbol
            .as_ref()
            .map(|symbols| {
                symbols
                    .iter()
                    .filter(|item| item.symbol_name.as_deref().unwrap_or("").contains("어센틱"))
                    .filter_map(|item| item.symbol_force.as_deref())
                    .filter_map(|s| s.parse::<i64>().ok())
                    .sum()
            })
            .unwrap_or(0)
    }
}

// ─── Set Effect ───────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/set-effect`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSetEffect {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub set_effect: Option<Vec<SetEffectInfo>>,
}

/// A single set effect entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetEffectInfo {
    pub set_name: Option<String>,
    pub total_set_count: Option<i64>,
    /// Currently active set effect options
    pub set_effect_info: Option<Vec<SetEffectOption>>,
    /// All possible set effect options
    pub set_option_full: Option<Vec<SetEffectOption>>,
}

/// A single set effect option.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetEffectOption {
    pub set_count: Option<i64>,
    pub set_option: Option<String>,
}

// ─── Beauty Equipment ─────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/beauty-equipment`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterBeautyEquipment {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub character_gender: Option<String>,
    pub character_class: Option<String>,
    /// Alpha (Zero) / normal mode hair
    pub character_hair: Option<HairInfo>,
    /// Alpha (Zero) / normal mode face
    pub character_face: Option<FaceInfo>,
    /// Alpha (Zero) / normal mode skin
    pub character_skin: Option<SkinInfo>,
    /// Beta (Zero) / dress-up mode hair
    pub additional_character_hair: Option<HairInfo>,
    /// Beta (Zero) / dress-up mode face
    pub additional_character_face: Option<FaceInfo>,
    /// Beta (Zero) / dress-up mode skin
    pub additional_character_skin: Option<SkinInfo>,
}

// ─── Android Equipment ────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/android-equipment`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAndroidEquipment {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub android_name: Option<String>,
    pub android_nickname: Option<String>,
    pub android_icon: Option<String>,
    pub android_description: Option<String>,
    pub android_hair: Option<HairInfo>,
    pub android_face: Option<FaceInfo>,
    pub android_skin: Option<SkinInfo>,
    pub android_cash_item_equipment: Option<Vec<AndroidCashItem>>,
    pub android_ear_sensor_clip_flag: Option<String>,
    pub android_gender: Option<String>,
    pub android_grade: Option<String>,
    pub android_non_humanoid_flag: Option<String>,
    pub android_shop_usable_flag: Option<String>,
    pub preset_no: Option<i64>,
    pub android_preset_1: Option<AndroidPreset>,
    pub android_preset_2: Option<AndroidPreset>,
    pub android_preset_3: Option<AndroidPreset>,
}

/// An android cash item entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidCashItem {
    pub cash_item_equipment_part: Option<String>,
    pub cash_item_equipment_slot: Option<String>,
    pub cash_item_name: Option<String>,
    pub cash_item_icon: Option<String>,
    pub cash_item_description: Option<String>,
    pub cash_item_option: Option<Vec<ItemKvOption>>,
    pub date_expire: Option<String>,
    pub date_option_expire: Option<String>,
    pub cash_item_label: Option<String>,
    pub cash_item_coloring_prism: Option<ColoringPrism>,
    pub android_item_gender: Option<String>,
    pub freestyle_flag: Option<String>,
}

/// An android preset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidPreset {
    pub android_name: Option<String>,
    pub android_nickname: Option<String>,
    pub android_icon: Option<String>,
    pub android_description: Option<String>,
    pub android_gender: Option<String>,
    pub android_grade: Option<String>,
    pub android_skin: Option<SkinInfo>,
    pub android_hair: Option<HairInfo>,
    pub android_face: Option<FaceInfo>,
    pub android_ear_sensor_clip_flag: Option<String>,
    pub android_non_humanoid_flag: Option<String>,
    pub android_shop_usable_flag: Option<String>,
}

// ─── Pet Equipment ────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/pet-equipment`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterPetEquipment {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub pet_activate_flag: Option<String>,
    // Pet 1
    pub pet_1_name: Option<String>,
    pub pet_1_nickname: Option<String>,
    pub pet_1_icon: Option<String>,
    pub pet_1_description: Option<String>,
    pub pet_1_equipment: Option<PetItemEquipment>,
    pub pet_1_auto_skill: Option<PetAutoSkill>,
    pub pet_1_pet_type: Option<String>,
    pub pet_1_skill: Option<Vec<String>>,
    pub pet_1_date_expire: Option<String>,
    pub pet_1_appearance: Option<String>,
    pub pet_1_appearance_icon: Option<String>,
    // Pet 2
    pub pet_2_name: Option<String>,
    pub pet_2_nickname: Option<String>,
    pub pet_2_icon: Option<String>,
    pub pet_2_description: Option<String>,
    pub pet_2_equipment: Option<PetItemEquipment>,
    pub pet_2_auto_skill: Option<PetAutoSkill>,
    pub pet_2_pet_type: Option<String>,
    pub pet_2_skill: Option<Vec<String>>,
    pub pet_2_date_expire: Option<String>,
    pub pet_2_appearance: Option<String>,
    pub pet_2_appearance_icon: Option<String>,
    // Pet 3
    pub pet_3_name: Option<String>,
    pub pet_3_nickname: Option<String>,
    pub pet_3_icon: Option<String>,
    pub pet_3_description: Option<String>,
    pub pet_3_equipment: Option<PetItemEquipment>,
    pub pet_3_auto_skill: Option<PetAutoSkill>,
    pub pet_3_pet_type: Option<String>,
    pub pet_3_skill: Option<Vec<String>>,
    pub pet_3_date_expire: Option<String>,
    pub pet_3_appearance: Option<String>,
    pub pet_3_appearance_icon: Option<String>,
    // World share pets
    pub world_share_pet_1_name: Option<String>,
    pub world_share_pet_1_nickname: Option<String>,
    pub world_share_pet_1_icon: Option<String>,
    pub world_share_pet_1_description: Option<String>,
    pub world_share_pet_1_pet_type: Option<String>,
    pub world_share_pet_1_equipment_preset_no: Option<i64>,
    pub world_share_pet_1_equipment: Option<PetItemEquipment>,
    pub world_share_pet_1_auto_skill: Option<PetAutoSkill>,
    pub world_share_pet_1_skill: Option<Vec<String>>,
    pub world_share_pet_1_date_expire: Option<String>,
    pub world_share_pet_1_appearance: Option<String>,
    pub world_share_pet_1_appearance_icon: Option<String>,
    pub world_share_pet_2_name: Option<String>,
    pub world_share_pet_2_nickname: Option<String>,
    pub world_share_pet_2_icon: Option<String>,
    pub world_share_pet_2_description: Option<String>,
    pub world_share_pet_2_pet_type: Option<String>,
    pub world_share_pet_2_equipment_preset_no: Option<i64>,
    pub world_share_pet_2_equipment: Option<PetItemEquipment>,
    pub world_share_pet_2_auto_skill: Option<PetAutoSkill>,
    pub world_share_pet_2_skill: Option<Vec<String>>,
    pub world_share_pet_2_date_expire: Option<String>,
    pub world_share_pet_2_appearance: Option<String>,
    pub world_share_pet_2_appearance_icon: Option<String>,
    pub world_share_pet_3_name: Option<String>,
    pub world_share_pet_3_nickname: Option<String>,
    pub world_share_pet_3_icon: Option<String>,
    pub world_share_pet_3_description: Option<String>,
    pub world_share_pet_3_pet_type: Option<String>,
    pub world_share_pet_3_equipment_preset_no: Option<i64>,
    pub world_share_pet_3_equipment: Option<PetItemEquipment>,
    pub world_share_pet_3_auto_skill: Option<PetAutoSkill>,
    pub world_share_pet_3_skill: Option<Vec<String>>,
    pub world_share_pet_3_date_expire: Option<String>,
    pub world_share_pet_3_appearance: Option<String>,
    pub world_share_pet_3_appearance_icon: Option<String>,
    pub world_share_pet_equipment_preset: Option<WorldSharePetEquipmentPreset>,
    /// 루나 쁘띠 펫 스킬
    pub petite_luna_pet_skill: Option<Vec<PetiteLunaPetSkill>>,
}

/// World share pet equipment preset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSharePetEquipmentPreset {
    pub preset_no: Option<i64>,
    pub slot_no: Option<i64>,
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

/// 루나 쁘띠 펫 스킬 entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetiteLunaPetSkill {
    pub skill_name: Option<String>,
    pub skill_description: Option<String>,
    pub skill_effect: Option<String>,
    pub skill_icon: Option<String>,
}

// ─── Skill ───────────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/skill`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSkill {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub character_class: Option<String>,
    pub character_skill_grade: Option<String>,
    pub character_skill: Option<Vec<SkillDetail>>,
}

// ─── Link Skill ───────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/link-skill`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterLinkSkill {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub character_class: Option<String>,
    /// Active link skills
    pub character_link_skill: Option<Vec<SkillDetail>>,
    pub character_link_skill_preset_1: Option<Vec<LinkSkillPresetItem>>,
    pub character_link_skill_preset_2: Option<Vec<LinkSkillPresetItem>>,
    pub character_link_skill_preset_3: Option<Vec<LinkSkillPresetItem>>,
    /// This character's own link skill
    pub character_owned_link_skill: Option<SkillDetail>,
    pub character_owned_link_skill_preset_1: Option<SkillDetail>,
    pub character_owned_link_skill_preset_2: Option<SkillDetail>,
    pub character_owned_link_skill_preset_3: Option<SkillDetail>,
}

/// Link skill preset item (without skill_effect_next field).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkSkillPresetItem {
    pub skill_name: Option<String>,
    pub skill_description: Option<String>,
    pub skill_level: Option<i64>,
    pub skill_effect: Option<String>,
    pub skill_icon: Option<String>,
}

// ─── V Matrix ─────────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/vmatrix`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterVMatrix {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub character_class: Option<String>,
    pub character_v_core_equipment: Option<Vec<VCoreEquipment>>,
    pub character_v_matrix_remain_slot_upgrade_point: Option<i64>,
    pub character_v_core_equipment_preset_1: Option<Vec<VCorePresetItem>>,
    pub character_v_core_equipment_preset_2: Option<Vec<VCorePresetItem>>,
    pub character_v_core_equipment_preset_3: Option<Vec<VCorePresetItem>>,
    pub character_v_core_equipment_preset_4: Option<Vec<VCorePresetItem>>,
    pub character_v_core_equipment_preset_5: Option<Vec<VCorePresetItem>>,
}

/// A single V-core equipment entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VCoreEquipment {
    pub slot_id: Option<String>,
    #[deprecated(note = "Deprecated since 2025-12-18; always 0 after that date")]
    pub slot_level: Option<i64>,
    pub v_core_name: Option<String>,
    pub v_core_type: Option<String>,
    pub v_core_level: Option<i64>,
    #[deprecated(note = "Deprecated since 2025-12-18; always null after that date")]
    pub v_core_skill_1: Option<String>,
    #[deprecated(note = "Deprecated since 2025-12-18; always null after that date")]
    pub v_core_skill_2: Option<String>,
    #[deprecated(note = "Deprecated since 2025-12-18; always null after that date")]
    pub v_core_skill_3: Option<String>,
}

/// A V-core entry in a preset (simplified).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VCorePresetItem {
    pub v_core_name: Option<String>,
    pub v_core_type: Option<String>,
    pub v_core_level: Option<i64>,
}

// ─── HEXA Matrix ──────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/hexamatrix`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterHexaMatrix {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub character_hexa_core_equipment: Option<Vec<HexaCoreEquipment>>,
}

/// A single HEXA core equipment entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexaCoreEquipment {
    pub hexa_core_name: Option<String>,
    pub hexa_core_level: Option<i64>,
    pub hexa_core_event_level: Option<i64>,
    pub hexa_core_type: Option<String>,
    pub linked_skill: Option<Vec<HexaLinkedSkill>>,
}

/// A linked HEXA skill reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexaLinkedSkill {
    pub hexa_skill_id: Option<String>,
}

// ─── HEXA Matrix Stat ─────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/hexamatrix-stat`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterHexaMatrixStat {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub character_class: Option<String>,
    /// HEXA stat I cores
    pub character_hexa_stat_core: Option<Vec<HexaStatCore>>,
    /// HEXA stat II cores
    pub character_hexa_stat_core_2: Option<Vec<HexaStatCore>>,
    /// HEXA stat III cores
    pub character_hexa_stat_core_3: Option<Vec<HexaStatCore>>,
    /// Preset HEXA stat I cores
    pub preset_hexa_stat_core: Option<Vec<HexaStatCore>>,
    /// Preset HEXA stat II cores
    pub preset_hexa_stat_core_2: Option<Vec<HexaStatCore>>,
    /// Preset HEXA stat III cores
    pub preset_hexa_stat_core_3: Option<Vec<HexaStatCore>>,
}

/// A HEXA stat core entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexaStatCore {
    pub slot_id: Option<String>,
    pub main_stat_name: Option<String>,
    pub sub_stat_name_1: Option<String>,
    pub sub_stat_name_2: Option<String>,
    pub main_stat_level: Option<i64>,
    pub sub_stat_level_1: Option<i64>,
    pub sub_stat_level_2: Option<i64>,
    pub stat_grade: Option<i64>,
}

// ─── Dojang ───────────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/dojang`.
///
/// Contains the character's best Mu Lung Dojo (무릉도장) record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDojang {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub character_class: Option<String>,
    pub world_name: Option<String>,
    /// Best floor reached
    pub dojang_best_floor: Option<i64>,
    /// Date when the best floor was achieved (KST)
    pub date_dojang_record: Option<String>,
    /// Time taken to clear the best floor (seconds)
    pub dojang_best_time: Option<i64>,
}

// ─── Other Stat ───────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/other-stat`.
///
/// Available from 2025-08-21.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterOtherStat {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub other_stat: Option<Vec<OtherStatEntry>>,
}

/// A stat type group within other stats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtherStatEntry {
    pub other_stat_type: Option<String>,
    pub stat_info: Option<Vec<OtherStatInfo>>,
}

/// A single stat name/value pair in other stats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtherStatInfo {
    pub stat_name: Option<String>,
    pub stat_value: Option<String>,
}

// ─── Ring Exchange Skill Equipment ───────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/ring-exchange-skill-equipment`.
///
/// Available from 2025-08-21 to 2026-03-18.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingExchangeSkillEquipment {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub character_class: Option<String>,
    pub special_ring_exchange_name: Option<String>,
    pub special_ring_exchange_level: Option<i64>,
    pub special_ring_exchange_icon: Option<String>,
    pub special_ring_exchange_description: Option<String>,
}

// ─── Ring Reserve Skill Equipment ────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/ring-reserve-skill-equipment`.
///
/// Available from 2026-03-19.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingReserveSkillEquipment {
    /// Query reference date (KST)
    pub date: Option<String>,
    pub character_class: Option<String>,
    pub special_ring_reserve_name: Option<String>,
    pub special_ring_reserve_level: Option<i64>,
    pub special_ring_reserve_icon: Option<String>,
    pub special_ring_reserve_description: Option<String>,
}

// ─── CharacterStat Helpers ────────────────────────────────────────────────────

impl CharacterStat {
    /// Look up a stat value by its Korean name (e.g. "전투력", "보스 몬스터 데미지", "스탯공격력").
    pub fn get_stat(&self, name: &str) -> Option<&str> {
        self.final_stat.as_ref()?.iter().find_map(|entry| {
            if entry.stat_name.as_deref() == Some(name) {
                entry.stat_value.as_deref()
            } else {
                None
            }
        })
    }

    /// Look up a stat and parse its integer value as `i64`.
    pub fn get_stat_i64(&self, name: &str) -> Option<i64> {
        self.get_stat(name)?.parse::<i64>().ok()
    }

    /// Look up a stat and parse its floating-point or percentage value as `f64`.
    pub fn get_stat_f64(&self, name: &str) -> Option<f64> {
        self.get_stat(name)?.parse::<f64>().ok()
    }

    /// Combat power (전투력).
    pub fn combat_power(&self) -> Option<i64> {
        self.get_stat_i64("전투력")
    }

    /// Stat attack power as `(min, max)` (최소 스탯공격력, 최대 스탯공격력).
    pub fn stat_attack(&self) -> (Option<i64>, Option<i64>) {
        (
            self.get_stat_i64("최소 스탯공격력"),
            self.get_stat_i64("최대 스탯공격력"),
        )
    }

    /// Boss monster damage percentage (보스 몬스터 데미지).
    pub fn boss_damage(&self) -> Option<f64> {
        self.get_stat_f64("보스 몬스터 데미지")
    }

    /// Ignore monster armor percentage (방어율 무시).
    pub fn ignore_monster_armor(&self) -> Option<f64> {
        self.get_stat_f64("방어율 무시")
    }

    /// Damage percentage (데미지).
    pub fn damage(&self) -> Option<f64> {
        self.get_stat_f64("데미지")
    }

    /// Critical hit rate percentage (크리티컬 확률).
    pub fn critical_rate(&self) -> Option<f64> {
        self.get_stat_f64("크리티컬 확률")
    }

    /// Critical damage percentage (크리티컬 데미지).
    pub fn critical_damage(&self) -> Option<f64> {
        self.get_stat_f64("크리티컬 데미지")
    }
}

// ─── CharacterItemEquipment Helpers ───────────────────────────────────────────

impl CharacterItemEquipment {
    /// Returns the currently active list of equipped items according to `preset_no`.
    pub fn active_items(&self) -> &[ItemEquipmentDetail] {
        match self.preset_no {
            Some(1) => self
                .item_equipment_preset_1
                .as_deref()
                .unwrap_or(self.item_equipment.as_deref().unwrap_or(&[])),
            Some(2) => self
                .item_equipment_preset_2
                .as_deref()
                .unwrap_or(self.item_equipment.as_deref().unwrap_or(&[])),
            Some(3) => self
                .item_equipment_preset_3
                .as_deref()
                .unwrap_or(self.item_equipment.as_deref().unwrap_or(&[])),
            _ => self.item_equipment.as_deref().unwrap_or(&[]),
        }
    }

    /// Find an equipped item by its slot or part name (e.g. "무기", "모자", "상의").
    pub fn find_slot(&self, slot_or_part: &str) -> Option<&ItemEquipmentDetail> {
        self.active_items().iter().find(|item| {
            item.item_equipment_slot.as_deref() == Some(slot_or_part)
                || item.item_equipment_part.as_deref() == Some(slot_or_part)
        })
    }

    /// Weapon (무기).
    pub fn weapon(&self) -> Option<&ItemEquipmentDetail> {
        self.find_slot("무기")
    }

    /// Sub-weapon (보조무기).
    pub fn sub_weapon(&self) -> Option<&ItemEquipmentDetail> {
        self.find_slot("보조무기")
    }

    /// Emblem (엠블렘).
    pub fn emblem(&self) -> Option<&ItemEquipmentDetail> {
        self.find_slot("엠블렘")
    }

    /// Hat (모자).
    pub fn hat(&self) -> Option<&ItemEquipmentDetail> {
        self.find_slot("모자")
    }

    /// Top (상의 / 한벌옷).
    pub fn top(&self) -> Option<&ItemEquipmentDetail> {
        self.find_slot("상의")
    }

    /// Bottom (하의).
    pub fn bottom(&self) -> Option<&ItemEquipmentDetail> {
        self.find_slot("하의")
    }

    /// Shoes (신발).
    pub fn shoes(&self) -> Option<&ItemEquipmentDetail> {
        self.find_slot("신발")
    }

    /// Gloves (장갑).
    pub fn gloves(&self) -> Option<&ItemEquipmentDetail> {
        self.find_slot("장갑")
    }

    /// Cape (망토).
    pub fn cape(&self) -> Option<&ItemEquipmentDetail> {
        self.find_slot("망토")
    }

    /// Calculate total Starforce count across all currently active equipped items.
    pub fn total_starforce(&self) -> i64 {
        self.active_items()
            .iter()
            .filter_map(|item| item.starforce.as_deref())
            .filter_map(|s| s.parse::<i64>().ok())
            .sum()
    }
}

// ─── CharacterCashItemEquipment Helpers ───────────────────────────────────────

impl CharacterCashItemEquipment {
    /// Returns the currently active list of cash items according to `preset_no`.
    pub fn active_items(&self) -> &[CashItemEquipmentDetail] {
        match self.preset_no {
            Some(1) => self
                .cash_item_equipment_preset_1
                .as_deref()
                .unwrap_or(self.cash_item_equipment_base.as_deref().unwrap_or(&[])),
            Some(2) => self
                .cash_item_equipment_preset_2
                .as_deref()
                .unwrap_or(self.cash_item_equipment_base.as_deref().unwrap_or(&[])),
            Some(3) => self
                .cash_item_equipment_preset_3
                .as_deref()
                .unwrap_or(self.cash_item_equipment_base.as_deref().unwrap_or(&[])),
            _ => self.cash_item_equipment_base.as_deref().unwrap_or(&[]),
        }
    }

    /// Find an equipped cash item by its slot or part name.
    pub fn find_slot(&self, slot_or_part: &str) -> Option<&CashItemEquipmentDetail> {
        self.active_items().iter().find(|item| {
            item.cash_item_equipment_slot.as_deref() == Some(slot_or_part)
                || item.cash_item_equipment_part.as_deref() == Some(slot_or_part)
        })
    }
}

// ─── Coordination & Visible Appearance ────────────────────────────────────────

/// A single visible equipment item in a character's final look/coordination.
///
/// If cash equipment is worn in a slot, it takes precedence. Otherwise,
/// normal equipment is shown (with anvil shape taking priority if applied).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisibleEquipItem {
    /// Slot name (e.g. "모자", "상의", "하의", "신발", "장갑", "망토", "무기", "보조무기", "눈장식", "얼굴장식", "귀고리")
    pub slot: String,
    /// Part category name
    pub part: Option<String>,
    /// Final displayed item name
    pub name: String,
    /// Final displayed icon URL
    pub icon: Option<String>,
    /// True if this slot is covered by a cash item
    pub is_cash: bool,
    /// True if this is a transparent cash item (e.g. "투명 장갑", "투명 신발") hiding the normal equipment
    pub is_transparent: bool,
    /// True if this slot is a normal item with an anvil (신비의 모루) shape applied
    pub is_anvil: bool,
    /// Original equipment name if anvil shape is applied
    pub original_name: Option<String>,
    /// Cash label (e.g. "마스터라벨", "스페셜라벨") if cash item
    pub cash_label: Option<String>,
    /// Coloring prism dye info if cash item
    pub coloring_prism: Option<ColoringPrism>,
}

impl VisibleEquipItem {
    /// Returns `true` if this item has custom coloring prism dye settings.
    pub fn has_prism(&self) -> bool {
        self.coloring_prism.is_some()
    }

    /// Returns the coloring prism dye information if applied.
    pub fn prism(&self) -> Option<&ColoringPrism> {
        self.coloring_prism.as_ref()
    }

    /// Returns `true` if this slot is typically visible on the character sprite.
    ///
    /// Note: Non-shield secondary weapons (such as spellbooks/마도서, orbs/오브,
    /// arrows/화살깃) are not visually rendered on characters in MapleStory.
    pub fn is_visually_rendered(&self) -> bool {
        if self.is_transparent {
            return false;
        }

        if self.slot == "보조무기" {
            // In MapleStory, only shields, Katara/Blades, Force Shields, etc. are visible.
            // Items like 마도서, 오브, 부적, 쥬얼 are invisible.
            if let Some(part) = &self.part {
                let non_rendered_parts = [
                    "마도서",
                    "오브",
                    "부적",
                    "보석",
                    "체스피스",
                    "화살",
                    "화살깃",
                    "탄창",
                    "카트리지",
                    "컨트롤러",
                    "무기전송장치",
                    "매직제어구",
                    "소울링",
                    "청심환",
                    "유물",
                    "무기전송",
                ];
                return !non_rendered_parts.iter().any(|&p| part.contains(p));
            }
        }

        true
    }
}

/// Consolidated character look, beauty, and coordination (최종 노출 코디/외형 정보).
///
/// Synthesizes beauty features (hair, face, skin) and combines cash equipment
/// with normal equipment according to in-game display rules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterCoordination {
    /// Character name
    pub character_name: Option<String>,
    /// World name
    pub world_name: Option<String>,
    /// Character class / job
    pub character_class: Option<String>,
    /// Character level
    pub character_level: Option<i64>,
    /// Official character avatar look image URL from OpenAPI
    pub character_image: Option<String>,
    /// 1024-character AvatarCode parsed from character_image (if available)
    pub avatar_code: Option<crate::avatar::AvatarCode>,
    /// Hair information
    pub hair: Option<HairInfo>,
    /// Face / cosmetic information
    pub face: Option<FaceInfo>,
    /// Skin information
    pub skin: Option<SkinInfo>,
    /// Active cash preset number
    pub cash_preset_no: Option<i64>,
    /// Final visible equipment items across all slots
    pub visible_items: Vec<VisibleEquipItem>,
}

/// Alias for [`CharacterCoordination`].
pub type CharacterAppearance = CharacterCoordination;

impl CharacterCoordination {
    /// Synthesizes coordination from separate beauty, cash, and normal equipment models.
    pub fn from_parts(
        beauty: &CharacterBeautyEquipment,
        cash_equip: &CharacterCashItemEquipment,
        normal_equip: &CharacterItemEquipment,
    ) -> Self {
        // Standard visual slots in typical display order
        let standard_slots = [
            "모자",
            "얼굴장식",
            "눈장식",
            "귀고리",
            "상의",
            "하의",
            "신발",
            "장갑",
            "망토",
            "무기",
            "보조무기",
            "반지1",
            "반지2",
            "반지3",
            "반지4",
        ];

        let cash_items = cash_equip.active_items();
        let base_cash_items = cash_equip
            .cash_item_equipment_base
            .as_deref()
            .unwrap_or(&[]);
        let normal_items = normal_equip.active_items();

        let mut visible_items = Vec::new();
        let mut handled_cash_indices = std::collections::HashSet::new();
        let mut has_overall = false;

        // 1. Process standard slots
        for &slot in &standard_slots {
            // If character is wearing an overall (한벌옷), bottom is covered and not visible
            if slot == "하의" && has_overall {
                continue;
            }

            // Find cash item for this slot in active preset, with fallback to base
            let cash_match = cash_items
                .iter()
                .enumerate()
                .find(|(idx, item)| {
                    !handled_cash_indices.contains(idx)
                        && (item.cash_item_equipment_slot.as_deref() == Some(slot)
                            || item.cash_item_equipment_part.as_deref() == Some(slot))
                })
                .or_else(|| {
                    // Fallback to base cash items if active preset slot is empty
                    base_cash_items.iter().enumerate().find(|(_, item)| {
                        item.cash_item_equipment_slot.as_deref() == Some(slot)
                            || item.cash_item_equipment_part.as_deref() == Some(slot)
                    })
                });

            if let Some((idx, cash)) = cash_match {
                handled_cash_indices.insert(idx);
                let is_overall = cash.cash_item_equipment_part.as_deref() == Some("한벌옷");
                if is_overall
                    || slot == "상의" && cash.cash_item_equipment_part.as_deref() == Some("한벌옷")
                {
                    has_overall = true;
                }

                let display_slot = if is_overall {
                    "한벌옷".to_string()
                } else {
                    slot.to_string()
                };

                let item_name = cash
                    .cash_item_name
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string());
                let is_transparent = item_name.starts_with("투명 ")
                    || item_name == "투명장갑"
                    || item_name == "투명모자"
                    || item_name == "투명망토";

                visible_items.push(VisibleEquipItem {
                    slot: display_slot,
                    part: cash.cash_item_equipment_part.clone(),
                    name: item_name,
                    icon: cash.cash_item_icon.clone(),
                    is_cash: true,
                    is_transparent,
                    is_anvil: false,
                    original_name: None,
                    cash_label: cash.cash_item_label.clone(),
                    coloring_prism: cash.cash_item_coloring_prism.clone(),
                });
            } else {
                // Check normal equipment for this slot
                let normal_match = normal_items.iter().find(|item| {
                    item.item_equipment_slot.as_deref() == Some(slot)
                        || item.item_equipment_part.as_deref() == Some(slot)
                });

                if let Some(norm) = normal_match {
                    let is_overall = norm.item_equipment_part.as_deref() == Some("한벌옷");
                    if is_overall {
                        has_overall = true;
                    }

                    let display_slot = if is_overall {
                        "한벌옷".to_string()
                    } else {
                        slot.to_string()
                    };
                    let has_anvil =
                        norm.item_shape_name.is_some() && norm.item_shape_name != norm.item_name;

                    let (name, icon) = if has_anvil {
                        (
                            norm.item_shape_name
                                .clone()
                                .unwrap_or_else(|| norm.item_name.clone().unwrap_or_default()),
                            norm.item_shape_icon
                                .clone()
                                .or_else(|| norm.item_icon.clone()),
                        )
                    } else {
                        (
                            norm.item_name
                                .clone()
                                .unwrap_or_else(|| "Unknown".to_string()),
                            norm.item_icon.clone(),
                        )
                    };

                    visible_items.push(VisibleEquipItem {
                        slot: display_slot,
                        part: norm.item_equipment_part.clone(),
                        name,
                        icon,
                        is_cash: false,
                        is_transparent: false,
                        is_anvil: has_anvil,
                        original_name: if has_anvil {
                            norm.item_name.clone()
                        } else {
                            None
                        },
                        cash_label: None,
                        coloring_prism: None,
                    });
                }
            }
        }

        // 2. Include any remaining cash items not in standard slot list (e.g. effect, medals, extra rings)
        for (idx, cash) in cash_items.iter().enumerate() {
            if !handled_cash_indices.contains(&idx) {
                let slot = cash
                    .cash_item_equipment_slot
                    .clone()
                    .or_else(|| cash.cash_item_equipment_part.clone())
                    .unwrap_or_else(|| "기타".to_string());

                let item_name = cash
                    .cash_item_name
                    .clone()
                    .unwrap_or_else(|| "Unknown".to_string());
                let is_transparent = item_name.starts_with("투명 ")
                    || item_name == "투명장갑"
                    || item_name == "투명모자"
                    || item_name == "투명망토";

                visible_items.push(VisibleEquipItem {
                    slot,
                    part: cash.cash_item_equipment_part.clone(),
                    name: item_name,
                    icon: cash.cash_item_icon.clone(),
                    is_cash: true,
                    is_transparent,
                    is_anvil: false,
                    original_name: None,
                    cash_label: cash.cash_item_label.clone(),
                    coloring_prism: cash.cash_item_coloring_prism.clone(),
                });
            }
        }

        Self {
            character_name: None,
            world_name: None,
            character_class: None,
            character_level: None,
            character_image: None,
            avatar_code: None,
            hair: beauty.character_hair.clone(),
            face: beauty.character_face.clone(),
            skin: beauty.character_skin.clone(),
            cash_preset_no: cash_equip.preset_no,
            visible_items,
        }
    }

    /// Synthesizes coordination including basic character profile information.
    pub fn from_full_parts(
        basic: &CharacterBasic,
        beauty: &CharacterBeautyEquipment,
        cash_equip: &CharacterCashItemEquipment,
        normal_equip: &CharacterItemEquipment,
    ) -> Self {
        let mut coord = Self::from_parts(beauty, cash_equip, normal_equip);
        coord.character_name = basic.character_name.clone();
        coord.world_name = basic.world_name.clone();
        coord.character_class = basic.character_class.clone();
        coord.character_level = basic.character_level;
        coord.character_image = basic.character_image.clone();
        if let Some(ref img_url) = basic.character_image {
            coord.avatar_code = crate::avatar::AvatarCode::from_url_or_code(img_url).ok();
        }
        coord
    }

    /// Returns an iterator over all visible items that have a Coloring Prism (컬러링 프리즘) applied.
    pub fn prism_items(&self) -> impl Iterator<Item = &VisibleEquipItem> {
        self.visible_items.iter().filter(|item| item.has_prism())
    }

    /// Generates a customized avatar look URL with specific action, emotion, and weapon motion.
    pub fn custom_image(
        &self,
        action: crate::avatar::AvatarAction,
        emotion: crate::avatar::AvatarEmotion,
        wmotion: crate::avatar::WeaponMotion,
        width: u32,
        height: u32,
    ) -> Option<String> {
        let code = self.avatar_code.as_ref()?;
        let query = crate::avatar::AvatarLookQuery::new()
            .action(action)
            .emotion(emotion)
            .weapon_motion(wmotion)
            .size(width, height);
        Some(code.to_look_url(&query))
    }

    /// Generates a custom avatar look URL using an [`AvatarLookQuery`](crate::avatar::AvatarLookQuery).
    pub fn custom_avatar_url(&self, query: &crate::avatar::AvatarLookQuery) -> Option<String> {
        self.avatar_code
            .as_ref()
            .map(|code| code.to_look_url(query))
    }

    /// Generates a list of frame URLs and frame durations (ms) for an animation preset.
    pub fn animation_frame_urls(
        &self,
        preset: &crate::avatar::AnimationPreset,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Option<Vec<(String, u32)>> {
        self.avatar_code
            .as_ref()
            .map(|code| code.animation_frame_urls(preset, width, height))
    }

    /// Find visible item by slot or part name.
    pub fn get_slot(&self, slot_or_part: &str) -> Option<&VisibleEquipItem> {
        self.visible_items
            .iter()
            .find(|item| item.slot == slot_or_part || item.part.as_deref() == Some(slot_or_part))
    }

    /// Visible hat (모자).
    pub fn hat(&self) -> Option<&VisibleEquipItem> {
        self.get_slot("모자")
    }

    /// Visible top or overall (상의/한벌옷).
    pub fn top(&self) -> Option<&VisibleEquipItem> {
        self.get_slot("상의").or_else(|| self.get_slot("한벌옷"))
    }

    /// Visible bottom (하의).
    pub fn bottom(&self) -> Option<&VisibleEquipItem> {
        self.get_slot("하의")
    }

    /// Visible shoes (신발).
    pub fn shoes(&self) -> Option<&VisibleEquipItem> {
        self.get_slot("신발")
    }

    /// Visible gloves (장갑).
    pub fn gloves(&self) -> Option<&VisibleEquipItem> {
        self.get_slot("장갑")
    }

    /// Visible cape (망토).
    pub fn cape(&self) -> Option<&VisibleEquipItem> {
        self.get_slot("망토")
    }

    /// Visible weapon (무기).
    pub fn weapon(&self) -> Option<&VisibleEquipItem> {
        self.get_slot("무기")
    }

    /// Visible sub-weapon (보조무기).
    pub fn sub_weapon(&self) -> Option<&VisibleEquipItem> {
        self.get_slot("보조무기")
    }
}

// ─── Aggregated Character Profile ─────────────────────────────────────────────

/// Full aggregated character profile (종합 캐릭터 프로필).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterProfile {
    /// Basic information
    pub basic: CharacterBasic,
    /// Detailed stats and combat power
    pub stat: CharacterStat,
    /// Normal equipment
    pub item_equipment: CharacterItemEquipment,
    /// Cash equipment
    pub cash_item_equipment: CharacterCashItemEquipment,
    /// Beauty features
    pub beauty_equipment: CharacterBeautyEquipment,
    /// Synthesized final visual look (코디 정보)
    pub coordination: CharacterCoordination,
    /// Popularity
    pub popularity: Option<CharacterPopularity>,
    /// Mu Lung Dojo records
    pub dojang: Option<CharacterDojang>,
}

// ─── All-In-One Character Dashboard ──────────────────────────────────────────

/// Comprehensive all-in-one character dashboard (올인원 캐릭터 종합 대시보드).
///
/// Designed to provide all information required by modern MapleStory ranking &
/// analytics services (e.g. MapleScouter, ChuChu.gg, Meaegi, MapleRoad, Mezzang)
/// in a single aggregated structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDashboard {
    /// Basic information (이름, 월드, 직업, 레벨, 경험치, 이미지 등)
    pub basic: CharacterBasic,
    /// Detailed stats and combat power (스탯, 전투력, 보공, 방무 등)
    pub stat: CharacterStat,
    /// Normal equipment (일반 장비, 스타포스, 잠재 등)
    pub item_equipment: CharacterItemEquipment,
    /// Cash equipment (캐시 장비 및 프리즘 정보)
    pub cash_item_equipment: CharacterCashItemEquipment,
    /// Beauty features (헤어, 성형, 피부, 믹스염색)
    pub beauty_equipment: CharacterBeautyEquipment,
    /// Synthesized final visual look (최종 외형 코디 + 아바타 코드)
    pub coordination: CharacterCoordination,
    /// Arcane / Authentic Symbol equipment (아케인/어센틱 심볼 장비 및 포스)
    pub symbols: Option<CharacterSymbolEquipment>,
    /// Character ability (어빌리티 등급 및 옵션 3줄)
    pub ability: Option<CharacterAbility>,
    /// Hyper stats (하이퍼 스탯)
    pub hyper_stat: Option<CharacterHyperStat>,
    /// 6th job HEXA Matrix cores (6차 헥사 코어 장착 정보)
    pub hexa_matrix: Option<CharacterHexaMatrix>,
    /// 6th job HEXA Matrix stats (6차 헥사 스탯 1, 2)
    pub hexa_matrix_stat: Option<CharacterHexaMatrixStat>,
    /// 5th job V Matrix cores (5차 V매트릭스 코어 정보)
    pub v_matrix: Option<CharacterVMatrix>,
    /// Set effects of equipped items (장비 세트 효과)
    pub set_effect: Option<CharacterSetEffect>,
    /// Pet equipment and pet buffs (장착 펫 3마리 및 펫 장비 스탯)
    pub pet_equipment: Option<CharacterPetEquipment>,
    /// Android equipment and mechanical heart (안드로이드 및 기계심장)
    pub android_equipment: Option<CharacterAndroidEquipment>,
    /// Link skills (링크 스킬)
    pub link_skill: Option<CharacterLinkSkill>,
    /// Propensities (성향 6종: 카리스마, 감성, 통찰력, 의지, 손재주, 매력)
    pub propensity: Option<CharacterPropensity>,
    /// Popularity (인기도)
    pub popularity: Option<CharacterPopularity>,
    /// Mu Lung Dojo record (무릉도장 최고 기록)
    pub dojang: Option<CharacterDojang>,
    /// Union basic info (유니온 레벨, 등급)
    pub union: Option<crate::models::union::UserUnion>,
    /// Union artifact info (아티팩트 레벨, 효과, 크리스탈)
    pub union_artifact: Option<crate::models::union::UserUnionArtifact>,
    /// Guild basic info (길드명, 레벨, 길드마스터, 노블레스 스킬)
    /// Automatically looked up if the character belongs to a guild.
    pub guild: Option<crate::models::guild::GuildBasic>,
}

impl CharacterDashboard {
    /// Character name.
    pub fn character_name(&self) -> &str {
        self.basic.character_name.as_deref().unwrap_or("")
    }

    /// World / server name (e.g. "스카니아").
    pub fn world_name(&self) -> &str {
        self.basic.world_name.as_deref().unwrap_or("")
    }

    /// Character class / job (e.g. "아크메이지(썬,콜)").
    pub fn character_class(&self) -> &str {
        self.basic.character_class.as_deref().unwrap_or("")
    }

    /// Character level.
    pub fn character_level(&self) -> i64 {
        self.basic.character_level.unwrap_or(0)
    }

    /// Character official render image URL.
    pub fn character_image(&self) -> Option<&str> {
        self.basic.character_image.as_deref()
    }

    /// Combat power (전투력).
    pub fn combat_power(&self) -> i64 {
        self.stat.combat_power().unwrap_or(0)
    }

    /// Total Starforce sum across all currently active equipped items.
    pub fn total_starforce(&self) -> i64 {
        self.item_equipment.total_starforce()
    }

    /// Arcane Force sum from symbol equipment (아케인포스 총합).
    pub fn arcane_force(&self) -> i64 {
        self.symbols
            .as_ref()
            .map(|s| s.total_arcane_force())
            .unwrap_or(0)
    }

    /// Authentic Force sum from symbol equipment (어센틱포스 총합).
    pub fn authentic_force(&self) -> i64 {
        self.symbols
            .as_ref()
            .map(|s| s.total_authentic_force())
            .unwrap_or(0)
    }

    /// Union level if available.
    pub fn union_level(&self) -> Option<i64> {
        self.union.as_ref().and_then(|u| u.union_level)
    }

    /// Union grade if available (e.g. "그랜드 마스터 유니온 4").
    pub fn union_grade(&self) -> Option<&str> {
        self.union.as_ref().and_then(|u| u.union_grade.as_deref())
    }

    /// Union artifact level if available.
    pub fn union_artifact_level(&self) -> Option<i64> {
        self.union.as_ref().and_then(|u| u.union_artifact_level)
    }

    /// Guild name if the character belongs to a guild.
    pub fn guild_name(&self) -> Option<&str> {
        self.basic.character_guild_name.as_deref()
    }

    /// Guild master name if guild info is loaded.
    pub fn guild_master_name(&self) -> Option<&str> {
        self.guild
            .as_ref()
            .and_then(|g| g.guild_master_name.as_deref())
    }

    /// Guild level if guild info is loaded.
    pub fn guild_level(&self) -> Option<i64> {
        self.guild.as_ref().and_then(|g| g.guild_level)
    }

    /// Total Noblesse skill points of the character's guild (노블레스 스킬 총합 포인트).
    pub fn noblesse_skill_points(&self) -> i64 {
        self.guild
            .as_ref()
            .map(|g| g.total_noblesse_points())
            .unwrap_or(0)
    }

    /// Ability grade (e.g. "레전드리", "유니크") if available.
    pub fn ability_grade(&self) -> Option<&str> {
        self.ability
            .as_ref()
            .and_then(|a| a.ability_grade.as_deref())
    }

    /// Highest floor reached in Mu Lung Dojo (무릉도장 최고 층수).
    pub fn dojang_best_floor(&self) -> Option<i64> {
        self.dojang.as_ref().and_then(|d| d.dojang_best_floor)
    }

    /// Character popularity score if available.
    pub fn popularity(&self) -> i64 {
        self.popularity
            .as_ref()
            .and_then(|p| p.popularity)
            .unwrap_or(0)
    }

    /// Parsed AvatarCode if available from coordination.
    pub fn avatar_code(&self) -> Option<&crate::avatar::AvatarCode> {
        self.coordination.avatar_code.as_ref()
    }
}
