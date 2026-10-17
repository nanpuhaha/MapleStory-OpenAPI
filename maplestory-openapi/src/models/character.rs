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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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

/// Category of character stat boosted by potential or equipment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatType {
    /// STR (힘)
    Str,
    /// DEX (민첩)
    Dex,
    /// INT (지력)
    Int,
    /// LUK (행운)
    Luk,
    /// All Stat (올스탯)
    AllStat,
    /// Max HP (최대 HP)
    MaxHp,
    /// Max MP (최대 MP)
    MaxMp,
    /// Attack Power (공격력)
    AttackPower,
    /// Magic Power (마력)
    MagicPower,
    /// Armor / Defense (방어력)
    Armor,
    /// Boss Damage (보스 몬스터 공격 시 데미지)
    BossDamage,
    /// Ignore Monster Armor / Defense (몬스터 방어율 무시)
    IgnoreMonsterArmor,
    /// Damage (데미지)
    Damage,
    /// Critical Damage (크리티컬 데미지)
    CriticalDamage,
    /// Critical Rate (크리티컬 확률)
    CriticalRate,
    /// Item Drop Rate (아이템 드롭률 / 획득 확률)
    ItemDropRate,
    /// Meso Drop Rate (메소 획득량)
    MesoDropRate,
    /// Cooldown Reduction (스킬 재사용 대기시간 감소)
    CooldownReduction,
    /// Other / Unclassified stat
    Other,
}

impl StatType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatType::Str => "STR",
            StatType::Dex => "DEX",
            StatType::Int => "INT",
            StatType::Luk => "LUK",
            StatType::AllStat => "올스탯",
            StatType::MaxHp => "최대 HP",
            StatType::MaxMp => "최대 MP",
            StatType::AttackPower => "공격력",
            StatType::MagicPower => "마력",
            StatType::Armor => "방어력",
            StatType::BossDamage => "보스 몬스터 공격 시 데미지",
            StatType::IgnoreMonsterArmor => "몬스터 방어율 무시",
            StatType::Damage => "데미지",
            StatType::CriticalDamage => "크리티컬 데미지",
            StatType::CriticalRate => "크리티컬 확률",
            StatType::ItemDropRate => "아이템 드롭률",
            StatType::MesoDropRate => "메소 획득량",
            StatType::CooldownReduction => "모든 스킬의 재사용 대기시간",
            StatType::Other => "기타",
        }
    }

    /// Classify a Korean or English MapleStory stat name into a [`StatType`].
    pub fn from_stat_name(stat_name: &str) -> Option<Self> {
        let stat_name = stat_name.trim();
        if stat_name == "STR" || stat_name == "힘" {
            Some(StatType::Str)
        } else if stat_name == "DEX" || stat_name == "민첩" {
            Some(StatType::Dex)
        } else if stat_name == "INT" || stat_name == "지력" {
            Some(StatType::Int)
        } else if stat_name == "LUK" || stat_name == "행운" {
            Some(StatType::Luk)
        } else if stat_name == "올스탯" {
            Some(StatType::AllStat)
        } else if stat_name == "최대 HP" || stat_name == "HP" {
            Some(StatType::MaxHp)
        } else if stat_name == "최대 MP" || stat_name == "MP" {
            Some(StatType::MaxMp)
        } else if stat_name == "공격력" {
            Some(StatType::AttackPower)
        } else if stat_name == "마력" {
            Some(StatType::MagicPower)
        } else if stat_name == "방어력" {
            Some(StatType::Armor)
        } else if stat_name.contains("보스") && stat_name.contains("데미지") {
            Some(StatType::BossDamage)
        } else if stat_name.contains("방어율") && stat_name.contains("무시") {
            Some(StatType::IgnoreMonsterArmor)
        } else if stat_name == "데미지" {
            Some(StatType::Damage)
        } else if stat_name.contains("크리티컬") && stat_name.contains("데미지") {
            Some(StatType::CriticalDamage)
        } else if stat_name.contains("크리티컬") && stat_name.contains("확률") {
            Some(StatType::CriticalRate)
        } else if stat_name.contains("아이템")
            && (stat_name.contains("드롭") || stat_name.contains("획득"))
        {
            Some(StatType::ItemDropRate)
        } else if stat_name.contains("메소") && stat_name.contains("획득") {
            Some(StatType::MesoDropRate)
        } else if stat_name.contains("재사용") && stat_name.contains("대기시간") {
            Some(StatType::CooldownReduction)
        } else {
            None
        }
    }
}

impl std::fmt::Display for StatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A parsed potential or additional potential line with typed category and numeric value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedPotentialOption {
    /// Raw unparsed string from NEXON OpenAPI (e.g. `"STR : +12%"`).
    pub raw: String,
    /// Stat category.
    pub stat_type: StatType,
    /// Parsed numeric magnitude (e.g. `12.0` from `"+12%"` or `30.0` from `"+30"`).
    pub value: f64,
    /// Whether the value represents a percentage boost (`true`) or flat boost (`false`).
    pub is_percentage: bool,
}

impl ParsedPotentialOption {
    /// Attempt to parse a potential line string from NEXON MapleStory OpenAPI.
    ///
    /// Returns `None` for non-stat lines (e.g. "피격 시 일정 확률로 데미지 무시",
    /// "쓸만한 윈드 부스터 스킬 사용 가능") or malformed strings.
    pub fn parse(line: &str) -> Option<Self> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return None;
        }

        // Split by ':' if present, or before '+' / '-' sign, or at the last space
        let (stat_name, val_part) = if let Some(idx) = trimmed.find(':') {
            (trimmed[..idx].trim(), trimmed[idx + 1..].trim())
        } else if let Some(idx) = trimmed.rfind(['+', '-']) {
            (trimmed[..idx].trim(), trimmed[idx..].trim())
        } else {
            let idx = trimmed.rfind(' ')?;
            (trimmed[..idx].trim(), trimmed[idx + 1..].trim())
        };

        let stat_type = StatType::from_stat_name(stat_name)?;

        // Parse value & percentage
        if stat_type == StatType::CooldownReduction {
            // E.g. "-2초(10초 이하는 10%감소, 5초 미만으로 감소 불가)" or "-1초"
            let sec_idx = val_part.find('초')?;
            let num_str = &val_part[..sec_idx];
            let num: f64 = num_str
                .trim()
                .trim_start_matches('+')
                .trim_start_matches('-')
                .parse()
                .ok()?;
            return Some(Self {
                raw: line.to_string(),
                stat_type,
                value: num,
                is_percentage: false,
            });
        }

        let is_percentage = val_part.contains('%');
        // Extract numeric digits and optional decimal point
        let mut num_str = String::new();
        for ch in val_part.chars() {
            if ch.is_ascii_digit() || ch == '.' {
                num_str.push(ch);
            } else if !num_str.is_empty() {
                break;
            }
        }

        if num_str.is_empty() {
            return None;
        }

        let value = num_str.parse::<f64>().ok()?;

        Some(Self {
            raw: line.to_string(),
            stat_type,
            value,
            is_percentage,
        })
    }
}

impl ItemEquipmentDetail {
    /// Parsed main potential line 1.
    pub fn potential_option_1_parsed(&self) -> Option<ParsedPotentialOption> {
        self.potential_option_1
            .as_deref()
            .and_then(ParsedPotentialOption::parse)
    }

    /// Parsed main potential line 2.
    pub fn potential_option_2_parsed(&self) -> Option<ParsedPotentialOption> {
        self.potential_option_2
            .as_deref()
            .and_then(ParsedPotentialOption::parse)
    }

    /// Parsed main potential line 3.
    pub fn potential_option_3_parsed(&self) -> Option<ParsedPotentialOption> {
        self.potential_option_3
            .as_deref()
            .and_then(ParsedPotentialOption::parse)
    }

    /// Parsed additional potential line 1.
    pub fn additional_potential_option_1_parsed(&self) -> Option<ParsedPotentialOption> {
        self.additional_potential_option_1
            .as_deref()
            .and_then(ParsedPotentialOption::parse)
    }

    /// Parsed additional potential line 2.
    pub fn additional_potential_option_2_parsed(&self) -> Option<ParsedPotentialOption> {
        self.additional_potential_option_2
            .as_deref()
            .and_then(ParsedPotentialOption::parse)
    }

    /// Parsed additional potential line 3.
    pub fn additional_potential_option_3_parsed(&self) -> Option<ParsedPotentialOption> {
        self.additional_potential_option_3
            .as_deref()
            .and_then(ParsedPotentialOption::parse)
    }

