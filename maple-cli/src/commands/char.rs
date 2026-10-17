use crate::cli::CharArgs;
use maplestory_openapi::client::MapleStoryClient;
use std::error::Error;

pub async fn run(
    client: &MapleStoryClient,
    args: &CharArgs,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let dashboard = client
        .character()
        .get_dashboard_by_name(&args.name)
        .await?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&dashboard)?);
        return Ok(());
    }

    let basic = &dashboard.basic;
    let name = basic.character_name.as_deref().unwrap_or("Unknown");
    let level = basic.character_level.unwrap_or(0);
    let class = basic.character_class.as_deref().unwrap_or("Unknown");
    let world = basic.world_name.as_deref().unwrap_or("Unknown");
    let guild = basic.character_guild_name.as_deref().unwrap_or("None");

    println!("\n========================================================");
    println!(" [Character] {} (Lv.{} {})", name, level, class);
    println!(" World: {} | Guild: {}", world, guild);
    println!("--------------------------------------------------------");

    let combat = dashboard.combat_analytics();
    println!(" [Combat & Stats]");
    println!("   Combat Power (전투력) : {:>12}", format_number(combat.combat_power));
    println!("   Damage                : {:>11.2}%", combat.damage);
    println!("   Boss Damage           : {:>11.2}%", combat.boss_damage);
    println!("   Ignore Enemy DEF (IED): {:>11.2}%", combat.ignore_monster_armor);
    println!("   Critical Damage       : {:>11.2}%", combat.critical_damage);
    println!("   Critical Rate         : {:>11.2}%", combat.critical_rate);
    println!("--------------------------------------------------------");
    println!(" [Force & Equipment]");
    println!("   Starforce Total       : {:>12} stars", combat.total_starforce);
    println!("   Arcane Force          : {:>12}", combat.arcane_force);
    println!("   Authentic Force       : {:>12}", combat.authentic_force);
    if let Some(floor) = combat.dojang_best_floor {
        println!("   Mu Lung Dojo Best     : {:>12}F", floor);
    }

    if let Some(hexa) = &dashboard.hexa_matrix {
        let remaining = hexa.remaining_materials(30);
        let cores = hexa.character_hexa_core_equipment.as_deref().unwrap_or(&[]);
        println!("--------------------------------------------------------");
        println!(" [6th Job HEXA Matrix Progression (Target: Lv.30)]");
        println!(
            "   Cores Active: {} (Maxed: {})",
            cores.len(),
            cores.iter().filter(|c| c.hexa_core_level >= Some(30)).count()
        );
        println!(
            "   Remaining Materials   : {} Sol Erda / {} Fragments",
            remaining.total_remaining.sol_erda, remaining.total_remaining.sol_erda_fragments
        );
    }

    if let Some(union) = &dashboard.union {
        println!("--------------------------------------------------------");
        println!(
            " [Union Raider] Total Level: {} (Grade: {})",
            union.union_level.unwrap_or(0),
            union.union_grade.as_deref().unwrap_or("None")
        );
    }

    println!("========================================================\n");

    Ok(())
}

fn format_number(n: i64) -> String {
    if n >= 100_000_000 {
        let eok = n / 100_000_000;
        let man = (n % 100_000_000) / 10_000;
        if man > 0 {
            format!("{}억 {:04}만", eok, man)
        } else {
            format!("{}억", eok)
        }
    } else if n >= 10_000 {
        format!("{}만", n / 10_000)
    } else {
        n.to_string()
    }
}
