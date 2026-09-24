//! Unit tests for domain helpers, error helpers, and coordination synthesis.

use maplestory_openapi::models::{
    character::{
        CashItemEquipmentDetail, CharacterBeautyEquipment, CharacterCashItemEquipment,
        CharacterCoordination, CharacterItemEquipment, CharacterStat, FinalStatEntry,
        ItemEquipmentDetail,
    },
    common::{FaceInfo, HairInfo, SkinInfo},
};
use maplestory_openapi::MapleStoryError;

#[test]
fn test_stat_helpers() {
    let stat = CharacterStat {
        date: Some("2024-01-15T00:00+09:00".to_string()),
        character_class: Some("아델".to_string()),
        final_stat: Some(vec![
            FinalStatEntry {
                stat_name: Some("전투력".to_string()),
                stat_value: Some("123456789".to_string()),
            },
            FinalStatEntry {
                stat_name: Some("최소 스탯공격력".to_string()),
                stat_value: Some("50000000".to_string()),
            },
            FinalStatEntry {
                stat_name: Some("최대 스탯공격력".to_string()),
                stat_value: Some("60000000".to_string()),
            },
            FinalStatEntry {
                stat_name: Some("보스 몬스터 데미지".to_string()),
                stat_value: Some("385.5".to_string()),
            },
            FinalStatEntry {
                stat_name: Some("방어율 무시".to_string()),
                stat_value: Some("95.2".to_string()),
            },
        ]),
        remain_ap: Some(0),
    };

    assert_eq!(stat.combat_power(), Some(123456789));
    assert_eq!(stat.stat_attack(), (Some(50000000), Some(60000000)));
    assert_eq!(stat.boss_damage(), Some(385.5));
    assert_eq!(stat.ignore_monster_armor(), Some(95.2));
    assert_eq!(stat.get_stat("전투력"), Some("123456789"));
    assert_eq!(stat.get_stat("없는스탯"), None);
}

#[test]
fn test_equipment_helpers() {
    let equip = CharacterItemEquipment {
        date: None,
        character_gender: None,
        character_class: None,
        preset_no: Some(1),
        item_equipment: None,
        item_equipment_preset_1: Some(vec![
            ItemEquipmentDetail {
                item_equipment_part: Some("모자".to_string()),
                item_equipment_slot: Some("모자".to_string()),
                item_name: Some("하이네스 던위치햇".to_string()),
                item_icon: Some("https://example.com/hat.png".to_string()),
                item_description: None,
                item_shape_name: Some("모루외형햇".to_string()),
                item_shape_icon: Some("https://example.com/shape_hat.png".to_string()),
                item_gender: None,
                item_total_option: None,
                item_base_option: None,
                potential_option_grade: None,
                additional_potential_option_grade: None,
                potential_option_flag: None,
                additional_potential_option_flag: None,
                potential_option_1: None,
                potential_option_2: None,
                potential_option_3: None,
                additional_potential_option_1: None,
                additional_potential_option_2: None,
                additional_potential_option_3: None,
                equipment_level_increase: None,
                item_exceptional_option: None,
                item_add_option: None,
                growth_exp: None,
                growth_level: None,
                scroll_upgrade: None,
                cuttable_count: None,
                golden_hammer_flag: None,
                scroll_resilience_count: None,
                scroll_upgradable_count: None,
                soul_name: None,
                soul_option: None,
                soul_pad: None,
                soul_mad: None,
                soul_active: None,
                soul_potential_grade: None,
                soul_potential_amplified_grade: None,
                soul_potential_option_1: None,
                soul_potential_option_2: None,
                soul_potential_option_3: None,
                item_etc_option: None,
                starforce: None,
                starforce_scroll_flag: None,
                item_starforce_option: None,
                special_ring_level: None,
                date_expire: None,
                freestyle_flag: None,
            },
            ItemEquipmentDetail {
                item_equipment_part: Some("스태프".to_string()),
                item_equipment_slot: Some("무기".to_string()),
                item_name: Some("제네시스 스태프".to_string()),
                item_icon: Some("https://example.com/staff.png".to_string()),
                item_description: None,
                item_shape_name: None,
                item_shape_icon: None,
                item_gender: None,
                item_total_option: None,
                item_base_option: None,
                potential_option_grade: None,
                additional_potential_option_grade: None,
                potential_option_flag: None,
                additional_potential_option_flag: None,
                potential_option_1: None,
                potential_option_2: None,
                potential_option_3: None,
                additional_potential_option_1: None,
                additional_potential_option_2: None,
                additional_potential_option_3: None,
                equipment_level_increase: None,
                item_exceptional_option: None,
                item_add_option: None,
                growth_exp: None,
                growth_level: None,
                scroll_upgrade: None,
                cuttable_count: None,
                golden_hammer_flag: None,
                scroll_resilience_count: None,
                scroll_upgradable_count: None,
                soul_name: None,
                soul_option: None,
                soul_pad: None,
                soul_mad: None,
                soul_active: None,
                soul_potential_grade: None,
                soul_potential_amplified_grade: None,
                soul_potential_option_1: None,
                soul_potential_option_2: None,
                soul_potential_option_3: None,
                item_etc_option: None,
                starforce: None,
                starforce_scroll_flag: None,
                item_starforce_option: None,
                special_ring_level: None,
                date_expire: None,
                freestyle_flag: None,
            },
        ]),
        item_equipment_preset_2: None,
        item_equipment_preset_3: None,
        title: None,
        dragon_equipment: None,
        mechanic_equipment: None,
    };

    let hat = equip.hat().expect("hat must exist");
    assert_eq!(hat.item_name.as_deref(), Some("하이네스 던위치햇"));

    let weapon = equip.weapon().expect("weapon must exist");
    assert_eq!(weapon.item_name.as_deref(), Some("제네시스 스태프"));
}

