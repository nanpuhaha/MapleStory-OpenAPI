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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
}

impl CharacterCombatAnalytics {
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

    /// Additional main stat bonus contributed by Arcane Force (`(arcane_force / 10) * 100`).
    /// (For Demon Avenger: HP +2,100 per 10 AF; Xenon: +48 per 10 AF; standard jobs: +100 main stat per 10 AF).
    pub fn arcane_force_stat_bonus(&self) -> i64 {
        (self.arcane_force / 10) * 100
    }

    /// Additional main stat bonus contributed by Authentic Force (`(authentic_force / 10) * 200`).
    /// (Standard jobs: +200 main stat per 10 AutF).
    pub fn authentic_force_stat_bonus(&self) -> i64 {
        (self.authentic_force / 10) * 200
    }

    /// Estimated Mu Lung Dojo floor based on combat power benchmarks.
    pub fn estimated_dojang_floor(&self) -> i64 {
        const DOJANG_BENCHMARKS: &[(i64, i64)] = &[
            (3_000_000, 40),
            (6_000_000, 45),
            (15_000_000, 50),
            (35_000_000, 55),
            (70_000_000, 60),
            (150_000_000, 65),
            (300_000_000, 70),
            (600_000_000, 75),
            (1_000_000_000, 80),
        ];

        for &(threshold, floor) in DOJANG_BENCHMARKS {
            if self.combat_power < threshold {
                return floor;
            }
        }
        85
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
    /// Returns the Hyper Stat entries for the specified preset (`1..=3`), or the currently active
    /// preset (`use_preset_no`) when `preset_no` is `None`.
    ///
    /// If an explicit `preset_no` is provided that is not `1..=3`, an empty slice is returned.
    pub fn active_preset_hyper_stats(&self, preset_no: Option<u8>) -> &[HyperStatEntry] {
        if let Some(n) = preset_no {
            return match n {
                1 => self.hyper_stat_preset_1.as_deref().unwrap_or(&[]),
                2 => self.hyper_stat_preset_2.as_deref().unwrap_or(&[]),
                3 => self.hyper_stat_preset_3.as_deref().unwrap_or(&[]),
                _ => &[],
            };
        }

        let target = self
            .use_preset_no
            .as_deref()
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(1);
        match target {
            1 => self.hyper_stat_preset_1.as_deref().unwrap_or(&[]),
            2 => self.hyper_stat_preset_2.as_deref().unwrap_or(&[]),
            3 => self.hyper_stat_preset_3.as_deref().unwrap_or(&[]),
            _ => self.hyper_stat_preset_1.as_deref().unwrap_or(&[]),
        }
    }

    /// Finds a specific Hyper Stat entry by its `stat_type` (e.g. `"보스 몬스터 공격 시 데미지 증가"`, `"크리티컬 데미지"`)
    /// in the currently active preset.
    pub fn find_hyper_stat(&self, stat_type: &str) -> Option<&HyperStatEntry> {
        self.active_preset_hyper_stats(None)
            .iter()
            .find(|entry| entry.stat_type.as_deref() == Some(stat_type))
    }
}
