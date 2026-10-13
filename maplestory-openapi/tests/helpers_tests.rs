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

/// Helper to spin up a mock TCP server that responds to requests matching a closure.
async fn spawn_mock_server<F>(handler: F) -> (String, tokio::task::JoinHandle<()>)
where
    F: Fn(&str) -> (u16, String, Option<u64>) + Send + Sync + 'static,
{
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base_url = format!("http://{}", addr);

    let handler = std::sync::Arc::new(handler);
    let server_task = tokio::spawn(async move {
        while let Ok((mut stream, _)) = listener.accept().await {
            let handler = handler.clone();
            tokio::spawn(async move {
                let mut buf = [0u8; 1024];
                let n = stream.read(&mut buf).await.unwrap_or(0);
                let req_text = String::from_utf8_lossy(&buf[..n]);

                let (status, body, retry_after) = handler(&req_text);
                let status_str = match status {
                    200 => "200 OK",
                    401 => "401 Unauthorized",
                    429 => "429 Too Many Requests",
                    503 => "503 Service Unavailable",
                    _ => "200 OK",
                };

                let retry_header = retry_after
                    .map(|s| format!("Retry-After: {}\r\n", s))
                    .unwrap_or_default();

                let resp = format!(
                    "HTTP/1.1 {}\r\n{}Content-Length: {}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n{}",
                    status_str,
                    retry_header,
                    body.len(),
                    body
                );
                let _ = stream.write_all(resp.as_bytes()).await;
            });
        }
    });

    (base_url, server_task)
}

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

#[tokio::test]
async fn test_client_concurrency_limiter_builder_and_default() {
    use maplestory_openapi::MapleStoryClient;

    // Default concurrency limit (10)
    let client = MapleStoryClient::builder()
        .api_key("test-key")
        .build()
        .unwrap();
    assert_eq!(client.max_concurrency(), 10);

    // Custom concurrency limit
    let client_custom = MapleStoryClient::builder()
        .api_key("test-key")
        .max_concurrency(5)
        .build()
        .unwrap();
    assert_eq!(client_custom.max_concurrency(), 5);
    assert_eq!(client_custom.available_concurrency_permits(), 5);
}

#[tokio::test]
async fn test_client_retry_builder_and_default() {
    use maplestory_openapi::MapleStoryClient;

    // Default max_retries is 3
    let client = MapleStoryClient::builder()
        .api_key("test-key")
        .build()
        .unwrap();
    assert_eq!(client.max_retries(), 3);

    // Custom max_retries
    let client_custom = MapleStoryClient::builder()
        .api_key("test-key")
        .max_retries(5)
        .build()
        .unwrap();
    assert_eq!(client_custom.max_retries(), 5);
}

#[tokio::test]
async fn test_concurrency_limiter_throttles_in_flight_permits() {
    use maplestory_openapi::MapleStoryClient;

    let client = MapleStoryClient::builder()
        .api_key("test-key")
        .max_concurrency(3)
        .build()
        .unwrap();

    assert_eq!(client.available_concurrency_permits(), 3);

    // Simulate 30 tasks attempting concurrent lookups against an invalid mock URL
    // All permits are properly returned to the semaphore even on network failure.
    let mut handles = Vec::new();
    for _ in 0..30 {
        let c = client.clone();
        handles.push(tokio::spawn(async move {
            let _ = c.get_character_id("non_existent").await;
        }));
    }

    for h in handles {
        let _ = h.await;
    }

    // All permits must be restored
    assert_eq!(client.available_concurrency_permits(), 3);
}

#[test]
fn test_dashboard_options_and_section_masking() {
    use maplestory_openapi::models::character::{DashboardOptions, DashboardSection};

    let all_opts = DashboardOptions::all();
    assert!(all_opts.is_enabled(DashboardSection::Basic));
    assert!(all_opts.is_enabled(DashboardSection::Stat));
    assert!(all_opts.is_enabled(DashboardSection::VMatrix));
    assert!(all_opts.is_enabled(DashboardSection::HexaMatrix));
    assert!(all_opts.is_enabled(DashboardSection::Guild));

    let custom_opts = DashboardOptions::builder()
        .include(DashboardSection::Basic)
        .include(DashboardSection::Stat)
        .include(DashboardSection::ItemEquipment)
        .build();

    assert!(custom_opts.is_enabled(DashboardSection::Basic));
    assert!(custom_opts.is_enabled(DashboardSection::Stat));
    assert!(custom_opts.is_enabled(DashboardSection::ItemEquipment));
    assert!(!custom_opts.is_enabled(DashboardSection::VMatrix));
    assert!(!custom_opts.is_enabled(DashboardSection::Guild));

    let exclude_opts =
        DashboardOptions::all_except(&[DashboardSection::Guild, DashboardSection::PetEquipment]);
    assert!(exclude_opts.is_enabled(DashboardSection::Stat));
    assert!(!exclude_opts.is_enabled(DashboardSection::Guild));
    assert!(!exclude_opts.is_enabled(DashboardSection::PetEquipment));
}

#[test]
fn test_dashboard_partial_failure_diagnostics() {
    use maplestory_openapi::models::character::{
        CharacterBasic, CharacterCoordination, CharacterDashboard, CharacterItemEquipment,
        CharacterStat, DashboardSection, DashboardSectionError,
    };

    let mut dashboard = CharacterDashboard {
        basic: CharacterBasic {
            date: None,
            character_name: Some("테스터".to_string()),
            world_name: Some("스카니아".to_string()),
            character_gender: None,
            character_class: Some("히어로".to_string()),
            character_class_level: None,
            character_level: Some(260),
            character_exp: None,
            character_exp_rate: None,
            character_guild_name: None,
            character_image: None,
            character_date_create: None,
            access_flag: None,
            liberation_quest_clear_flag: None,
        },
        stat: CharacterStat {
            date: None,
            character_class: None,
            final_stat: None,
            remain_ap: None,
        },
        item_equipment: CharacterItemEquipment {
            date: None,
            character_gender: None,
            character_class: None,
            preset_no: None,
            item_equipment: None,
            item_equipment_preset_1: None,
            item_equipment_preset_2: None,
            item_equipment_preset_3: None,
            title: None,
            dragon_equipment: None,
            mechanic_equipment: None,
        },
        cash_item_equipment: Default::default(),
        beauty_equipment: Default::default(),
        coordination: CharacterCoordination {
            character_name: Some("테스터".to_string()),
            world_name: Some("스카니아".to_string()),
            character_class: Some("히어로".to_string()),
            character_level: Some(260),
            character_image: None,
            avatar_code: None,
            hair: None,
            face: None,
            skin: None,
            cash_preset_no: None,
            visible_items: vec![],
        },
        symbols: None,
        ability: None,
        hyper_stat: None,
        hexa_matrix: None,
        hexa_matrix_stat: None,
        v_matrix: None,
        set_effect: None,
        pet_equipment: None,
        android_equipment: None,
        link_skill: None,
        propensity: None,
        popularity: None,
        dojang: None,
        union: None,
        union_artifact: None,
        guild: None,
        errors: std::collections::HashMap::new(),
    };

    assert!(!dashboard.has_errors());
    assert!(dashboard
        .section_error(DashboardSection::HexaMatrix)
        .is_none());

    // Record a 429 rate limit error for 6th job
    dashboard.errors.insert(
        DashboardSection::HexaMatrix,
        DashboardSectionError {
            section: DashboardSection::HexaMatrix,
            message: "Rate limit exceeded (HTTP 429)".to_string(),
            api_code: Some("OPENAPI00004".to_string()),
            status_code: Some(429),
        },
    );

    assert!(dashboard.has_errors());
    let err = dashboard
        .section_error(DashboardSection::HexaMatrix)
        .unwrap();
    assert_eq!(err.status_code, Some(429));
    assert_eq!(err.api_code.as_deref(), Some("OPENAPI00004"));
}