    /// All parsed main potential lines.
    pub fn all_potential_options_parsed(&self) -> Vec<ParsedPotentialOption> {
        [
            self.potential_option_1_parsed(),
            self.potential_option_2_parsed(),
            self.potential_option_3_parsed(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    /// All parsed additional potential lines.
    pub fn all_additional_potential_options_parsed(&self) -> Vec<ParsedPotentialOption> {
        [
            self.additional_potential_option_1_parsed(),
            self.additional_potential_option_2_parsed(),
            self.additional_potential_option_3_parsed(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    /// All parsed potential lines across both main and additional potentials.
    pub fn all_potentials_parsed(&self) -> Vec<ParsedPotentialOption> {
        let mut all = self.all_potential_options_parsed();
        all.extend(self.all_additional_potential_options_parsed());
        all
    }

    /// Total potential stat contribution for a given [`StatType`].
    ///
    /// Returns `(flat_boost, percentage_boost)`.
    pub fn total_potential_stat(&self, stat: StatType) -> (f64, f64) {
        let mut flat = 0.0;
        let mut percent = 0.0;

        for opt in self.all_potentials_parsed() {
            if opt.stat_type == stat {
                if opt.is_percentage {
                    percent += opt.value;
                } else {
                    flat += opt.value;
                }
            }
        }

        (flat, percent)
    }
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

impl CharacterSetEffect {
    /// Check whether a set effect is active with at least `required_count` items equipped.
    pub fn is_set_active(&self, set_name: &str, required_count: i64) -> bool {
        self.find_set(set_name)
            .and_then(|s| s.total_set_count)
            .map(|count| count >= required_count)
            .unwrap_or(false)
    }

    /// Find a set effect by name (substring match).
    pub fn find_set(&self, set_name: &str) -> Option<&SetEffectInfo> {
        self.set_effect.as_deref().unwrap_or(&[]).iter().find(|s| {
            s.set_name
                .as_deref()
                .map(|name| name.contains(set_name))
                .unwrap_or(false)
        })
    }

    /// Retrieve all active set effect options across all equipped sets.
    ///
    /// Filters set effect options by checking whether the equipped piece count
    /// ([`SetEffectInfo::total_set_count`]) meets or exceeds the required threshold
    /// ([`SetEffectOption::set_count`]).
    pub fn active_set_options(&self) -> Vec<&SetEffectOption> {
        self.set_effect
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .flat_map(|s| {
                let total = s.total_set_count.unwrap_or(0);
                s.set_effect_info
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter(move |opt| opt.set_count.is_none_or(|req| total >= req))
            })
            .collect()
    }
}

// ─── Beauty Equipment ─────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/character/beauty-equipment`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

    /// Calculate total Starforce stat contribution breakdown across all currently active equipped items.
    pub fn starforce_stat_breakdown(&self) -> StarforceStatBreakdown {
        let mut breakdown = StarforceStatBreakdown {
            total_starforce: self.total_starforce(),
            ..Default::default()
        };
        for item in self.active_items() {
            if let Some(opt) = &item.item_starforce_option {
                if let Some(v) = opt.str.as_deref().and_then(|s| s.parse::<i64>().ok()) {
                    breakdown.str += v;
                }
                if let Some(v) = opt.dex.as_deref().and_then(|s| s.parse::<i64>().ok()) {
                    breakdown.dex += v;
                }
                if let Some(v) = opt.int.as_deref().and_then(|s| s.parse::<i64>().ok()) {
                    breakdown.int += v;
                }
                if let Some(v) = opt.luk.as_deref().and_then(|s| s.parse::<i64>().ok()) {
                    breakdown.luk += v;
                }
                if let Some(v) = opt.max_hp.as_deref().and_then(|s| s.parse::<i64>().ok()) {
                    breakdown.max_hp += v;
                }
                if let Some(v) = opt.max_mp.as_deref().and_then(|s| s.parse::<i64>().ok()) {
                    breakdown.max_mp += v;
                }
                if let Some(v) = opt
                    .attack_power
                    .as_deref()
                    .and_then(|s| s.parse::<i64>().ok())
                {
                    breakdown.attack_power += v;
                }
                if let Some(v) = opt
                    .magic_power
                    .as_deref()
                    .and_then(|s| s.parse::<i64>().ok())
                {
                    breakdown.magic_power += v;
                }
                if let Some(v) = opt.armor.as_deref().and_then(|s| s.parse::<i64>().ok()) {
                    breakdown.armor += v;
                }
                if let Some(v) = opt.speed.as_deref().and_then(|s| s.parse::<i64>().ok()) {
                    breakdown.speed += v;
                }
                if let Some(v) = opt.jump.as_deref().and_then(|s| s.parse::<i64>().ok()) {
                    breakdown.jump += v;
                }
            }
        }
        breakdown
    }
}

/// Aggregated Starforce statistics across all equipped items.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarforceStatBreakdown {
    /// Total stars across equipped items
    pub total_starforce: i64,
    /// Total STR gained from Starforce
    pub str: i64,
    /// Total DEX gained from Starforce
    pub dex: i64,
    /// Total INT gained from Starforce
    pub int: i64,
    /// Total LUK gained from Starforce
    pub luk: i64,
    /// Total MaxHP gained from Starforce
    pub max_hp: i64,
    /// Total MaxMP gained from Starforce
    pub max_mp: i64,
    /// Total Attack Power gained from Starforce
    pub attack_power: i64,
    /// Total Magic Power gained from Starforce
    pub magic_power: i64,
    /// Total Armor gained from Starforce
    pub armor: i64,
    /// Total Speed gained from Starforce
    pub speed: i64,
    /// Total Jump gained from Starforce
    pub jump: i64,
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
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
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

/// Dedicated layering engine for equipment appearance, cash overrides, anvil items, and coordination.
#[derive(Debug, Clone, Copy, Default)]
pub struct EquipLayerResolver;

impl EquipLayerResolver {
    /// Resolves visible equipment items and beauty state into a [`CharacterCoordination`].
    pub fn resolve(
        beauty: &CharacterBeautyEquipment,
        cash_equip: &CharacterCashItemEquipment,
        normal_equip: &CharacterItemEquipment,
    ) -> CharacterCoordination {
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

            // Helper closure to match cash item with slot including overall (한벌옷) precedence
            let is_matching_cash_slot = |item: &CashItemEquipmentDetail, slot_name: &str| -> bool {
                item.cash_item_equipment_slot.as_deref() == Some(slot_name)
                    || item.cash_item_equipment_part.as_deref() == Some(slot_name)
                    || (slot_name == "상의"
                        && (item.cash_item_equipment_part.as_deref() == Some("한벌옷")
                            || item.cash_item_equipment_slot.as_deref() == Some("한벌옷")))
            };

            // Find cash item for this slot in active preset, with fallback to base
            let cash_match = cash_items
                .iter()
                .enumerate()
                .find(|(idx, item)| {
                    !handled_cash_indices.contains(idx) && is_matching_cash_slot(item, slot)
                })
                .or_else(|| {
                    // Fallback to base cash items if active preset slot is empty
                    base_cash_items
                        .iter()
                        .enumerate()
                        .find(|(_, item)| is_matching_cash_slot(item, slot))
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
                let is_transparent = item_name.starts_with("투명") || item_name.contains("투명");

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
                let is_transparent = item_name.starts_with("투명") || item_name.contains("투명");

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

        CharacterCoordination {
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
}

impl CharacterCoordination {
    /// Synthesizes coordination from separate beauty, cash, and normal equipment models.
    pub fn from_parts(
        beauty: &CharacterBeautyEquipment,
        cash_equip: &CharacterCashItemEquipment,
        normal_equip: &CharacterItemEquipment,
    ) -> Self {
        EquipLayerResolver::resolve(beauty, cash_equip, normal_equip)
    }

    /// Synthesizes coordination from equipment and cash layers (alias for [`EquipLayerResolver::resolve`]).
    pub fn from_equipment_and_cash(
        beauty: &CharacterBeautyEquipment,
        cash_equip: &CharacterCashItemEquipment,
        normal_equip: &CharacterItemEquipment,
    ) -> Self {
        EquipLayerResolver::resolve(beauty, cash_equip, normal_equip)
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

/// Individual sections that compose the [`CharacterDashboard`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DashboardSection {
    /// Basic character profile (이름, 월드, 직업, 레벨, 경험치, 이미지 등)
    Basic,
    /// Detailed stats and combat power (스탯, 전투력 등)
    Stat,
    /// Normal equipment (일반 장비, 스타포스 등)
    ItemEquipment,
    /// Cash equipment (캐시 장비 및 프리즘 정보)
    CashItemEquipment,
    /// Beauty features (헤어, 성형, 피부)
    BeautyEquipment,
    /// Arcane and authentic symbols (심볼 장비 및 포스)
    Symbols,
    /// Character ability (어빌리티 등급 및 옵션)
    Ability,
    /// Hyper stats (하이퍼 스탯)
    HyperStat,
    /// 6th job HEXA Matrix cores (6차 헥사 코어)
    HexaMatrix,
    /// 6th job HEXA Matrix stats (6차 헥사 스탯)
    HexaMatrixStat,
    /// 5th job V Matrix cores (5차 V매트릭스 코어)
    VMatrix,
    /// Set effects of equipped items (장비 세트 효과)
    SetEffect,
    /// Pet equipment (장착 펫 및 펫 장비)
    PetEquipment,
    /// Android equipment (안드로이드 및 기계심장)
    AndroidEquipment,
    /// Link skills (링크 스킬)
    LinkSkill,
    /// Propensities (성향 6종)
    Propensity,
    /// Popularity (인기도)
    Popularity,
    /// Mu Lung Dojo record (무릉도장 최고 기록)
    Dojang,
    /// Union basic info (유니온 레벨, 등급)
    Union,
    /// Union artifact info (아티팩트 레벨, 효과)
    UnionArtifact,
    /// Guild basic info (길드명, 레벨, 노블레스 스킬)
    Guild,
}

/// Diagnostic failure information for a specific dashboard section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSectionError {
    /// The section that experienced an error
    pub section: DashboardSection,
    /// Formatted error message
    pub message: String,
    /// OpenAPI error code if returned by NEXON (e.g. "OPENAPI00004")
    pub api_code: Option<String>,
    /// HTTP status code if available (e.g. 429, 500)
    pub status_code: Option<u16>,
}

impl DashboardSectionError {
    /// Create a new section error from a [`MapleStoryError`](crate::error::MapleStoryError).
    pub fn from_error(section: DashboardSection, err: &crate::error::MapleStoryError) -> Self {
        Self {
            section,
            message: err.to_string(),
            api_code: err.api_code().map(str::to_string),
            status_code: err.status_code(),
        }
    }
}

/// Options and section mask controlling what data is queried when fetching [`CharacterDashboard`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardOptions {
    /// Enabled dashboard sections
    pub enabled_sections: std::collections::HashSet<DashboardSection>,
}

impl Default for DashboardOptions {
    fn default() -> Self {
        Self::all()
    }
}

impl DashboardOptions {
    /// Enable all dashboard sections (default behavior).
    pub fn all() -> Self {
        use DashboardSection::*;
        let sections = [
            Basic,
            Stat,
            ItemEquipment,
            CashItemEquipment,
            BeautyEquipment,
            Symbols,
            Ability,
            HyperStat,
            HexaMatrix,
            HexaMatrixStat,
            VMatrix,
            SetEffect,
            PetEquipment,
            AndroidEquipment,
            LinkSkill,
            Propensity,
            Popularity,
            Dojang,
            Union,
            UnionArtifact,
            Guild,
        ];
        Self {
            enabled_sections: sections.into_iter().collect(),
        }
    }

    /// Enable all sections except the specified excluded sections.
    pub fn all_except(excluded: &[DashboardSection]) -> Self {
        let mut opts = Self::all();
        for section in excluded {
            opts.enabled_sections.remove(section);
        }
        opts
    }

    /// Create a builder to selectively configure enabled sections.
    pub fn builder() -> DashboardOptionsBuilder {
        DashboardOptionsBuilder::default()
    }

    /// Returns `true` if the given section is enabled.
    pub fn is_enabled(&self, section: DashboardSection) -> bool {
        self.enabled_sections.contains(&section)
    }
}

/// Builder for [`DashboardOptions`].
#[derive(Debug, Default)]
pub struct DashboardOptionsBuilder {
    sections: std::collections::HashSet<DashboardSection>,
}

impl DashboardOptionsBuilder {
    /// Include a section.
    pub fn include(mut self, section: DashboardSection) -> Self {
        self.sections.insert(section);
        self
    }

    /// Exclude a section.
    pub fn exclude(mut self, section: DashboardSection) -> Self {
        self.sections.remove(&section);
        self
    }

    /// Build the [`DashboardOptions`].
    pub fn build(self) -> DashboardOptions {
        DashboardOptions {
            enabled_sections: self.sections,
        }
    }
}

/// Raw result payloads collected for all character dashboard sections before assembly and degradation.
#[derive(Debug)]
pub struct RawDashboardSections {
    pub basic: Result<CharacterBasic, crate::error::MapleStoryError>,
    pub stat: Option<Result<CharacterStat, crate::error::MapleStoryError>>,
    pub item_equipment: Option<Result<CharacterItemEquipment, crate::error::MapleStoryError>>,
    pub cash_item_equipment: Option<Result<CharacterCashItemEquipment, crate::error::MapleStoryError>>,
    pub beauty_equipment: Option<Result<CharacterBeautyEquipment, crate::error::MapleStoryError>>,
    pub symbols: Option<Result<CharacterSymbolEquipment, crate::error::MapleStoryError>>,
    pub ability: Option<Result<CharacterAbility, crate::error::MapleStoryError>>,
    pub hyper_stat: Option<Result<CharacterHyperStat, crate::error::MapleStoryError>>,
    pub hexa_matrix: Option<Result<CharacterHexaMatrix, crate::error::MapleStoryError>>,
    pub hexa_matrix_stat: Option<Result<CharacterHexaMatrixStat, crate::error::MapleStoryError>>,
    pub v_matrix: Option<Result<CharacterVMatrix, crate::error::MapleStoryError>>,
    pub set_effect: Option<Result<CharacterSetEffect, crate::error::MapleStoryError>>,
    pub pet_equipment: Option<Result<CharacterPetEquipment, crate::error::MapleStoryError>>,
    pub android_equipment: Option<Result<CharacterAndroidEquipment, crate::error::MapleStoryError>>,
    pub link_skill: Option<Result<CharacterLinkSkill, crate::error::MapleStoryError>>,
    pub propensity: Option<Result<CharacterPropensity, crate::error::MapleStoryError>>,
    pub popularity: Option<Result<CharacterPopularity, crate::error::MapleStoryError>>,
    pub dojang: Option<Result<CharacterDojang, crate::error::MapleStoryError>>,
    pub union: Option<Result<crate::models::union::UserUnion, crate::error::MapleStoryError>>,
    pub union_artifact: Option<Result<crate::models::union::UserUnionArtifact, crate::error::MapleStoryError>>,
    pub guild: Option<Result<crate::models::guild::GuildBasic, crate::error::MapleStoryError>>,
}

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
    /// Diagnostic errors for individual sections that failed during aggregation.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub errors: std::collections::HashMap<DashboardSection, DashboardSectionError>,
}

impl CharacterDashboard {
    /// Pure synchronous assembler that aggregates raw section results into a complete [`CharacterDashboard`].
    ///
    /// Essential section (`basic`) error is propagated as a top-level error.
    /// Non-essential section errors are caught, mapped to default fallback models,
    /// and registered in `self.errors` for diagnostic reporting.
    pub fn assemble(
        raw: RawDashboardSections,
        options: &DashboardOptions,
    ) -> Result<Self, crate::error::MapleStoryError> {
        let _ = options; // Options metadata retained for interface depth and callers
        let basic = raw.basic?;
        let mut errors = std::collections::HashMap::new();

        macro_rules! unwrap_sec {
            ($field:expr, $section:ident) => {
                match $field {
                    Some(Ok(v)) => Some(v),
                    Some(Err(e)) => {
                        errors.insert(
                            DashboardSection::$section,
                            DashboardSectionError::from_error(DashboardSection::$section, &e),
                        );
                        None
                    }
                    None => None,
                }
            };
        }

        let stat = match raw.stat {
            Some(Ok(s)) => s,
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::Stat,
                    DashboardSectionError::from_error(DashboardSection::Stat, &e),
                );
                CharacterStat {
                    character_class: basic.character_class.clone(),
                    ..Default::default()
                }
            }
            None => CharacterStat {
                character_class: basic.character_class.clone(),
                ..Default::default()
            },
        };

        let item_equipment = match raw.item_equipment {
            Some(Ok(eq)) => eq,
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::ItemEquipment,
                    DashboardSectionError::from_error(DashboardSection::ItemEquipment, &e),
                );
                CharacterItemEquipment {
                    character_class: basic.character_class.clone(),
                    ..Default::default()
                }
            }
            None => CharacterItemEquipment {
                character_class: basic.character_class.clone(),
                ..Default::default()
            },
        };

        let cash_item_equipment = match raw.cash_item_equipment {
            Some(Ok(c)) => c,
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::CashItemEquipment,
                    DashboardSectionError::from_error(DashboardSection::CashItemEquipment, &e),
                );
                CharacterCashItemEquipment::default()
            }
            None => CharacterCashItemEquipment::default(),
        };

        let beauty_equipment = match raw.beauty_equipment {
            Some(Ok(b)) => b,
            Some(Err(e)) => {
                errors.insert(
                    DashboardSection::BeautyEquipment,
                    DashboardSectionError::from_error(DashboardSection::BeautyEquipment, &e),
                );
                CharacterBeautyEquipment::default()
            }
            None => CharacterBeautyEquipment::default(),
        };

        let coordination = CharacterCoordination::from_full_parts(
            &basic,
            &beauty_equipment,
            &cash_item_equipment,
            &item_equipment,
        );

        let symbols = unwrap_sec!(raw.symbols, Symbols);
        let ability = unwrap_sec!(raw.ability, Ability);
        let hyper_stat = unwrap_sec!(raw.hyper_stat, HyperStat);
        let hexa_matrix = unwrap_sec!(raw.hexa_matrix, HexaMatrix);
        let hexa_matrix_stat = unwrap_sec!(raw.hexa_matrix_stat, HexaMatrixStat);
        let v_matrix = unwrap_sec!(raw.v_matrix, VMatrix);
        let set_effect = unwrap_sec!(raw.set_effect, SetEffect);
        let pet_equipment = unwrap_sec!(raw.pet_equipment, PetEquipment);
        let android_equipment = unwrap_sec!(raw.android_equipment, AndroidEquipment);
        let link_skill = unwrap_sec!(raw.link_skill, LinkSkill);
        let propensity = unwrap_sec!(raw.propensity, Propensity);
        let popularity = unwrap_sec!(raw.popularity, Popularity);
        let dojang = unwrap_sec!(raw.dojang, Dojang);
        let union = unwrap_sec!(raw.union, Union);
        let union_artifact = unwrap_sec!(raw.union_artifact, UnionArtifact);
        let guild = unwrap_sec!(raw.guild, Guild);

        Ok(CharacterDashboard {
            basic,
            stat,
            item_equipment,
            cash_item_equipment,
            beauty_equipment,
            coordination,
            symbols,
            ability,
            hyper_stat,
            hexa_matrix,
            hexa_matrix_stat,
            v_matrix,
            set_effect,
            pet_equipment,
            android_equipment,
            link_skill,
            propensity,
            popularity,
            dojang,
            union,
            union_artifact,
            guild,
            errors,
        })
    }

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

    /// Total Starforce stat contribution breakdown across equipped items.
    pub fn starforce_stat_breakdown(&self) -> StarforceStatBreakdown {
        self.item_equipment.starforce_stat_breakdown()
    }

    /// Returns `true` if any non-critical sections experienced errors during aggregation.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns diagnostic error information for a specific section, if it failed.
    pub fn section_error(&self, section: DashboardSection) -> Option<&DashboardSectionError> {
        self.errors.get(&section)
    }

    /// Extracts combat power, boss multipliers, and spec breakdowns into a [`CharacterCombatAnalytics`].
    pub fn combat_analytics(&self) -> CharacterCombatAnalytics {
        CharacterCombatAnalytics {
            character_name: self.character_name().to_string(),
            world_name: self.basic.world_name.clone(),
            character_class: self.basic.character_class.clone(),
            character_level: self.character_level(),
            combat_power: self.combat_power(),
            boss_damage: self.stat.boss_damage().unwrap_or(0.0),
            ignore_monster_armor: self.stat.ignore_monster_armor().unwrap_or(0.0),
            damage: self.stat.damage().unwrap_or(0.0),
            critical_rate: self.stat.critical_rate().unwrap_or(0.0),
            critical_damage: self.stat.critical_damage().unwrap_or(0.0),
            total_starforce: self.total_starforce(),
            arcane_force: self.arcane_force(),
            authentic_force: self.authentic_force(),
            dojang_best_floor: self.dojang_best_floor(),
            starforce_stat_breakdown: Some(self.starforce_stat_breakdown()),
        }
    }
}

// ─── Character Combat Analytics (전투 스펙 및 보스 대미지 심화 분석) ──────────────

/// Comprehensive combat power, boss armor penetration, and damage factor analytics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterCombatAnalytics {
    /// Character name
    pub character_name: String,
    /// World name
    pub world_name: Option<String>,
    /// Character class
    pub character_class: Option<String>,
    /// Character level
    pub character_level: i64,
    /// In-game Combat Power (전투력)
    pub combat_power: i64,
    /// Boss monster damage percentage (보스 몬스터 데미지 %)
    pub boss_damage: f64,
    /// Ignore Monster Armor percentage (방어율 무시 %)
    pub ignore_monster_armor: f64,
    /// General damage percentage (데미지 %)
    pub damage: f64,
    /// Critical rate percentage (크리티컬 확률 %)
    pub critical_rate: f64,
    /// Critical damage percentage (크리티컬 데미지 %)
    pub critical_damage: f64,
    /// Total Starforce across equipped items (스타포스 총합)
    pub total_starforce: i64,
    /// Total Arcane Force from symbols (아케인포스 총합)
    pub arcane_force: i64,
    /// Total Authentic Force from symbols (어센틱포스 총합)
    pub authentic_force: i64,
    /// Highest floor reached in Mu Lung Dojo (무릉도장 최고 기록 층수)
    pub dojang_best_floor: Option<i64>,
    /// Starforce stat contribution breakdown (if item equipment is available)
    pub starforce_stat_breakdown: Option<StarforceStatBreakdown>,
}

impl CharacterCombatAnalytics {
    /// Constructs [`CharacterCombatAnalytics`] directly from a full [`CharacterDashboard`].
    pub fn from_dashboard(dashboard: &CharacterDashboard) -> Self {
        dashboard.combat_analytics()
    }

