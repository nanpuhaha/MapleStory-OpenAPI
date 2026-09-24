// examples/quick_lookup.rs
//! One-call nickname quick lookup with combat power and coordination look preview.
//!
//! Run with:
//! ```sh
//! cargo run --example quick_lookup -- 온앤온
//! ```

use maplestory_openapi::MapleStoryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();
    let _ = dotenvy::from_filename("../.env");

    let client = MapleStoryClient::from_env()?;
    let character_name = std::env::args().nth(1).unwrap_or_else(|| "온앤온".to_string());

    println!("🔍 Looking up '{}'...\n", character_name);

    // ─── 1. One-call basic info ───────────────────────────────────────────────
    let info = client.get_character_by_name(&character_name).await?;

    println!("✅ Character found!");
    println!("──────────────────────────────────────────────────");
    println!("  이름   : {}", info.character_name.as_deref().unwrap_or("?"));
    println!("  월드   : {}", info.world_name.as_deref().unwrap_or("?"));
    println!("  직업   : {}", info.character_class.as_deref().unwrap_or("?"));
    println!("  레벨   : Lv.{}", info.character_level.unwrap_or(0));
    println!("  경험치 : {}%", info.character_exp_rate.as_deref().unwrap_or("?"));
    println!("  길드   : {}", info.character_guild_name.as_deref().unwrap_or("(없음)"));

    // ─── 2. Direct combat power & stats ───────────────────────────────────────
    let stat = client.get_character_stat_by_name(&character_name).await?;
    if let Some(cp) = stat.combat_power() {
        // Format with thousand separators
        println!("  전투력 : {}", format_number(cp));
    }
    let (min_att, max_att) = stat.stat_attack();
    if let (Some(min), Some(max)) = (min_att, max_att) {
        println!("  스공   : {} ~ {}", format_number(min), format_number(max));
    }

    // ─── 3. Quick coordination look preview ──────────────────────────────────
    let coord = client.get_character_coordination_by_name(&character_name).await?;
    println!("\n👗 [대표 코디]");
    if let Some(hat) = coord.hat() {
        println!("  - 모자 : {} ({})", hat.name, if hat.is_cash { "캐시" } else { "일반" });
    }
    if let Some(suit) = coord.top() {
        println!("  - 의상 : {} ({})", suit.name, if suit.is_cash { "캐시" } else { "일반" });
    }
    if let Some(weapon) = coord.weapon() {
        println!("  - 무기 : {} ({})", weapon.name, if weapon.is_cash { "캐시" } else { "일반" });
    }

    if let Some(img) = &info.character_image {
        println!("\n🖼  아바타 이미지: {}", img);
    }

    Ok(())
}

fn format_number(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let mut count = 0;
    for ch in s.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
        count += 1;
    }
    result.chars().rev().collect()
}
