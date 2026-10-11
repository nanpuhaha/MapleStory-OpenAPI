//! Integration and serialization tests for MapleStory OpenAPI models.

use maplestory_openapi::models::{
    battle_practice::{BattlePracticeResult, ReplayId, SkillTimeline},
    character::{CharacterBasic, CharacterItemEquipment, CharacterOcid, CharacterStat},
    common::SkillGrade,
    history::{CubeHistory, PotentialHistory, SoulPotentialHistory, StarforceHistory},
    notice::{CashshopNoticeList, EventNoticeList, NoticeDetail, NoticeList},
    ranking::{
        AchievementRanking, DojangRanking, GuildRanking, OverallRanking, TheSeedRanking,
        UnionRanking,
    },
    scheduler::CharacterStateResponse,
    union::UserUnionChampion,
    user::{CharacterList, User},
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

    let basic: CharacterBasic =
        serde_json::from_str(json_data).expect("failed to deserialize basic");
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

    let equip: CharacterItemEquipment =
        serde_json::from_str(json_data).expect("failed to deserialize equip");
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

    let list: CharacterList =
        serde_json::from_str(json_data).expect("failed to deserialize character list");
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

#[test]
fn test_union_champion_deserialization() {
    let json_data = r#"{
        "date": "2024-01-15T00:00+09:00",
        "union_champion": [
            {
                "champion_name": "챔피언1",
                "champion_slot": 1,
                "champion_grade": "SSS",
                "champion_class": "히어로",
                "champion_badge_info": [
                    {"stat": "공격력 10 증가"}
                ]
            }
        ],
        "champion_badge_total_info": [
            {"stat": "공격력 10 증가"}
        ]
    }"#;

    let champion: UserUnionChampion =
        serde_json::from_str(json_data).expect("failed to deserialize union champion");
    assert_eq!(champion.date.as_deref(), Some("2024-01-15T00:00+09:00"));
    let champs = champion.union_champion.unwrap();
    assert_eq!(champs.len(), 1);
    assert_eq!(champs[0].champion_name.as_deref(), Some("챔피언1"));
    assert_eq!(champs[0].champion_slot, Some(1));
}

#[test]
fn test_user_ouid_deserialization() {
    let json_data = r#"{"ouid": "ouid-test-abcde-12345"}"#;
    let user: User = serde_json::from_str(json_data).expect("failed to deserialize ouid");
    assert_eq!(user.ouid.as_deref(), Some("ouid-test-abcde-12345"));
}

#[test]
fn test_starforce_history_deserialization() {
    let json_data = r#"{
        "count": 1,
        "next_cursor": "cursor123",
        "starforce_history": [
            {
                "id": "sf-1",
                "item_upgrade_result": "성공",
                "before_starforce_count": 21,
                "after_starforce_count": 22,
                "starcatch_result": "성공",
                "target_item": "아케인셰이드 투구",
                "date_create": "2024-01-15T12:00:00.000+09:00"
            }
        ]
    }"#;

    let history: StarforceHistory =
        serde_json::from_str(json_data).expect("failed to deserialize starforce history");
    assert_eq!(history.count, Some(1));
    assert_eq!(history.next_cursor.as_deref(), Some("cursor123"));
    let entries = history.starforce_history.unwrap();
    assert_eq!(entries[0].item_upgrade_result.as_deref(), Some("성공"));
    assert_eq!(entries[0].after_starforce_count, Some(22));
}

#[test]
fn test_potential_history_deserialization() {
    let json_data = r#"{
        "count": 1,
        "next_cursor": null,
        "potential_history": [
            {
                "id": "pot-1",
                "character_name": "테스트",
                "potential_type": "잠재능력",
                "item_upgrade_result": "유지",
                "target_item": "아케인셰이드 무기",
                "potential_option_grade": "레전드리",
                "after_potential_option": [
                    {"value": "보스 몬스터 공격 시 데미지 : +40%", "grade": "레전드리"}
                ]
            }
        ]
    }"#;

    let history: PotentialHistory =
        serde_json::from_str(json_data).expect("failed to deserialize potential history");
    assert_eq!(history.count, Some(1));
    let entries = history.potential_history.unwrap();
    assert_eq!(entries[0].potential_option_grade.as_deref(), Some("레전드리"));
}

#[test]
fn test_cube_and_soul_potential_history_deserialization() {
    let cube_json = r#"{
        "count": 1,
        "cube_history": [
            {
                "id": "cube-1",
                "character_name": "테스터",
                "cube_type": "블랙 큐브",
                "target_item": "장비1"
            }
        ]
    }"#;
    let cube: CubeHistory = serde_json::from_str(cube_json).unwrap();
    assert_eq!(cube.count, Some(1));

    let soul_json = r#"{
        "count": 1,
        "soul_potential_history": [
            {
                "id": "soul-1",
                "character_name": "테스터",
                "soul_potential_grade": "위대한",
                "target_item": "무기1"
            }
        ]
    }"#;
    let soul: SoulPotentialHistory = serde_json::from_str(soul_json).unwrap();
    assert_eq!(soul.count, Some(1));
}