    /// Constructs [`CharacterCombatAnalytics`] directly from individual stats without requiring a full dashboard.
    pub fn from_stats(
        basic: &CharacterBasic,
        stat: &CharacterStat,
        item_equipment: Option<&CharacterItemEquipment>,
        dojang: Option<&CharacterDojang>,
        arcane_force: i64,
        authentic_force: i64,
        hyper_stat: Option<&CharacterHyperStat>,
    ) -> Self {
        let _ = hyper_stat; // Hyper stat parameter retained for deep spec calculation
        let starforce_stat_breakdown = item_equipment.map(|eq| eq.starforce_stat_breakdown());
        let total_starforce = item_equipment
            .map(|eq| eq.total_starforce())
            .unwrap_or(0);

        Self {
            character_name: basic.character_name.clone().unwrap_or_default(),
            world_name: basic.world_name.clone(),
            character_class: basic.character_class.clone(),
            character_level: basic.character_level.unwrap_or(0),
            combat_power: stat.combat_power().unwrap_or(0),
            boss_damage: stat.boss_damage().unwrap_or(0.0),
            ignore_monster_armor: stat.ignore_monster_armor().unwrap_or(0.0),
            damage: stat.damage().unwrap_or(0.0),
            critical_rate: stat.critical_rate().unwrap_or(0.0),
            critical_damage: stat.critical_damage().unwrap_or(0.0),
            total_starforce,
            arcane_force,
            authentic_force,
            dojang_best_floor: dojang.and_then(|d| d.dojang_best_floor),
            starforce_stat_breakdown,
        }
    }

