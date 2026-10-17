use clap::Parser;
use maple_cli::cli::{CalcCommands, Cli, Commands};
use maple_cli::client_factory::create_client;
use maple_cli::commands::{avatar, calc, char as char_cmd, compare};
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Calc(CalcCommands::Starforce(args)) => {
            // Starforce calculation does not require an API key or network access
            if let Err(e) = calc::run_starforce(args) {
                eprintln!("Error calculating Starforce: {}", e);
                return ExitCode::FAILURE;
            }
        }
        Commands::Char(args) => {
            let client = match create_client(cli.api_key.as_deref(), cli.cache_dir.as_deref(), cli.no_cache) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Initialization Error: {}", e);
                    return ExitCode::FAILURE;
                }
            };
            if let Err(e) = char_cmd::run(&client, args).await {
                eprintln!("Error fetching character: {}", e);
                return ExitCode::FAILURE;
            }
        }
        Commands::Avatar(args) => {
            let client = match create_client(cli.api_key.as_deref(), cli.cache_dir.as_deref(), cli.no_cache) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Initialization Error: {}", e);
                    return ExitCode::FAILURE;
                }
            };
            if let Err(e) = avatar::run(&client, args).await {
                eprintln!("Error rendering avatar: {}", e);
                return ExitCode::FAILURE;
            }
        }
        Commands::Compare(args) => {
            let client = match create_client(cli.api_key.as_deref(), cli.cache_dir.as_deref(), cli.no_cache) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Initialization Error: {}", e);
                    return ExitCode::FAILURE;
                }
            };
            if let Err(e) = compare::run(&client, args).await {
                eprintln!("Error comparing characters: {}", e);
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
