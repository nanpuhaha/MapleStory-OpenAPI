//! Live API and Offline Cassette Replay E2E tests for MapleStory OpenAPI.
//!
//! If `NEXON_API_KEY` is present in the environment (or in a `.env` file),
//! this test suite executes real API calls for "온앤온" and "니니류".
//! When `NEXON_API_KEY` is absent (e.g., in offline CI environments), it automatically
//! falls back to a local golden-fixture replay server so all E2E assertions still run.

use maplestory_openapi::MapleStoryClient;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn spawn_offline_fixture_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        while let Ok((mut stream, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                let n = stream.read(&mut buf).await.unwrap_or(0);
                if n == 0 {
                    return;
                }
                let req = String::from_utf8_lossy(&buf[..n]);
                let first_line = req.lines().next().unwrap_or("");
                let path = first_line.split_whitespace().nth(1).unwrap_or("");

                let avatar_hex = "a1".repeat(512);
                let look_url = format!(
                    "https://open.api.nexon.com/static/maplestory/character/look/{}",
                    avatar_hex
                );

                let body = if path.starts_with("/maplestory/v1/id") {
                    if path.contains("%EB%8B%88%EB%8B%88%EB%A5%98") || path.contains("니니류") {
                        r#"{"ocid": "offline-ocid-niniryu"}"#.to_string()
                    } else {
                        r#"{"ocid": "offline-ocid-onandon"}"#.to_string()
                    }
                } else if path.starts_with("/maplestory/v1/character/basic") {
                    if path.contains("offline-ocid-niniryu") {
                        format!(
                            r#"{{"character_name": "니니류", "world_name": "스카니아", "character_gender": "여", "character_class": "비숍", "character_level": 285, "character_guild_name": "온앤온", "character_image": "{}"}}"#,
                            look_url
                        )
                    } else {
                        format!(
                            r#"{{"character_name": "온앤온", "world_name": "스카니아", "character_gender": "여", "character_class": "아델", "character_level": 290, "character_guild_name": "온앤온", "character_image": "{}"}}"#,
                            look_url
                        )
                    }
                } else if path.starts_with("/maplestory/v1/character/stat") {
                    r#"{"character_class": "아델", "final_stat": [{"stat_name": "전투력", "stat_value": "450000000"}, {"stat_name": "아케인포스", "stat_value": "1350"}, {"stat_name": "어센틱포스", "stat_value": "660"}]}"#.to_string()
                } else if path.starts_with("/maplestory/v1/character/item-equipment") {
                    r#"{"item_equipment": [{"item_equipment_part": "모자", "item_equipment_slot": "모자", "item_name": "에테르넬 나이트헬름", "item_icon": "https://example.com/hat.png", "starforce": "22", "potential_option_1": "STR +12%", "potential_option_2": "STR +9%", "potential_option_3": "올스탯 +9%", "additional_potential_option_1": "공격력 +12%"}]}"#.to_string()
                } else if path.starts_with("/maplestory/v1/character/cashitem-equipment") {
                    r#"{"cash_item_equipment_base": [{"cash_item_equipment_part": "모자", "cash_item_equipment_slot": "모자", "cash_item_name": "트윙클 글리터", "cash_item_icon": "https://example.com/cash_hat.png"}]}"#.to_string()
                } else if path.starts_with("/maplestory/v1/character/beauty-equipment") {
                    r#"{"character_hair": {"hair_name": "블링 신비 헤어", "base_color": "검은색"}, "character_face": {"face_name": "조용한 눈 얼굴", "base_color": "검은색"}, "character_skin": {"skin_name": "뽀송 꽃잎 피부"}}"#.to_string()
                } else if path.starts_with("/maplestory/v1/character/popularity") {
                    r#"{"popularity": 1234}"#.to_string()
                } else if path.starts_with("/maplestory/v1/character/dojang") {
                    r#"{"dojang_best_floor": 85, "dojang_best_time": 720}"#.to_string()
                } else if path.starts_with("/maplestory/v1/character/symbol-equipment") {
                    r#"{"symbol": [{"symbol_name": "아케인심볼 : 소멸의 여로", "symbol_force": "220", "symbol_level": 20}, {"symbol_name": "어센틱심볼 : 세르니움", "symbol_force": "110", "symbol_level": 11}]}"#.to_string()
                } else if path.starts_with("/maplestory/v1/character/set-effect") {
                    r#"{"set_effect": [{"set_name": "에테르넬 세트 (전사)", "total_set_count": 4, "set_effect_info": [{"set_count": 2, "set_option": "보스 몬스터 공격 시 데미지 +10%"}, {"set_count": 4, "set_option": "크리티컬 데미지 +8%"}]}]}"#.to_string()
                } else if path.starts_with("/maplestory/v1/user/union-artifact") {
                    r#"{"union_artifact_effect": [{"name": "보스 몬스터 공격 시 데미지 +15%", "level": 10}, {"name": "크리티컬 데미지 +7.50%", "level": 10}], "union_artifact_crystal": [{"name": "크리스탈 : 주황버섯", "level": 5, "validity_flag": "0", "crystal_option_name_1": "보스 몬스터 공격 시 데미지 증가", "crystal_option_name_2": "크리티컬 데미지 증가", "crystal_option_name_3": "올스탯 증가"}], "union_artifact_remain_ap": 12}"#.to_string()
                } else if path.starts_with("/maplestory/v1/user/union") {
                    r#"{"union_level": 9500, "union_grade": "그랜드 마스터 유니온 4", "union_artifact_level": 50}"#.to_string()
                } else if path.starts_with("/maplestory/v1/guild/id") {
                    r#"{"oguild_id": "offline-oguild-onandon"}"#.to_string()
                } else if path.starts_with("/maplestory/v1/guild/basic") {
                    r#"{"guild_name": "온앤온", "world_name": "스카니아", "guild_level": 30, "guild_fame": 100000, "guild_point": 10000000, "guild_master_name": "온앤온", "guild_member_count": 2, "guild_member": ["온앤온", "니니류"], "guild_skill": [{"skill_name": "길드 정기 지원 I", "skill_level": 4}], "guild_noblesse_skill": [{"skill_name": "보스 슬레이어", "skill_level": 15}, {"skill_name": "크게 한방", "skill_level": 15}, {"skill_name": "방어력 무시", "skill_level": 15}]}"#.to_string()
                } else if path.starts_with("/maplestory/v1/ranking/overall") {
                    r#"{"ranking": [{"date": "2026-10-10", "ranking": 1, "character_name": "온앤온", "world_name": "스카니아", "class_name": "전사", "sub_class_name": "아델", "character_level": 295, "character_exp": 100000000, "character_popularity": 5000, "character_guildname": "온앤온"}]}"#.to_string()
                } else if path.starts_with("/maplestory/v1/notice") {
                    r#"{"notice": [{"title": "[공지] 클라이언트 패치 안내", "url": "https://maplestory.nexon.com/News/Notice/1", "notice_id": 1, "date": "2026-10-10T00:00+09:00"}]}"#.to_string()
                } else {
                    r#"{}"#.to_string()
                };

                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(response.as_bytes()).await;
            });
        }
    });

    format!("http://{}", addr)
}