    /// Calculate effective damage multiplier against a monster with the given armor/defense rate percentage.
    ///
    /// For example, against a 300% defense boss (`defense_rate_pct = 300.0`):
    /// - If character IED is 95%: reduction = 3.0 * (1.0 - 0.95) = 0.15 (15% reduction) -> 85% damage multiplier (0.85).
    /// - If character IED is 90%: reduction = 3.0 * (1.0 - 0.90) = 0.30 (30% reduction) -> 70% damage multiplier (0.70).
    pub fn effective_damage_multiplier_against_defense(&self, defense_rate_pct: f64) -> f64 {
        let monster_armor_ratio = defense_rate_pct / 100.0;
        let ied_ratio = (self.ignore_monster_armor / 100.0).clamp(0.0, 1.0);
        let damage_reduction = monster_armor_ratio * (1.0 - ied_ratio);
        (1.0 - damage_reduction).max(0.0)
    }

    /// Effective damage multiplier against standard 300% defense bosses (루시드, 윌, 더스크, 듄케일, 진힐라, 검은마법사 등).
    pub fn effective_damage_multiplier_300(&self) -> f64 {
        self.effective_damage_multiplier_against_defense(300.0)
    }

    /// Effective damage multiplier against extreme 380% defense bosses (익스트림 세렌, 익스트림 칼로스, 카링 등).
    pub fn effective_damage_multiplier_380(&self) -> f64 {
        self.effective_damage_multiplier_against_defense(380.0)
    }

    /// Total boss damage multiplier factor `1.0 + (damage + boss_damage) / 100.0`.
    pub fn total_damage_factor(&self) -> f64 {
        1.0 + (self.damage + self.boss_damage) / 100.0
    }

    /// Critical damage multiplier factor assuming 100% critical rate:
    /// In MapleStory, base critical damage is an average of 135% (1.35).
    pub fn critical_damage_factor(&self) -> f64 {
        1.35 + (self.critical_damage / 100.0)
    }

    fn force_stat_multiplier(job: Option<&str>, is_authentic: bool) -> i64 {
        match (job, is_authentic) {
            (Some("데몬어벤져"), false) => 2100,
            (Some("데몬어벤져"), true) => 4200,
            (Some("제논"), _) => 48,
            (_, false) => 100,
            (_, true) => 200,
        }
    }

    /// Additional main stat bonus contributed by Arcane Force.
    ///
    /// - Demon Avenger: Max HP +2,100 per 10 AF
    /// - Xenon: +48 per 10 AF (STR/DEX/LUK)
    /// - Standard classes: +100 main stat per 10 AF
    pub fn arcane_force_stat_bonus(&self) -> i64 {
        Self::force_stat_multiplier(self.character_class.as_deref(), false) * (self.arcane_force / 10)
    }

    /// Additional main stat bonus contributed by Authentic Force.
    ///
    /// - Demon Avenger: Max HP +4,200 per 10 AutF
    /// - Xenon: +48 per 10 AutF
    /// - Standard classes: +200 main stat per 10 AutF
    pub fn authentic_force_stat_bonus(&self) -> i64 {
        Self::force_stat_multiplier(self.character_class.as_deref(), true) * (self.authentic_force / 10)
    }

    /// Estimated main stat gain from Starforce enhancements.
    pub fn starforce_main_stat_bonus(&self) -> i64 {
        self.starforce_stat_breakdown
            .as_ref()
            .map(|b| b.str.max(b.dex).max(b.int).max(b.luk))
            .unwrap_or(0)
    }

    /// Estimated attack power gain from Starforce enhancements.
    pub fn starforce_attack_bonus(&self) -> i64 {
        self.starforce_stat_breakdown
            .as_ref()
            .map(|b| b.attack_power.max(b.magic_power))
            .unwrap_or(0)
    }

    /// Estimated Mu Lung Dojo floor based on combat power benchmarks.
    pub fn estimated_dojang_floor(&self) -> i64 {
        if self.combat_power <= 0 {
            return 0;
        }
        const DOJANG_BENCHMARKS: &[(i64, i64)] = &[
            (1_000_000_000, 80),
            (600_000_000, 75),
            (300_000_000, 70),
            (150_000_000, 65),
            (70_000_000, 60),
            (35_000_000, 55),
            (15_000_000, 50),
            (6_000_000, 45),
            (3_000_000, 40),
            (1_500_000, 35),
            (500_000, 30),
        ];

        for &(threshold, floor) in DOJANG_BENCHMARKS {
            if self.combat_power >= threshold {
                return floor;
            }
        }
        (self.combat_power / 25_000).clamp(1, 29)
    }

