//! Integration and serialization tests for MapleStory OpenAPI models.

use maplestory_openapi::models::{
    character::{CharacterBasic, CharacterItemEquipment, CharacterOcid, CharacterStat},
    common::SkillGrade,
    user::CharacterList,
};

#[test]
fn test_character_ocid_deserialization() {
    let json_data = r#"{"ocid":"test-ocid-12345"}"#;
    let ocid: CharacterOcid = serde_json::from_str(json_data).expect("failed to deserialize ocid");
    assert_eq!(ocid.ocid.as_deref(), Some("test-ocid-12345"));
}

#[test]
fn test_character_basic_deserialization() {
    let json_data = r#"{
        "date": "2024-01-15T00:00+09:00",
        "character_name": "아델",
        "world_name": "스카니아",
        "character_gender": "여",
        "character_class": "아델",
        "character_class_level": "6",
        "character_level": 260,
        "character_exp": 123456789,
        "character_exp_rate": "15.234",
        "character_guild_name": "영웅",
        "character_image": "https://example.com/avatar.png",
        "character_date_create": "2020-01-16T10:00+09:00",
        "access_flag": "true",
        "liberation_quest_clear_flag": "true"
    }"#;

    let basic: CharacterBasic = serde_json::from_str(json_data).expect("failed to deserialize basic");
    assert_eq!(basic.character_name.as_deref(), Some("아델"));
    assert_eq!(basic.character_level, Some(260));
    assert_eq!(basic.world_name.as_deref(), Some("스카니아"));
    assert_eq!(basic.character_guild_name.as_deref(), Some("영웅"));
}

#[test]
fn test_character_stat_deserialization() {
    let json_data = r#"{
        "date": "2024-01-15T00:00+09:00",
        "character_class": "히어로",
        "final_stat": [
            {"stat_name": "최소 스탯공격력", "stat_value": "50000000"},
            {"stat_name": "최대 스탯공격력", "stat_value": "60000000"},
            {"stat_name": "보스 몬스터 데미지", "stat_value": "350"}
        ],
        "remain_ap": 0
    }"#;

    let stat: CharacterStat = serde_json::from_str(json_data).expect("failed to deserialize stat");
    assert_eq!(stat.character_class.as_deref(), Some("히어로"));
    let stats = stat.final_stat.expect("stats missing");
    assert_eq!(stats.len(), 3);
    assert_eq!(stats[0].stat_name.as_deref(), Some("최소 스탯공격력"));
    assert_eq!(stats[0].stat_value.as_deref(), Some("50000000"));
}

#[test]
fn test_character_item_equipment_deserialization() {
    let json_data = r#"{
        "date": "2024-01-15T00:00+09:00",
        "character_gender": "남",
        "character_class": "나이트로드",
        "preset_no": 1,
        "item_equipment": [
            {
                "item_equipment_part": "모자",
                "item_equipment_slot": "모자",
                "item_name": "하이네스 던위치햇",
                "item_icon": "https://example.com/hat.png",
                "starforce": "22",
                "potential_option_grade": "레전드리",
                "item_total_option": {
                    "str": "150",
                    "luk": "200"
                }
            }
        ]
    }"#;

    let equip: CharacterItemEquipment = serde_json::from_str(json_data).expect("failed to deserialize equip");
    assert_eq!(equip.preset_no, Some(1));
    let items = equip.item_equipment.expect("missing items");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].item_name.as_deref(), Some("하이네스 던위치햇"));
    assert_eq!(items[0].starforce.as_deref(), Some("22"));
    let total_opt = items[0].item_total_option.as_ref().unwrap();
    assert_eq!(total_opt.luk.as_deref(), Some("200"));
}

#[test]
fn test_character_list_deserialization() {
    let json_data = r#"{
        "account_list": [
            {
                "account_id": "acc-1",
                "character_list": [
                    {
                        "character_name": "캐릭터1",
                        "world_name": "루나",
                        "character_class": "비숍",
                        "character_level": 280
                    }
                ]
            }
        ]
    }"#;

    let list: CharacterList = serde_json::from_str(json_data).expect("failed to deserialize character list");
    let accounts = list.account_list.unwrap();
    assert_eq!(accounts.len(), 1);
    let chars = accounts[0].character_list.as_ref().unwrap();
    assert_eq!(chars[0].character_name.as_deref(), Some("캐릭터1"));
    assert_eq!(chars[0].character_level, Some(280));
}

#[test]
fn test_skill_grade_display() {
    assert_eq!(SkillGrade::Zero.to_string(), "0");
    assert_eq!(SkillGrade::First.to_string(), "1");
    assert_eq!(SkillGrade::FirstHalf.to_string(), "1.5");
    assert_eq!(SkillGrade::Second.to_string(), "2");
    assert_eq!(SkillGrade::SecondHalf.to_string(), "2.5");
    assert_eq!(SkillGrade::Third.to_string(), "3");
    assert_eq!(SkillGrade::Fourth.to_string(), "4");
    assert_eq!(SkillGrade::HyperPassive.to_string(), "hyperpassive");
    assert_eq!(SkillGrade::HyperActive.to_string(), "hyperactive");
    assert_eq!(SkillGrade::Fifth.to_string(), "5");
    assert_eq!(SkillGrade::Sixth.to_string(), "6");
}
