use crate::cli::CompareArgs;
use crate::ui::{
    format_diff_colored, format_number, BOLD, BOLD_CYAN, BOLD_WHITE, DIM, RESET,
};
use maplestory_openapi::client::MapleStoryClient;
use serde::Serialize;
use std::error::Error;

#[derive(Serialize)]
struct ComparisonReport {
    character1: String,
    character2: String,
    level_diff: i64,
    combat_power_diff: i64,
    boss_damage_diff: f64,
    ied_diff: f64,
    starforce_diff: i64,
    arcane_force_diff: i64,
    authentic_force_diff: i64,
}

pub async fn run(
    client: &MapleStoryClient,
    args: &CompareArgs,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let char_client = client.character();
    let (dash1_res, dash2_res) = tokio::join!(
        char_client.get_dashboard_by_name(&args.name1),
        char_client.get_dashboard_by_name(&args.name2)
    );

    let dash1 = dash1_res?;
    let dash2 = dash2_res?;

    let b1 = &dash1.basic;
    let b2 = &dash2.basic;

    let name1 = b1.character_name.as_deref().unwrap_or(&args.name1);
    let name2 = b2.character_name.as_deref().unwrap_or(&args.name2);

    let class1 = b1.character_class.as_deref().unwrap_or("Unknown");
    let class2 = b2.character_class.as_deref().unwrap_or("Unknown");

    let world1 = b1.world_name.as_deref().unwrap_or("Unknown");
    let world2 = b2.world_name.as_deref().unwrap_or("Unknown");

    let lvl1 = b1.character_level.unwrap_or(0);
    let lvl2 = b2.character_level.unwrap_or(0);

    let c1 = dash1.combat_analytics();
    let c2 = dash2.combat_analytics();

    let cp1 = c1.combat_power;
    let cp2 = c2.combat_power;

    let bd1 = c1.boss_damage;
    let bd2 = c2.boss_damage;

    let ied1 = c1.ignore_monster_armor;
    let ied2 = c2.ignore_monster_armor;

    let sf1 = c1.total_starforce;
    let sf2 = c2.total_starforce;

    let af1 = c1.arcane_force;
    let af2 = c2.arcane_force;

    let aut1 = c1.authentic_force;
    let aut2 = c2.authentic_force;

    if args.json {
        let report = ComparisonReport {
            character1: name1.to_string(),
            character2: name2.to_string(),
            level_diff: lvl2 - lvl1,
            combat_power_diff: cp2 - cp1,
            boss_damage_diff: bd2 - bd1,
            ied_diff: ied2 - ied1,
            starforce_diff: sf2 - sf1,
            arcane_force_diff: af2 - af1,
            authentic_force_diff: aut2 - aut1,
        };
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("\n{BOLD_CYAN}=========================================================================={RESET}");
    println!(" {BOLD_WHITE}[Spec Comparison]{RESET} {BOLD}{name1}{RESET} vs {BOLD}{name2}{RESET}");
    println!("{DIM}--------------------------------------------------------------------------{RESET}");
    println!(
        " {:<20} | {:<22} | {:<22}",
        "Attribute", name1, name2
    );
    println!("{DIM}--------------------------------------------------------------------------{RESET}");
    println!(
        " {:<20} | {:<22} | {:<22}",
        "Class", class1, class2
    );
    println!(
        " {:<20} | {:<22} | {:<22}",
        "World", world1, world2
    );
    println!(
        " {:<20} | Lv.{:<19} | Lv.{:<19} ({})",
        "Level",
        lvl1,
        lvl2,
        format_diff_colored(lvl2 - lvl1)
    );
    println!(
        " {:<20} | {:>22} | {:>22} ({})",
        "Combat Power (전투력)",
        format_number(cp1),
        format_number(cp2),
        format_diff_colored(cp2 - cp1)
    );
    println!(
        " {:<20} | {:>21.2}% | {:>21.2}% ({:+.2}%)",
        "Boss Damage", bd1, bd2, bd2 - bd1
    );
    println!(
        " {:<20} | {:>21.2}% | {:>21.2}% ({:+.2}%)",
        "IED (방무)", ied1, ied2, ied2 - ied1
    );
    println!(
        " {:<20} | {:>16} stars | {:>16} stars ({})",
        "Starforce Total",
        sf1,
        sf2,
        format_diff_colored(sf2 - sf1)
    );
    println!(
        " {:<20} | {:>22} | {:>22} ({})",
        "Arcane Force",
        af1,
        af2,
        format_diff_colored(af2 - af1)
    );
    println!(
        " {:<20} | {:>22} | {:>22} ({})",
        "Authentic Force",
        aut1,
        aut2,
        format_diff_colored(aut2 - aut1)
    );

    // Slot-by-slot equipment comparison
    println!("{DIM}--------------------------------------------------------------------------{RESET}");
    println!(" {BOLD_WHITE}[Key Equipment Slots Comparison]{RESET}");
    println!("{DIM}--------------------------------------------------------------------------{RESET}");
    let key_slots = ["무기", "모자", "상의", "하의", "장갑", "신발", "망토"];
    for &slot in &key_slots {
        let item1 = dash1.item_equipment.item_equipment.as_deref().unwrap_or(&[])
            .iter()
            .find(|i| i.item_equipment_slot.as_deref() == Some(slot) || i.item_equipment_part.as_deref() == Some(slot));
        let item2 = dash2.item_equipment.item_equipment.as_deref().unwrap_or(&[])
            .iter()
            .find(|i| i.item_equipment_slot.as_deref() == Some(slot) || i.item_equipment_part.as_deref() == Some(slot));

        let desc1 = item1.map(|i| {
            let n = i.item_name.as_deref().unwrap_or("Unknown");
            let sf = i.starforce.as_deref().unwrap_or("0");
            format!("{} ({}★)", n, sf)
        }).unwrap_or_else(|| "-".to_string());

        let desc2 = item2.map(|i| {
            let n = i.item_name.as_deref().unwrap_or("Unknown");
            let sf = i.starforce.as_deref().unwrap_or("0");
            format!("{} ({}★)", n, sf)
        }).unwrap_or_else(|| "-".to_string());

        println!(
            " {:<20} | {:<22} | {:<22}",
            slot, desc1, desc2
        );
    }

    println!("{BOLD_CYAN}=========================================================================={RESET}\n");

    Ok(())
}