    /// Simulates swapping an equipped item in the given slot with `new_item`.
    ///
    /// Evaluates:
    /// - Item total option differences (STR, DEX, INT, LUK, HP, MP, Attack, Magic, Boss Damage, IED, All Stat)
    /// - Potential and additional potential differences
    /// - Starforce differences
    /// - Set effect changes (piece counts, tier activations/deactivations) if `set_effect` is provided
    /// - Estimated combat power impact
    pub fn simulate_equipment_swap(
        &self,
        current_equipment: &CharacterItemEquipment,
        new_item: &ItemEquipmentDetail,
        set_effect: Option<&CharacterSetEffect>,
    ) -> EquipmentDiff {
        let slot_name = new_item
            .item_equipment_slot
            .as_deref()
            .or(new_item.item_equipment_part.as_deref())
            .unwrap_or("")
            .to_string();

        let previous_item = current_equipment.active_items().iter().find(|item| {
            if !slot_name.is_empty() {
                item.item_equipment_slot.as_deref() == Some(&slot_name)
                    || item.item_equipment_part.as_deref() == Some(&slot_name)
            } else {
                false
            }
        }).cloned();

        let mut prev_bag = ItemStatBag::default();
        if let Some(ref prev) = previous_item {
            fill_item_stat_bag(prev, &mut prev_bag);
        }

        let mut new_bag = ItemStatBag::default();
        fill_item_stat_bag(new_item, &mut new_bag);

        // Calculate set effect diff if set_effect is provided
        let mut set_bag = ItemStatBag::default();
        let set_effect_diff = set_effect.map(|effects| {
            let mut set_count_changes = Vec::new();
            let mut gained_options = Vec::new();
            let mut lost_options = Vec::new();

            if let Some(list) = &effects.set_effect {
                for s in list {
                    let prev_matched = previous_item.as_ref().is_some_and(|it| item_matches_set(it, s));
                    let new_matched = item_matches_set(new_item, s);

                    if prev_matched != new_matched {
                        let old_count = s.total_set_count.unwrap_or(0);
                        let delta = if new_matched { 1 } else { 0 } - if prev_matched { 1 } else { 0 };
                        let new_count = (old_count + delta).max(0);

                        set_count_changes.push(SetCountChange {
                            set_name: s.set_name.clone().unwrap_or_default(),
                            previous_count: old_count,
                            new_count,
                        });

                        // Check tier options
                        let empty_vec = Vec::new();
                        let all_tiers = s.set_option_full.as_ref().or(s.set_effect_info.as_ref()).unwrap_or(&empty_vec);
                        for tier in all_tiers {
                            let req = tier.set_count.unwrap_or(0);
                            if let Some(opt_str) = &tier.set_option {
                                if req <= old_count && req > new_count {
                                    // Lost tier
                                    lost_options.push(format!("{}-set: {}", req, opt_str));
                                    parse_set_option_string(opt_str, &mut set_bag, -1.0);
                                } else if req > old_count && req <= new_count {
                                    // Gained tier
                                    gained_options.push(format!("{}-set: {}", req, opt_str));
                                    parse_set_option_string(opt_str, &mut set_bag, 1.0);
                                }
                            }
                        }
                    }
                }
            }

            SetEffectDiff {
                set_count_changes,
                gained_options,
                lost_options,
            }
        });

        // Combine item diff + set effect diff
        let str_diff = (new_bag.str - prev_bag.str) + set_bag.str;
        let dex_diff = (new_bag.dex - prev_bag.dex) + set_bag.dex;
        let int_diff = (new_bag.int - prev_bag.int) + set_bag.int;
        let luk_diff = (new_bag.luk - prev_bag.luk) + set_bag.luk;
        let max_hp_diff = (new_bag.max_hp - prev_bag.max_hp) + set_bag.max_hp;
        let max_mp_diff = (new_bag.max_mp - prev_bag.max_mp) + set_bag.max_mp;
        let attack_power_diff = (new_bag.attack_power - prev_bag.attack_power) + set_bag.attack_power;
        let magic_power_diff = (new_bag.magic_power - prev_bag.magic_power) + set_bag.magic_power;
        let armor_diff = (new_bag.armor - prev_bag.armor) + set_bag.armor;
        let boss_damage_diff = (new_bag.boss_damage - prev_bag.boss_damage) + set_bag.boss_damage;
        let ignore_monster_armor_diff = (new_bag.ignore_monster_armor - prev_bag.ignore_monster_armor) + set_bag.ignore_monster_armor;
        let damage_diff = (new_bag.damage - prev_bag.damage) + set_bag.damage;
        let all_stat_diff = (new_bag.all_stat - prev_bag.all_stat) + set_bag.all_stat;
        let critical_rate_diff = (new_bag.critical_rate - prev_bag.critical_rate) + set_bag.critical_rate;
        let critical_damage_diff = (new_bag.critical_damage - prev_bag.critical_damage) + set_bag.critical_damage;
        let starforce_diff = new_bag.starforce - prev_bag.starforce;

        let mut stat_diff = StatDiff {
            str: str_diff,
            dex: dex_diff,
            int: int_diff,
            luk: luk_diff,
            max_hp: max_hp_diff,
            max_mp: max_mp_diff,
            attack_power: attack_power_diff,
            magic_power: magic_power_diff,
            armor: armor_diff,
            boss_damage: boss_damage_diff,
            ignore_monster_armor: ignore_monster_armor_diff,
            damage: damage_diff,
            all_stat: all_stat_diff,
            critical_rate: critical_rate_diff,
            critical_damage: critical_damage_diff,
            starforce: starforce_diff,
            combat_power_diff: 0,
            estimated_combat_power: 0,
        };

        // Estimate combat power impact
        let cp = self.combat_power;
        if cp > 0 {
            let current_dmg_factor = (1.0 + (self.damage + self.boss_damage) / 100.0).max(0.01);
            let new_dmg_factor = (1.0 + (self.damage + self.boss_damage + boss_damage_diff + damage_diff) / 100.0).max(0.01);
            let dmg_ratio = new_dmg_factor / current_dmg_factor;

            let current_crit_factor = (1.35 + self.critical_damage / 100.0).max(0.01);
            let new_crit_factor = (1.35 + (self.critical_damage + critical_damage_diff) / 100.0).max(0.01);
            let crit_ratio = new_crit_factor / current_crit_factor;

            let current_ied = (self.ignore_monster_armor / 100.0).clamp(0.0, 0.999);
            let new_ied = ((self.ignore_monster_armor + ignore_monster_armor_diff) / 100.0).clamp(0.0, 0.999);
            let current_ied_factor = 1.0 - 0.5 * (1.0 - current_ied);
            let new_ied_factor = 1.0 - 0.5 * (1.0 - new_ied);
            let ied_ratio = new_ied_factor / current_ied_factor;

            let is_magic = self.character_class.as_deref().is_some_and(|job| {
                job.contains("마법사") || job.contains("메이지") || job.contains("비숍") || job.contains("플레임위자드") || job.contains("에반") || job.contains("배틀메이지") || job.contains("루미너스") || job.contains("일리움") || job.contains("라라") || job.contains("키네시스")
            });
            let att_delta = if is_magic { magic_power_diff as f64 } else { attack_power_diff as f64 };
            let base_att: f64 = current_equipment.active_items().iter().map(|it| {
                if is_magic {
                    parse_stat_i64(it.item_total_option.as_ref().and_then(|o| o.magic_power.as_deref())) as f64
                } else {
                    parse_stat_i64(it.item_total_option.as_ref().and_then(|o| o.attack_power.as_deref())) as f64
                }
            }).sum::<f64>().max(500.0);
            let att_ratio = (base_att + att_delta).max(0.01) / base_att;

            let main_stat_delta = stat_diff.main_stat_for_class(self.character_class.as_deref()) as f64;
            let base_stat: f64 = (self.arcane_force_stat_bonus() + self.authentic_force_stat_bonus()).max(5000) as f64;
            let stat_ratio = (base_stat + main_stat_delta).max(0.01) / base_stat;

            let total_ratio = (dmg_ratio * crit_ratio * ied_ratio * att_ratio * stat_ratio).max(0.0);
            let estimated_cp = (cp as f64 * total_ratio).round() as i64;
            stat_diff.estimated_combat_power = estimated_cp;
            stat_diff.combat_power_diff = estimated_cp - cp;
        } else {
            let main_stat_delta = stat_diff.main_stat_for_class(self.character_class.as_deref());
            let att_delta = attack_power_diff.max(magic_power_diff);
            let rough_cp = (main_stat_delta * 4 + att_delta * 20).max(0);
            stat_diff.estimated_combat_power = rough_cp;
            stat_diff.combat_power_diff = rough_cp;
        }

        EquipmentDiff {
            slot: slot_name,
            previous_item,
            new_item: new_item.clone(),
            starforce_diff,
            stat_diff,
            set_effect_diff,
        }
    }
}

// ─── Gear Swap Simulation & Stat Diff Engine (장비 교체 및 스펙 비교 엔진) ─────────

/// Result of simulating an equipment item swap.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EquipmentDiff {
    /// Slot where the swap occurs (e.g. "무기", "모자", "장갑", "반지4")
    pub slot: String,
    /// Previously equipped item detail, if any
    pub previous_item: Option<ItemEquipmentDetail>,
    /// Newly equipped item detail
    pub new_item: ItemEquipmentDetail,
    /// Starforce difference (new starforce - old starforce)
    pub starforce_diff: i64,
    /// Detailed stat and combat power differences
    pub stat_diff: StatDiff,
    /// Set effect modifications (if set effect data was provided)
    pub set_effect_diff: Option<SetEffectDiff>,
}

impl EquipmentDiff {
    /// Whether the simulated swap is an upgrade (combat power increases).
    pub fn is_upgrade(&self) -> bool {
        self.stat_diff.combat_power_diff > 0
    }

    /// Estimated combat power difference.
    pub fn combat_power_diff(&self) -> i64 {
        self.stat_diff.combat_power_diff
    }

    /// Main stat difference.
    pub fn main_stat_diff(&self) -> i64 {
        self.stat_diff.main_stat()
    }

    /// Attack power difference.
    pub fn attack_power_diff(&self) -> i64 {
        self.stat_diff.attack_power
    }

    /// Magic power difference.
    pub fn magic_power_diff(&self) -> i64 {
        self.stat_diff.magic_power
    }

    /// Boss damage difference percentage.
    pub fn boss_damage_diff(&self) -> f64 {
        self.stat_diff.boss_damage
    }

    /// Ignore monster armor (방어율 무시) difference percentage.
    pub fn ignore_monster_armor_diff(&self) -> f64 {
        self.stat_diff.ignore_monster_armor
    }
}

/// Stat differences resulting from an equipment swap.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StatDiff {
    /// STR difference (힘)
    pub str: i64,
    /// DEX difference (민첩)
    pub dex: i64,
    /// INT difference (지력)
    pub int: i64,
    /// LUK difference (행운)
    pub luk: i64,
    /// Max HP difference (최대 HP)
    pub max_hp: i64,
    /// Max MP difference (최대 MP)
    pub max_mp: i64,
    /// Attack Power difference (공격력)
    pub attack_power: i64,
    /// Magic Power difference (마력)
    pub magic_power: i64,
    /// Armor difference (방어력)
    pub armor: i64,
    /// Boss damage difference percentage (보스 몬스터 데미지 %)
    pub boss_damage: f64,
    /// Ignore monster armor difference percentage (방어율 무시 %)
    pub ignore_monster_armor: f64,
    /// Damage difference percentage (데미지 %)
    pub damage: f64,
    /// All stat difference percentage (올스탯 %)
    pub all_stat: f64,
    /// Critical rate difference percentage (크리티컬 확률 %)
    pub critical_rate: f64,
    /// Critical damage difference percentage (크리티컬 데미지 %)
    pub critical_damage: f64,
    /// Starforce difference (스타포스)
    pub starforce: i64,
    /// Estimated combat power difference (예상 전투력 증감치)
    pub combat_power_diff: i64,
    /// Estimated total combat power after swap (교체 후 예상 최종 전투력)
    pub estimated_combat_power: i64,
}

impl StatDiff {
    /// Returns the main stat difference (max of STR, DEX, INT, LUK).
    pub fn main_stat(&self) -> i64 {
        self.str.max(self.dex).max(self.int).max(self.luk)
    }

    /// Returns the main stat difference for a specific character class.
    pub fn main_stat_for_class(&self, character_class: Option<&str>) -> i64 {
        match character_class {
            Some("데몬어벤져") => self.max_hp,
            Some("제논") => self.str.max(self.dex).max(self.luk),
            Some(job) if job.contains("마법사") || job.contains("메이지") || job.contains("비숍") || job.contains("플레임위자드") || job.contains("에반") || job.contains("배틀메이지") || job.contains("루미너스") || job.contains("일리움") || job.contains("라라") || job.contains("키네시스") => self.int,
            Some(job) if job.contains("궁수") || job.contains("보우마스터") || job.contains("신궁") || job.contains("패스파인더") || job.contains("윈드브레이커") || job.contains("와일드헌터") || job.contains("메르세데스") || job.contains("카인") => self.dex,
            Some(job) if job.contains("도적") || job.contains("나이트로드") || job.contains("섀도어") || job.contains("듀얼블레이더") || job.contains("나이트워커") || job.contains("팬텀") || job.contains("카데나") || job.contains("호영") || job.contains("칼리") => self.luk,
            Some(job) if job.contains("전사") || job.contains("히어로") || job.contains("팔라딘") || job.contains("다크나이트") || job.contains("소울마스터") || job.contains("미하일") || job.contains("블래스터") || job.contains("데몬슬레이어") || job.contains("아란") || job.contains("카이저") || job.contains("아델") || job.contains("제로") => self.str,
            Some(job) if job.contains("캡틴") || job.contains("메카닉") => self.dex,
            Some(job) if job.contains("해적") || job.contains("바이퍼") || job.contains("캐논슈터") || job.contains("스트라이커") || job.contains("은월") || job.contains("아크") => self.str,
            _ => self.main_stat(),
        }
    }
}

/// Set effect changes resulting from an equipment swap.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SetEffectDiff {
    /// Changes in piece counts per set
    pub set_count_changes: Vec<SetCountChange>,
    /// Set effect options gained by reaching higher thresholds
    pub gained_options: Vec<String>,
    /// Set effect options lost by dropping below thresholds
    pub lost_options: Vec<String>,
}

/// Piece count change for an item set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetCountChange {
    /// Set effect name
    pub set_name: String,
    /// Piece count before swap
    pub previous_count: i64,
    /// Piece count after swap
    pub new_count: i64,
}

