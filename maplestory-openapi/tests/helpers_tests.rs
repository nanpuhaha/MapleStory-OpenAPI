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

#[test]
fn test_equip_layer_resolver_direct_and_custom_rules() {
    use maplestory_openapi::models::character::{
        CashItemEquipmentDetail, CharacterBeautyEquipment, CharacterCashItemEquipment,
        CharacterItemEquipment, EquipLayerResolver, ItemEquipmentDetail,
    };

    let beauty = CharacterBeautyEquipment::default();
    let cash_equip = CharacterCashItemEquipment {
        preset_no: Some(1),
        cash_item_equipment_preset_1: Some(vec![CashItemEquipmentDetail {
            cash_item_equipment_part: Some("한벌옷".to_string()),
            cash_item_equipment_slot: Some("한벌옷".to_string()),
            cash_item_name: Some("인형의 꿈".to_string()),
            cash_item_icon: Some("https://example.com/dream.png".to_string()),
            cash_item_label: Some("마스터라벨".to_string()),
            ..Default::default()
        }]),
        ..Default::default()
    };
    let normal_equip = CharacterItemEquipment {
        item_equipment: Some(vec![
            ItemEquipmentDetail {
                item_equipment_part: Some("상의".to_string()),
                item_equipment_slot: Some("상의".to_string()),
                item_name: Some("이글아이 던위치로브".to_string()),
                ..Default::default()
            },
            ItemEquipmentDetail {
                item_equipment_part: Some("하의".to_string()),
                item_equipment_slot: Some("하의".to_string()),
                item_name: Some("트릭스터 던위치팬츠".to_string()),
                ..Default::default()
            },
        ]),
        ..Default::default()
    };

    let resolved = EquipLayerResolver::resolve(&beauty, &cash_equip, &normal_equip);
    assert_eq!(resolved.visible_items.len(), 1);
    let overall = &resolved.visible_items[0];
    assert_eq!(overall.slot, "한벌옷");
    assert_eq!(overall.name, "인형의 꿈");
    assert!(overall.is_cash);
    assert_eq!(overall.cash_label.as_deref(), Some("마스터라벨"));

    // 2. Anvil and Transparent items layering
    use maplestory_openapi::models::character::CharacterCoordination;
    let cash_with_transparent = CharacterCashItemEquipment {
        preset_no: Some(1),
        cash_item_equipment_preset_1: Some(vec![
            CashItemEquipmentDetail {
                cash_item_equipment_part: Some("눈장식".to_string()),
                cash_item_equipment_slot: Some("눈장식".to_string()),
                cash_item_name: Some("투명안경".to_string()),
                ..Default::default()
            },
        ]),
        ..Default::default()
    };
    let normal_with_anvil = CharacterItemEquipment {
        item_equipment: Some(vec![
            ItemEquipmentDetail {
                item_equipment_part: Some("모자".to_string()),
                item_equipment_slot: Some("모자".to_string()),
                item_name: Some("하이네스 던위치햇".to_string()),
                item_shape_name: Some("빨간 산타모자".to_string()),
                item_shape_icon: Some("https://example.com/santa.png".to_string()),
                ..Default::default()
            },
        ]),
        ..Default::default()
    };

    let coord_alias = CharacterCoordination::from_equipment_and_cash(
        &beauty,
        &cash_with_transparent,
        &normal_with_anvil,
    );

    let transparent_item = coord_alias.visible_items.iter().find(|i| i.slot == "눈장식").unwrap();
    assert!(transparent_item.is_transparent);
    assert_eq!(transparent_item.name, "투명안경");

    let anvil_item = coord_alias.visible_items.iter().find(|i| i.slot == "모자").unwrap();
    assert!(anvil_item.is_anvil);
    assert_eq!(anvil_item.name, "빨간 산타모자");
    assert_eq!(anvil_item.original_name.as_deref(), Some("하이네스 던위치햇"));
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

#[test]
fn test_pure_character_dashboard_assemble_and_partial_failures() {
    use maplestory_openapi::models::character::{
        CharacterBasic, CharacterDashboard, DashboardOptions, DashboardSection, RawDashboardSections,
    };
    use maplestory_openapi::MapleStoryError;

    let basic = CharacterBasic {
        character_name: Some("순수대시보드".to_string()),
        world_name: Some("베라".to_string()),
        character_class: Some("나이트로드".to_string()),
        character_level: Some(250),
        ..Default::default()
    };

    // Simulate 6th job not unlocked (HTTP 400 with OPENAPI00003) and union service unavailable (HTTP 503)
    let raw = RawDashboardSections {
        basic: Ok(basic),
        stat: None,
        item_equipment: None,
        cash_item_equipment: None,
        beauty_equipment: None,
        symbols: None,
        ability: None,
        hyper_stat: None,
        hexa_matrix: Some(Err(MapleStoryError::ApiError {
            status: 400,
            code: "OPENAPI00003".to_string(),
            message: "Hexa matrix not available for level < 260".to_string(),
        })),
        hexa_matrix_stat: None,
        v_matrix: None,
        set_effect: None,
        pet_equipment: None,
        android_equipment: None,
        link_skill: None,
        propensity: None,
        popularity: None,
        dojang: None,
        union: Some(Err(MapleStoryError::ServiceUnavailable {
            code: "OPENAPI00005".to_string(),
            message: "Service Unavailable".to_string(),
        })),
        union_artifact: None,
        guild: None,
    };

    let dashboard = CharacterDashboard::assemble(raw, &DashboardOptions::all())
        .expect("dashboard assembly must succeed with degraded sections");

    assert_eq!(dashboard.character_name(), "순수대시보드");
    assert_eq!(dashboard.character_level(), 250);
    assert!(dashboard.has_errors());

    // Verify HexaMatrix degradation error recorded
    let hexa_err = dashboard.section_error(DashboardSection::HexaMatrix).unwrap();
    assert_eq!(hexa_err.api_code.as_deref(), Some("OPENAPI00003"));
    assert_eq!(hexa_err.status_code, Some(400));
    assert!(dashboard.hexa_matrix.is_none());

    // Verify Union degradation error recorded
    let union_err = dashboard.section_error(DashboardSection::Union).unwrap();
    assert_eq!(union_err.status_code, Some(503));
    assert!(dashboard.union.is_none());
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
        UnionBlock, UnionChampionBadgeInfo, UnionChampionInfo, UnionInnerStat, UnionRaiderPreset,
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
        union_inner_stat: Some(vec![UnionInnerStat {
            stat_field_id: Some("0".to_string()),
            stat_field_effect: Some("공격력 15 증가".to_string()),
        }]),
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
        union_inner_stat: Some(vec![
            UnionInnerStat {
                stat_field_id: Some("0".to_string()),
                stat_field_effect: Some("보스 몬스터 공격 시 데미지 5% 증가".to_string()),
            },
            UnionInnerStat {
                stat_field_id: Some("1".to_string()),
                stat_field_effect: Some("마력 10 증가".to_string()),
            },
        ]),
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

    let inner_stats = raider.total_inner_stats();
    assert_eq!(inner_stats.get("보스 몬스터 공격 시 데미지"), Some(&5.0));
    assert_eq!(inner_stats.get("마력"), Some(&10.0));

    let inner_typed = raider.total_inner_stats_typed();
    assert_eq!(inner_typed.get(&StatType::BossDamage), Some(&5.0));
    assert_eq!(inner_typed.get(&StatType::MagicPower), Some(&10.0));

    let active_p = raider
        .active_preset(None)
        .expect("preset 2 should be active");
    assert_eq!(active_p.total_raider_stats().get("STR"), Some(&120.0));
    assert_eq!(
        active_p.total_occupied_stats().get("크리티컬 데미지"),
        Some(&20.0)
    );
    assert_eq!(active_p.total_inner_stats().get("공격력"), Some(&15.0));
    assert_eq!(
        active_p
            .total_inner_stats_typed()
            .get(&StatType::AttackPower),
        Some(&15.0)
    );
    assert_eq!(raider.blocks_by_class("배틀메이지").len(), 1);
    assert_eq!(raider.blocks_by_min_level(290).len(), 1);
    assert_eq!(raider.blocks_by_min_level(250).len(), 2);
    assert_eq!(raider.blocks_by_min_level(300).len(), 0);

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

    let grade_summary = champion.champion_grade_summary();
    assert_eq!(grade_summary.get("SSS"), Some(&1));
    assert_eq!(grade_summary.get("SS"), Some(&1));
    assert_eq!(champion.champions_by_grade("SSS").len(), 1);
    assert_eq!(champion.champions_by_grade("C").len(), 0);

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
    assert_eq!(hyper_stat.active_preset_hyper_stats(Some(2)).len(), 1);
    assert_eq!(hyper_stat.active_preset_hyper_stats(Some(1)).len(), 0);
    assert_eq!(hyper_stat.active_preset_hyper_stats(Some(99)).len(), 0);
    assert_eq!(
        hyper_stat
            .find_hyper_stat("보스 몬스터 공격 시 데미지 증가")
            .and_then(|e| e.stat_level),
        Some(15)
    );
}

#[test]
fn test_character_combat_analytics() {
    use maplestory_openapi::models::character::{
        CharacterBasic, CharacterCombatAnalytics, CharacterCoordination, CharacterDashboard,
        CharacterDojang, CharacterItemEquipment, CharacterStat, FinalStatEntry,
        StarforceStatBreakdown,
    };

    let analytics = CharacterCombatAnalytics {
        character_name: "테스터".to_string(),
        world_name: Some("스카니아".to_string()),
        character_class: Some("아델".to_string()),
        character_level: 285,
        combat_power: 450_000_000,
        boss_damage: 380.0,
        ignore_monster_armor: 95.0,
        damage: 60.0,
        critical_rate: 100.0,
        critical_damage: 85.0,
        total_starforce: 380,
        arcane_force: 1320,
        authentic_force: 660,
        dojang_best_floor: Some(72),
        starforce_stat_breakdown: None,
    };

    // 1. Effective boss damage multiplier calculations
    // 300% Boss: reduction = 3.0 * (1.0 - 0.95) = 0.15 -> 0.85 (85%)
    let mult_300 = analytics.effective_damage_multiplier_300();
    assert!((mult_300 - 0.85).abs() < 1e-6);

    // 380% Extreme Boss: reduction = 3.8 * 0.05 = 0.19 -> 0.81 (81%)
    let mult_380 = analytics.effective_damage_multiplier_380();
    assert!((mult_380 - 0.81).abs() < 1e-6);

    // Extreme case: 0% IED against 300% boss -> 0% damage
    let zero_ied = CharacterCombatAnalytics {
        ignore_monster_armor: 0.0,
        ..analytics.clone()
    };
    assert_eq!(zero_ied.effective_damage_multiplier_300(), 0.0);

    // 2. Damage factors
    // Total damage factor: 1.0 + (60 + 380) / 100 = 5.40
    assert!((analytics.total_damage_factor() - 5.40).abs() < 1e-6);

    // Critical damage factor: 1.35 + 0.85 = 2.20
    assert!((analytics.critical_damage_factor() - 2.20).abs() < 1e-6);

    // 3. Force stat bonus calculations
    // 1320 AF -> (1320 / 10) * 100 = 13,200 main stat
    assert_eq!(analytics.arcane_force_stat_bonus(), 13_200);
    // 660 Authentic Force -> (660 / 10) * 200 = 13,200 main stat
    assert_eq!(analytics.authentic_force_stat_bonus(), 13_200);

    // 4. Estimated Dojo floor
    // 450,000,000 CP -> Floor 70 tier (>= 300M benchmark)
    assert_eq!(analytics.estimated_dojang_floor(), 70);

    // Verify sub-benchmark and zero combat power edge cases
    let mut zero_analytics = analytics.clone();
    zero_analytics.combat_power = 0;
    assert_eq!(zero_analytics.estimated_dojang_floor(), 0);

    let mut low_analytics = analytics.clone();
    low_analytics.combat_power = 100_000;
    assert_eq!(low_analytics.estimated_dojang_floor(), 4);

    let mut endgame_analytics = analytics.clone();
    endgame_analytics.combat_power = 1_200_000_000;
    assert_eq!(endgame_analytics.estimated_dojang_floor(), 80);

    // 5. Starforce breakdown tests
    let mut sf_analytics = analytics.clone();
    sf_analytics.starforce_stat_breakdown = Some(StarforceStatBreakdown {
        total_starforce: 180,
        str: 350,
        dex: 280,
        int: 120,
        luk: 120,
        attack_power: 145,
        magic_power: 0,
        ..Default::default()
    });
    assert_eq!(sf_analytics.starforce_main_stat_bonus(), 350);
    assert_eq!(sf_analytics.starforce_attack_bonus(), 145);

    // 4. Extraction from CharacterDashboard
    let dashboard = CharacterDashboard {
        basic: CharacterBasic {
            date: None,
            character_name: Some("대시보드캐릭".to_string()),
            world_name: Some("루나".to_string()),
            character_gender: None,
            character_class: Some("히어로".to_string()),
            character_class_level: None,
            character_level: Some(275),
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
            final_stat: Some(vec![
                FinalStatEntry {
                    stat_name: Some("전투력".to_string()),
                    stat_value: Some("120000000".to_string()),
                },
                FinalStatEntry {
                    stat_name: Some("보스 몬스터 데미지".to_string()),
                    stat_value: Some("320.0".to_string()),
                },
                FinalStatEntry {
                    stat_name: Some("방어율 무시".to_string()),
                    stat_value: Some("96.0".to_string()),
                },
                FinalStatEntry {
                    stat_name: Some("데미지".to_string()),
                    stat_value: Some("45.0".to_string()),
                },
                FinalStatEntry {
                    stat_name: Some("크리티컬 데미지".to_string()),
                    stat_value: Some("72.0".to_string()),
                },
            ]),
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
            character_name: Some("대시보드캐릭".to_string()),
            world_name: Some("루나".to_string()),
            character_class: Some("히어로".to_string()),
            character_level: Some(275),
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
        dojang: Some(CharacterDojang {
            date: None,
            character_class: None,
            world_name: None,
            dojang_best_floor: Some(64),
            date_dojang_record: None,
            dojang_best_time: None,
        }),
        union: None,
        union_artifact: None,
        guild: None,
        errors: std::collections::HashMap::new(),
    };

    let extracted = dashboard.combat_analytics();
    assert_eq!(extracted.character_name, "대시보드캐릭");
    assert_eq!(extracted.combat_power, 120_000_000);
    assert_eq!(extracted.boss_damage, 320.0);
    assert_eq!(extracted.ignore_monster_armor, 96.0);
    assert_eq!(extracted.dojang_best_floor, Some(64));
    // 120,000,000 CP -> Floor 60 tier (>= 70M benchmark)
    assert_eq!(extracted.estimated_dojang_floor(), 60);
    assert!(extracted.starforce_stat_breakdown.is_some());
}

#[test]
fn test_combat_analytics_decoupled_and_job_specific_force_scaling() {
    use maplestory_openapi::models::character::{
        CharacterBasic, CharacterCombatAnalytics, CharacterItemEquipment, CharacterStat,
        FinalStatEntry,
    };

    let basic = CharacterBasic {
        character_name: Some("제논유저".to_string()),
        world_name: Some("스카니아".to_string()),
        character_class: Some("제논".to_string()),
        character_level: Some(270),
        ..Default::default()
    };
    let stat = CharacterStat {
        final_stat: Some(vec![
            FinalStatEntry {
                stat_name: Some("전투력".to_string()),
                stat_value: Some("80000000".to_string()),
            },
            FinalStatEntry {
                stat_name: Some("보스 몬스터 데미지".to_string()),
                stat_value: Some("300.0".to_string()),
            },
            FinalStatEntry {
                stat_name: Some("방어율 무시".to_string()),
                stat_value: Some("94.0".to_string()),
            },
        ]),
        ..Default::default()
    };
    let equip = CharacterItemEquipment::default();

    // 1. Decoupled construction for Xenon
    let xenon_analytics = CharacterCombatAnalytics::from_stats(
        &basic,
        &stat,
        Some(&equip),
        None,
        1320, // Arcane Force
        400,  // Authentic Force
        None, // Hyper Stat
    );

    assert_eq!(xenon_analytics.character_name, "제논유저");
    assert_eq!(xenon_analytics.combat_power, 80_000_000);
    // Xenon: 1320 / 10 = 132 * 48 = 6336 main stat bonus
    assert_eq!(xenon_analytics.arcane_force_stat_bonus(), 6336);
    // Xenon: 400 / 10 = 40 * 48 = 1920 main stat bonus (Ticket 21 spec)
    assert_eq!(xenon_analytics.authentic_force_stat_bonus(), 1920);

    // 2. Decoupled construction for Demon Avenger
    let da_basic = CharacterBasic {
        character_name: Some("데벤유저".to_string()),
        world_name: Some("루나".to_string()),
        character_class: Some("데몬어벤져".to_string()),
        character_level: Some(275),
        ..Default::default()
    };
    let da_analytics = CharacterCombatAnalytics::from_stats(
        &da_basic,
        &stat,
        Some(&equip),
        None,
        1320,
        400,
        None,
    );
    // Demon Avenger: 1320 / 10 = 132 * 2100 = 277,200 Max HP
    assert_eq!(da_analytics.arcane_force_stat_bonus(), 277_200);
    // Demon Avenger: 400 / 10 = 40 * 4200 = 168,000 Max HP
    assert_eq!(da_analytics.authentic_force_stat_bonus(), 168_000);

    // 3. Standard class (e.g. 히어로)
    let hero_basic = CharacterBasic {
        character_name: Some("히어로유저".to_string()),
        character_class: Some("히어로".to_string()),
        ..Default::default()
    };
    let hero_analytics = CharacterCombatAnalytics::from_stats(
        &hero_basic,
        &stat,
        Some(&equip),
        None,
        1320,
        400,
        None,
    );
    assert_eq!(hero_analytics.arcane_force_stat_bonus(), 13_200);
    assert_eq!(hero_analytics.authentic_force_stat_bonus(), 8_000);

    // 4. from_dashboard convenience constructor
    let dash = maplestory_openapi::models::character::CharacterDashboard {
        basic: hero_basic,
        stat,
        item_equipment: equip,
        cash_item_equipment: Default::default(),
        beauty_equipment: Default::default(),
        coordination: Default::default(),
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
    let from_dash = CharacterCombatAnalytics::from_dashboard(&dash);
    assert_eq!(from_dash.character_name, "히어로유저");
}

#[tokio::test]
async fn test_avatar_client_pipeline_and_custom_frames() {
    use maplestory_openapi::MapleStoryClient;
    use maplestory_openapi::avatar::{AvatarAction, AvatarCode, AvatarEmotion, FrameConfig, WeaponMotion};

    let client = MapleStoryClient::builder().api_key("test").build().unwrap();
    let avatar = client.avatar();

    let code_str = "ABCD".repeat(32);
    let avatar_code = AvatarCode::new(&code_str);

    let frame = FrameConfig {
        action: AvatarAction::Stand1,
        emotion: AvatarEmotion::Default,
        wmotion: WeaponMotion::Default,
        delay_ms: 500,
    };

    // Verify avatar pipeline URL construction and frame query formatting
    let mut query = maplestory_openapi::avatar::AvatarLookQuery::default()
        .action(frame.action)
        .emotion(frame.emotion)
        .weapon_motion(frame.wmotion);
    query = query.width(120).height(120);
    let frame_url = avatar_code.unwrap().to_look_url(&query);
    assert!(frame_url.contains("width=120"));
    assert!(frame_url.contains("height=120"));
    assert!(frame_url.contains("action=A00"));

    // Verify that AvatarClient sub-client is accessible
    let _ = avatar;
}

#[test]
fn test_gear_swap_simulation_weapon() {
    use maplestory_openapi::models::character::{
        CharacterBasic, CharacterCombatAnalytics, CharacterItemEquipment, CharacterStat,
        FinalStatEntry, ItemEquipmentDetail,
    };
    use maplestory_openapi::models::common::ItemTotalOption;
    use maplestory_openapi::{EquipmentDiff, StatDiff};

    let basic = CharacterBasic {
        character_name: Some("히어로대표".to_string()),
        character_class: Some("히어로".to_string()),
        character_level: Some(275),
        ..Default::default()
    };
    let stat = CharacterStat {
        final_stat: Some(vec![
            FinalStatEntry {
                stat_name: Some("전투력".to_string()),
                stat_value: Some("50000000".to_string()),
            },
            FinalStatEntry {
                stat_name: Some("보스 몬스터 데미지".to_string()),
                stat_value: Some("320.0".to_string()),
            },
            FinalStatEntry {
                stat_name: Some("방어율 무시".to_string()),
                stat_value: Some("95.0".to_string()),
            },
            FinalStatEntry {
                stat_name: Some("데미지".to_string()),
                stat_value: Some("45.0".to_string()),
            },
            FinalStatEntry {
                stat_name: Some("크리티컬 데미지".to_string()),
                stat_value: Some("80.0".to_string()),
            },
        ]),
        ..Default::default()
    };

    // Current equipped weapon: 17-star Absolab two-handed sword
    let current_weapon = ItemEquipmentDetail {
        item_equipment_part: Some("무기".to_string()),
        item_equipment_slot: Some("무기".to_string()),
        item_name: Some("앱솔랩스 브로드소드".to_string()),
        starforce: Some("17".to_string()),
        item_total_option: Some(ItemTotalOption {
            str: Some("120".to_string()),
            dex: Some("80".to_string()),
            attack_power: Some("320".to_string()),
            boss_damage: Some("30".to_string()),
            ignore_monster_armor: Some("10".to_string()),
            ..Default::default()
        }),
        potential_option_1: Some("공격력 : +9%".to_string()),
        potential_option_2: Some("보스 몬스터 공격 시 데미지 : +30%".to_string()),
        ..Default::default()
    };

    let equip = CharacterItemEquipment {
        preset_no: Some(1),
        item_equipment_preset_1: Some(vec![current_weapon]),
        ..Default::default()
    };

    let analytics = CharacterCombatAnalytics::from_stats(
        &basic,
        &stat,
        Some(&equip),
        None,
        1320,
        300,
        None,
    );

    // Target upgrade weapon: 22-star Arcane Umbra two-handed sword
    let new_weapon = ItemEquipmentDetail {
        item_equipment_part: Some("무기".to_string()),
        item_equipment_slot: Some("무기".to_string()),
        item_name: Some("아케인셰이드 투핸드소드".to_string()),
        starforce: Some("22".to_string()),
        item_total_option: Some(ItemTotalOption {
            str: Some("270".to_string()),
            dex: Some("150".to_string()),
            attack_power: Some("500".to_string()),
            boss_damage: Some("40".to_string()),
            ignore_monster_armor: Some("20".to_string()),
            ..Default::default()
        }),
        potential_option_1: Some("보스 몬스터 공격 시 데미지 : +40%".to_string()),
        potential_option_2: Some("공격력 : +12%".to_string()),
        ..Default::default()
    };

    let diff: EquipmentDiff = analytics.simulate_equipment_swap(&equip, &new_weapon, None);

    assert_eq!(diff.slot, "무기");
    assert!(diff.previous_item.is_some());
    assert_eq!(diff.previous_item.as_ref().unwrap().item_name.as_deref(), Some("앱솔랩스 브로드소드"));
    assert_eq!(diff.new_item.item_name.as_deref(), Some("아케인셰이드 투핸드소드"));

    // Starforce diff: 22 - 17 = 5
    assert_eq!(diff.starforce_diff, 5);
    let stat_diff: &StatDiff = &diff.stat_diff;
    assert_eq!(stat_diff.starforce, 5);

    // Attack power diff: 500 - 320 = 180
    assert_eq!(diff.stat_diff.attack_power, 180);
    assert_eq!(diff.attack_power_diff(), 180);

    // Main stat (STR for Hero): 270 - 120 = 150
    assert_eq!(diff.stat_diff.str, 150);
    assert_eq!(diff.stat_diff.main_stat(), 150);
    assert_eq!(diff.stat_diff.main_stat_for_class(Some("히어로")), 150);
    assert_eq!(diff.main_stat_diff(), 150);

    // Boss damage diff: (40 - 30) total option + (40 - 30) potential = 20.0%
    assert_eq!(diff.stat_diff.boss_damage, 20.0);
    assert_eq!(diff.boss_damage_diff(), 20.0);

    // IED diff: 20 - 10 = 10.0%
    assert_eq!(diff.stat_diff.ignore_monster_armor, 10.0);
    assert_eq!(diff.ignore_monster_armor_diff(), 10.0);

    // Combat power diff is strictly positive
    assert!(diff.stat_diff.combat_power_diff > 0);
    assert!(diff.combat_power_diff() > 0);
    assert_eq!(
        diff.stat_diff.estimated_combat_power,
        analytics.combat_power + diff.stat_diff.combat_power_diff
    );
    assert!(diff.is_upgrade());
}

#[test]
fn test_gear_swap_simulation_set_effect_recalculation() {
    use maplestory_openapi::models::character::{
        CharacterBasic, CharacterCombatAnalytics, CharacterItemEquipment, CharacterSetEffect,
        CharacterStat, FinalStatEntry, ItemEquipmentDetail, SetEffectInfo, SetEffectOption,
    };
    use maplestory_openapi::models::common::ItemTotalOption;
    use maplestory_openapi::EquipmentDiff;

    let basic = CharacterBasic {
        character_name: Some("썬콜법사".to_string()),
        character_class: Some("아크메이지(썬,콜)".to_string()),
        character_level: Some(270),
        ..Default::default()
    };
    let stat = CharacterStat {
        final_stat: Some(vec![
            FinalStatEntry {
                stat_name: Some("전투력".to_string()),
                stat_value: Some("40000000".to_string()),
            },
            FinalStatEntry {
                stat_name: Some("보스 몬스터 데미지".to_string()),
                stat_value: Some("300.0".to_string()),
            },
            FinalStatEntry {
                stat_name: Some("방어율 무시".to_string()),
                stat_value: Some("94.0".to_string()),
            },
        ]),
        ..Default::default()
    };

    let old_hat = ItemEquipmentDetail {
        item_equipment_part: Some("모자".to_string()),
        item_equipment_slot: Some("모자".to_string()),
        item_name: Some("하이네스 던위치햇".to_string()), // Root Abyss hat
        starforce: Some("18".to_string()),
        item_total_option: Some(ItemTotalOption {
            int: Some("120".to_string()),
            magic_power: Some("60".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let equip = CharacterItemEquipment {
        preset_no: Some(1),
        item_equipment_preset_1: Some(vec![old_hat]),
        ..Default::default()
    };

    let set_effects = CharacterSetEffect {
        date: None,
        set_effect: Some(vec![
            SetEffectInfo {
                set_name: Some("루타비스 세트 (마법사)".to_string()),
                total_set_count: Some(4),
                set_effect_info: Some(vec![
                    SetEffectOption {
                        set_count: Some(2),
                        set_option: Some("올스탯: +20, 최대 HP/최대 MP: +1000".to_string()),
                    },
                    SetEffectOption {
                        set_count: Some(3),
                        set_option: Some("공격력: +50, 마력: +50".to_string()),
                    },
                    SetEffectOption {
                        set_count: Some(4),
                        set_option: Some("보스 몬스터 공격 시 데미지: +30%".to_string()),
                    },
                ]),
                set_option_full: None,
            },
            SetEffectInfo {
                set_name: Some("아케인셰이드 세트 (마법사)".to_string()),
                total_set_count: Some(4),
                set_effect_info: Some(vec![
                    SetEffectOption {
                        set_count: Some(2),
                        set_option: Some("올스탯: +30, 마력: +10".to_string()),
                    },
                    SetEffectOption {
                        set_count: Some(3),
                        set_option: Some("마력: +30".to_string()),
                    },
                    SetEffectOption {
                        set_count: Some(4),
                        set_option: Some("방어율 무시: +10%".to_string()),
                    },
                    SetEffectOption {
                        set_count: Some(5),
                        set_option: Some("보스 몬스터 공격 시 데미지: +30%, 마력: +30".to_string()),
                    },
                ]),
                set_option_full: None,
            },
        ]),
    };

    let analytics = CharacterCombatAnalytics::from_stats(
        &basic,
        &stat,
        Some(&equip),
        None,
        1320,
        300,
        None,
    );

    // Swapping Root Abyss hat with Arcane Umbra hat
    let new_hat = ItemEquipmentDetail {
        item_equipment_part: Some("모자".to_string()),
        item_equipment_slot: Some("모자".to_string()),
        item_name: Some("아케인셰이드 메이지햇".to_string()), // Arcane Umbra hat
        starforce: Some("22".to_string()),
        item_total_option: Some(ItemTotalOption {
            int: Some("200".to_string()),
            magic_power: Some("95".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let diff: EquipmentDiff = analytics.simulate_equipment_swap(&equip, &new_hat, Some(&set_effects));

    // Verify set count changes
    assert!(diff.set_effect_diff.is_some());
    let sed = diff.set_effect_diff.as_ref().unwrap();
    let root_abyss_change = sed.set_count_changes.iter().find(|c| c.set_name.contains("루타비스")).unwrap();
    assert_eq!(root_abyss_change.previous_count, 4);
    assert_eq!(root_abyss_change.new_count, 3); // Dropped 4 -> 3

    let arcane_change = sed.set_count_changes.iter().find(|c| c.set_name.contains("아케인")).unwrap();
    assert_eq!(arcane_change.previous_count, 4);
    assert_eq!(arcane_change.new_count, 5); // Increased 4 -> 5

    // Lost 4-set Root Abyss option ("보스 몬스터 공격 시 데미지: +30%")
    assert!(sed.lost_options.iter().any(|o| o.contains("보스 몬스터 공격 시 데미지")));
    // Gained 5-set Arcane Umbra option ("보스 몬스터 공격 시 데미지: +30%, 마력: +30")
    assert!(sed.gained_options.iter().any(|o| o.contains("5-set")));

    // Net set effect boss damage: -30% + 30% = 0.0%
    assert_eq!(diff.stat_diff.boss_damage, 0.0);
    // Net magic power: item diff (95 - 60 = 35) + set diff (+30) = 65
    assert_eq!(diff.stat_diff.magic_power, 65);
    // Net INT: item diff (200 - 120 = 80) + set diff (0) = 80
    assert_eq!(diff.stat_diff.int, 80);
}

#[test]
fn test_gear_swap_simulation_empty_slot() {
    use maplestory_openapi::models::character::{
        CharacterBasic, CharacterCombatAnalytics, CharacterItemEquipment, CharacterStat,
        ItemEquipmentDetail,
    };
    use maplestory_openapi::models::common::ItemTotalOption;

    let basic = CharacterBasic::default();
    let stat = CharacterStat::default();
    let equip = CharacterItemEquipment::default(); // completely empty equipment
    let analytics = CharacterCombatAnalytics::from_stats(&basic, &stat, Some(&equip), None, 0, 0, None);

    let ring = ItemEquipmentDetail {
        item_equipment_part: Some("반지".to_string()),
        item_equipment_slot: Some("반지4".to_string()),
        item_name: Some("마이티 링".to_string()),
        starforce: Some("10".to_string()),
        item_total_option: Some(ItemTotalOption {
            str: Some("25".to_string()),
            attack_power: Some("15".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let diff = analytics.simulate_equipment_swap(&equip, &ring, None);
    assert_eq!(diff.slot, "반지4");
    assert!(diff.previous_item.is_none());
    assert_eq!(diff.starforce_diff, 10);
    assert_eq!(diff.stat_diff.str, 25);
    assert_eq!(diff.stat_diff.attack_power, 15);
}

#[test]
fn test_starforce_cost_per_attempt_and_discounts() {
    use maplestory_openapi::models::calculator::{StarforceCalculator, StarforceOptions};

    let default_opts = StarforceOptions::default();

    // Level 150 gear, 0 star -> 1 star
    // Base formula: 1000 + 150^3 * 1 / 2500 * 100 = 1000 + 135000 = 136,000 meso
    let cost_0_to_1 = StarforceCalculator::cost_per_attempt(150, 0, &default_opts);
    assert_eq!(cost_0_to_1, 136_000);

    // 30% discount option
    let disc_opts = StarforceOptions::default().with_discount_30(true);
    let cost_disc = StarforceCalculator::cost_per_attempt(150, 0, &disc_opts);
    assert_eq!(cost_disc, (136_000.0 * 0.7) as u64); // 95,200 meso

    // Safeguard on 15 stars: adds 100% extra base cost (2x)
    let sf_opts = StarforceOptions::default().with_safeguard(true);
    let cost_15_normal = StarforceCalculator::cost_per_attempt(150, 15, &default_opts);
    let cost_15_safeguard = StarforceCalculator::cost_per_attempt(150, 15, &sf_opts);
    assert_eq!(cost_15_safeguard, cost_15_normal * 2);
}

#[test]
fn test_starforce_markov_chain_from_12_to_17() {
    use maplestory_openapi::models::calculator::{StarforceCalculator, StarforceOptions};

    let opts = StarforceOptions::default()
        .with_starcatch(true)
        .with_safeguard(true); // safeguard 15-16 stars

    let estimate = StarforceCalculator::calculate(160, 12, 17, &opts).expect("calculate starforce 12 to 17");

    assert_eq!(estimate.item_level, 160);
    assert_eq!(estimate.start_star, 12);
    assert_eq!(estimate.target_star, 17);
    assert!(estimate.expected_meso_cost > 0.0);
    assert!(estimate.expected_attempts > 5.0);
    // Because safeguard was on for 15-16, and 12-14 has no destruction, expected destructions must be 0!
    assert_eq!(estimate.expected_destructions, 0.0);
    assert_eq!(estimate.no_destruction_probability, 1.0);

    // Percentiles ordering: 10th <= 50th <= 90th
    assert!(estimate.percentile_10_cost <= estimate.percentile_50_cost);
    assert!(estimate.percentile_50_cost <= estimate.percentile_90_cost);
}

#[test]
fn test_starforce_markov_chain_from_17_to_22_high_risk() {
    use maplestory_openapi::models::calculator::{StarforceCalculator, StarforceOptions};

    let opts = StarforceOptions::default()
        .with_starcatch(true)
        .with_spare_cost(200_000_000); // 200M spare cost

    let estimate = StarforceCalculator::calculate(200, 17, 22, &opts).expect("calculate starforce 17 to 22");

    assert_eq!(estimate.item_level, 200);
    assert_eq!(estimate.start_star, 17);
    assert_eq!(estimate.target_star, 22);
    assert!(estimate.expected_meso_cost > 1_000_000_000.0); // usually several billion mesos
    assert!(estimate.expected_destructions > 0.0);
    assert!(estimate.no_destruction_probability < 1.0);
    assert!(estimate.percentile_50_cost > 0);
    assert!(estimate.percentile_90_cost >= estimate.percentile_50_cost);
    assert!(estimate.std_deviation_cost > 0.0);
}

#[test]
fn test_starforce_events_and_options() {
    use maplestory_openapi::models::calculator::{StarforceCalculator, StarforceOptions};

    // 1+1 event test under 12 stars
    let normal_opts = StarforceOptions::default().with_starcatch(false);
    let event_opts = StarforceOptions::default().with_starcatch(false).with_1_plus_1(true);

    let norm_est = StarforceCalculator::calculate(150, 0, 12, &normal_opts).unwrap();
    let event_est = StarforceCalculator::calculate(150, 0, 12, &event_opts).unwrap();

    // 1+1 should significantly reduce required attempts from 0 to 12
    assert!(event_est.expected_attempts < norm_est.expected_attempts);
    assert!(event_est.expected_meso_cost < norm_est.expected_meso_cost);

    // Shining Starforce (30% off + 5/10/15 guarantee)
    let shining_opts = StarforceOptions::default().with_shining_starforce();
    assert!(shining_opts.discount_30);
    assert!(shining_opts.sunday_5_10_15);

    let norm_12_17 = StarforceCalculator::calculate(150, 12, 17, &normal_opts).unwrap();
    let shining_est = StarforceCalculator::calculate(150, 12, 17, &shining_opts).unwrap();

    // Shining Starforce should significantly reduce cost and attempts due to 30% discount and 15-star guarantee
    assert!(shining_est.expected_meso_cost < norm_12_17.expected_meso_cost);
    assert!(shining_est.expected_attempts < norm_12_17.expected_attempts);
}

#[test]
fn test_cube_grade_up_nexon_official_probabilities_and_ceiling() {
    use maplestory_openapi::models::calculator::{CubeCalculator, CubeType, PotentialGrade};

    // Red Cube: Rare -> Epic (6.0%, ceiling 25)
    let red_rare_to_epic = CubeCalculator::calculate(CubeType::Red, PotentialGrade::Rare, PotentialGrade::Epic, None).unwrap();
    assert_eq!(red_rare_to_epic.ceiling_cubes, Some(25));
    assert!(red_rare_to_epic.is_cash);
    assert_eq!(red_rare_to_epic.ceiling_cost, Some(25 * 1200));
    assert!(red_rare_to_epic.expected_cubes > 10.0 && red_rare_to_epic.expected_cubes <= 25.0);

    // Black Cube: Unique -> Legendary (1.4%, ceiling 107)
    let black_unique_to_legendary = CubeCalculator::calculate(CubeType::Black, PotentialGrade::Unique, PotentialGrade::Legendary, None).unwrap();
    assert_eq!(black_unique_to_legendary.ceiling_cubes, Some(107));
    assert!(black_unique_to_legendary.expected_cubes < 107.0);
    assert_eq!(black_unique_to_legendary.ceiling_cost, Some(107 * 2200));

    // Multi-tier grade up: Rare -> Legendary via Black Cube
    let black_rare_to_leg = CubeCalculator::calculate(CubeType::Black, PotentialGrade::Rare, PotentialGrade::Legendary, None).unwrap();
    assert_eq!(black_rare_to_leg.step_details.len(), 3); // Rare->Epic, Epic->Unique, Unique->Legendary
    assert_eq!(black_rare_to_leg.ceiling_cubes, Some(10 + 42 + 107)); // sum of ceilings
    assert!(black_rare_to_leg.expected_cubes < (10 + 42 + 107) as f64);
    assert!(black_rare_to_leg.percentile_50_cubes <= black_rare_to_leg.percentile_90_cubes);

    // Additional Cube: Unique -> Legendary (0.7%, ceiling 214)
    let add_unique_to_leg = CubeCalculator::calculate(CubeType::Additional, PotentialGrade::Unique, PotentialGrade::Legendary, None).unwrap();
    assert_eq!(add_unique_to_leg.ceiling_cubes, Some(214));
    assert_eq!(add_unique_to_leg.ceiling_cost, Some(214 * 2400));
}

#[test]
fn test_cube_target_option_cost_estimate() {
    use maplestory_openapi::models::calculator::CubeCalculator;

    // Rolling a 1% probability target option
    let estimate = CubeCalculator::calculate_target_option(0.01, 2200).unwrap();
    assert_eq!(estimate.target_probability, 0.01);
    assert_eq!(estimate.expected_attempts, 100.0);
    assert_eq!(estimate.expected_cost, 220_000.0);
    // 50th percentile: ln(0.5)/ln(0.99) = 69 attempts
    assert_eq!(estimate.percentile_50_attempts, 69);
    // 90th percentile: ln(0.1)/ln(0.99) = 230 attempts
    assert_eq!(estimate.percentile_90_attempts, 230);
}

#[tokio::test]
async fn test_disk_identity_cache_basic_operations() {
    use maplestory_openapi::client::{DiskIdentityCache, IdentityCache};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let temp_dir = std::env::temp_dir().join(format!("maple_cache_test_{}_{}", std::process::id(), nanos));

    let cache = DiskIdentityCache::new(&temp_dir, Duration::from_secs(3600));

    // Initially empty
    assert_eq!(cache.get_ocid("아델").await, None);
    assert_eq!(cache.get_oguild_id("테스트길드", "스카니아").await, None);

    // Set OCID
    cache.set_ocid("아델", "ocid-adel-1234").await;
    assert_eq!(cache.get_ocid("아델").await, Some("ocid-adel-1234".to_string()));

    // Set OGuildID
    cache.set_oguild_id("테스트길드", "스카니아", "oguild-guild-5678").await;
    assert_eq!(
        cache.get_oguild_id("테스트길드", "스카니아").await,
        Some("oguild-guild-5678".to_string())
    );

    // Overwrite OCID
    cache.set_ocid("아델", "ocid-adel-updated").await;
    assert_eq!(cache.get_ocid("아델").await, Some("ocid-adel-updated".to_string()));

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[tokio::test]
async fn test_disk_identity_cache_persistence_across_instances() {
    use maplestory_openapi::client::{DiskIdentityCache, IdentityCache};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let temp_dir = std::env::temp_dir().join(format!("maple_cache_persist_{}_{}", std::process::id(), nanos));

    // Instance 1 writes
    {
        let cache1 = DiskIdentityCache::new(&temp_dir, Duration::from_secs(3600));
        cache1.set_ocid("온앤온", "ocid-onandon-9999").await;
        cache1.set_oguild_id("스타", "루나", "oguild-star-1111").await;
    }

    // Instance 2 (simulating process restart) reads from same directory
    {
        let cache2 = DiskIdentityCache::new(&temp_dir, Duration::from_secs(3600));
        assert_eq!(
            cache2.get_ocid("온앤온").await,
            Some("ocid-onandon-9999".to_string())
        );
        assert_eq!(
            cache2.get_oguild_id("스타", "루나").await,
            Some("oguild-star-1111".to_string())
        );
        assert_eq!(cache2.get_ocid("미등록캐릭터").await, None);
        assert_eq!(cache2.get_oguild_id("없는길드", "루나").await, None);
    }

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[tokio::test]
async fn test_disk_identity_cache_expiration_past_ttl() {
    use maplestory_openapi::client::{DiskIdentityCache, IdentityCache};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let temp_dir = std::env::temp_dir().join(format!("maple_cache_expire_{}_{}", std::process::id(), nanos));

    let cache = DiskIdentityCache::new(&temp_dir, Duration::from_millis(300));

    cache.set_ocid("단명캐릭터", "ocid-short-lived").await;
    cache.set_oguild_id("단명길드", "크로아", "oguild-short-lived").await;

    // Immediately available
    assert_eq!(
        cache.get_ocid("단명캐릭터").await,
        Some("ocid-short-lived".to_string())
    );
    assert_eq!(
        cache.get_oguild_id("단명길드", "크로아").await,
        Some("oguild-short-lived".to_string())
    );

    // Wait for TTL to expire
    tokio::time::sleep(Duration::from_millis(450)).await;

    // Should return None after expiration
    assert_eq!(cache.get_ocid("단명캐릭터").await, None);
    assert_eq!(cache.get_oguild_id("단명길드", "크로아").await, None);

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[tokio::test]
async fn test_disk_identity_cache_with_maplestory_client_persistence() {
    use maplestory_openapi::client::DiskIdentityCache;
    use maplestory_openapi::MapleStoryClient;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let temp_dir = std::env::temp_dir().join(format!("maple_cache_client_{}_{}", std::process::id(), nanos));

    let req_counter = Arc::new(AtomicUsize::new(0));
    let req_counter_clone = req_counter.clone();

    let (base_url, server_task) = spawn_mock_server(move |req_text| {
        req_counter_clone.fetch_add(1, Ordering::SeqCst);
        let body = if req_text.contains("/maplestory/v1/id") {
            r#"{"ocid":"test-client-disk-ocid"}"#.to_string()
        } else if req_text.contains("/maplestory/v1/guild/id") {
            r#"{"oguild_id":"test-client-disk-oguild"}"#.to_string()
        } else {
            r#"{}"#.to_string()
        };
        (200, body, None)
    })
    .await;

    // Client 1 fetches character ocid via HTTP and persists it to disk
    {
        let cache = Arc::new(DiskIdentityCache::new(&temp_dir, Duration::from_secs(3600)));
        let client = MapleStoryClient::builder()
            .api_key("test-key")
            .base_url(&base_url)
            .identity_cache(cache)
            .build()
            .unwrap();

        let ocid = client.get_ocid("hero").await.unwrap();
        assert_eq!(ocid, "test-client-disk-ocid");
        assert_eq!(req_counter.load(Ordering::SeqCst), 1);
    }

    // Client 2 (new instance, new cache reading same directory) fetches character ocid from disk
    {
        let cache = Arc::new(DiskIdentityCache::new(&temp_dir, Duration::from_secs(3600)));
        let client = MapleStoryClient::builder()
            .api_key("test-key")
            .base_url(&base_url)
            .identity_cache(cache)
            .build()
            .unwrap();

        // Hit from disk without HTTP request!
        let ocid = client.get_ocid("hero").await.unwrap();
        assert_eq!(ocid, "test-client-disk-ocid");
        assert_eq!(req_counter.load(Ordering::SeqCst), 1);

        // Fetch guild id (hits HTTP)
        let oguild = client.get_oguild_id("testguild", "scania").await.unwrap();
        assert_eq!(oguild, "test-client-disk-oguild");
        assert_eq!(req_counter.load(Ordering::SeqCst), 2);
    }

    // Client 3 (another new instance) fetches guild id from disk
    {
        let cache = Arc::new(DiskIdentityCache::new(&temp_dir, Duration::from_secs(3600)));
        let client = MapleStoryClient::builder()
            .api_key("test-key")
            .base_url(&base_url)
            .identity_cache(cache)
            .build()
            .unwrap();

        let oguild = client.get_oguild_id("testguild", "scania").await.unwrap();
        assert_eq!(oguild, "test-client-disk-oguild");
        assert_eq!(req_counter.load(Ordering::SeqCst), 2);
    }

    server_task.abort();
    let _ = std::fs::remove_dir_all(&temp_dir);
}