#[test]
fn test_coordination_synthesis_rules() {
    let beauty = CharacterBeautyEquipment {
        date: None,
        character_gender: Some("여".to_string()),
        character_class: Some("비숍".to_string()),
        character_hair: Some(HairInfo {
            hair_name: Some("안젤라 헤어".to_string()),
            base_color: Some("검은색".to_string()),
            mix_color: None,
            mix_rate: None,
            freestyle_flag: None,
        }),
        character_face: Some(FaceInfo {
            face_name: Some("조용한 눈 얼굴".to_string()),
            base_color: Some("자주색".to_string()),
            mix_color: None,
            mix_rate: None,
            freestyle_flag: None,
        }),
        character_skin: Some(SkinInfo {
            skin_name: Some("뽀송 라벤더".to_string()),
            color_style: None,
            hue: None,
            saturation: None,
            brightness: None,
        }),
        additional_character_hair: None,
        additional_character_face: None,
        additional_character_skin: None,
    };

    let cash_equip = CharacterCashItemEquipment {
        date: None,
        character_gender: None,
        character_class: None,
        character_look_mode: None,
        preset_no: None,
        cash_item_equipment_base: Some(vec![
            CashItemEquipmentDetail {
                cash_item_equipment_part: Some("모자".to_string()),
                cash_item_equipment_slot: Some("모자".to_string()),
                cash_item_name: Some("로미 리본".to_string()),
                cash_item_icon: Some("https://example.com/cash_hat.png".to_string()),
                cash_item_description: None,
                cash_item_option: None,
                date_expire: None,
                date_option_expire: None,
                cash_item_label: Some("스페셜라벨".to_string()),
                cash_item_coloring_prism_flag: None,
                cash_item_coloring_prism: None,
                cash_item_effect_prism_flag: None,
                cash_item_effect_prism: None,
                item_gender: None,
                skills: None,
                freestyle_flag: None,
                emotion_name: None,
                custom_origin: None,
            },
            CashItemEquipmentDetail {
                cash_item_equipment_part: Some("한벌옷".to_string()),
                cash_item_equipment_slot: Some("상의".to_string()),
                cash_item_name: Some("행복한 소망".to_string()),
                cash_item_icon: Some("https://example.com/cash_suit.png".to_string()),
                cash_item_description: None,
                cash_item_option: None,
                date_expire: None,
                date_option_expire: None,
                cash_item_label: Some("마스터라벨".to_string()),
                cash_item_coloring_prism_flag: None,
                cash_item_coloring_prism: None,
                cash_item_effect_prism_flag: None,
                cash_item_effect_prism: None,
                item_gender: None,
                skills: None,
                freestyle_flag: None,
                emotion_name: None,
                custom_origin: None,
            },
        ]),
        cash_item_equipment_preset_1: None,
        cash_item_equipment_preset_2: None,
        cash_item_equipment_preset_3: None,
        additional_cash_item_equipment_base: None,
        additional_cash_item_equipment_preset_1: None,
        additional_cash_item_equipment_preset_2: None,
        additional_cash_item_equipment_preset_3: None,
    };

    let normal_equip = CharacterItemEquipment {
        date: None,
        character_gender: None,
        character_class: None,
        preset_no: None,
        item_equipment: Some(vec![
            ItemEquipmentDetail {
                item_equipment_part: Some("모자".to_string()),
                item_equipment_slot: Some("모자".to_string()),
                item_name: Some("하이네스 던위치햇".to_string()),
                item_icon: Some("https://example.com/hat.png".to_string()),
                item_description: None,
                item_shape_name: None,
                item_shape_icon: None,
                item_gender: None,
                item_total_option: None,
                item_base_option: None,
                potential_option_grade: None,
                additional_potential_option_grade: None,
                potential_option_flag: None,
                additional_potential_option_flag: None,
                potential_option_1: None,
                potential_option_2: None,
                potential_option_3: None,
                additional_potential_option_1: None,
                additional_potential_option_2: None,
                additional_potential_option_3: None,
                equipment_level_increase: None,
                item_exceptional_option: None,
                item_add_option: None,
                growth_exp: None,
                growth_level: None,
                scroll_upgrade: None,
                cuttable_count: None,
                golden_hammer_flag: None,
                scroll_resilience_count: None,
                scroll_upgradable_count: None,
                soul_name: None,
                soul_option: None,
                soul_pad: None,
                soul_mad: None,
                soul_active: None,
                soul_potential_grade: None,
                soul_potential_amplified_grade: None,
                soul_potential_option_1: None,
                soul_potential_option_2: None,
                soul_potential_option_3: None,
                item_etc_option: None,
                starforce: None,
                starforce_scroll_flag: None,
                item_starforce_option: None,
                special_ring_level: None,
                date_expire: None,
                freestyle_flag: None,
            },
            ItemEquipmentDetail {
                item_equipment_part: Some("망토".to_string()),
                item_equipment_slot: Some("망토".to_string()),
                item_name: Some("앱솔랩스 메이지케이프".to_string()),
                item_icon: Some("https://example.com/cape.png".to_string()),
                item_description: None,
                item_shape_name: Some("에테르넬 나이트케이프".to_string()),
                item_shape_icon: Some("https://example.com/anvil_cape.png".to_string()),
                item_gender: None,
                item_total_option: None,
                item_base_option: None,
                potential_option_grade: None,
                additional_potential_option_grade: None,
                potential_option_flag: None,
                additional_potential_option_flag: None,
                potential_option_1: None,
                potential_option_2: None,
                potential_option_3: None,
                additional_potential_option_1: None,
                additional_potential_option_2: None,
                additional_potential_option_3: None,
                equipment_level_increase: None,
                item_exceptional_option: None,
                item_add_option: None,
                growth_exp: None,
                growth_level: None,
                scroll_upgrade: None,
                cuttable_count: None,
                golden_hammer_flag: None,
                scroll_resilience_count: None,
                scroll_upgradable_count: None,
                soul_name: None,
                soul_option: None,
                soul_pad: None,
                soul_mad: None,
                soul_active: None,
                soul_potential_grade: None,
                soul_potential_amplified_grade: None,
                soul_potential_option_1: None,
                soul_potential_option_2: None,
                soul_potential_option_3: None,
                item_etc_option: None,
                starforce: None,
                starforce_scroll_flag: None,
                item_starforce_option: None,
                special_ring_level: None,
                date_expire: None,
                freestyle_flag: None,
            },
            ItemEquipmentDetail {
                item_equipment_part: Some("하의".to_string()),
                item_equipment_slot: Some("하의".to_string()),
                item_name: Some("트릭스터 던위치팬츠".to_string()),
                item_icon: Some("https://example.com/pants.png".to_string()),
                item_description: None,
                item_shape_name: None,
                item_shape_icon: None,
                item_gender: None,
                item_total_option: None,
                item_base_option: None,
                potential_option_grade: None,
                additional_potential_option_grade: None,
                potential_option_flag: None,
                additional_potential_option_flag: None,
                potential_option_1: None,
                potential_option_2: None,
                potential_option_3: None,
                additional_potential_option_1: None,
                additional_potential_option_2: None,
                additional_potential_option_3: None,
                equipment_level_increase: None,
                item_exceptional_option: None,
                item_add_option: None,
                growth_exp: None,
                growth_level: None,
                scroll_upgrade: None,
                cuttable_count: None,
                golden_hammer_flag: None,
                scroll_resilience_count: None,
                scroll_upgradable_count: None,
                soul_name: None,
                soul_option: None,
                soul_pad: None,
                soul_mad: None,
                soul_active: None,
                soul_potential_grade: None,
                soul_potential_amplified_grade: None,
                soul_potential_option_1: None,
                soul_potential_option_2: None,
                soul_potential_option_3: None,
                item_etc_option: None,
                starforce: None,
                starforce_scroll_flag: None,
                item_starforce_option: None,
                special_ring_level: None,
                date_expire: None,
                freestyle_flag: None,
            },
        ]),
        item_equipment_preset_1: None,
        item_equipment_preset_2: None,
        item_equipment_preset_3: None,
        title: None,
        dragon_equipment: None,
        mechanic_equipment: None,
    };

    let coord = CharacterCoordination::from_parts(&beauty, &cash_equip, &normal_equip);

    // Rule 1: Hat has cash item -> Cash item wins
    let hat = coord.hat().expect("hat must be visible");
    assert_eq!(hat.name, "로미 리본");
    assert!(hat.is_cash);
    assert_eq!(hat.cash_label.as_deref(), Some("스페셜라벨"));

    // Rule 2: Suit has cash item -> Overrides top and covers bottom
    let top = coord.top().expect("top/suit must be visible");
    assert_eq!(top.slot, "한벌옷");
    assert_eq!(top.name, "행복한 소망");
    assert!(top.is_cash);
    assert_eq!(coord.bottom(), None, "bottom should be covered by overall");

    // Rule 3: Cape has no cash item -> Normal item wins, with Anvil shape applied
    let cape = coord.cape().expect("cape must be visible");
    assert_eq!(cape.name, "에테르넬 나이트케이프");
    assert!(!cape.is_cash);
    assert!(cape.is_anvil);
    assert_eq!(cape.original_name.as_deref(), Some("앱솔랩스 메이지케이프"));
}

