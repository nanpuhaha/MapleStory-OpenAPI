// src/lib.rs
//! # MapleStory OpenAPI Rust Client
//!
//! A production-quality Rust client library for the [NEXON MapleStory OpenAPI](https://openapi.nexon.com/ko/game/maplestory/).
//!
//! ## Features
//! - Full async/await support via `tokio`
//! - All 25 MapleStory OpenAPI endpoints
//! - Type-safe request/response models
//! - One-call nickname lookup (`get_character_by_name`)
//! - Ergonomic builder pattern for client configuration
//! - Comprehensive error handling
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use maplestory_openapi::MapleStoryClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = MapleStoryClient::builder()
//!         .api_key("YOUR_API_KEY")
//!         .build()?;
//!
//!     // One-call lookup by character name
//!     let basic = client.get_character_by_name("캐릭터명").await?;
//!     println!("World: {:?}", basic.world_name);
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod models;

// Re-export commonly used types at crate root
pub use client::{MapleStoryClient, MapleStoryClientBuilder};
pub use error::{MapleStoryError, Result};
pub use models::character::CharacterBasic;
pub use models::common::SkillGrade;