// examples/character_full_info.rs
//! Fetch multiple pieces of character info in parallel using tokio::join!
//!
//! Run with:
//! ```sh
//! NEXON_API_KEY=your_key cargo run --example character_full_info -- 캐릭터명
//! ```

use maplestory_openapi::{MapleStoryClient, SkillGrade};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("NEXON_API_KEY")
        .expect("Set NEXON_API_KEY environment variable");

    let character_name = std::env::args().nth(1).unwrap_or_else(|| "캐릭터명".to_string());

    let client = MapleStoryClient::new(api_key)?;

    // Step 1: Resolve OCID
    println!("Resolving OCID for '{}'...", character_name);
    let ocid = client.get_ocid(&character_name).await?;
    println!("OCID: {}\n", ocid);

    // Step 2: Fetch multiple endpoints in parallel
    println!("Fetching character data in parallel...");
    let (basic, stat, popularity, propensity, dojang) = tokio::join!(
        client.get_character_basic(&ocid, None),
        client.get_character_stat(&ocid, None),
        client.get_character_popularity(&ocid, None),
        client.get_character_propensity(&ocid, None),
        client.get_character_dojang(&ocid, None),
    );

    // ─── Basic ────────────────────────────────────────────────────────────────
    if let Ok(b) = basic {
        println!("=== 기본 정보 ===");
        println!("  이름   : {}", b.character_name.as_deref().unwrap_or("?"));
        println!("  월드   : {}", b.world_name.as_deref().unwrap_or("?"));
        println!("  직업   : {}", b.character_class.as_deref().unwrap_or("?"));
        println!("  레벨   : Lv.{}", b.character_level.unwrap_or(0));
        println!("  경험치 : {}%", b.character_exp_rate.as_deref().unwrap_or("?"));
        println!("  길드   : {}", b.character_guild_name.as_deref().unwrap_or("(없음)"));
    }

    // ─── Popularity ───────────────────────────────────────────────────────────
    if let Ok(p) = popularity {
        println!("\n=== 인기도 ===");
        println!("  인기도 : {}", p.popularity.unwrap_or(0));
    }

    // ─── Propensity ───────────────────────────────────────────────────────────
    if let Ok(pr) = propensity {
        println!("\n=== 성향 ===");
        println!("  카리스마 : {}", pr.charisma_level.unwrap_or(0));
        println!("  감성     : {}", pr.sensibility_level.unwrap_or(0));
        println!("  통찰     : {}", pr.insight_level.unwrap_or(0));
        println!("  의지     : {}", pr.willingness_level.unwrap_or(0));
        println!("  손재주   : {}", pr.handicraft_level.unwrap_or(0));
        println!("  매력     : {}", pr.charm_level.unwrap_or(0));
    }

    // ─── Stat ────────────────────────────────────────────────────────────────
    if let Ok(s) = stat {
        println!("\n=== 주요 스탯 ===");
        if let Some(stats) = s.final_stat {
            // Print the first 10 stats
            for entry in stats.iter().take(10) {
                println!(
                    "  {} : {}",
                    entry.stat_name.as_deref().unwrap_or("?"),
                    entry.stat_value.as_deref().unwrap_or("?")
                );
            }
        }
    }

    // ─── Dojang ───────────────────────────────────────────────────────────────
    if let Ok(d) = dojang {
        println!("\n=== 무릉도장 ===");
        println!("  최고 기록 : {}층", d.dojang_best_floor.unwrap_or(0));
        println!("  클리어 시간 : {}초", d.dojang_best_time.unwrap_or(0));
    }

    // ─── 5th job skills (sequential example) ─────────────────────────────────
    println!("\n=== 5차 스킬 ===");
    match client.get_character_skill(&ocid, SkillGrade::Fifth, None).await {
        Ok(skills) => {
            if let Some(list) = skills.character_skill {
                for skill in list.iter().take(5) {
                    println!(
                        "  [Lv.{}] {}",
                        skill.skill_level.unwrap_or(0),
                        skill.skill_name.as_deref().unwrap_or("?")
                    );
                }
            }
        }
        Err(e) => println!("  스킬 조회 실패: {}", e),
    }

    Ok(())
}