#[derive(Debug, Default, Clone)]
struct ItemStatBag {
    str: i64,
    dex: i64,
    int: i64,
    luk: i64,
    max_hp: i64,
    max_mp: i64,
    attack_power: i64,
    magic_power: i64,
    armor: i64,
    boss_damage: f64,
    ignore_monster_armor: f64,
    damage: f64,
    all_stat: f64,
    critical_damage: f64,
    critical_rate: f64,
    starforce: i64,
}

fn parse_stat_i64(s: Option<&str>) -> i64 {
    s.and_then(|v| {
        let v = v.trim().trim_start_matches('+');
        v.parse::<i64>().ok()
    })
    .unwrap_or(0)
}

fn parse_stat_f64(s: Option<&str>) -> f64 {
    s.and_then(|v| {
        let v = v.trim().trim_start_matches('+').trim_end_matches('%');
        v.parse::<f64>().ok()
    })
    .unwrap_or(0.0)
}

fn fill_item_stat_bag(item: &ItemEquipmentDetail, bag: &mut ItemStatBag) {
    bag.starforce = item.starforce.as_deref().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);

    if let Some(ref tot) = item.item_total_option {
        bag.str = parse_stat_i64(tot.str.as_deref());
        bag.dex = parse_stat_i64(tot.dex.as_deref());
        bag.int = parse_stat_i64(tot.int.as_deref());
        bag.luk = parse_stat_i64(tot.luk.as_deref());
        bag.max_hp = parse_stat_i64(tot.max_hp.as_deref());
        bag.max_mp = parse_stat_i64(tot.max_mp.as_deref());
        bag.attack_power = parse_stat_i64(tot.attack_power.as_deref());
        bag.magic_power = parse_stat_i64(tot.magic_power.as_deref());
        bag.armor = parse_stat_i64(tot.armor.as_deref());
        bag.boss_damage = parse_stat_f64(tot.boss_damage.as_deref());
        bag.ignore_monster_armor = parse_stat_f64(tot.ignore_monster_armor.as_deref());
        bag.damage = parse_stat_f64(tot.damage.as_deref());
        bag.all_stat = parse_stat_f64(tot.all_stat.as_deref());
    } else {
        let options = [
            item.item_base_option.as_ref().map(|o| (o.str.as_deref(), o.dex.as_deref(), o.int.as_deref(), o.luk.as_deref(), o.max_hp.as_deref(), o.max_mp.as_deref(), o.attack_power.as_deref(), o.magic_power.as_deref(), o.armor.as_deref(), o.boss_damage.as_deref(), o.ignore_monster_armor.as_deref(), None, o.all_stat.as_deref())),
            item.item_add_option.as_ref().map(|o| (o.str.as_deref(), o.dex.as_deref(), o.int.as_deref(), o.luk.as_deref(), o.max_hp.as_deref(), o.max_mp.as_deref(), o.attack_power.as_deref(), o.magic_power.as_deref(), o.armor.as_deref(), o.boss_damage.as_deref(), None, o.damage.as_deref(), o.all_stat.as_deref())),
            item.item_etc_option.as_ref().map(|o| (o.str.as_deref(), o.dex.as_deref(), o.int.as_deref(), o.luk.as_deref(), o.max_hp.as_deref(), o.max_mp.as_deref(), o.attack_power.as_deref(), o.magic_power.as_deref(), o.armor.as_deref(), None, None, None, None)),
            item.item_starforce_option.as_ref().map(|o| (o.str.as_deref(), o.dex.as_deref(), o.int.as_deref(), o.luk.as_deref(), o.max_hp.as_deref(), o.max_mp.as_deref(), o.attack_power.as_deref(), o.magic_power.as_deref(), o.armor.as_deref(), None, None, None, None)),
            item.item_exceptional_option.as_ref().map(|o| (o.str.as_deref(), o.dex.as_deref(), o.int.as_deref(), o.luk.as_deref(), o.max_hp.as_deref(), o.max_mp.as_deref(), o.attack_power.as_deref(), o.magic_power.as_deref(), None, None, None, None, None)),
        ];
        for opt in options.into_iter().flatten() {
            bag.str += parse_stat_i64(opt.0);
            bag.dex += parse_stat_i64(opt.1);
            bag.int += parse_stat_i64(opt.2);
            bag.luk += parse_stat_i64(opt.3);
            bag.max_hp += parse_stat_i64(opt.4);
            bag.max_mp += parse_stat_i64(opt.5);
            bag.attack_power += parse_stat_i64(opt.6);
            bag.magic_power += parse_stat_i64(opt.7);
            bag.armor += parse_stat_i64(opt.8);
            bag.boss_damage += parse_stat_f64(opt.9);
            bag.ignore_monster_armor += parse_stat_f64(opt.10);
            bag.damage += parse_stat_f64(opt.11);
            bag.all_stat += parse_stat_f64(opt.12);
        }
    }

    for pot in item.all_potentials_parsed() {
        match pot.stat_type {
            StatType::Str => {
                if !pot.is_percentage {
                    bag.str += pot.value as i64;
                }
            }
            StatType::Dex => {
                if !pot.is_percentage {
                    bag.dex += pot.value as i64;
                }
            }
            StatType::Int => {
                if !pot.is_percentage {
                    bag.int += pot.value as i64;
                }
            }
            StatType::Luk => {
                if !pot.is_percentage {
                    bag.luk += pot.value as i64;
                }
            }
            StatType::AttackPower => {
                if !pot.is_percentage {
                    bag.attack_power += pot.value as i64;
                }
            }
            StatType::MagicPower => {
                if !pot.is_percentage {
                    bag.magic_power += pot.value as i64;
                }
            }
            StatType::BossDamage => {
                bag.boss_damage += pot.value;
            }
            StatType::IgnoreMonsterArmor => {
                bag.ignore_monster_armor += pot.value;
            }
            StatType::Damage => {
                bag.damage += pot.value;
            }
            StatType::CriticalDamage => {
                bag.critical_damage += pot.value;
            }
            StatType::CriticalRate => {
                bag.critical_rate += pot.value;
            }
            StatType::AllStat => {
                bag.all_stat += pot.value;
            }
            _ => {}
        }
    }
}

fn item_matches_set(item: &ItemEquipmentDetail, set_info: &SetEffectInfo) -> bool {
    let item_name = match item.item_name.as_deref() {
        Some(name) => name,
        None => return false,
    };
    let set_name = match set_info.set_name.as_deref() {
        Some(name) => name,
        None => return false,
    };

    if item_name.contains(set_name) || set_name.contains(item_name) {
        return true;
    }

    let base_set = set_name
        .split('(')
        .next()
        .unwrap_or(set_name)
        .replace(" 세트", "")
        .replace("세트", "")
        .trim()
        .to_string();

    if !base_set.is_empty() && item_name.contains(&base_set) {
        return true;
    }

    (base_set.contains("루타비스")
        && (item_name.contains("하이네스")
            || item_name.contains("이글아이")
            || item_name.contains("트릭스터")
            || item_name.contains("파프니르")
            || item_name.contains("루타비스")))
        || (base_set.contains("앱솔랩스") && item_name.contains("앱솔랩스"))
        || (base_set.contains("아케인") && item_name.contains("아케인"))
        || (base_set.contains("에테르넬") && item_name.contains("에테르넬"))
        || (base_set.contains("칠흑")
            && (item_name.contains("칠흑")
                || item_name.contains("루즈 컨트롤")
                || item_name.contains("마력이 깃든")
                || item_name.contains("마깃안")
                || item_name.contains("커맨더 포스")
                || item_name.contains("몽환의 벨트")
                || item_name.contains("창세의 뱃지")
                || item_name.contains("거대한 공포")
                || item_name.contains("고통의 근원")
                || item_name.contains("미트라의 분노")
                || item_name.contains("블랙 하트")))
        || (base_set.contains("여명")
            && (item_name.contains("여명")
                || item_name.contains("트와일라이트")
                || item_name.contains("에스텔라")
                || item_name.contains("데이브레이크")))
        || (base_set.contains("보스 장신구")
            && (item_name.contains("응축된")
                || item_name.contains("아쿠아틱")
                || item_name.contains("블랙빈")
                || item_name.contains("골든 클로버")
                || item_name.contains("핑크빛")
                || item_name.contains("데아 시두스")
                || item_name.contains("매커네이터")
                || item_name.contains("도미네이터")
                || item_name.contains("가디언 엔젤")))
}

fn parse_set_option_chunk(chunk: &str, bag: &mut ItemStatBag, multiplier: f64) {
    let trimmed = chunk.trim();
    if trimmed.is_empty() {
        return;
    }

    let mut num_start = None;
    for (i, c) in trimmed.char_indices() {
        if c.is_ascii_digit()
            || ((c == '+' || c == '-')
                && trimmed[i..]
                    .chars()
                    .nth(1)
                    .is_some_and(|next| next.is_ascii_digit()))
        {
            num_start = Some(i);
            break;
        }
    }

    let val = if let Some(idx) = num_start {
        let part = &trimmed[idx..];
        let end_idx = part
            .find(|c: char| !c.is_ascii_digit() && c != '.' && c != '+' && c != '-')
            .unwrap_or(part.len());
        part[..end_idx].parse::<f64>().unwrap_or(0.0)
    } else {
        0.0
    } * multiplier;

    let text = if let Some(idx) = num_start {
        &trimmed[..idx]
    } else {
        trimmed
    };

    if (text.contains("보스") && text.contains("데미지")) || text.contains("보스 몬스터 공격 시 데미지") {
        bag.boss_damage += val;
    } else if text.contains("방어율") && text.contains("무시") {
        bag.ignore_monster_armor += val;
    } else if text.contains("크리티컬") && text.contains("데미지") {
        bag.critical_damage += val;
    } else if text.contains("크리티컬") && text.contains("확률") {
        bag.critical_rate += val;
    } else if text.contains("올스탯") {
        bag.all_stat += val;
    } else if text.contains("공격력") && text.contains("마력") {
        bag.attack_power += val as i64;
        bag.magic_power += val as i64;
    } else if text.contains("공격력") {
        bag.attack_power += val as i64;
    } else if text.contains("마력") {
        bag.magic_power += val as i64;
    } else if text.contains("HP") && text.contains("MP") {
        bag.max_hp += val as i64;
        bag.max_mp += val as i64;
    } else if text.contains("HP") {
        bag.max_hp += val as i64;
    } else if text.contains("MP") {
        bag.max_mp += val as i64;
    } else if text.contains("STR") || text.contains("힘") {
        bag.str += val as i64;
    } else if text.contains("DEX") || text.contains("민첩") {
        bag.dex += val as i64;
    } else if text.contains("INT") || text.contains("지력") {
        bag.int += val as i64;
    } else if text.contains("LUK") || text.contains("행운") {
        bag.luk += val as i64;
    } else if text.contains("데미지") {
        bag.damage += val;
    }
}