#[test]
fn test_error_helpers() {
    let not_found = MapleStoryError::CharacterNotFound {
        name: "test".to_string(),
    };
    assert!(not_found.is_not_found());
    assert!(!not_found.is_rate_limited());

    let rate_limited = MapleStoryError::RateLimited {
        retry_after: 60,
        code: "OPENAPI00004".to_string(),
        message: "too many requests".to_string(),
    };
    assert!(rate_limited.is_rate_limited());
    assert_eq!(
        rate_limited.retry_after(),
        Some(std::time::Duration::from_secs(60))
    );
    assert_eq!(rate_limited.status_code(), Some(429));
    assert_eq!(rate_limited.api_code(), Some("OPENAPI00004"));
}

#[test]
fn test_transparent_equipment_and_invisible_sub_weapon() {
    let beauty = CharacterBeautyEquipment {
        date: None,
        character_gender: None,
        character_class: None,
        character_hair: None,
        character_face: None,
        character_skin: None,
        additional_character_hair: None,
        additional_character_face: None,
        additional_character_skin: None,
    };

    // Preset 1 is active, but gloves are only present in base as "투명 장갑"
    let cash_equip = CharacterCashItemEquipment {
        date: None,
        character_gender: None,
        character_class: None,
        character_look_mode: None,
        preset_no: Some(1),
        cash_item_equipment_base: Some(vec![CashItemEquipmentDetail {
            cash_item_equipment_part: Some("장갑".to_string()),
            cash_item_equipment_slot: Some("장갑".to_string()),
            cash_item_name: Some("투명 장갑".to_string()),
            cash_item_icon: None,
            cash_item_description: None,
            cash_item_option: None,
            date_expire: None,
            date_option_expire: None,
            cash_item_label: None,
            cash_item_coloring_prism_flag: None,
            cash_item_coloring_prism: None,
            cash_item_effect_prism_flag: None,
            cash_item_effect_prism: None,
            item_gender: None,
            skills: None,
            freestyle_flag: None,
            emotion_name: None,
            custom_origin: None,
        }]),
        cash_item_equipment_preset_1: Some(vec![]),
        cash_item_equipment_preset_2: None,
        cash_item_equipment_preset_3: None,
        additional_cash_item_equipment_base: None,
        additional_cash_item_equipment_preset_1: None,
        additional_cash_item_equipment_preset_2: None,
        additional_cash_item_equipment_preset_3: None,
    };

    let normal_equip = CharacterItemEquipment {
        date: None,
        character_gender: None,
        character_class: None,
        preset_no: None,
        item_equipment: Some(vec![
            ItemEquipmentDetail {
                item_equipment_part: Some("장갑".to_string()),
                item_equipment_slot: Some("장갑".to_string()),
                item_name: Some("앱솔랩스 메이지글러브".to_string()),
                item_icon: None,
                item_description: None,
                item_shape_name: None,
                item_shape_icon: None,
                item_gender: None,
                item_total_option: None,
                item_base_option: None,
                potential_option_grade: None,
                additional_potential_option_grade: None,
                potential_option_flag: None,
                additional_potential_option_flag: None,
                potential_option_1: None,
                potential_option_2: None,
                potential_option_3: None,
                additional_potential_option_1: None,
                additional_potential_option_2: None,
                additional_potential_option_3: None,
                equipment_level_increase: None,
                item_exceptional_option: None,
                item_add_option: None,
                growth_exp: None,
                growth_level: None,
                scroll_upgrade: None,
                cuttable_count: None,
                golden_hammer_flag: None,
                scroll_resilience_count: None,
                scroll_upgradable_count: None,
                soul_name: None,
                soul_option: None,
                soul_pad: None,
                soul_mad: None,
                soul_active: None,
                soul_potential_grade: None,
                soul_potential_amplified_grade: None,
                soul_potential_option_1: None,
                soul_potential_option_2: None,
                soul_potential_option_3: None,
                item_etc_option: None,
                starforce: None,
                starforce_scroll_flag: None,
                item_starforce_option: None,
                special_ring_level: None,
                date_expire: None,
                freestyle_flag: None,
            },
            ItemEquipmentDetail {
                item_equipment_part: Some("마도서".to_string()),
                item_equipment_slot: Some("보조무기".to_string()),
                item_name: Some("청은의 서 <종장>".to_string()),
                item_icon: None,
                item_description: None,
                item_shape_name: None,
                item_shape_icon: None,
                item_gender: None,
                item_total_option: None,
                item_base_option: None,
                potential_option_grade: None,
                additional_potential_option_grade: None,
                potential_option_flag: None,
                additional_potential_option_flag: None,
                potential_option_1: None,
                potential_option_2: None,
                potential_option_3: None,
                additional_potential_option_1: None,
                additional_potential_option_2: None,
                additional_potential_option_3: None,
                equipment_level_increase: None,
                item_exceptional_option: None,
                item_add_option: None,
                growth_exp: None,
                growth_level: None,
                scroll_upgrade: None,
                cuttable_count: None,
                golden_hammer_flag: None,
                scroll_resilience_count: None,
                scroll_upgradable_count: None,
                soul_name: None,
                soul_option: None,
                soul_pad: None,
                soul_mad: None,
                soul_active: None,
                soul_potential_grade: None,
                soul_potential_amplified_grade: None,
                soul_potential_option_1: None,
                soul_potential_option_2: None,
                soul_potential_option_3: None,
                item_etc_option: None,
                starforce: None,
                starforce_scroll_flag: None,
                item_starforce_option: None,
                special_ring_level: None,
                date_expire: None,
                freestyle_flag: None,
            },
        ]),
        item_equipment_preset_1: None,
        item_equipment_preset_2: None,
        item_equipment_preset_3: None,
        title: None,
        dragon_equipment: None,
        mechanic_equipment: None,
    };

    let coord = CharacterCoordination::from_parts(&beauty, &cash_equip, &normal_equip);

    // Fallback to base cash: Gloves are transparent cash item
    let gloves = coord.gloves().expect("gloves must exist");
    assert_eq!(gloves.name, "투명 장갑");
    assert!(gloves.is_cash);
    assert!(gloves.is_transparent);
    assert!(
        !gloves.is_visually_rendered(),
        "transparent gloves are visually hidden"
    );

    // Sub weapon: Spellbook (마도서) is in inventory, but not visually rendered on sprite
    let sub = coord.sub_weapon().expect("sub weapon must exist");
    assert_eq!(sub.name, "청은의 서 <종장>");
    assert!(
        !sub.is_visually_rendered(),
        "spellbooks are not visually rendered on sprites"
    );
}
