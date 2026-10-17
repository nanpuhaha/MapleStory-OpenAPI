use clap::Parser;
use maple_cli::cli::{AvatarFormat, CalcCommands, Cli, Commands, StarforceArgs};
use maple_cli::client_factory::create_client;
use maple_cli::commands::calc;

#[test]
fn test_cli_parsing_char_command() {
    let args = vec!["maple", "char", "아델", "--json"];
    let cli = Cli::try_parse_from(args).expect("Failed to parse char command");

    match cli.command {
        Commands::Char(c) => {
            assert_eq!(c.name, "아델");
            assert!(c.json);
        }
        _ => panic!("Expected Commands::Char"),
    }
}

#[test]
fn test_cli_parsing_avatar_command() {
    let args = vec![
        "maple",
        "avatar",
        "히어로",
        "--action",
        "walk1",
        "--emotion",
        "wink",
        "--format",
        "webp",
        "--out",
        "hero.webp",
    ];
    let cli = Cli::try_parse_from(args).expect("Failed to parse avatar command");

    match cli.command {
        Commands::Avatar(a) => {
            assert_eq!(a.name, "히어로");
            assert_eq!(a.action, "walk1");
            assert_eq!(a.emotion, "wink");
            assert_eq!(a.format, AvatarFormat::Webp);
            assert_eq!(a.out.unwrap().to_str().unwrap(), "hero.webp");
        }
        _ => panic!("Expected Commands::Avatar"),
    }
}

#[test]
fn test_cli_parsing_compare_command() {
    let args = vec!["maple", "compare", "아델", "카인", "--json"];
    let cli = Cli::try_parse_from(args).expect("Failed to parse compare command");

    match cli.command {
        Commands::Compare(c) => {
            assert_eq!(c.name1, "아델");
            assert_eq!(c.name2, "카인");
            assert!(c.json);
        }
        _ => panic!("Expected Commands::Compare"),
    }
}

#[test]
fn test_cli_parsing_starforce_command() {
    let args = vec![
        "maple",
        "calc",
        "starforce",
        "--level",
        "200",
        "--from",
        "17",
        "--to",
        "22",
        "--starcatch",
        "--discount30",
        "--safeguard",
    ];
    let cli = Cli::try_parse_from(args).expect("Failed to parse calc starforce command");

    match cli.command {
        Commands::Calc(CalcCommands::Starforce(s)) => {
            assert_eq!(s.level, 200);
            assert_eq!(s.from, 17);
            assert_eq!(s.to, 22);
            assert!(s.starcatch);
            assert!(s.discount30);
            assert!(s.safeguard);
            assert!(!s.event1plus1);
        }
        _ => panic!("Expected Commands::Calc(CalcCommands::Starforce)"),
    }
}

#[test]
fn test_calc_starforce_execution() {
    let args = StarforceArgs {
        level: 160,
        from: 12,
        to: 17,
        starcatch: true,
        discount30: true,
        safeguard: false,
        event1plus1: false,
        json: true,
    };

    let result = calc::run_starforce(&args);
    assert!(result.is_ok(), "Starforce calculation failed: {:?}", result.err());
}

#[test]
fn test_client_factory_missing_api_key() {
    // When no API key is provided and NEXON_API_KEY is not set or cleared
    let res = create_client(Some("   "), None, false);
    // If env is set, it might pick it up, but empty string should error if env is also empty
    if std::env::var("NEXON_API_KEY").is_err() {
        assert!(res.is_err());
        let err_msg = res.err().unwrap().to_string();
        assert!(err_msg.contains("Missing Nexon API Key"));
    }
}

#[test]
fn test_client_factory_valid_api_key_with_cache() {
    let temp_cache = std::env::temp_dir().join("maple_cli_test_cache");
    let res = create_client(Some("test_dummy_key_12345"), Some(&temp_cache), false);
    assert!(res.is_ok());
    let client = res.unwrap();
    assert_eq!(client.max_retries(), 3);
}
