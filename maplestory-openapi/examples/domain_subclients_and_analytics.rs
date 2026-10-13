// examples/domain_subclients_and_analytics.rs
//! Hierarchical Domain Sub-Clients & Deep Analytics Example
//! (계층형 도메인 서브클라이언트 및 심화 스펙 분석 예제)
//!
//! Demonstrates:
//! 1. Hierarchical sub-clients (`client.character()`, `client.union()`, `client.guild()`, `client.ranking()`, `client.notice()`)
//! 2. Equipment Potential & Additional Potential aggregation (`StatType`, `total_potential_stat`)
//! 3. Character Progression helpers (6th HEXA Matrix, V-Matrix, Ability, Hyper Stat)
//! 4. Union Raider Board, Union Artifact, and Union Champion typed stat aggregation (`StatType`)
//! 5. Guild Profile aggregator (`GuildProfile`)
//!
//! Run with:
//! ```bash
//! cargo run --example domain_subclients_and_analytics -- 온앤온
//! ```

use maplestory_openapi::{MapleStoryClient, StatType};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    dotenvy::from_filename("../.env").ok();

    let args: Vec<String> = env::args().collect();
    let character_name = args.get(1).map(|s| s.as_str()).unwrap_or("온앤온");

    let client = MapleStoryClient::from_env()?;
    let char_api = client.character();
    let union_api = client.union();
    let guild_api = client.guild();

    println!("======================================================================");
    println!(
        "🧭 [계층형 서브클라이언트 & 심화 분석] 대상 캐릭터: {}",
        character_name
    );
    println!("======================================================================");

    // 1. 캐릭터 기본 정보 및 OCID 조회 (인메모리 캐시 연동)
    let ocid = char_api.get_ocid(character_name).await?;
    let basic = char_api.get_basic(&ocid, None).await?;
    println!(
        "\n📋 [기본 정보] Lv.{} {} ({} / {})",
        basic.character_level.unwrap_or(0),
        basic.character_name.as_deref().unwrap_or(""),
        basic.world_name.as_deref().unwrap_or(""),
        basic.character_class.as_deref().unwrap_or("")
    );

    // 2. 장착 장비 잠재능력 + 에디셔널 스탯 합산 파싱
    let equip = char_api.get_item_equipment(&ocid, None).await?;
    println!("\n💎 [장착 장비 잠재능력 + 에디셔널 합산]");
    for stat in [
        StatType::BossDamage,
        StatType::IgnoreMonsterArmor,
        StatType::AttackPower,
        StatType::MagicPower,
        StatType::CriticalDamage,
        StatType::Str,
        StatType::Dex,
        StatType::Int,
        StatType::Luk,
        StatType::AllStat,
        StatType::CooldownReduction,
    ] {
        let (flat_sum, pct_sum) =
            equip
                .active_items()
                .iter()
                .fold((0.0, 0.0), |(f_acc, p_acc), item| {
                    let (f, p) = item.total_potential_stat(stat);
                    (f_acc + f, p_acc + p)
                });
        if flat_sum > 0.0 || pct_sum > 0.0 {
            println!("  - {}: +{:.0}% (고정 +{:.0})", stat, pct_sum, flat_sum);
        }
    }

    // 3. 캐릭터 육성 지표 (6차 HEXA, 5차 V매트릭스, 어빌리티, 하이퍼스탯)
    println!("\n🔮 [캐릭터 육성 지표 (HEXA / V매트릭스 / 어빌리티 / 하이퍼스탯)]");
    if let Ok(hexa) = char_api.get_hexamatrix(&ocid, None).await {
        println!(
            "  - 활성 6차 HEXA 코어 수: {}개 (마스터리 코어: {}개)",
            hexa.active_cores().len(),
            hexa.cores_by_type("마스터리 코어").len()
        );
    }
    if let Ok(hexa_stat) = char_api.get_hexamatrix_stat(&ocid, None).await {
        println!(
            "  - 활성 HEXA 스탯 코어 수: {}개",
            hexa_stat.active_stat_cores().len()
        );
        if let Some(main_lv) = hexa_stat.main_stat_level(1) {
            let sub_lvs = hexa_stat.additional_stat_levels(1).unwrap_or((0, 0));
            println!(
                "    • 슬롯 1: 메인 Lv.{}, 에디셔널 Lv.({}, {})",
                main_lv, sub_lvs.0, sub_lvs.1
            );
        }
    }
    if let Ok(vmatrix) = char_api.get_vmatrix(&ocid, None).await {
        println!(
            "  - V매트릭스 슬롯 강화 레벨 합계: +{} (만렙 코어: {}개)",
            vmatrix.total_slot_enhancement_level(),
            vmatrix.max_level_cores().len()
        );
    }
    if let Ok(ability) = char_api.get_ability(&ocid, None).await {
        println!(
            "  - 활성 프리셋 어빌리티 줄 수: {}줄",
            ability.active_preset_abilities(None).len()
        );
    }
    if let Ok(hyper) = char_api.get_hyper_stat(&ocid, None).await {
        println!(
            "  - 활성 프리셋 하이퍼스탯 항목 수: {}개",
            hyper.active_preset_hyper_stats(None).len()
        );
    }

    // 4. 유니온 공격대(Raider), 아티팩트(Artifact), 챔피언(Champion) 분석
    println!("\n🛡️ [유니온 공격대 & 아티팩트 & 챔피언 통합 분석]");
    if let Ok(raider) = union_api.get_raider(&ocid, None).await {
        let occupied = raider.total_occupied_stats_typed();
        println!(
            "  - 유니온 점령 크리티컬 데미지: +{:.2}%",
            occupied
                .get(&StatType::CriticalDamage)
                .copied()
                .unwrap_or(0.0)
        );
        println!(
            "  - 유니온 점령 보스 데미지: +{:.2}%",
            occupied.get(&StatType::BossDamage).copied().unwrap_or(0.0)
        );
    }
    if let Ok(artifact) = union_api.get_artifact(&ocid, None).await {
        let effects = artifact.total_artifact_stats_typed();
        println!("  - 아티팩트 발동 스탯 종류: {}가지", effects.len());
    }
    if let Ok(champion) = union_api.get_champion(&ocid, None).await {
        println!(
            "  - 등록된 유니온 챔피언 수: {}명 (휘장 효과 종류: {}가지)",
            champion.champion_count(),
            champion.total_badge_stats_typed().len()
        );
    }

    // 5. 소속 길드 종합 프로필 조회 (GuildProfile)
    if let (Some(guild_name), Some(world_name)) = (
        basic.character_guild_name.as_deref(),
        basic.world_name.as_deref(),
    ) {
        if !guild_name.is_empty() {
            if let Ok(guild_profile) = guild_api.get_profile(guild_name, world_name, None).await {
                println!("\n🏰 [길드 종합 프로필: {} ({})]", guild_name, world_name);
                println!(
                    "  - 노블레스 스킬 포인트: {}p",
                    guild_profile.total_noblesse_points
                );
                if let Some(avg_lv) = guild_profile.average_member_level() {
                    println!("  - 길드원 평균 레벨: Lv.{:.1}", avg_lv);
                }
            }
        }
    }

    // 6. 전투 스펙 및 보스 방어율 관통/대미지 배율 심화 분석 (CharacterCombatAnalytics)
    if let Ok(combat) = char_api.get_combat_analytics(&ocid, None).await {
        println!("\n⚔️ [전투 스펙 및 보스 방어율 관통 분석 (CombatAnalytics)]");
        println!(
            "  - 300% 방어율 보스 실대미지 비율: {:.1}%",
            combat.effective_damage_multiplier_300() * 100.0
        );
        println!(
            "  - 380% 익스트림 보스 실대미지 비율: {:.1}%",
            combat.effective_damage_multiplier_380() * 100.0
        );
        println!(
            "  - 보공+뎀 총합 배율: {:.2}x, 크리티컬 대미지 배율: {:.2}x",
            combat.total_damage_factor(),
            combat.critical_damage_factor()
        );
        println!(
            "  - 전투력 기준 예상 무릉 층수: {}층 (기록된 최고 층수: {:?}층)",
            combat.estimated_dojang_floor(),
            combat.dojang_best_floor
        );
    }

    Ok(())
}
