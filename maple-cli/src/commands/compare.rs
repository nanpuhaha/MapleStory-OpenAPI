use crate::cli::CompareArgs;
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

    println!("\n==========================================================================");
    println!(" [Spec Comparison] {} vs {}", name1, name2);
    println!("--------------------------------------------------------------------------");
    println!(
        " {:<20} | {:<22} | {:<22}",
        "Attribute", name1, name2
    );
    println!("--------------------------------------------------------------------------");
    println!(
        " {:<20} | {:<22} | {:<22}",
        "Class", class1, class2
    );
    println!(
        " {:<20} | {:<22} | {:<22}",
        "World", world1, world2
    );
    println!(
        " {:<20} | Lv.{:<19} | Lv.{:<19} ({:+})",
        "Level", lvl1, lvl2, lvl2 - lvl1
    );
    println!(
        " {:<20} | {:>22} | {:>22} ({})",
        "Combat Power (전투력)",
        format_number(cp1),
        format_number(cp2),
        format_diff(cp2 - cp1)
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
        " {:<20} | {:>16} stars | {:>16} stars ({:+})",
        "Starforce Total", sf1, sf2, sf2 - sf1
    );
    println!(
        " {:<20} | {:>22} | {:>22} ({:+})",
        "Arcane Force", af1, af2, af2 - af1
    );
    println!(
        " {:<20} | {:>22} | {:>22} ({:+})",
        "Authentic Force", aut1, aut2, aut2 - aut1
    );
    println!("==========================================================================\n");

    Ok(())
}

fn format_diff(diff: i64) -> String {
    if diff > 0 {
        format!("+{}", format_number(diff))
    } else if diff < 0 {
        format!("-{}", format_number(diff.abs()))
    } else {
        "0".to_string()
    }
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