#[test]
fn test_ranking_deserialization() {
    let overall_json = r#"{
        "ranking": [
            {
                "date": "2024-01-15",
                "ranking": 1,
                "character_name": "랭커1",
                "world_name": "스카니아",
                "class_name": "아델",
                "character_level": 300
            }
        ]
    }"#;
    let overall: OverallRanking = serde_json::from_str(overall_json).unwrap();
    let ranks = overall.ranking.unwrap();
    assert_eq!(ranks[0].character_name.as_deref(), Some("랭커1"));

    let union_json = r#"{"ranking": [{"ranking": 1, "union_level": 9500}]}"#;
    let union: UnionRanking = serde_json::from_str(union_json).unwrap();
    assert_eq!(union.ranking.unwrap()[0].union_level, Some(9500));

    let guild_json = r#"{"ranking": [{"ranking": 1, "guild_name": "최강"}]}"#;
    let guild: GuildRanking = serde_json::from_str(guild_json).unwrap();
    assert_eq!(guild.ranking.unwrap()[0].guild_name.as_deref(), Some("최강"));

    let dojang_json = r#"{"ranking": [{"ranking": 1, "dojang_floor": 100}]}"#;
    let dojang: DojangRanking = serde_json::from_str(dojang_json).unwrap();
    assert_eq!(dojang.ranking.unwrap()[0].dojang_floor, Some(100));

    let theseed_json = r#"{"ranking": [{"ranking": 1, "theseed_floor": 50}]}"#;
    let theseed: TheSeedRanking = serde_json::from_str(theseed_json).unwrap();
    assert_eq!(theseed.ranking.unwrap()[0].theseed_floor, Some(50));

    let ach_json = r#"{"ranking": [{"ranking": 1, "trophy_score": 50000}]}"#;
    let ach: AchievementRanking = serde_json::from_str(ach_json).unwrap();
    assert_eq!(ach.ranking.unwrap()[0].trophy_score, Some(50000));
}

#[test]
fn test_notice_deserialization() {
    let list_json = r#"{
        "notice": [
            {
                "title": "정기점검 안내",
                "url": "https://maplestory.nexon.com/notice/1",
                "notice_id": 1001,
                "date": "2024-01-15T00:00+09:00"
            }
        ]
    }"#;
    let list: NoticeList = serde_json::from_str(list_json).unwrap();
    assert_eq!(list.notice.unwrap()[0].notice_id, Some(1001));

    let detail_json = r#"{
        "title": "공지 상세",
        "url": "https://maplestory.nexon.com/notice/1",
        "contents": "<p>본문 내용</p>",
        "date": "2024-01-15T00:00+09:00"
    }"#;
    let detail: NoticeDetail = serde_json::from_str(detail_json).unwrap();
    assert_eq!(detail.contents.as_deref(), Some("<p>본문 내용</p>"));

    let event_json = r#"{
        "event_notice": [
            {
                "title": "겨울 이벤트",
                "date_event_start": "2024-01-01T00:00+09:00",
                "date_event_end": "2024-02-01T00:00+09:00"
            }
        ]
    }"#;
    let event: EventNoticeList = serde_json::from_str(event_json).unwrap();
    assert_eq!(event.event_notice.unwrap()[0].title.as_deref(), Some("겨울 이벤트"));

    let cash_json = r#"{
        "cashshop_notice": [
            {
                "title": "로얄 스타일 쿠폰",
                "ongoing_flag": "false"
            }
        ]
    }"#;
    let cash: CashshopNoticeList = serde_json::from_str(cash_json).unwrap();
    assert_eq!(cash.cashshop_notice.unwrap()[0].ongoing_flag.as_deref(), Some("false"));
}

#[test]
fn test_scheduler_deserialization() {
    let json_data = r#"{
        "character_name": "스케줄러캐릭터",
        "world_name": "루나",
        "character_level": 275,
        "character_class": "섀도어",
        "daily_contents": [
            {
                "content_name": "소멸의 여로",
                "type": "quest",
                "registration_flag": "true",
                "now_count": 1,
                "max_count": 1,
                "quest_state": "2"
            }
        ],
        "boss_contents": [
            {
                "content_name": "자쿰",
                "difficulty": "카오스",
                "cycle": "주간",
                "complete_flag": "true"
            }
        ],
        "weekly_boss_clear_count": 12,
        "weekly_boss_clear_limit_count": 12
    }"#;

    let scheduler: CharacterStateResponse = serde_json::from_str(json_data).unwrap();
    assert_eq!(scheduler.character_name.as_deref(), Some("스케줄러캐릭터"));
    assert_eq!(scheduler.weekly_boss_clear_count, Some(12));
    assert_eq!(scheduler.daily_contents.unwrap()[0].content_name.as_deref(), Some("소멸의 여로"));
}

#[test]
fn test_battle_practice_deserialization() {
    let replay_json = r#"{
        "replay_list": [
            {
                "period_no": 1,
                "register_date": "2024-01-15T15:00:00+09:00",
                "replay_id": "rep-abc-123"
            }
        ]
    }"#;
    let replay: ReplayId = serde_json::from_str(replay_json).unwrap();
    assert_eq!(replay.replay_list.unwrap()[0].replay_id.as_deref(), Some("rep-abc-123"));

    let result_json = r#"{
        "total_play_time": 180000,
        "total_damage": 5000000000000,
        "total_dps": 27777777777,
        "end_type": "1",
        "like_count": 5,
        "skill_statistic": [
            {
                "skill_name": "블래스트",
                "damage": 2000000000000,
                "damage_percent": "40.0"
            }
        ]
    }"#;
    let result: BattlePracticeResult = serde_json::from_str(result_json).unwrap();
    assert_eq!(result.total_play_time, Some(180000));
    assert_eq!(result.like_count, Some(5));

    let timeline_json = r#"{
        "page_no": 1,
        "total_page_no": 10,
        "skill_timeline": [
            {
                "elapse_time": 1500,
                "skill_name": "스킬1",
                "hexa_skill_specificity_flag": "0"
            }
        ]
    }"#;
    let timeline: SkillTimeline = serde_json::from_str(timeline_json).unwrap();
    assert_eq!(timeline.page_no, Some(1));
    assert_eq!(timeline.skill_timeline.unwrap()[0].elapse_time, Some(1500));
}
