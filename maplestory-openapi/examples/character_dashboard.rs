// examples/character_dashboard.rs
//! All-in-one Character Dashboard Example (올인원 캐릭터 종합 스펙 대시보드 예제)
//!
//! Demonstrates how a single method call `get_character_dashboard_by_name(name)`
//! concurrently fetches all combat specs, symbols, abilities, 6th HEXA matrix,
//! union, artifact, guild info, and synthesized coordination looks.
//!
//! Run with:
//! ```bash
//! cargo run --example character_dashboard
//! ```

use maplestory_openapi::{AvatarAction, AvatarEmotion, MapleStoryClient, WeaponMotion};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env from workspace root if present
    dotenvy::dotenv().ok();
    dotenvy::from_filename("../.env").ok();

    let client = MapleStoryClient::from_env()?;

    let target_names = ["온앤온", "니니류"];

    for name in target_names {
        println!("\n{}", "=".repeat(70));
        println!("🚀 [올인원 대시보드 검색] 닉네임: {}", name);
        println!("{}", "=".repeat(70));

        let dash = match client.get_character_dashboard_by_name(name).await {
            Ok(d) => d,
            Err(e) => {
                eprintln!("❌ Failed to retrieve dashboard for '{}': {}", name, e);
                continue;
            }
        };

        // 1. 기본 프로필
        println!("\n📋 [기본 정보]");
        println!("  - 이름: {}", dash.character_name());
        println!("  - 월드: {}", dash.world_name());
        println!("  - 직업: {}", dash.character_class());
        println!("  - 레벨: Lv.{}", dash.character_level());
        println!("  - 인기도: {}", dash.popularity());
        if let Some(img) = dash.character_image() {
            println!("  - 기본 렌더 이미지: {}", img);
        }

        // 2. 전투력 & 핵심 스펙
        println!("\n⚔️ [전투력 및 종합 스펙]");
        println!("  - 전투력: {:>12}", format_number(dash.combat_power()));
        println!("  - 스타포스 합계: {}성", dash.total_starforce());
        println!("  - 아케인포스: {}", dash.arcane_force());
        println!("  - 어센틱포스: {}", dash.authentic_force());
        if let Some(floor) = dash.dojang_best_floor() {
            println!("  - 무릉도장 최고 기록: {}층", floor);
        }

        // 3. 어빌리티
        if let Some(grade) = dash.ability_grade() {
            println!("\n✨ [어빌리티: {}]", grade);
            if let Some(ability) = &dash.ability {
                if let Some(info) = &ability.ability_info {
                    for line in info {
                        println!(
                            "  - [{}] {}",
                            line.ability_grade.as_deref().unwrap_or(""),
                            line.ability_value.as_deref().unwrap_or("")
                        );
                    }
                }
            }
        }

        // 4. 유니온 & 유니온 아티팩트
        println!("\n🛡️ [유니온 & 아티팩트]");
        if let Some(u) = &dash.union {
            println!(
                "  - 유니온: Lv.{} ({})",
                u.union_level.unwrap_or(0),
                u.union_grade.as_deref().unwrap_or("등급 미등록")
            );
            println!("  - 아티팩트: Lv.{}", u.union_artifact_level.unwrap_or(0));
        } else {
            println!("  - 유니온 정보 없음");
        }
        if let Some(artifact) = &dash.union_artifact {
            if let Some(effects) = &artifact.union_artifact_effect {
                println!("  - 아티팩트 발동 효과 ({}개):", effects.len());
                for eff in effects.iter().take(4) {
                    println!(
                        "    • {} (Lv.{})",
                        eff.name.as_deref().unwrap_or(""),
                        eff.level.unwrap_or(0)
                    );
                }
                if effects.len() > 4 {
                    println!("    • ... 외 {}개 효과", effects.len() - 4);
                }
            }
        }

        // 5. 길드 정보 (자동 연계 조회)
        println!("\n🏰 [길드 정보]");
        if let Some(guild) = &dash.guild {
            println!(
                "  - 길드명: {} (Lv.{})",
                guild.guild_name.as_deref().unwrap_or(""),
                guild.guild_level.unwrap_or(0)
            );
            println!(
                "  - 길드마스터: {}",
                guild.guild_master_name.as_deref().unwrap_or("")
            );
            println!("  - 길드원 수: {}명", guild.guild_member_count.unwrap_or(0));
            println!(
                "  - 노블레스 스킬 총합: {} 포인트",
                guild.total_noblesse_points()
            );
            if let Some(noblesse) = &guild.guild_noblesse_skill {
                for skill in noblesse {
                    println!(
                        "    • {} (Lv.{})",
                        skill.skill_name.as_deref().unwrap_or(""),
                        skill.skill_level.unwrap_or(0)
                    );
                }
            }
        } else if let Some(g_name) = dash.guild_name() {
            println!("  - 길드명: {} (상세 정보 미조회)", g_name);
        } else {
            println!("  - 소속 길드 없음");
        }

        // 6. 6차 헥사 매트릭스 & 스탯
        if let Some(hexa) = &dash.hexa_matrix {
            let core_count = hexa
                .character_hexa_core_equipment
                .as_ref()
                .map(|c| c.len())
                .unwrap_or(0);
            println!("\n🔮 [6차 전직 헥사 매트릭스]");
            println!("  - 장착 헥사 코어: {}개", core_count);
            if let Some(stat) = &dash.hexa_matrix_stat {
                if let Some(stats) = &stat.character_hexa_stat_core {
                    for s in stats {
                        println!(
                            "  - 코어: {} (주스탯: {} Lv.{}, 부스탯1: {} Lv.{}, 부스탯2: {} Lv.{})",
                            s.slot_id.as_deref().unwrap_or(""),
                            s.main_stat_name.as_deref().unwrap_or(""),
                            s.main_stat_level.unwrap_or(0),
                            s.sub_stat_name_1.as_deref().unwrap_or(""),
                            s.sub_stat_level_1.unwrap_or(0),
                            s.sub_stat_name_2.as_deref().unwrap_or(""),
                            s.sub_stat_level_2.unwrap_or(0),
                        );
                    }
                }
            }
        }

        // 7. 코디 & 외형 (프리즘 및 아바타 코드)
        println!("\n👗 [원스톱 코디 및 아바타 분석]");
        let coord = &dash.coordination;
        if let Some(hair) = &coord.hair {
            println!(
                "  - 헤어: {} (베이스: {}, 믹스: {}, 믹스비율: {}%)",
                hair.hair_name.as_deref().unwrap_or(""),
                hair.base_color.as_deref().unwrap_or(""),
                hair.mix_color.as_deref().unwrap_or(""),
                hair.mix_rate.as_deref().unwrap_or("0")
            );
        }
        if let Some(face) = &coord.face {
            println!(
                "  - 성형: {} (베이스: {}, 믹스: {}, 믹스비율: {}%)",
                face.face_name.as_deref().unwrap_or(""),
                face.base_color.as_deref().unwrap_or(""),
                face.mix_color.as_deref().unwrap_or(""),
                face.mix_rate.as_deref().unwrap_or("0")
            );
        }

        // 프리즘 리컬러링 아이템
        let prism_items: Vec<_> = coord.prism_items().collect();
        if !prism_items.is_empty() {
            println!("  - 컬러링 프리즘 적용 아이템 ({}개):", prism_items.len());
            for item in prism_items {
                if let Some(prism) = item.prism() {
                    println!(
                        "    • [{}] {} -> 색상범위: {}, 색조(Hue): {}, 채도(Sat): {}, 명도(Val): {}",
                        item.slot,
                        item.name,
                        prism.color_range.as_deref().unwrap_or(""),
                        prism.hue.unwrap_or(0),
                        prism.saturation.unwrap_or(0),
                        prism.value.unwrap_or(0)
                    );
                }
            }
        }

        // 아바타 코드 및 동적 렌더링 URL
        if let Some(code) = dash.avatar_code() {
            println!(
                "  - 아바타 1024자 코드: {}... (총 {}자)",
                &code.as_str()[..32],
                code.as_str().len()
            );
            println!(
                "  - [동적 렌더러] 윙크 + 점프 포즈 (96x96):\n    {}",
                coord
                    .custom_image(
                        AvatarAction::Jump,
                        AvatarEmotion::Wink,
                        WeaponMotion::Default,
                        96,
                        96,
                    )
                    .unwrap_or_default()
            );
        }
    }

    Ok(())
}

fn format_number(val: i64) -> String {
    let s = val.to_string();
    let mut result = String::new();
    for (count, c) in s.chars().rev().enumerate() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}