fn parse_set_option_string(s: &str, bag: &mut ItemStatBag, multiplier: f64) {
    for chunk in s.split(',') {
        parse_set_option_chunk(chunk, bag, multiplier);
    }
}


// ─── Character Progression Helpers (HEXA, V-Matrix, Ability, Hyper Stat) ─────

impl CharacterHexaMatrix {
    /// Returns all HEXA cores whose level is greater than 0.
    pub fn active_cores(&self) -> Vec<&HexaCoreEquipment> {
        self.character_hexa_core_equipment
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter(|c| c.hexa_core_level.unwrap_or(0) > 0)
            .collect()
    }

    /// Look up the level of a specific HEXA core by name (e.g. `"마스터리 코어"`, `"오리진 스킬"`).
    pub fn core_level(&self, name: &str) -> Option<i64> {
        self.character_hexa_core_equipment
            .as_deref()?
            .iter()
            .find(|c| c.hexa_core_name.as_deref() == Some(name))
            .and_then(|c| c.hexa_core_level)
    }

    /// Filter HEXA cores by their core type (e.g. `"스킬 코어"`, `"마스터리 코어"`, `"강화 코어"`, `"공용 코어"`).
    pub fn cores_by_type(&self, core_type: &str) -> Vec<&HexaCoreEquipment> {
        self.character_hexa_core_equipment
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter(|c| c.hexa_core_type.as_deref() == Some(core_type))
            .collect()
    }

    /// Calculate remaining Sol Erda and Fragments required to upgrade all active cores to `target_level` (1..=30).
    pub fn remaining_materials(&self, target_level: i64) -> HexaProgressionSummary {
        let target = target_level.clamp(1, 30);
        let mut total = HexaMaterials::default();
        let mut cores = Vec::new();

        if let Some(equip_cores) = &self.character_hexa_core_equipment {
            for core in equip_cores {
                let name = core.hexa_core_name.clone().unwrap_or_default();
                let core_type = core.hexa_core_type.clone().unwrap_or_default();
                let current = core.hexa_core_level.unwrap_or(0).clamp(0, 30);
                if current >= target {
                    continue;
                }
                let mut remaining = HexaMaterials::default();
                for lvl in (current as usize)..(target as usize) {
                    let (erda, frag) = hexa_step_cost(&core_type, lvl);
                    remaining.sol_erda += erda;
                    remaining.sol_erda_fragments += frag;
                }
                total += remaining;
                cores.push(HexaCoreProgression {
                    name,
                    core_type,
                    current_level: current,
                    target_level: target,
                    remaining,
                });
            }
        }

        HexaProgressionSummary {
            target_level: target,
            total_remaining: total,
            cores,
        }
    }

    /// Calculate remaining Sol Erda and Fragments required to reach maximum level (30).
    pub fn remaining_materials_to_max(&self) -> HexaProgressionSummary {
        self.remaining_materials(30)
    }
}

impl CharacterHexaMatrixStat {
    fn slot_core(&self, slot_index: usize) -> Option<&HexaStatCore> {
        let list = match slot_index {
            1 => self.character_hexa_stat_core.as_deref(),
            2 => self.character_hexa_stat_core_2.as_deref(),
            3 => self.character_hexa_stat_core_3.as_deref(),
            _ => None,
        };
        list?.first()
    }

    /// Collects all active HEXA Stat cores across slots I, II, and III.
    pub fn active_stat_cores(&self) -> Vec<&HexaStatCore> {
        [
            self.character_hexa_stat_core.as_deref(),
            self.character_hexa_stat_core_2.as_deref(),
            self.character_hexa_stat_core_3.as_deref(),
        ]
        .into_iter()
        .flatten()
        .flatten()
        .collect()
    }

    /// Returns the main stat level for the given 1-based HEXA Stat core slot (`1..=3`).
    pub fn main_stat_level(&self, slot_index: usize) -> Option<i64> {
        self.slot_core(slot_index)?.main_stat_name.as_ref()?;
        self.slot_core(slot_index)?.main_stat_level
    }

    /// Returns the two additional/sub stat levels `(sub_stat_level_1, sub_stat_level_2)` for the given 1-based slot (`1..=3`).
    pub fn additional_stat_levels(&self, slot_index: usize) -> Option<(i64, i64)> {
        let core = self.slot_core(slot_index)?;
        Some((core.sub_stat_level_1?, core.sub_stat_level_2?))
    }

    /// Calculate remaining Sol Erda Fragments required to upgrade all active HEXA Stat cores to `target_grade` (1..=20).
    pub fn remaining_materials(&self, target_grade: i64) -> HexaStatProgressionSummary {
        let target = target_grade.clamp(1, 20);
        let mut total_attempts = 0i64;
        let mut total_frags = 0i64;
        let mut total_min = 0i64;
        let mut total_max = 0i64;
        let mut total_expected = 0.0f64;
        let mut stat_cores_remaining = Vec::new();

        for slot_idx in 1..=3 {
            if let Some(core) = self.slot_core(slot_idx) {
                let main_lvl = core.main_stat_level.unwrap_or(0);
                let sub1_lvl = core.sub_stat_level_1.unwrap_or(0);
                let sub2_lvl = core.sub_stat_level_2.unwrap_or(0);
                let current_grade = core
                    .stat_grade
                    .unwrap_or_else(|| main_lvl + sub1_lvl + sub2_lvl)
                    .clamp(0, 20);

                if current_grade < target {
                    let attempts = target - current_grade;
                    let (min_f, max_f, exp_f) = calculate_hexa_stat_cost_range(main_lvl, attempts);
                    let frags = exp_f.round() as i64;

                    total_attempts += attempts;
                    total_min += min_f;
                    total_max += max_f;
                    total_expected += exp_f;
                    total_frags += frags;

                    stat_cores_remaining.push(HexaStatCoreProgression {
                        slot_index: slot_idx,
                        main_stat_name: core.main_stat_name.clone(),
                        current_grade,
                        target_grade: target,
                        remaining_attempts: attempts,
                        current_main_stat_level: main_lvl,
                        current_sub_stat_level_1: sub1_lvl,
                        current_sub_stat_level_2: sub2_lvl,
                        min_fragments_remaining: min_f,
                        max_fragments_remaining: max_f,
                        expected_fragments_remaining: exp_f,
                        fragments_remaining: frags,
                    });
                }
            }
        }

        HexaStatProgressionSummary {
            target_grade: target,
            total_remaining_attempts: total_attempts,
            total_fragments_remaining: total_frags,
            min_fragments_remaining: total_min,
            max_fragments_remaining: total_max,
            expected_fragments_remaining: total_expected,
            stat_cores_remaining,
        }
    }

    /// Calculate remaining Sol Erda Fragments required to reach maximum grade (20).
    pub fn remaining_materials_to_max(&self) -> HexaStatProgressionSummary {
        self.remaining_materials(20)
    }
}

/// Material requirements for HEXA progression (솔 에르다 및 솔 에르다 조각).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HexaMaterials {
    /// Sol Erda count (솔 에르다)
    pub sol_erda: i64,
    /// Sol Erda Fragment count (솔 에르다 조각)
    pub sol_erda_fragments: i64,
}

impl HexaMaterials {
    /// Create a new material pair.
    pub const fn new(sol_erda: i64, sol_erda_fragments: i64) -> Self {
        Self {
            sol_erda,
            sol_erda_fragments,
        }
    }
}

impl std::ops::Add for HexaMaterials {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            sol_erda: self.sol_erda + rhs.sol_erda,
            sol_erda_fragments: self.sol_erda_fragments + rhs.sol_erda_fragments,
        }
    }
}

impl std::ops::AddAssign for HexaMaterials {
    fn add_assign(&mut self, rhs: Self) {
        self.sol_erda += rhs.sol_erda;
        self.sol_erda_fragments += rhs.sol_erda_fragments;
    }
}

/// Detailed material progression breakdown for an individual HEXA core.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HexaCoreProgression {
    /// Core name
    pub name: String,
    /// Core type (e.g. "스킬 코어", "마스터리 코어")
    pub core_type: String,
    /// Current core level
    pub current_level: i64,
    /// Target level
    pub target_level: i64,
    /// Remaining materials required
    pub remaining: HexaMaterials,
}

/// Comprehensive progression summary for all equipped HEXA cores in a matrix.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HexaProgressionSummary {
    /// Target level evaluated
    pub target_level: i64,
    /// Total remaining Sol Erda and Fragments
    pub total_remaining: HexaMaterials,
    /// Breakdown per core needing upgrade
    pub cores: Vec<HexaCoreProgression>,
}

impl HexaProgressionSummary {
    /// Total Sol Erda required across all cores.
    pub fn total_sol_erda(&self) -> i64 {
        self.total_remaining.sol_erda
    }

    /// Total Sol Erda Fragments required across all cores.
    pub fn total_sol_erda_fragments(&self) -> i64 {
        self.total_remaining.sol_erda_fragments
    }

    /// Material cost breakdown filtered by core type (e.g. `"스킬 코어"`, `"마스터리 코어"`).
    pub fn cost_by_type(&self, core_type: &str) -> HexaMaterials {
        let mut total = HexaMaterials::default();
        for c in &self.cores {
            if c.core_type == core_type {
                total += c.remaining;
            }
        }
        total
    }

    /// Returns `true` if all equipped cores have reached or exceeded the target level.
    pub fn is_maxed(&self) -> bool {
        self.cores.is_empty()
    }
}

/// Material progression summary for HEXA Stat cores.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HexaStatProgressionSummary {
    /// Target stat grade
    pub target_grade: i64,
    /// Total enhancement attempts remaining across all active stat cores
    pub total_remaining_attempts: i64,
    /// Total Sol Erda Fragments required (expected rounded to nearest integer)
    pub total_fragments_remaining: i64,
    /// Total minimum Sol Erda Fragments required
    pub min_fragments_remaining: i64,
    /// Total maximum Sol Erda Fragments required
    pub max_fragments_remaining: i64,
    /// Total expected Sol Erda Fragments required
    pub expected_fragments_remaining: f64,
    /// Breakdown per active stat core
    pub stat_cores_remaining: Vec<HexaStatCoreProgression>,
}

