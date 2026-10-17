use crate::cli::StarforceArgs;
use crate::ui::{format_mesos, BOLD_CYAN, BOLD_GREEN, BOLD_WHITE, BOLD_YELLOW, DIM, RESET};
use maplestory_openapi::models::calculator::{StarforceCalculator, StarforceOptions};
use std::error::Error;

pub fn run_starforce(args: &StarforceArgs) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut options = StarforceOptions::default()
        .with_starcatch(args.starcatch)
        .with_1_plus_1(args.event1plus1)
        .with_discount_30(args.discount30)
        .with_safeguard(args.safeguard);

    // Fast simulation samples for CLI responsiveness
    options.simulation_samples = 2_000;

    let estimate = StarforceCalculator::calculate(args.level, args.from, args.to, &options)?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&estimate)?);
        return Ok(());
    }

    println!("\n{BOLD_CYAN}========================================================{RESET}");
    println!(
        " {BOLD_WHITE}[Starforce Calculator]{RESET} Lv.{} Equipment ({BOLD_YELLOW}{}★ -> {}★{RESET})",
        args.level, args.from, args.to
    );
    println!("{DIM}--------------------------------------------------------{RESET}");
    println!(" Applied Modifiers:");
    println!(
        "   - Starcatch (+5%): {}",
        if args.starcatch { "Enabled" } else { "Disabled" }
    );
    println!(
        "   - Sunday 30% Discount: {}",
        if args.discount30 { "Enabled" } else { "Disabled" }
    );
    println!(
        "   - 15~16★ Safeguard (파괴방지): {}",
        if args.safeguard { "Enabled" } else { "Disabled" }
    );
    println!(
        "   - 1+1 Event (<=10★): {}",
        if args.event1plus1 { "Enabled" } else { "Disabled" }
    );
    println!("{DIM}--------------------------------------------------------{RESET}");
    println!(" {BOLD_WHITE}[Expected Results]{RESET}");
    println!(
        "   Expected Meso Cost     : {BOLD_GREEN}{:>14}{RESET}",
        format_mesos(estimate.expected_meso_cost as u64)
    );
    println!(
        "   Expected Attempts      : {:>14.1} times",
        estimate.expected_attempts
    );
    println!(
        "   Expected Destructions  : {:>14.2} booms",
        estimate.expected_destructions
    );
    println!(
        "   Zero-Boom Probability  : {:>13.1}%",
        estimate.no_destruction_probability * 100.0
    );
    println!("{DIM}--------------------------------------------------------{RESET}");
    println!(" {BOLD_WHITE}[Percentile Distribution (Budget Planning)]{RESET}");
    println!(
        "   10% Lucky Run (하위 10%) : {:>14}",
        format_mesos(estimate.percentile_10_cost)
    );
    println!(
        "   50% Median Run (중위값)  : {:>14}",
        format_mesos(estimate.percentile_50_cost)
    );
    println!(
        "   90% Unlucky Run (상위 10%): {:>14}",
        format_mesos(estimate.percentile_90_cost)
    );
    println!("{BOLD_CYAN}========================================================{RESET}\n");

    Ok(())
}