#[tokio::test]
async fn test_client_retry_and_recovery_on_transient_503() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let (base_url, server_task) = spawn_mock_server(move |_| {
        let count = counter_clone.fetch_add(1, Ordering::SeqCst);
        if count == 0 {
            (503, "{}".to_string(), None)
        } else {
            (200, r#"{"ocid":"recovered-503"}"#.to_string(), None)
        }
    })
    .await;

    let client = maplestory_openapi::MapleStoryClient::builder()
        .api_key("test-key")
        .base_url(&base_url)
        .max_retries(2)
        .build()
        .unwrap();

    let result = client.get_character_id("hero").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().ocid.as_deref(), Some("recovered-503"));
    assert_eq!(counter.load(Ordering::SeqCst), 2);

    server_task.abort();
}

#[tokio::test]
async fn test_client_retry_and_recovery_on_rate_limit_429() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let (base_url, server_task) = spawn_mock_server(move |_| {
        let count = counter_clone.fetch_add(1, Ordering::SeqCst);
        if count == 0 {
            (
                429,
                r#"{"error":{"name":"OPENAPI00004","message":"Rate limit exceeded"}}"#.to_string(),
                Some(0),
            )
        } else {
            (200, r#"{"ocid":"recovered-429"}"#.to_string(), None)
        }
    })
    .await;

    let client = maplestory_openapi::MapleStoryClient::builder()
        .api_key("test-key")
        .base_url(&base_url)
        .max_retries(2)
        .build()
        .unwrap();

    let result = client.get_character_id("hero").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().ocid.as_deref(), Some("recovered-429"));
    assert_eq!(counter.load(Ordering::SeqCst), 2);

    server_task.abort();
}

#[tokio::test]
async fn test_client_non_retryable_401_fails_immediately() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    let (base_url, server_task) = spawn_mock_server(move |_| {
        let _ = counter_clone.fetch_add(1, Ordering::SeqCst);
        (
            401,
            r#"{"error":{"name":"OPENAPI00003","message":"Invalid API key"}}"#.to_string(),
            None,
        )
    })
    .await;

    let client = maplestory_openapi::MapleStoryClient::builder()
        .api_key("bad-key")
        .base_url(&base_url)
        .max_retries(3)
        .build()
        .unwrap();

    let result = client.get_character_id("hero").await;
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        maplestory_openapi::error::MapleStoryError::AuthenticationFailed { .. }
    ));
    assert_eq!(counter.load(Ordering::SeqCst), 1);

    server_task.abort();
}

#[test]
fn test_maple_date_conversions() {
    use chrono::{NaiveDate, TimeZone, Utc};
    use maplestory_openapi::models::MapleDate;

    // From &str and String
    let d1 = MapleDate::from("2024-05-01");
    assert_eq!(d1.as_str(), "2024-05-01");
    let d2 = MapleDate::from("2024-05-01".to_string());
    assert_eq!(d2.as_str(), "2024-05-01");

    // From NaiveDate
    let naive = NaiveDate::from_ymd_opt(2024, 5, 1).unwrap();
    let d3 = MapleDate::from(naive);
    assert_eq!(d3.as_str(), "2024-05-01");

    // From DateTime<Utc> — converts UTC to KST (+9)
    // 2024-05-01 16:00:00 UTC == 2024-05-02 01:00:00 KST
    let dt_utc = Utc.with_ymd_and_hms(2024, 5, 1, 16, 0, 0).unwrap();
    let d4 = MapleDate::from(dt_utc);
    assert_eq!(d4.as_str(), "2024-05-02");
}

#[test]
fn test_maple_date_latest_available_and_kst_refresh() {
    use chrono::{FixedOffset, TimeZone};
    use maplestory_openapi::models::MapleDate;

    let kst = FixedOffset::east_opt(9 * 3600).unwrap();

    // Case 1: At 00:30 KST on 2024-05-10 (before 01:00 AM refresh)
    // Latest available must be 2 days ago: 2024-05-08
    let before_refresh = kst.with_ymd_and_hms(2024, 5, 10, 0, 30, 0).unwrap();
    let date_before = MapleDate::latest_available_at(before_refresh);
    assert_eq!(date_before.as_str(), "2024-05-08");

    // Case 2: At 01:05 KST on 2024-05-10 (after 01:00 AM refresh)
    // Latest available must be yesterday: 2024-05-09
    let after_refresh = kst.with_ymd_and_hms(2024, 5, 10, 1, 5, 0).unwrap();
    let date_after = MapleDate::latest_available_at(after_refresh);
    assert_eq!(date_after.as_str(), "2024-05-09");
}

