// examples/basic_usage.rs
//! Basic usage example — look up a character's OCID and fetch basic info.
//!
//! Run with:
//! ```sh
//! NEXON_API_KEY=your_key cargo run --example basic_usage
//! ```

use maplestory_openapi::MapleStoryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for debug output
    tracing_subscriber::fmt::init();

    // Read API key from environment variable
    let api_key = std::env::var("NEXON_API_KEY")
        .expect("Set NEXON_API_KEY environment variable to your Nexon OpenAPI key");

    // Build the client
    let client = MapleStoryClient::new(api_key)?;

    // 1. Look up OCID by character name
    let character_name = "캐릭터명을입력하세요";
    println!("Looking up OCID for '{}'...", character_name);

    let ocid_result = client.get_character_id(character_name).await?;
    let ocid = ocid_result.ocid.as_deref().unwrap_or("unknown");
    println!("OCID: {}", ocid);

    // 2. Fetch basic info using the OCID
    let basic = client.get_character_basic(ocid, None).await?;
    println!("\n=== Character Basic Info ===");
    println!("Name  : {}", basic.character_name.as_deref().unwrap_or("?"));
    println!("World : {}", basic.world_name.as_deref().unwrap_or("?"));
    println!(
        "Class : {}",
        basic.character_class.as_deref().unwrap_or("?")
    );
    println!("Level : {}", basic.character_level.unwrap_or(0));
    println!(
        "Guild : {}",
        basic.character_guild_name.as_deref().unwrap_or("(없음)")
    );

    Ok(())
}
