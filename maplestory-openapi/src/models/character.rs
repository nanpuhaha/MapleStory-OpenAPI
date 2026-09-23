// src/models/character.rs
//! Models for Character tag API endpoints (23 endpoints).

use serde::{Deserialize, Serialize};

use crate::models::common::{
    ColoringPrism, FaceInfo, HairInfo, ItemAddOption, ItemBaseOption, ItemEtcOption,
    ItemExceptionalOption, ItemKvOption, ItemStarforceOption, ItemTotalOption, PetAutoSkill,
    PetItemEquipment, SkinInfo, SkillDetail,
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
