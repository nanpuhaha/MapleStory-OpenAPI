// src/client/guild.rs
//! Client methods for Guild tag API endpoints.

use crate::{
    client::MapleStoryClient,
    error::{MapleStoryError, Result},
    models::common::DateParam,
    models::guild::{GuildBasic, GuildId},
};

impl MapleStoryClient {
    /// Retrieve the guild identifier (oguild_id) by guild name and world name.
    ///
    /// Corresponds to `GET /maplestory/v1/guild/id`.
    ///
    /// # Arguments
    /// * `guild_name` — exact name of the guild
    /// * `world_name` — world/server where the guild is located (e.g. "스카니아")
    ///
    /// # Example
    /// ```rust,no_run
    /// # use maplestory_openapi::MapleStoryClient;
    /// # tokio_test::block_on(async {
    /// let client = MapleStoryClient::new("YOUR_API_KEY").unwrap();
    /// let res = client.get_guild_id("온앤온", "스카니아").await.unwrap();
    /// println!("oguild_id: {:?}", res.oguild_id);
    /// # });
    /// ```
    pub async fn get_guild_id(&self, guild_name: &str, world_name: &str) -> Result<GuildId> {
        let params = [
            ("guild_name", guild_name.to_string()),
            ("world_name", world_name.to_string()),
        ];
        let result: GuildId = self.get("/maplestory/v1/guild/id", &params).await?;
        if result
            .oguild_id
            .as_deref()
            .map(|s| s.is_empty())
            .unwrap_or(true)
        {
            return Err(MapleStoryError::InvalidParameter(format!(
                "Guild '{}' not found in world '{}'",
                guild_name, world_name
            )));
        }
        Ok(result)
    }

    /// Retrieve only the `oguild_id` string for a guild.
    pub async fn get_oguild_id(&self, guild_name: &str, world_name: &str) -> Result<String> {
        if let Some(cached_id) = self.identity_cache.get_oguild_id(guild_name, world_name).await {
            tracing::debug!("OGuildID cache hit for '{}:{}'", guild_name, world_name);
            return Ok(cached_id);
        }

        let res = self.get_guild_id(guild_name, world_name).await?;
        let id = res.oguild_id.ok_or_else(|| {
            MapleStoryError::InvalidParameter(format!(
                "Guild '{}' not found in world '{}'",
                guild_name, world_name
            ))
        })?;

        self.identity_cache.set_oguild_id(guild_name, world_name, &id).await;

        Ok(id)
    }

    /// Retrieve basic information of a guild using its `oguild_id`.
    ///
    /// Corresponds to `GET /maplestory/v1/guild/basic`.
    ///
    /// # Arguments
    /// * `oguild_id` — unique open guild identifier
    /// * `date` — optional reference date `"YYYY-MM-DD"` (KST)
    pub async fn get_guild_basic(
        &self,
        oguild_id: &str,
        date: impl Into<DateParam>,
    ) -> Result<GuildBasic> {
        let mut params = vec![("oguild_id", oguild_id.to_string())];
        let date_param = date.into();
        if let Some(d) = date_param.as_deref() {
            params.push(("date", Self::validate_date(d)?));
        }
        self.get("/maplestory/v1/guild/basic", &params).await
    }

    /// Convenience method: Retrieve basic information of a guild directly by its name and world.
    ///
    /// Performs `get_guild_id` followed by `get_guild_basic`.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use maplestory_openapi::MapleStoryClient;
    /// # tokio_test::block_on(async {
    /// let client = MapleStoryClient::new("YOUR_API_KEY").unwrap();
    /// let guild = client.get_guild_by_name("온앤온", "스카니아", None).await.unwrap();
    /// println!("Guild Lv: {:?}, Master: {:?}", guild.guild_level, guild.guild_master_name);
    /// # });
    /// ```
    pub async fn get_guild_by_name(
        &self,
        guild_name: &str,
        world_name: &str,
        date: impl Into<DateParam>,
    ) -> Result<GuildBasic> {
        let oguild_id = self.get_oguild_id(guild_name, world_name).await?;
        self.get_guild_basic(&oguild_id, date).await
    }
}

// ─── GuildClient ─────────────────────────────────────────────────────────────

/// Domain sub-client for Guild tag API endpoints.
#[derive(Clone, Copy)]
pub struct GuildClient<'a> {
    pub(crate) client: &'a MapleStoryClient,
}

impl<'a> GuildClient<'a> {
    /// Create a new `GuildClient` referencing a [`MapleStoryClient`].
    pub fn new(client: &'a MapleStoryClient) -> Self {
        Self { client }
    }

    /// Retrieve the guild identifier (oguild_id) by guild name and world name.
    pub async fn get_id(&self, guild_name: &str, world_name: &str) -> Result<GuildId> {
        self.client.get_guild_id(guild_name, world_name).await
    }

    /// Retrieve only the `oguild_id` string for a guild (consults identity cache).
    pub async fn get_oguild_id(&self, guild_name: &str, world_name: &str) -> Result<String> {
        self.client.get_oguild_id(guild_name, world_name).await
    }

    /// Retrieve basic information of a guild using its `oguild_id`.
    pub async fn get_basic(&self, oguild_id: &str, date: impl Into<DateParam>) -> Result<GuildBasic> {
        self.client.get_guild_basic(oguild_id, date).await
    }

    /// Retrieve basic information of a guild directly by its name and world.
    pub async fn get_by_name(
        &self,
        guild_name: &str,
        world_name: &str,
        date: impl Into<DateParam>,
    ) -> Result<GuildBasic> {
        self.client.get_guild_by_name(guild_name, world_name, date).await
    }
}
