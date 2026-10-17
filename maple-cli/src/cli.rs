use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "maple",
    author = "MapleStory OpenAPI Contributors",
    version = "0.1.0",
    about = "Lightweight terminal CLI utility for MapleStory OpenAPI",
    long_about = "A fast, ergonomic command-line tool for MapleStory players and developers.\nQuery character profiles, render avatars, compare specs, and calculate Starforce expected costs."
)]
pub struct Cli {
    /// Nexon OpenAPI key (defaults to $NEXON_API_KEY)
    #[arg(short = 'k', long = "api-key", env = "NEXON_API_KEY")]
    pub api_key: Option<String>,

    /// Custom directory path for local identity cache
    #[arg(long = "cache-dir")]
    pub cache_dir: Option<PathBuf>,

    /// Disable local persistent disk cache
    #[arg(long = "no-cache")]
    pub no_cache: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Inspect character dashboard and combat stats
    Char(CharArgs),

    /// Download and render character avatar as static PNG, animated GIF, or WebP
    Avatar(AvatarArgs),

    /// Compare stats, force, and equipment between two characters
    Compare(CompareArgs),

    /// Calculators and simulators (Starforce, Hexa Matrix, etc.)
    #[command(subcommand)]
    Calc(CalcCommands),
}

#[derive(Args, Debug)]
pub struct CharArgs {
    /// Character name to query
    pub name: String,

    /// Output raw JSON instead of human-readable terminal table
    #[arg(short, long)]
    pub json: bool,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarFormat {
    Png,
    Gif,
    Webp,
}

#[derive(Args, Debug)]
pub struct AvatarArgs {
    /// Character name
    pub name: String,

    /// Action/motion (e.g., stand1, stand2, walk1, alert, shoot1, swingO1)
    #[arg(short, long, default_value = "stand1")]
    pub action: String,

    /// Emotion expression (e.g., default, wink, smile, angry, bewildered)
    #[arg(short, long, default_value = "default")]
    pub emotion: String,

    /// Weapon motion preset (e.g., default, stand1)
    #[arg(short, long, default_value = "default")]
    pub wmotion: String,

    /// Output image format (png, gif, webp)
    #[arg(short, long, value_enum, default_value_t = AvatarFormat::Png)]
    pub format: AvatarFormat,

    /// Destination file path (defaults to <name>.<format>)
    #[arg(short, long)]
    pub out: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct CompareArgs {
    /// First character name (baseline)
    pub name1: String,

    /// Second character name (target to compare)
    pub name2: String,

    /// Output comparison diff as JSON
    #[arg(short, long)]
    pub json: bool,
}

#[derive(Subcommand, Debug)]
pub enum CalcCommands {
    /// Starforce expected cost, destruction risk, and percentile simulation
    Starforce(StarforceArgs),
}

#[derive(Args, Debug)]
pub struct StarforceArgs {
    /// Item equipment level (e.g., 150, 160, 200)
    #[arg(short, long, default_value_t = 160)]
    pub level: u32,

    /// Starting starforce rank (0..24)
    #[arg(long, default_value_t = 12)]
    pub from: u32,

    /// Target starforce rank (1..25)
    #[arg(long, default_value_t = 17)]
    pub to: u32,

    /// Apply Starcatch success rate bonus (+5% multiplicative)
    #[arg(long)]
    pub starcatch: bool,

    /// Apply Sunday Maple 30% meso discount
    #[arg(long)]
    pub discount30: bool,

    /// Apply 15-16 star Safeguard (파괴방지, costs 2x base meso)
    #[arg(long)]
    pub safeguard: bool,

    /// Apply 1+1 event (1 star under 12 gives +2 stars upon success)
    #[arg(long)]
    pub event1plus1: bool,

    /// Output calculation result as JSON
    #[arg(short, long)]
    pub json: bool,
}
