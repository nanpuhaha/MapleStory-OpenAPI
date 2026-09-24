// examples/avatar_animation_gif.rs
//! Example demonstrating how to generate animated GIFs from character look sequences.
//!
//! Run with:
//! ```bash
//! cargo run --example avatar_animation_gif --features animation
//! ```

use maplestory_openapi::{AnimationPreset, MapleStoryClient};
use std::env;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    let character_name = env::args()
        .nth(1)
        .unwrap_or_else(|| "온앤온".to_string());

    println!("========================================================");
    println!(" [메이플스토리 아바타 애니메이션 GIF 생성기]");
    println!(" 대상 캐릭터: {}", character_name);
    println!("========================================================");

    let client = MapleStoryClient::from_env()
        .or_else(|_| MapleStoryClient::new("test"))?;

    let presets = [
        ("walk", AnimationPreset::Walk),
        ("wink", AnimationPreset::Wink),
        ("stand", AnimationPreset::Stand),
        ("jump", AnimationPreset::Jump),
    ];

    let output_dir = Path::new("output_gifs");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    for (name, preset) in presets {
        println!("\n>> [{}] 애니메이션 렌더링 중... (프레임 수: {})", name, preset.frames().len());
        
        let start = std::time::Instant::now();
        let gif_bytes = client
            .render_character_gif_by_name(&character_name, &preset, Some(128), Some(128))
            .await?;
        let elapsed = start.elapsed();

        let file_path = output_dir.join(format!("{}_{}.gif", character_name, name));
        fs::write(&file_path, &gif_bytes)?;

        println!(
            "   [OK] 생성 완료: {} (크기: {} bytes, 소요시간: {:.2?})",
            file_path.display(),
            gif_bytes.len(),
            elapsed
        );
    }

    println!("\n모든 애니메이션 GIF 생성이 완료되었습니다! output_gifs 디렉토리를 확인하세요.");
    Ok(())
}