#[test]
fn test_maple_date_validation_boundaries() {
    use chrono::{Duration, Utc};
    use maplestory_openapi::error::MapleStoryError;
    use maplestory_openapi::MapleStoryClient;

    // Minimum boundary: 2023-12-21 is the first allowed date
    assert_eq!(
        MapleStoryClient::validate_date("2023-12-21").unwrap(),
        "2023-12-21"
    );

    // One day before MIN_DATE must fail with DateOutOfRange
    let err_before_min = MapleStoryClient::validate_date("2023-12-20");
    assert!(matches!(
        err_before_min,
        Err(MapleStoryError::DateOutOfRange { .. })
    ));

    // Today's KST date must fail because daily snapshots only exist up to latest_available()
    let kst_today = (Utc::now() + Duration::hours(9))
        .date_naive()
        .format("%Y-%m-%d")
        .to_string();
    let err_today = MapleStoryClient::validate_date(&kst_today);
    assert!(matches!(
        err_today,
        Err(MapleStoryError::DateOutOfRange { .. })
    ));

    // Far future date must fail with DateOutOfRange
    let err_future = MapleStoryClient::validate_date("2099-01-01");
    assert!(matches!(
        err_future,
        Err(MapleStoryError::DateOutOfRange { .. })
    ));

    // Malformed date strings must fail with InvalidDate
    let err_invalid = MapleStoryClient::validate_date("not-a-date");
    assert!(matches!(
        err_invalid,
        Err(MapleStoryError::InvalidDate { .. })
    ));
}

#[test]
fn test_equipment_potential_parser_and_total_stat() {
    use maplestory_openapi::models::character::{
        ItemEquipmentDetail, ParsedPotentialOption, StatType,
    };

    // 1. Basic parser tests
    let opt_str = ParsedPotentialOption::parse("STR : +12%").unwrap();
    assert_eq!(opt_str.stat_type, StatType::Str);
    assert_eq!(opt_str.value, 12.0);
    assert!(opt_str.is_percentage);

    let opt_all = ParsedPotentialOption::parse("올스탯 : +9%").unwrap();
    assert_eq!(opt_all.stat_type, StatType::AllStat);
    assert_eq!(opt_all.value, 9.0);
    assert!(opt_all.is_percentage);

    let opt_att = ParsedPotentialOption::parse("공격력 : +30").unwrap();
    assert_eq!(opt_att.stat_type, StatType::AttackPower);
    assert_eq!(opt_att.value, 30.0);
    assert!(!opt_att.is_percentage);

    let opt_boss = ParsedPotentialOption::parse("보스 몬스터 공격 시 데미지 : +40%").unwrap();
    assert_eq!(opt_boss.stat_type, StatType::BossDamage);
    assert_eq!(opt_boss.value, 40.0);
    assert!(opt_boss.is_percentage);

    let opt_ied = ParsedPotentialOption::parse("몬스터 방어율 무시 : +30%").unwrap();
    assert_eq!(opt_ied.stat_type, StatType::IgnoreMonsterArmor);
    assert_eq!(opt_ied.value, 30.0);
    assert!(opt_ied.is_percentage);

    let opt_crit_dmg = ParsedPotentialOption::parse("크리티컬 데미지 : +8%").unwrap();
    assert_eq!(opt_crit_dmg.stat_type, StatType::CriticalDamage);
    assert_eq!(opt_crit_dmg.value, 8.0);
    assert!(opt_crit_dmg.is_percentage);

    let opt_cd = ParsedPotentialOption::parse(
        "모든 스킬의 재사용 대기시간 : -2초(10초 이하는 10%감소, 5초 미만으로 감소 불가)",
    )
    .unwrap();
    assert_eq!(opt_cd.stat_type, StatType::CooldownReduction);
    assert_eq!(opt_cd.value, 2.0);
    assert!(!opt_cd.is_percentage);

    // 2. Non-stat lines or malformed lines should return None
    assert!(ParsedPotentialOption::parse("피격 시 일정 확률로 데미지 무시").is_none());
    assert!(ParsedPotentialOption::parse("쓸만한 윈드 부스터 스킬 사용 가능").is_none());
    assert!(ParsedPotentialOption::parse("").is_none());
    assert!(ParsedPotentialOption::parse("just random text").is_none());

    // 3. Item-level parsed helpers and total_potential_stat aggregation
    let detail = ItemEquipmentDetail {
        item_equipment_part: Some("모자".to_string()),
        item_equipment_slot: Some("모자".to_string()),
        item_name: Some("에테르넬 나이트헬름".to_string()),
        item_icon: None,
        item_description: None,
        item_shape_name: None,
        item_shape_icon: None,
        item_gender: None,
        item_total_option: None,
        item_base_option: None,
        potential_option_grade: Some("레전드리".to_string()),
        additional_potential_option_grade: Some("에픽".to_string()),
        potential_option_flag: None,
        additional_potential_option_flag: None,
        potential_option_1: Some("STR : +12%".to_string()),
        potential_option_2: Some("STR : +9%".to_string()),
        potential_option_3: Some("최대 HP : +9%".to_string()),
        additional_potential_option_1: Some("STR : +14".to_string()),
        additional_potential_option_2: Some("공격력 : +10".to_string()),
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
        ..Default::default()
    };

    assert_eq!(
        detail.potential_option_1_parsed().unwrap().stat_type,
        StatType::Str
    );
    assert_eq!(
        detail.potential_option_2_parsed().unwrap().stat_type,
        StatType::Str
    );
    assert_eq!(
        detail.potential_option_3_parsed().unwrap().stat_type,
        StatType::MaxHp
    );
    assert_eq!(
        detail
            .additional_potential_option_1_parsed()
            .unwrap()
            .stat_type,
        StatType::Str
    );
    assert_eq!(
        detail
            .additional_potential_option_2_parsed()
            .unwrap()
            .stat_type,
        StatType::AttackPower
    );
    assert!(detail.additional_potential_option_3_parsed().is_none());

    // Aggregation: (flat, percent)
    // STR: additional_1 (14 flat), potential_1 (12%) + potential_2 (9%) = 21%
    let (flat_str, pct_str) = detail.total_potential_stat(StatType::Str);
    assert_eq!(flat_str, 14.0);
    assert_eq!(pct_str, 21.0);

    // Attack Power: additional_2 (10 flat), 0%
    let (flat_att, pct_att) = detail.total_potential_stat(StatType::AttackPower);
    assert_eq!(flat_att, 10.0);
    assert_eq!(pct_att, 0.0);

    // DEX: none
    let (flat_dex, pct_dex) = detail.total_potential_stat(StatType::Dex);
    assert_eq!(flat_dex, 0.0);
    assert_eq!(pct_dex, 0.0);
}

