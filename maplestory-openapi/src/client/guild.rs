// src/client/guild.rs
//! Client methods for Guild tag API endpoints.

use crate::{
    client::MapleStoryClient,
    error::{MapleStoryError, Result},
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
        let res = self.get_guild_id(guild_name, world_name).await?;
        res.oguild_id.ok_or_else(|| {
            MapleStoryError::InvalidParameter(format!(
                "Guild '{}' not found in world '{}'",
                guild_name, world_name
            ))
        })
    }

    /// Retrieve basic information of a guild using its `oguild_id`.
    ///
    /// Corresponds to `GET /maplestory/v1/guild/basic`.
    ///
    /// # Arguments
    /// * `oguild_id` — unique open guild identifier
    /// * `date` — optional reference date `"YYYY-MM-DD"` (KST)
    pub async fn get_guild_basic(&self, oguild_id: &str, date: Option<&str>) -> Result<GuildBasic> {
        let mut params = vec![("oguild_id", oguild_id.to_string())];
        if let Some(d) = date {
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
        date: Option<&str>,
    ) -> Result<GuildBasic> {
        let oguild_id = self.get_oguild_id(guild_name, world_name).await?;
        self.get_guild_basic(&oguild_id, date).await
    }
}
