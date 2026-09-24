// examples/coordination_lookup.rs
//! Demonstrates fetching synthesized final coordination (코디 정보).
//!
//! Automatically prioritizes cash items over normal equipment,
//! applies anvil (신비의 모루) shapes, and properly handles overall/suit covering bottom pants.
//!
//! Run with:
//! ```sh
//! cargo run --example coordination_lookup -- 온앤온
//! ```

use maplestory_openapi::MapleStoryClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Attempt to load .env from current directory or parent directory
    let _ = dotenvy::dotenv();
    let _ = dotenvy::from_filename("../.env");

    let client = MapleStoryClient::from_env()?;
    let character_name = std::env::args().nth(1).unwrap_or_else(|| "온앤온".to_string());

    println!("🔍 Fetching coordination & fashion for '{}'...\n", character_name);

    let coord = client.get_character_coordination_by_name(&character_name).await?;

    println!("============================================================");
    println!("  👗 {} 님의 최종 코디 (Coordination) 정보", character_name);
    println!("============================================================");

    println!("\n[뷰티 / 외형 기본]");
    if let Some(hair) = &coord.hair {
        println!("  - 헤어: {} (기본: {}, 믹스: {}, 비율: {}%)",
            hair.hair_name.as_deref().unwrap_or("?"),
            hair.base_color.as_deref().unwrap_or("없음"),
            hair.mix_color.as_deref().unwrap_or("없음"),
            hair.mix_rate.as_deref().unwrap_or("0")
        );
    }
    if let Some(face) = &coord.face {
        println!("  - 성형: {} (기본: {}, 믹스: {}, 비율: {}%)",
            face.face_name.as_deref().unwrap_or("?"),
            face.base_color.as_deref().unwrap_or("없음"),
            face.mix_color.as_deref().unwrap_or("없음"),
            face.mix_rate.as_deref().unwrap_or("0")
        );
    }
    if let Some(skin) = &coord.skin {
        println!("  - 피부: {}", skin.skin_name.as_deref().unwrap_or("?"));
    }

    println!("\n[최종 노출 장비 (캐시 우선 + 모루 외형 반영)]");
    for item in &coord.visible_items {
        let tag = if item.is_cash {
            if let Some(label) = &item.cash_label {
                format!("[캐시:{}]", label)
            } else {
                "[캐시]".to_string()
            }
        } else if item.is_anvil {
            format!("[모루 (원래: {})]", item.original_name.as_deref().unwrap_or("?"))
        } else {
            "[일반 장비]".to_string()
        };

        println!("  • {:<8} : {} {}", item.slot, item.name, tag);
    }

    println!("\n[도메인 헬퍼 접근자 테스트]");
    if let Some(hat) = coord.hat() {
        println!("  - 대표 모자: {} ({})", hat.name, if hat.is_cash { "캐시" } else { "일반" });
    }
    if let Some(weapon) = coord.weapon() {
        println!("  - 대표 무기: {} ({})", weapon.name, if weapon.is_cash { "캐시" } else { "일반" });
    }
    if let Some(cape) = coord.cape() {
        println!("  - 대표 망토: {} ({})", cape.name, if cape.is_cash { "캐시" } else { "일반" });
    }

    Ok(())
}