#[test]
fn test_union_artifact_helpers() {
    use maplestory_openapi::models::character::StatType;
    use maplestory_openapi::models::union::{
        UnionArtifactCrystal, UnionArtifactEffect, UserUnionArtifact,
    };

    let artifact = UserUnionArtifact {
        date: Some("2024-05-01".to_string()),
        union_artifact_effect: Some(vec![
            UnionArtifactEffect {
                name: Some("올스탯 150 증가, 공격력/마력 30 증가".to_string()),
                level: Some(10),
            },
            UnionArtifactEffect {
                name: Some("보스 몬스터 공격 시 데미지 15.00% 증가".to_string()),
                level: Some(10),
            },
            UnionArtifactEffect {
                name: Some("크리티컬 데미지 2.40% 증가".to_string()),
                level: Some(6),
            },
            UnionArtifactEffect {
                name: None,
                level: None,
            },
        ]),
        union_artifact_crystal: Some(vec![
            UnionArtifactCrystal {
                name: Some("크리스탈 : 주황".to_string()),
                validity_flag: Some("0".to_string()), // Active
                date_expire: Some("2099-01-01T00:00:00+09:00".to_string()),
                level: Some(5),
                crystal_option_name_1: Some("올스탯 증가".to_string()),
                crystal_option_name_2: None,
                crystal_option_name_3: None,
            },
            UnionArtifactCrystal {
                name: Some("크리스탈 : 파랑".to_string()),
                validity_flag: Some("1".to_string()), // Expired
                date_expire: Some("2023-01-01T00:00:00+09:00".to_string()),
                level: Some(5),
                crystal_option_name_1: Some("보스 몬스터 공격 시 데미지 증가".to_string()),
                crystal_option_name_2: None,
                crystal_option_name_3: None,
            },
            UnionArtifactCrystal {
                name: Some("크리스탈 : 보라".to_string()),
                validity_flag: Some("0".to_string()),
                date_expire: Some("2020-01-01T00:00:00+09:00".to_string()), // Expired timestamp
                level: Some(5),
                crystal_option_name_1: Some("크리티컬 데미지 증가".to_string()),
                crystal_option_name_2: None,
                crystal_option_name_3: None,
            },
        ]),
        union_artifact_remain_ap: Some(0),
    };

    // 1. active_effects should filter out empty/None effects
    let active_effects: Vec<_> = artifact.active_effects().collect();
    assert_eq!(active_effects.len(), 3);

    // 2. expired_crystals identifies both flag == "1" and past expiration dates
    let expired = artifact.expired_crystals();
    assert_eq!(expired.len(), 2);
    let expired_names: Vec<_> = expired.iter().map(|c| c.name.as_deref().unwrap()).collect();
    assert!(expired_names.contains(&"크리스탈 : 파랑"));
    assert!(expired_names.contains(&"크리스탈 : 보라"));

    // 3. total_artifact_stats and total_artifact_stats_typed aggregate numeric values
    let stats = artifact.total_artifact_stats();
    assert_eq!(stats.get("올스탯").copied(), Some(150.0));
    assert_eq!(stats.get("공격력").copied(), Some(30.0));
    assert_eq!(stats.get("마력").copied(), Some(30.0));
    assert_eq!(stats.get("보스 몬스터 공격 시 데미지").copied(), Some(15.0));
    assert_eq!(stats.get("크리티컬 데미지").copied(), Some(2.4));

    let typed_stats = artifact.total_artifact_stats_typed();
    assert_eq!(typed_stats.get(&StatType::AllStat).copied(), Some(150.0));
    assert_eq!(typed_stats.get(&StatType::AttackPower).copied(), Some(30.0));
    assert_eq!(typed_stats.get(&StatType::MagicPower).copied(), Some(30.0));
    assert_eq!(typed_stats.get(&StatType::BossDamage).copied(), Some(15.0));
    assert_eq!(
        typed_stats.get(&StatType::CriticalDamage).copied(),
        Some(2.4)
    );

    // 4. When all crystals are expired, is_active is false and active_effects is empty
    let fully_expired_artifact = UserUnionArtifact {
        date: Some("2024-05-01".to_string()),
        union_artifact_effect: artifact.union_artifact_effect.clone(),
        union_artifact_crystal: Some(vec![UnionArtifactCrystal {
            name: Some("크리스탈 : 파랑".to_string()),
            validity_flag: Some("1".to_string()),
            date_expire: Some("2023-01-01T00:00:00+09:00".to_string()),
            level: Some(5),
            crystal_option_name_1: None,
            crystal_option_name_2: None,
            crystal_option_name_3: None,
        }]),
        union_artifact_remain_ap: Some(0),
    };
    assert!(!fully_expired_artifact.is_active());
    assert_eq!(fully_expired_artifact.active_effects().count(), 0);
    assert!(fully_expired_artifact.total_artifact_stats().is_empty());
}

#[test]
fn test_set_effect_helpers() {
    use maplestory_openapi::models::character::{
        CharacterSetEffect, SetEffectInfo, SetEffectOption,
    };

    let set_effects = CharacterSetEffect {
        date: Some("2024-05-01".to_string()),
        set_effect: Some(vec![
            SetEffectInfo {
                set_name: Some("루타비스 세트 (도적)".to_string()),
                total_set_count: Some(4),
                set_effect_info: Some(vec![
                    SetEffectOption {
                        set_count: Some(2),
                        set_option: Some("올스탯 +20, 최대 HP/최대 MP +1000".to_string()),
                    },
                    SetEffectOption {
                        set_count: Some(3),
                        set_option: Some("공격력 +50, 최대 HP +10%".to_string()),
                    },
                    SetEffectOption {
                        set_count: Some(4),
                        set_option: Some("보스 몬스터 공격 시 데미지 +30%".to_string()),
                    },
                    SetEffectOption {
                        set_count: Some(5), // Inactive tier: player only has 4 pieces
                        set_option: Some("크리티컬 데미지 +10%".to_string()),
                    },
                ]),
                set_option_full: None,
            },
            SetEffectInfo {
                set_name: Some("에테르넬 세트 (도적)".to_string()),
                total_set_count: Some(2),
                set_effect_info: Some(vec![
                    SetEffectOption {
                        set_count: Some(2),
                        set_option: Some("올스탯 +40, 공격력 +40".to_string()),
                    },
                    SetEffectOption {
                        set_count: Some(3), // Inactive tier: player only has 2 pieces
                        set_option: Some("공격력 +40, 최대 HP +2000".to_string()),
                    },
                ]),
                set_option_full: None,
            },
        ]),
    };

    // 1. is_set_active checks
    assert!(set_effects.is_set_active("루타비스", 4));
    assert!(set_effects.is_set_active("루타비스", 3));
    assert!(!set_effects.is_set_active("루타비스", 5));
    assert!(set_effects.is_set_active("에테르넬", 2));
    assert!(!set_effects.is_set_active("에테르넬", 3));
    assert!(!set_effects.is_set_active("칠흑", 1));

    // 2. active_set_options strictly filters out inactive tiers where set_count > total_set_count
    let active_options = set_effects.active_set_options();
    assert_eq!(active_options.len(), 4); // 3 active from Root Abyss (2,3,4) + 1 active from Ethernal (2)
    assert!(!active_options.iter().any(|opt| opt.set_count == Some(5)));
    assert!(!active_options.iter().any(|opt| opt
        .set_option
        .as_deref()
        .unwrap_or("")
        .contains("크리티컬 데미지")));

    // 3. find_set
    let ethernel = set_effects.find_set("에테르넬").unwrap();
    assert_eq!(ethernel.total_set_count, Some(2));
}