impl HexaStatProgressionSummary {
    /// Total Sol Erda fragments required.
    pub fn total_fragments(&self) -> i64 {
        self.total_fragments_remaining
    }

    /// Total minimum Sol Erda fragments required.
    pub fn min_fragments(&self) -> i64 {
        self.min_fragments_remaining
    }

    /// Total maximum Sol Erda fragments required.
    pub fn max_fragments(&self) -> i64 {
        self.max_fragments_remaining
    }

    /// Total expected Sol Erda fragments required.
    pub fn expected_fragments(&self) -> f64 {
        self.expected_fragments_remaining
    }

    /// Returns `true` if all active stat cores have reached or exceeded the target grade.
    pub fn is_maxed(&self) -> bool {
        self.stat_cores_remaining.is_empty()
    }
}

/// Detailed material progression for an individual HEXA Stat slot core.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HexaStatCoreProgression {
    /// Slot index (1..=3)
    pub slot_index: usize,
    /// Main stat name
    pub main_stat_name: Option<String>,
    /// Current stat grade (sum of main and sub stat levels, 0..=20)
    pub current_grade: i64,
    /// Target stat grade (clamped 1..=20)
    pub target_grade: i64,
    /// Enhancement attempts remaining
    pub remaining_attempts: i64,
    /// Current main stat level
    pub current_main_stat_level: i64,
    /// Current sub stat 1 level
    pub current_sub_stat_level_1: i64,
    /// Current sub stat 2 level
    pub current_sub_stat_level_2: i64,
    /// Minimum fragments required (all remaining attempts hit sub stats)
    pub min_fragments_remaining: i64,
    /// Maximum fragments required (remaining attempts hit main stat first)
    pub max_fragments_remaining: i64,
    /// Expected fragments required
    pub expected_fragments_remaining: f64,
    /// Fragments remaining (expected rounded to nearest integer)
    pub fragments_remaining: i64,
}

// ─── HEXA Matrix Level Tables ────────────────────────────────────────────────

/// Sol Erda Fragment attempt cost for HEXA Stat based on current main stat level.
pub fn hexa_stat_attempt_cost(main_stat_level: i64) -> i64 {
    match main_stat_level {
        0..=2 => 10,
        3..=6 => 20,
        7..=8 => 30,
        _ => 50,
    }
}

/// Success probability of enhancing main stat given current main stat level.
pub fn hexa_stat_success_rate(main_stat_level: i64) -> f64 {
    match main_stat_level {
        0..=2 => 0.35,
        3..=6 => 0.20,
        7 => 0.15,
        8 => 0.10,
        9 => 0.05,
        _ => 0.0,
    }
}

/// Calculate min, max, and expected fragment cost for `attempts` remaining enhancements
/// starting from `current_main_stat_level`.
pub fn calculate_hexa_stat_cost_range(
    current_main_stat_level: i64,
    attempts: i64,
) -> (i64, i64, f64) {
    if attempts <= 0 {
        return (0, 0, 0.0);
    }

    let m_clamped = current_main_stat_level.clamp(0, 10);

    // 1. Minimum fragments: all attempts fail to upgrade main stat, remaining at current level
    let min_cost = attempts * hexa_stat_attempt_cost(m_clamped);

    // 2. Maximum fragments: all attempts upgrade main stat up to lv 10
    let mut max_cost = 0i64;
    let mut cur_m = m_clamped;
    for _ in 0..attempts {
        max_cost += hexa_stat_attempt_cost(cur_m);
        if cur_m < 10 {
            cur_m += 1;
        }
    }

    // 3. Expected fragments using DP distribution of main stat level
    let mut prob = [0.0f64; 11];
    prob[m_clamped as usize] = 1.0;
    let mut expected_cost = 0.0f64;

    for _ in 0..attempts {
        // Add expected cost for this attempt
        for (m, &p) in prob.iter().enumerate() {
            if p > 0.0 {
                expected_cost += p * (hexa_stat_attempt_cost(m as i64) as f64);
            }
        }

        // Transition to next attempt
        let mut next_prob = [0.0f64; 11];
        for (m, &p) in prob.iter().enumerate() {
            if p > 0.0 {
                let p_succ = hexa_stat_success_rate(m as i64);
                let p_stay = 1.0 - p_succ;
                let next_m = (m + 1).min(10);
                next_prob[next_m] += p * p_succ;
                next_prob[m] += p * p_stay;
            }
        }
        prob = next_prob;
    }

    (min_cost, max_cost, expected_cost)
}

fn hexa_step_cost(core_type: &str, from_level: usize) -> (i64, i64) {
    if from_level >= 30 {
        return (0, 0);
    }

    match core_type {
        "스킬 코어" | "오리진" => {
            const ERDA: [i64; 30] = [
                5, 1, 1, 1, 2, 2, 2, 3, 3, 10,
                3, 3, 4, 4, 4, 4, 4, 4, 5, 15,
                5, 5, 5, 5, 5, 6, 6, 6, 7, 20,
            ];
            const FRAG: [i64; 30] = [
                100, 30, 35, 40, 45, 50, 55, 60, 65, 200,
                80, 90, 100, 110, 120, 130, 140, 150, 160, 350,
                170, 180, 190, 200, 210, 220, 230, 240, 250, 500,
            ];
            (ERDA[from_level], FRAG[from_level])
        }
        "마스터리 코어" => {
            const ERDA: [i64; 30] = [
                3, 1, 1, 1, 1, 1, 1, 2, 2, 5,
                2, 2, 2, 2, 2, 2, 2, 2, 3, 8,
                3, 3, 3, 3, 3, 3, 3, 3, 4, 10,
            ];
            const FRAG: [i64; 30] = [
                50, 15, 18, 20, 23, 25, 28, 30, 33, 100,
                40, 45, 50, 55, 60, 65, 70, 75, 80, 175,
                85, 90, 95, 100, 105, 110, 115, 120, 125, 250,
            ];
            (ERDA[from_level], FRAG[from_level])
        }
        "강화 코어" => {
            const ERDA: [i64; 30] = [
                4, 1, 1, 1, 2, 2, 2, 3, 3, 8,
                3, 3, 3, 3, 3, 3, 3, 3, 4, 12,
                4, 4, 4, 4, 4, 5, 5, 5, 6, 15,
            ];
            const FRAG: [i64; 30] = [
                75, 23, 27, 30, 34, 38, 42, 45, 49, 150,
                60, 68, 75, 83, 90, 98, 105, 113, 120, 263,
                128, 135, 143, 150, 158, 165, 173, 180, 188, 375,
            ];
            (ERDA[from_level], FRAG[from_level])
        }
        "공용 코어" => {
            const ERDA: [i64; 30] = [
                7, 2, 2, 2, 3, 3, 3, 5, 5, 14,
                5, 5, 6, 6, 6, 6, 6, 6, 7, 17,
                7, 7, 7, 7, 7, 9, 9, 9, 10, 20,
            ];
            const FRAG: [i64; 30] = [
                125, 38, 44, 50, 57, 63, 69, 75, 82, 300,
                110, 124, 138, 152, 165, 179, 193, 207, 220, 525,
                234, 248, 262, 275, 289, 303, 317, 330, 344, 750,
            ];
            (ERDA[from_level], FRAG[from_level])
        }
        _ => (0, 0),
    }
}

impl CharacterVMatrix {
    /// Calculates the total V-Matrix slot enhancement level across all equipped V-Cores.
    #[allow(deprecated)]
    pub fn total_slot_enhancement_level(&self) -> i64 {
        self.character_v_core_equipment
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter_map(|c| c.slot_level)
            .sum()
    }

    /// Finds an equipped V-Core by its exact name.
    pub fn core_by_name(&self, name: &str) -> Option<&VCoreEquipment> {
        self.character_v_core_equipment
            .as_deref()?
            .iter()
            .find(|c| c.v_core_name.as_deref() == Some(name))
    }

    /// Returns all equipped V-Cores that have reached maximum base level (`>= 25`).
    pub fn max_level_cores(&self) -> Vec<&VCoreEquipment> {
        self.character_v_core_equipment
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter(|c| c.v_core_level.unwrap_or(0) >= 25)
            .collect()
    }
}

impl CharacterAbility {
    /// Returns the ability lines for the specified preset (`1..=3`), or the currently active
    /// `ability_info` when `preset_no` is `None`.
    pub fn active_preset_abilities(&self, preset_no: Option<u8>) -> &[AbilityInfo] {
        let preset = match preset_no {
            Some(1) => self.ability_preset_1.as_ref(),
            Some(2) => self.ability_preset_2.as_ref(),
            Some(3) => self.ability_preset_3.as_ref(),
            _ => None,
        };
        if let Some(p) = preset {
            if let Some(lines) = p.ability_info.as_deref() {
                return lines;
            }
        }
        self.ability_info.as_deref().unwrap_or(&[])
    }
}

impl CharacterHyperStat {
    /// Returns the Hyper Stat entries for the specified preset (`1..=3`).
    pub fn preset_hyper_stats(&self, preset_no: u8) -> &[HyperStatEntry] {
        match preset_no {
            1 => self.hyper_stat_preset_1.as_deref().unwrap_or(&[]),
            2 => self.hyper_stat_preset_2.as_deref().unwrap_or(&[]),
            3 => self.hyper_stat_preset_3.as_deref().unwrap_or(&[]),
            _ => &[],
        }
    }

    /// Returns the Hyper Stat entries for the specified preset (`1..=3`), or the currently active
    /// preset (`use_preset_no`) when `preset_no` is `None`.
    ///
    /// If an explicit `preset_no` is provided that is not `1..=3`, an empty slice is returned.
    pub fn active_preset_hyper_stats(&self, preset_no: Option<u8>) -> &[HyperStatEntry] {
        let n = preset_no.unwrap_or_else(|| {
            self.use_preset_no
                .as_deref()
                .and_then(|s| s.parse::<u8>().ok())
                .unwrap_or(1)
        });
        self.preset_hyper_stats(n)
    }

    /// Finds a specific Hyper Stat entry by its `stat_type` (e.g. `"보스 몬스터 공격 시 데미지 증가"`, `"크리티컬 데미지"`)
    /// in the currently active preset.
    pub fn find_hyper_stat(&self, stat_type: &str) -> Option<&HyperStatEntry> {
        self.active_preset_hyper_stats(None)
            .iter()
            .find(|entry| entry.stat_type.as_deref() == Some(stat_type))
    }
}
