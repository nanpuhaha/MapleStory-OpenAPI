//! Live API tests for MapleStory OpenAPI.
//!
//! If `NEXON_API_KEY` is present in the environment (or in a `.env` file),
//! this test executes real API calls for "온앤온" and "니니류" and asserts
//! that basic info, stats, coordination, and profile are all correctly synthesized.

use maplestory_openapi::MapleStoryClient;

fn setup_client() -> Option<MapleStoryClient> {
    // Attempt to load .env from current directory or parent directory
    let _ = dotenvy::dotenv();
    let _ = dotenvy::from_filename("../.env");

    MapleStoryClient::from_env().ok()
}

#[tokio::test]
async fn test_live_lookup_and_coordination_for_target_characters() {
    let client = match setup_client() {
        Some(c) => c,
        None => {
            eprintln!("Skipping live API test: NEXON_API_KEY not found in environment or .env");
            return;
        }
    };

    let test_targets = ["온앤온", "니니류"];

    for &name in &test_targets {
        println!("\n=== Live Test for Character: {} ===", name);

        // 1. One-call basic info
        let basic = client
            .get_character_by_name(name)
            .await
            .expect("failed to get basic info");
        assert_eq!(basic.character_name.as_deref(), Some(name));
        println!(
            "  Basic: Lv.{} {} ({})",
            basic.character_level.unwrap_or(0),
            basic.character_class.as_deref().unwrap_or("?"),
            basic.world_name.as_deref().unwrap_or("?")
        );

        // 2. One-call stat info and domain helpers
        let stat = client
            .get_character_stat_by_name(name)
            .await
            .expect("failed to get stat info");
        let cp = stat.combat_power();
        println!("  Combat Power: {:?}", cp);
        assert!(cp.is_some(), "combat power should be present");

        // 3. One-call coordination (코디 정보: 캐시 우선 + 일반 아이템 + 모루 외형)
        let coordination = client
            .get_character_coordination_by_name(name)
            .await
            .expect("failed to get coordination");

        println!("  [Coordination Beauty]");
        if let Some(hair) = &coordination.hair {
            println!("    Hair: {:?}", hair.hair_name);
        }
        if let Some(face) = &coordination.face {
            println!("    Face: {:?}", face.face_name);
        }
        if let Some(skin) = &coordination.skin {
            println!("    Skin: {:?}", skin.skin_name);
        }

        println!("  [Visible Items ({} slots)]", coordination.visible_items.len());
        assert!(
            !coordination.visible_items.is_empty(),
            "character should have visible equipment"
        );

        for item in &coordination.visible_items {
            println!(
                "    - Slot [{}]: {} (Cash: {}, Anvil: {})",
                item.slot, item.name, item.is_cash, item.is_anvil
            );
        }

        // Verify helper methods on coordination
        let hat = coordination.hat();
        println!("    Helper .hat(): {:?}", hat.map(|h| &h.name));

        // 4. One-call comprehensive profile
        let profile = client
            .get_character_profile_by_name(name)
            .await
            .expect("failed to get character profile");
        assert_eq!(profile.basic.character_name.as_deref(), Some(name));
        assert!(!profile.coordination.visible_items.is_empty());
        println!("  Profile fetched successfully!");

        // 5. One-call all-in-one CharacterDashboard
        let dash = client
            .get_character_dashboard_by_name(name)
            .await
            .expect("failed to get character dashboard");
        assert_eq!(dash.character_name(), name);
        assert!(dash.character_level() > 0);
        assert!(dash.combat_power() > 0);
        assert!(dash.union_level().is_some());

        if name == "온앤온" {
            assert_eq!(dash.guild_name(), Some("온앤온"));
            assert!(dash.noblesse_skill_points() > 0);
            assert!(dash.arcane_force() > 0);
            assert!(dash.authentic_force() > 0);
            assert!(dash.dojang_best_floor().is_some());
            assert!(dash.avatar_code().is_some());
            println!("  Dashboard verified for 온앤온: CP={}, Guild={}, Noblesse={}p",
                dash.combat_power(),
                dash.guild_name().unwrap_or(""),
                dash.noblesse_skill_points()
            );
        } else {
            println!("  Dashboard verified for {}: CP={}, Union Lv={:?}",
                name,
                dash.combat_power(),
                dash.union_level()
            );
        }
    }
}