#[tokio::test]
async fn test_identity_cache_custom_and_guild_caching() {
    use maplestory_openapi::client::{BoxFuture, IdentityCache};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    // Custom in-memory cache to verify pluggability
    struct MockCache {
        ocid_hits: Arc<AtomicUsize>,
        oguild_hits: Arc<AtomicUsize>,
        ocid_map: tokio::sync::RwLock<std::collections::HashMap<String, String>>,
        oguild_map: tokio::sync::RwLock<std::collections::HashMap<(String, String), String>>,
    }

    impl IdentityCache for MockCache {
        fn get_ocid<'a>(&'a self, name: &'a str) -> BoxFuture<'a, Option<String>> {
            Box::pin(async move {
                let map = self.ocid_map.read().await;
                if let Some(val) = map.get(name) {
                    self.ocid_hits.fetch_add(1, Ordering::SeqCst);
                    Some(val.clone())
                } else {
                    None
                }
            })
        }

        fn set_ocid<'a>(&'a self, name: &'a str, ocid: &'a str) -> BoxFuture<'a, ()> {
            let name = name.to_string();
            let ocid = ocid.to_string();
            Box::pin(async move {
                let mut map = self.ocid_map.write().await;
                map.insert(name, ocid);
            })
        }

        fn get_oguild_id<'a>(
            &'a self,
            guild: &'a str,
            world: &'a str,
        ) -> BoxFuture<'a, Option<String>> {
            Box::pin(async move {
                let map = self.oguild_map.read().await;
                if let Some(val) = map.get(&(guild.to_string(), world.to_string())) {
                    self.oguild_hits.fetch_add(1, Ordering::SeqCst);
                    Some(val.clone())
                } else {
                    None
                }
            })
        }

        fn set_oguild_id<'a>(
            &'a self,
            guild: &'a str,
            world: &'a str,
            oguild_id: &'a str,
        ) -> BoxFuture<'a, ()> {
            let key = (guild.to_string(), world.to_string());
            let val = oguild_id.to_string();
            Box::pin(async move {
                let mut map = self.oguild_map.write().await;
                map.insert(key, val);
            })
        }
    }

    let ocid_hits = Arc::new(AtomicUsize::new(0));
    let oguild_hits = Arc::new(AtomicUsize::new(0));
    let custom_cache = Arc::new(MockCache {
        ocid_hits: ocid_hits.clone(),
        oguild_hits: oguild_hits.clone(),
        ocid_map: tokio::sync::RwLock::new(std::collections::HashMap::new()),
        oguild_map: tokio::sync::RwLock::new(std::collections::HashMap::new()),
    });

    let req_counter = Arc::new(AtomicUsize::new(0));
    let req_counter_clone = req_counter.clone();

    let (base_url, server_task) = spawn_mock_server(move |req_text| {
        req_counter_clone.fetch_add(1, Ordering::SeqCst);
        let body = if req_text.contains("/maplestory/v1/id") {
            r#"{"ocid":"test-ocid-cached"}"#.to_string()
        } else if req_text.contains("/maplestory/v1/guild/id") {
            r#"{"oguild_id":"test-oguild-cached"}"#.to_string()
        } else {
            r#"{}"#.to_string()
        };
        (200, body, None)
    })
    .await;

    let client = maplestory_openapi::MapleStoryClient::builder()
        .api_key("test-key")
        .base_url(&base_url)
        .identity_cache(custom_cache)
        .build()
        .unwrap();

    // 1. First OCID lookup hits HTTP
    let ocid_1 = client.get_ocid("hero").await.unwrap();
    assert_eq!(ocid_1, "test-ocid-cached");
    assert_eq!(req_counter.load(Ordering::SeqCst), 1);
    assert_eq!(ocid_hits.load(Ordering::SeqCst), 0);

    // 2. Second OCID lookup hits cache (no HTTP request!)
    let ocid_2 = client.get_ocid("hero").await.unwrap();
    assert_eq!(ocid_2, "test-ocid-cached");
    assert_eq!(req_counter.load(Ordering::SeqCst), 1);
    assert_eq!(ocid_hits.load(Ordering::SeqCst), 1);

    // 3. First OGuildID lookup hits HTTP
    let oguild_1 = client.get_oguild_id("testguild", "scania").await.unwrap();
    assert_eq!(oguild_1, "test-oguild-cached");
    assert_eq!(req_counter.load(Ordering::SeqCst), 2);
    assert_eq!(oguild_hits.load(Ordering::SeqCst), 0);

    // 4. Second OGuildID lookup hits cache (no HTTP request!)
    let oguild_2 = client.get_oguild_id("testguild", "scania").await.unwrap();
    assert_eq!(oguild_2, "test-oguild-cached");
    assert_eq!(req_counter.load(Ordering::SeqCst), 2);
    assert_eq!(oguild_hits.load(Ordering::SeqCst), 1);

    server_task.abort();
}