async fn setup_client() -> MapleStoryClient {
    let _ = dotenvy::dotenv();
    let _ = dotenvy::from_filename("../.env");

    if let Ok(client) = MapleStoryClient::from_env() {
        return client;
    }

    eprintln!("NEXON_API_KEY not found; using offline fixture replay server for E2E verification");
    let base_url = spawn_offline_fixture_server().await;
    MapleStoryClient::builder()
        .api_key("offline_replay_key")
        .base_url(base_url)
        .build()
        .expect("failed to build offline replay client")
}

#[tokio::test]
async fn test_live_lookup_and_coordination_for_target_characters() {
    let client = setup_client().await;
    let test_targets = ["온앤온", "니니류"];

    for &name in &test_targets {
        println!("\n=== Live/Replay Test for Character: {} ===", name);

        // 1. One-call basic info
        let basic = client
            .get_character_by_name(name)
            .await
            .expect("failed to get basic info");
        assert_eq!(basic.character_name.as_deref(), Some(name));
        println!(
            "  Basic: Lv.{} {} ({})",
            basic.character_level.unwrap_or(0),
            basic.character_class.as_deref().unwrap_or("?"),
            basic.world_name.as_deref().unwrap_or("?")
        );

        // 2. One-call stat info and domain helpers
        let stat = client
            .get_character_stat_by_name(name)
            .await
            .expect("failed to get stat info");
        let cp = stat.combat_power();
        println!("  Combat Power: {:?}", cp);
        assert!(cp.is_some(), "combat power should be present");

        // 3. One-call coordination (코디 정보: 캐시 우선 + 일반 아이템 + 모루 외형)
        let coordination = client
            .get_character_coordination_by_name(name)
            .await
            .expect("failed to get coordination");

        assert!(
            !coordination.visible_items.is_empty(),
            "character should have visible equipment"
        );

        // 4. One-call comprehensive profile
        let profile = client
            .get_character_profile_by_name(name)
            .await
            .expect("failed to get character profile");
        assert_eq!(profile.basic.character_name.as_deref(), Some(name));
        assert!(!profile.coordination.visible_items.is_empty());

        // 5. One-call all-in-one CharacterDashboard
        let dash = client
            .get_character_dashboard_by_name(name)
            .await
            .expect("failed to get character dashboard");
        assert_eq!(dash.character_name(), name);
        assert!(dash.character_level() > 0);
        assert!(dash.combat_power() > 0);
        assert!(dash.union_level().is_some());

        if name == "온앤온" {
            assert_eq!(dash.guild_name(), Some("온앤온"));
            assert!(dash.noblesse_skill_points() > 0);
            assert!(dash.arcane_force() > 0);
            assert!(dash.authentic_force() > 0);
            assert!(dash.dojang_best_floor().is_some());
            assert!(dash.avatar_code().is_some());
            println!(
                "  Dashboard verified for 온앤온: CP={}, Guild={}, Noblesse={}p",
                dash.combat_power(),
                dash.guild_name().unwrap_or(""),
                dash.noblesse_skill_points()
            );
        }
    }
}

