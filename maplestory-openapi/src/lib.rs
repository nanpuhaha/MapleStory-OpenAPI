// src/lib.rs
//! # MapleStory OpenAPI Rust Client
//!
//! A production-quality, ergonomic Rust client library for the [NEXON MapleStory OpenAPI](https://openapi.nexon.com/ko/game/maplestory/).
//!
//! ## Key Features
//! - 🚀 **Runtime-agnostic async client** (lightweight dependency footprint)
//! - ⚡ **One-call nickname lookups** — `get_character_by_name()`, `get_character_stat_by_name()`, etc.
//! - 👗 **Consolidated Coordination / Fashion (코디/외형 합성)** — `get_character_coordination_by_name()` automatically prioritizes cash equipment over normal items and handles anvil (신비의 모루) shapes.
//! - 📊 **Aggregated Profile** — `get_character_profile_by_name()` fetches full character stats, equipment, beauty, and dojang in one parallel call.
//! - ⚔️ **Domain Helpers** — `stat.combat_power()`, `stat.boss_damage()`, `equip.weapon()`, `equip.hat()`, etc.
//! - ⚡ **In-memory OCID Caching** — Optional `cache` feature powered by `moka` to reduce API round-trips and preserve rate limits.
//! - 🔒 **Type-safe models** covering all 25 MapleStory OpenAPI endpoints.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use maplestory_openapi::MapleStoryClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 1. Initialize client using NEXON_API_KEY environment variable
//!     let client = MapleStoryClient::from_env()?;
//!
//!     // 2. One-call lookup by character nickname
//!     let basic = client.get_character_by_name("온앤온").await?;
//!     println!("이름: {}", basic.character_name.as_deref().unwrap_or("?"));
//!     println!("직업: {}", basic.character_class.as_deref().unwrap_or("?"));
//!     println!("레벨: Lv.{}", basic.character_level.unwrap_or(0));
//!
//!     // 3. Direct stat lookup with domain helpers
//!     let stat = client.get_character_stat_by_name("온앤온").await?;
//!     println!("전투력: {:?}", stat.combat_power());
//!
//!     // 4. One-call coordination (코디/외형) lookup
//!     let look = client.get_character_coordination_by_name("온앤온").await?;
//!     for item in &look.visible_items {
//!         println!("[{}] {} (캐시 여부: {})", item.slot, item.name, item.is_cash);
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod avatar;
pub mod client;
pub mod error;
pub mod models;

#[cfg(feature = "animation")]
pub use avatar::encode_animated_gif;
pub use avatar::{
    AnimationFrame, AnimationPreset, AvatarAction, AvatarCode, AvatarEmotion, AvatarLookQuery,
    WeaponMotion,
};
pub use client::{
    BattlePracticeClient, BoxFuture, CharacterClient, GuildClient, HistoryClient, IdentityCache,
    MapleStoryClient, MapleStoryClientBuilder, NoopIdentityCache, NoticeClient, RankingClient,
    SchedulerClient, UnionClient, UserClient, DEFAULT_BASE_URL, MIN_DATE,
};
pub use error::{MapleStoryError, Result};
pub use models::battle_practice::{
    BattlePracticeCharacterInfo, BattlePracticeResult, ReplayId, SkillTimeline,
};
pub use models::character::{
    AbilityInfo, CharacterAbility, CharacterAppearance, CharacterBasic, CharacterBeautyEquipment,
    CharacterCashItemEquipment, CharacterCombatAnalytics, CharacterCoordination,
    CharacterDashboard, CharacterDojang, CharacterHexaMatrix, CharacterHexaMatrixStat,
    CharacterHyperStat, CharacterItemEquipment, CharacterOcid, CharacterPopularity,
    CharacterProfile, CharacterStat, CharacterVMatrix, DashboardOptions, DashboardSection,
    DashboardSectionError, HexaCoreEquipment, HexaStatCore, HyperStatEntry, ItemEquipmentDetail,
    ParsedPotentialOption, StatType, VCoreEquipment, VisibleEquipItem,
};
pub use models::common::{DateParam, MapleDate, SkillGrade};
pub use models::guild::{GuildBasic, GuildId, GuildProfile, GuildSkill};
pub use models::history::{CubeHistory, PotentialHistory, SoulPotentialHistory, StarforceHistory};
pub use models::notice::{
    CashshopNoticeDetail, CashshopNoticeList, EventNoticeDetail, EventNoticeList, NoticeDetail,
    NoticeList, UpdateNoticeDetail, UpdateNoticeList,
};
pub use models::ranking::{
    AchievementRanking, AchievementRankingQuery, DojangRanking, DojangRankingQuery, GuildRanking,
    GuildRankingQuery, OverallRanking, OverallRankingQuery, TheSeedRanking, TheSeedRankingQuery,
    UnionRanking, UnionRankingQuery,
};
pub use models::scheduler::CharacterStateResponse;
pub use models::union::{
    UnionBlock, UnionChampionBadgeInfo, UnionChampionInfo, UnionRaiderPreset, UserUnion,
    UserUnionArtifact, UserUnionChampion, UserUnionRaider,
};
pub use models::user::{AccountSummary, User};
