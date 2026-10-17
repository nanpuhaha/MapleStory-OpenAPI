use crate::cli::CharArgs;
use crate::ui::{format_number, BOLD, BOLD_CYAN, BOLD_GREEN, BOLD_WHITE, BOLD_YELLOW, DIM, RESET};
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

    println!("\n{BOLD_CYAN}========================================================{RESET}");
    println!(" {BOLD_WHITE}[Character]{RESET} {BOLD}{name}{RESET} ({BOLD_YELLOW}Lv.{level}{RESET} {class})");
    println!(" {DIM}World:{RESET} {world} | {DIM}Guild:{RESET} {guild}");
    println!("{DIM}--------------------------------------------------------{RESET}");

    let combat = dashboard.combat_analytics();
    println!(" {BOLD_WHITE}[Combat & Stats]{RESET}");
    println!("   Combat Power (전투력) : {BOLD_GREEN}{:>12}{RESET}", format_number(combat.combat_power));
    println!("   Damage                : {:>11.2}%", combat.damage);
    println!("   Boss Damage           : {:>11.2}%", combat.boss_damage);
    println!("   Ignore Enemy DEF (IED): {:>11.2}%", combat.ignore_monster_armor);
    println!("   Critical Damage       : {:>11.2}%", combat.critical_damage);
    println!("   Critical Rate         : {:>11.2}%", combat.critical_rate);
    println!("{DIM}--------------------------------------------------------{RESET}");
    println!(" {BOLD_WHITE}[Force & Equipment]{RESET}");
    println!("   Starforce Total       : {BOLD_YELLOW}{:>12} stars{RESET}", combat.total_starforce);
    println!("   Arcane Force          : {:>12}", combat.arcane_force);
    println!("   Authentic Force       : {:>12}", combat.authentic_force);
    if let Some(floor) = combat.dojang_best_floor {
        println!("   Mu Lung Dojo Best     : {:>12}F", floor);
    }

    if let Some(hexa) = &dashboard.hexa_matrix {
        let remaining = hexa.remaining_materials(30);
        let cores = hexa.character_hexa_core_equipment.as_deref().unwrap_or(&[]);
        println!("{DIM}--------------------------------------------------------{RESET}");
        println!(" {BOLD_WHITE}[6th Job HEXA Matrix Progression (Target: Lv.30)]{RESET}");
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
        println!("{DIM}--------------------------------------------------------{RESET}");
        println!(
            " {BOLD_WHITE}[Union Raider]{RESET} Total Level: {} (Grade: {})",
            union.union_level.unwrap_or(0),
            union.union_grade.as_deref().unwrap_or("None")
        );
    }

    println!("{BOLD_CYAN}========================================================{RESET}\n");

    Ok(())
}