#[tokio::test]
async fn test_live_guild_and_profile() {
    let client = setup_client().await;

    println!("\n=== Live/Replay Test: Guild & GuildProfile ===");
    let guild_profile = client
        .guild()
        .get_guild_profile("온앤온", "스카니아")
        .await
        .expect("failed to get guild profile");

    assert_eq!(guild_profile.basic.guild_name.as_deref(), Some("온앤온"));
    assert_eq!(guild_profile.basic.world_name.as_deref(), Some("스카니아"));
    println!(
        "  Guild: {} (Lv.{}, Master: {:?}, Noblesse: {}p)",
        guild_profile.basic.guild_name.as_deref().unwrap_or(""),
        guild_profile.basic.guild_level.unwrap_or(0),
        guild_profile.basic.guild_master_name,
        guild_profile.total_noblesse_points
    );
    assert!(guild_profile.total_noblesse_points > 0);
    assert!(!guild_profile.noblesse_skills.is_empty());
}

#[tokio::test]
async fn test_live_union_and_artifact() {
    let client = setup_client().await;

    println!("\n=== Live/Replay Test: Union & Union Artifact ===");
    let union_info = client
        .union()
        .get_by_name("온앤온")
        .await
        .expect("failed to get union info");

    let union_level = union_info.union_level.unwrap_or(0);
    println!("  Union Level: {}", union_level);
    assert!(union_level > 0);

    let artifact = client
        .union()
        .get_artifact_by_name("온앤온")
        .await
        .expect("failed to get union artifact info");

    let active_effects_count = artifact.active_effects().count();
    let stats = artifact.total_artifact_stats();
    let typed_stats = artifact.total_artifact_stats_typed();
    println!(
        "  Union Artifact Active Effects: {}, Parsed Stat Categories: {}, Typed Stats: {}",
        active_effects_count,
        stats.len(),
        typed_stats.len()
    );
    assert!(active_effects_count > 0);
    assert!(!typed_stats.is_empty());
}

#[tokio::test]
async fn test_live_equipment_and_set_effects() {
    let client = setup_client().await;

    println!("\n=== Live/Replay Test: Item Equipment & Set Effects ===");
    let equip = client
        .get_character_item_equipment_by_name("온앤온")
        .await
        .expect("failed to get item equipment");

    let items = equip.item_equipment.unwrap_or_default();
    assert!(!items.is_empty());

    let mut parsed_potential_count = 0;
    for item in &items {
        let parsed_lines = item.all_potential_options_parsed();
        if !parsed_lines.is_empty() {
            parsed_potential_count += 1;
        }
    }
    assert!(parsed_potential_count > 0);

    let ocid = client.get_ocid("온앤온").await.expect("failed to get ocid");
    let set_effects = client
        .get_character_set_effect(&ocid, None)
        .await
        .expect("failed to get set effects");

    let sets = set_effects.set_effect.unwrap_or_default();
    assert!(!sets.is_empty());
}

#[tokio::test]
async fn test_live_rankings_and_notices() {
    let client = setup_client().await;

    println!("\n=== Live/Replay Test: Overall Ranking & Notices ===");
    use maplestory_openapi::models::ranking::OverallRankingQuery;
    let ranking_date = (chrono::Utc::now() + chrono::Duration::hours(9)
        - chrono::Duration::days(2))
    .format("%Y-%m-%d")
    .to_string();
    let ranking = client
        .ranking()
        .get_overall(OverallRankingQuery {
            date: Some(&ranking_date),
            page: Some(1),
            ..Default::default()
        })
        .await
        .expect("failed to fetch overall ranking");

    let rank_list = ranking.ranking.unwrap_or_default();
    assert!(!rank_list.is_empty());

    let notices = client
        .notice()
        .get_notice_list()
        .await
        .expect("failed to fetch notice list");

    let notice_items = notices.notice.unwrap_or_default();
    assert!(!notice_items.is_empty());
}

#[tokio::test]
async fn test_offline_fixture_replay_fallback_explicit() {
    let base_url = spawn_offline_fixture_server().await;
    let client = MapleStoryClient::builder()
        .api_key("offline_replay_key")
        .base_url(base_url)
        .build()
        .unwrap();

    let dash = client
        .get_character_dashboard_by_name("온앤온")
        .await
        .expect("offline fixture dashboard should succeed");
    assert_eq!(dash.character_name(), "온앤온");
    assert_eq!(dash.combat_power(), 450_000_000);
    assert_eq!(dash.union_level(), Some(9500));

    let guild_profile = client
        .guild()
        .get_profile_with_members("온앤온", "스카니아", None, true)
        .await
        .expect("offline fixture guild profile with members should succeed");
    assert_eq!(guild_profile.member_combat_powers.len(), 2);
    assert_eq!(guild_profile.total_member_combat_power(), 900_000_000);
}