#[tokio::test]
async fn test_hierarchical_domain_namespaces() {
    use maplestory_openapi::MapleStoryClient;

    let (base_url, server_task) = spawn_mock_server(|req_text| {
        let body = if req_text.contains("/maplestory/v1/character/basic") {
            r#"{"character_name":"HeroMock","character_level":285}"#.to_string()
        } else if req_text.contains("/maplestory/v1/guild/basic") {
            r#"{"guild_name":"GuildMock","guild_level":30}"#.to_string()
        } else if req_text.contains("/maplestory/v1/user/union") {
            r#"{"union_level":8500,"union_grade":"그랜드 마스터 유니온"}"#.to_string()
        } else {
            r#"{}"#.to_string()
        };
        (200, body, None)
    })
    .await;

    let client = MapleStoryClient::builder()
        .api_key("test-key")
        .base_url(base_url)
        .build()
        .unwrap();

    // 1. Verify sub-client accessors exist and can be instantiated
    let char_client = client.character();
    let guild_client = client.guild();
    let union_client = client.union();
    let _ranking_client = client.ranking();
    let _history_client = client.history();
    let _notice_client = client.notice();
    let _user_client = client.user();
    let _bp_client = client.battle_practice();
    let _scheduler_client = client.scheduler();

    // Verify Copy & Clone
    let char_copy = char_client;

    // 2. Query through namespaced sub-client with flexible date param
    let basic_from_sub = char_copy.get_basic("mock-ocid", None).await.unwrap();
    assert_eq!(basic_from_sub.character_name.as_deref(), Some("HeroMock"));
    assert_eq!(basic_from_sub.character_level, Some(285));

    // 3. Query through root convenience facade
    let basic_from_root = client
        .get_character_basic("mock-ocid", "2024-05-01")
        .await
        .unwrap();
    assert_eq!(basic_from_root.character_name.as_deref(), Some("HeroMock"));

    // 4. Query guild through sub-client and root facade
    let guild_from_sub = guild_client.get_basic("mock-oguild", None).await.unwrap();
    assert_eq!(guild_from_sub.guild_name.as_deref(), Some("GuildMock"));
    assert_eq!(guild_from_sub.guild_level, Some(30));

    let guild_from_root = client.get_guild_basic("mock-oguild", None).await.unwrap();
    assert_eq!(guild_from_root.guild_name.as_deref(), Some("GuildMock"));

    // 5. Query union through sub-client and root facade
    let union_from_sub = union_client.get_union("mock-ocid", None).await.unwrap();
    assert_eq!(union_from_sub.union_level, Some(8500));

    let union_from_root = client.get_user_union("mock-ocid", None).await.unwrap();
    assert_eq!(union_from_root.union_level, Some(8500));

    server_task.abort();
}

#[tokio::test]
async fn test_history_auto_paginating_stream() {
    use futures_util::StreamExt;
    use maplestory_openapi::MapleStoryClient;

    let (base_url, server_task) = spawn_mock_server(|req_text| {
        if req_text.contains("/maplestory/v1/history/starforce") {
            if req_text.contains("cursor=page2_cursor") {
                let body = r#"{
                    "count": 1,
                    "next_cursor": null,
                    "starforce_history": [
                        {"id": "sf_item_2", "item_upgrade_result": "성공", "before_starforce_count": 15, "after_starforce_count": 16}
                    ]
                }"#.to_string();
                (200, body, None)
            } else {
                let body = r#"{
                    "count": 1,
                    "next_cursor": "page2_cursor",
                    "starforce_history": [
                        {"id": "sf_item_1", "item_upgrade_result": "성공", "before_starforce_count": 14, "after_starforce_count": 15}
                    ]
                }"#.to_string();
                (200, body, None)
            }
        } else {
            (200, r#"{}"#.to_string(), None)
        }
    })
    .await;

    let client = MapleStoryClient::builder()
        .api_key("test-key")
        .base_url(base_url)
        .build()
        .unwrap();

    let mut stream = client.history().get_starforce_history_stream(10, None);
    let mut collected = Vec::new();
    while let Some(item_res) = stream.next().await {
        let entry = item_res.unwrap();
        collected.push(entry.id.unwrap());
    }

    assert_eq!(collected, vec!["sf_item_1", "sf_item_2"]);
    server_task.abort();
}

