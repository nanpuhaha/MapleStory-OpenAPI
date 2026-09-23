// examples/quick_lookup.rs
//! One-call nickname quick lookup — the main convenience feature.
//!
//! `get_character_by_name` performs OCID resolution + basic info fetch
//! in a single method call.
//!
//! Run with:
//! ```sh
//! NEXON_API_KEY=your_key cargo run --example quick_lookup -- 캐릭터명
//! ```

use maplestory_openapi::MapleStoryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("NEXON_API_KEY")
        .expect("Set NEXON_API_KEY environment variable");

    // Character name from CLI args (or a default)
    let character_name = std::env::args().nth(1).unwrap_or_else(|| "캐릭터명".to_string());

    let client = MapleStoryClient::new(api_key)?;

    println!("🔍 Looking up '{}'...\n", character_name);

    // ─── One-call quick lookup ────────────────────────────────────────────────
    let info = client.get_character_by_name(&character_name).await?;
    // ─────────────────────────────────────────────────────────────────────────

    println!("✅ Character found!");
    println!("─────────────────────────────────");
    println!("  이름   : {}", info.character_name.as_deref().unwrap_or("?"));
    println!("  월드   : {}", info.world_name.as_deref().unwrap_or("?"));
    println!("  직업   : {}", info.character_class.as_deref().unwrap_or("?"));
    println!("  레벨   : {}", info.character_level.unwrap_or(0));
    println!("  경험치 : {}%", info.character_exp_rate.as_deref().unwrap_or("?"));
    println!("  길드   : {}", info.character_guild_name.as_deref().unwrap_or("(없음)"));
    if let Some(img) = &info.character_image {
        println!("  이미지 : {}", img);
    }

    Ok(())
}