#[tokio::test]
async fn test_guild_profile_and_account_summary_aggregators() {
    use maplestory_openapi::MapleStoryClient;

    let (base_url, server_task) = spawn_mock_server(|req_text| {
        if req_text.contains("/maplestory/v1/guild/id") {
            (200, r#"{"oguild_id":"mock-oguild"}"#.to_string(), None)
        } else if req_text.contains("/maplestory/v1/guild/basic") {
            let body = r#"{
                "guild_name": "테스트길드",
                "world_name": "스카니아",
                "guild_level": 25,
                "guild_member": ["길드원1", "길드원2"],
                "guild_noblesse_skill": [
                    {"skill_name": "보스 킬러", "skill_level": 15},
                    {"skill_name": "방어율 무시", "skill_level": 15}
                ]
            }"#;
            (200, body.to_string(), None)
        } else if req_text.contains("/maplestory/v1/character/list") {
            let body = r#"{
                "account_list": [
                    {
                        "account_id": "acc-1",
                        "character_list": [
                            {"ocid": "ocid-hero1", "character_name": "영웅1", "character_level": 285},
                            {"ocid": "ocid-sub1", "character_name": "부캐1", "character_level": 230}
                        ]
                    }
                ]
            }"#;
            (200, body.to_string(), None)
        } else if req_text.contains("/maplestory/v1/id") {
            if req_text.contains("character_name=%EA%B8%B8%EB%93%9C%EC%9B%901") || req_text.contains("character_name=길드원1") {
                (200, r#"{"ocid":"ocid-1"}"#.to_string(), None)
            } else {
                (200, r#"{"ocid":"ocid-2"}"#.to_string(), None)
            }
        } else if req_text.contains("/maplestory/v1/character/basic") {
            if req_text.contains("ocid=ocid-1") {
                (200, r#"{"character_name":"길드원1","character_level":280}"#.to_string(), None)
            } else {
                (200, r#"{"character_name":"길드원2","character_level":270}"#.to_string(), None)
            }
        } else if req_text.contains("/maplestory/v1/character/stat") {
            if req_text.contains("ocid=ocid-1") {
                (200, r#"{"final_stat":[{"stat_name":"전투력","stat_value":"200000000"}]}"#.to_string(), None)
            } else {
                (200, r#"{"final_stat":[{"stat_name":"전투력","stat_value":"100000000"}]}"#.to_string(), None)
            }
        } else if req_text.contains("/maplestory/v1/user/achievement") {
            (200, r#"{"achievement_point": 12500}"#.to_string(), None)
        } else if req_text.contains("/maplestory/v1/user/union") {
            (200, r#"{"union_level": 9000, "union_grade": "그랜드 마스터 유니온 3"}"#.to_string(), None)
        } else {
            (200, r#"{}"#.to_string(), None)
        }
    })
    .await;

    let client = MapleStoryClient::builder()
        .api_key("test-key")
        .base_url(base_url)
        .build()
        .unwrap();

    // 1. GuildProfile testing
    let guild_profile = client
        .guild()
        .get_profile_with_members("테스트길드", "스카니아", None, true)
        .await
        .unwrap();

    assert_eq!(guild_profile.total_noblesse_points, 30);
    assert_eq!(guild_profile.noblesse_skills.len(), 2);
    assert_eq!(guild_profile.fetched_member_count(), 2);
    assert_eq!(guild_profile.average_member_level(), Some(275.0));
    assert_eq!(guild_profile.total_member_combat_power(), 300_000_000);
    assert_eq!(
        guild_profile.average_member_combat_power(),
        Some(150_000_000.0)
    );
    assert_eq!(guild_profile.max_member_combat_power(), Some(200_000_000));

    // 2. AccountSummary testing
    let account_summary = client.user().get_account_summary(None).await.unwrap();
    assert_eq!(account_summary.total_character_count(), 2);
    assert_eq!(
        account_summary
            .highest_level_character()
            .and_then(|c| c.character_name.as_deref()),
        Some("영웅1")
    );
    assert_eq!(account_summary.achievement_points(), Some(12500));
    assert_eq!(account_summary.main_character_union_level(), Some(9000));
    assert_eq!(
        account_summary.main_character_union_grade(),
        Some("그랜드 마스터 유니온 3")
    );

    server_task.abort();
}

#[test]
fn test_union_raider_and_champion_helpers() {
    use maplestory_openapi::models::union::{
        UnionBlock, UnionChampionBadgeInfo, UnionChampionInfo, UnionRaiderPreset,
        UserUnionChampion, UserUnionRaider,
    };
    use maplestory_openapi::StatType;

    let preset_2 = UnionRaiderPreset {
        union_raider_stat: Some(vec![
            "STR 80 증가".to_string(),
            "STR 40 증가".to_string(),
            "크리티컬 확률 5% 증가".to_string(),
        ]),
        union_occupied_stat: Some(vec![
            "보스 몬스터 공격 시 데미지 40% 증가".to_string(),
            "크리티컬 데미지 20.00% 증가".to_string(),
        ]),
        union_inner_stat: None,
        union_block: None,
    };

    let raider = UserUnionRaider {
        date: None,
        union_raider_stat: Some(vec![
            "INT 100 증가".to_string(),
            "INT 80 증가".to_string(),
            "공격력/마력 20 증가".to_string(),
        ]),
        union_occupied_stat: Some(vec![
            "보스 몬스터 공격 시 데미지 35% 증가".to_string(),
            "방어율 무시 40% 증가".to_string(),
        ]),
        union_block: Some(vec![
            UnionBlock {
                block_type: Some("마법사".to_string()),
                block_class: Some("배틀메이지".to_string()),
                block_level: Some("295".to_string()),
                block_control_point: None,
                block_position: None,
            },
            UnionBlock {
                block_type: Some("해적".to_string()),
                block_class: Some("메카닉".to_string()),
                block_level: Some("260".to_string()),
                block_control_point: None,
                block_position: None,
            },
        ]),
        union_inner_stat: None,
        use_preset_no: Some(2),
        union_raider_preset_1: None,
        union_raider_preset_2: Some(preset_2),
        union_raider_preset_3: None,
        union_raider_preset_4: None,
        union_raider_preset_5: None,
        union_state_stat: None,
        union_state_stat_preset: None,
        union_max_point: Some(100),
    };

    let occupied = raider.total_occupied_stats();
    assert_eq!(occupied.get("보스 몬스터 공격 시 데미지"), Some(&35.0));
    assert_eq!(occupied.get("방어율 무시"), Some(&40.0));

    let occupied_typed = raider.total_occupied_stats_typed();
    assert_eq!(occupied_typed.get(&StatType::BossDamage), Some(&35.0));
    assert_eq!(
        occupied_typed.get(&StatType::IgnoreMonsterArmor),
        Some(&40.0)
    );

    let raider_stats = raider.total_raider_stats();
    assert_eq!(raider_stats.get("INT"), Some(&180.0));
    assert_eq!(raider_stats.get("공격력"), Some(&20.0));
    assert_eq!(raider_stats.get("마력"), Some(&20.0));

    let active_p = raider
        .active_preset(None)
        .expect("preset 2 should be active");
    assert_eq!(active_p.total_raider_stats().get("STR"), Some(&120.0));
    assert_eq!(
        active_p.total_occupied_stats().get("크리티컬 데미지"),
        Some(&20.0)
    );
    assert_eq!(raider.blocks_by_class("배틀메이지").len(), 1);

    let champion = UserUnionChampion {
        date: None,
        union_champion: Some(vec![
            UnionChampionInfo {
                champion_name: Some("메인챔피언".to_string()),
                champion_slot: Some(1),
                champion_grade: Some("SSS".to_string()),
                champion_class: Some("배틀메이지".to_string()),
                champion_badge_info: Some(vec![UnionChampionBadgeInfo {
                    stat: Some("올스탯 100 증가".to_string()),
                }]),
            },
            UnionChampionInfo {
                champion_name: Some("서브챔피언".to_string()),
                champion_slot: Some(2),
                champion_grade: Some("SS".to_string()),
                champion_class: Some("비숍".to_string()),
                champion_badge_info: Some(vec![UnionChampionBadgeInfo {
                    stat: Some("공격력/마력 50 증가".to_string()),
                }]),
            },
        ]),
        champion_badge_total_info: Some(vec![
            UnionChampionBadgeInfo {
                stat: Some("올스탯 100 증가".to_string()),
            },
            UnionChampionBadgeInfo {
                stat: Some("공격력/마력 50 증가".to_string()),
            },
            UnionChampionBadgeInfo {
                stat: Some("보스 몬스터 공격 시 데미지 10% 증가".to_string()),
            },
        ]),
    };

    assert_eq!(champion.champion_count(), 2);
    assert_eq!(
        champion
            .find_champion_by_slot(1)
            .and_then(|c| c.champion_name.as_deref()),
        Some("메인챔피언")
    );
    assert!(champion.find_champion_by_slot(3).is_none());

    let badge_stats = champion.total_badge_stats();
    assert_eq!(badge_stats.get("올스탯"), Some(&100.0));
    assert_eq!(badge_stats.get("공격력"), Some(&50.0));
    assert_eq!(badge_stats.get("마력"), Some(&50.0));
    assert_eq!(badge_stats.get("보스 몬스터 공격 시 데미지"), Some(&10.0));

    let badge_typed = champion.total_badge_stats_typed();
    assert_eq!(badge_typed.get(&StatType::AllStat), Some(&100.0));
    assert_eq!(badge_typed.get(&StatType::BossDamage), Some(&10.0));
}

#[test]
#[allow(deprecated)]
fn test_character_progression_domain_helpers() {
    use maplestory_openapi::models::character::{
        AbilityInfo, AbilityPreset, CharacterAbility, CharacterHexaMatrix, CharacterHexaMatrixStat,
        CharacterHyperStat, CharacterVMatrix, HexaCoreEquipment, HexaStatCore, HyperStatEntry,
        VCoreEquipment,
    };

    // 1. HEXA Matrix
    let hexa = CharacterHexaMatrix {
        date: None,
        character_hexa_core_equipment: Some(vec![
            HexaCoreEquipment {
                hexa_core_name: Some("마스터리 코어 I".to_string()),
                hexa_core_level: Some(30),
                hexa_core_event_level: None,
                hexa_core_type: Some("마스터리 코어".to_string()),
                linked_skill: None,
            },
            HexaCoreEquipment {
                hexa_core_name: Some("오리진 스킬".to_string()),
                hexa_core_level: Some(20),
                hexa_core_event_level: None,
                hexa_core_type: Some("스킬 코어".to_string()),
                linked_skill: None,
            },
            HexaCoreEquipment {
                hexa_core_name: Some("미개방 코어".to_string()),
                hexa_core_level: Some(0),
                hexa_core_event_level: None,
                hexa_core_type: Some("강화 코어".to_string()),
                linked_skill: None,
            },
        ]),
    };
    assert_eq!(hexa.active_cores().len(), 2);
    assert_eq!(hexa.core_level("마스터리 코어 I"), Some(30));
    assert_eq!(hexa.cores_by_type("스킬 코어").len(), 1);

    // 2. HEXA Matrix Stat
    let hexa_stat = CharacterHexaMatrixStat {
        date: None,
        character_class: Some("배틀메이지".to_string()),
        character_hexa_stat_core: Some(vec![HexaStatCore {
            slot_id: Some("0".to_string()),
            main_stat_name: Some("크리티컬 데미지 증가".to_string()),
            sub_stat_name_1: Some("마력 증가".to_string()),
            sub_stat_name_2: Some("보스 데미지 증가".to_string()),
            main_stat_level: Some(8),
            sub_stat_level_1: Some(7),
            sub_stat_level_2: Some(5),
            stat_grade: Some(20),
        }]),
        character_hexa_stat_core_2: Some(vec![HexaStatCore {
            slot_id: Some("1".to_string()),
            main_stat_name: Some("보스 데미지 증가".to_string()),
            sub_stat_name_1: Some("크리티컬 데미지 증가".to_string()),
            sub_stat_name_2: Some("마력 증가".to_string()),
            main_stat_level: Some(9),
            sub_stat_level_1: Some(6),
            sub_stat_level_2: Some(5),
            stat_grade: Some(20),
        }]),
        character_hexa_stat_core_3: None,
        preset_hexa_stat_core: None,
        preset_hexa_stat_core_2: None,
        preset_hexa_stat_core_3: None,
    };
    assert_eq!(hexa_stat.active_stat_cores().len(), 2);
    assert_eq!(hexa_stat.main_stat_level(1), Some(8));
    assert_eq!(hexa_stat.additional_stat_levels(1), Some((7, 5)));
    assert_eq!(hexa_stat.main_stat_level(2), Some(9));
    assert_eq!(hexa_stat.main_stat_level(3), None);

    // 3. V-Matrix
    let vmatrix = CharacterVMatrix {
        date: None,
        character_class: Some("배틀메이지".to_string()),
        character_v_core_equipment: Some(vec![
            VCoreEquipment {
                slot_id: Some("0".to_string()),
                slot_level: Some(5),
                v_core_name: Some("유니온 오라".to_string()),
                v_core_type: Some("스킬코어".to_string()),
                v_core_level: Some(25),
                v_core_skill_1: None,
                v_core_skill_2: None,
                v_core_skill_3: None,
            },
            VCoreEquipment {
                slot_id: Some("1".to_string()),
                slot_level: Some(5),
                v_core_name: Some("블랙 매직 알터".to_string()),
                v_core_type: Some("스킬코어".to_string()),
                v_core_level: Some(20),
                v_core_skill_1: None,
                v_core_skill_2: None,
                v_core_skill_3: None,
            },
        ]),
        character_v_matrix_remain_slot_upgrade_point: Some(0),
        character_v_core_equipment_preset_1: None,
        character_v_core_equipment_preset_2: None,
        character_v_core_equipment_preset_3: None,
        character_v_core_equipment_preset_4: None,
        character_v_core_equipment_preset_5: None,
    };
    assert_eq!(vmatrix.total_slot_enhancement_level(), 10);
    assert_eq!(
        vmatrix
            .core_by_name("유니온 오라")
            .and_then(|c| c.v_core_level),
        Some(25)
    );
    assert_eq!(vmatrix.max_level_cores().len(), 1);

    // 4. Ability
    let ability = CharacterAbility {
        date: None,
        ability_grade: Some("레전드리".to_string()),
        ability_info: Some(vec![AbilityInfo {
            ability_no: Some("1".to_string()),
            ability_grade: Some("레전드리".to_string()),
            ability_value: Some("보스 몬스터 공격 시 데미지 20% 증가".to_string()),
        }]),
        remain_fame: Some(100_000),
        ability_preset_1: None,
        ability_preset_2: Some(AbilityPreset {
            ability_preset_grade: Some("레전드리".to_string()),
            ability_info: Some(vec![AbilityInfo {
                ability_no: Some("1".to_string()),
                ability_grade: Some("레전드리".to_string()),
                ability_value: Some("아이템 드롭률 20% 증가".to_string()),
            }]),
        }),
        ability_preset_3: None,
    };
    assert_eq!(
        ability.active_preset_abilities(None)[0]
            .ability_value
            .as_deref(),
        Some("보스 몬스터 공격 시 데미지 20% 증가")
    );
    assert_eq!(
        ability.active_preset_abilities(Some(2))[0]
            .ability_value
            .as_deref(),
        Some("아이템 드롭률 20% 증가")
    );

    // 5. Hyper Stat
    let hyper_stat = CharacterHyperStat {
        date: None,
        character_class: Some("배틀메이지".to_string()),
        use_preset_no: Some("2".to_string()),
        use_available_hyper_stat: Some(0),
        hyper_stat_preset_1: Some(vec![]),
        hyper_stat_preset_1_remain_point: Some(0),
        hyper_stat_preset_2: Some(vec![HyperStatEntry {
            stat_type: Some("보스 몬스터 공격 시 데미지 증가".to_string()),
            stat_point: Some(250),
            stat_level: Some(15),
            stat_increase: Some("보스 몬스터 공격 시 데미지 51% 증가".to_string()),
        }]),
        hyper_stat_preset_2_remain_point: Some(0),
        hyper_stat_preset_3: None,
        hyper_stat_preset_3_remain_point: None,
    };
    assert_eq!(hyper_stat.active_preset_hyper_stats(None).len(), 1);
    assert_eq!(
        hyper_stat
            .find_hyper_stat("보스 몬스터 공격 시 데미지 증가")
            .and_then(|e| e.stat_level),
        Some(15)
    );
}
