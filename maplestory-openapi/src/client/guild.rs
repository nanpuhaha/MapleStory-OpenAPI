// src/client/guild.rs
//! Client methods for Guild tag API endpoints.

use crate::{
    client::MapleStoryClient,
    error::{MapleStoryError, Result},
    models::common::DateParam,
    models::guild::{GuildBasic, GuildId},
};

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
        let params = [
            ("guild_name", guild_name.to_string()),
            ("world_name", world_name.to_string()),
        ];
        let result: GuildId = self.client.get("/maplestory/v1/guild/id", &params).await?;
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

    /// Retrieve only the `oguild_id` string for a guild (consults identity cache).
    pub async fn get_oguild_id(&self, guild_name: &str, world_name: &str) -> Result<String> {
        if let Some(cached_id) = self
            .client
            .identity_cache
            .get_oguild_id(guild_name, world_name)
            .await
        {
            tracing::debug!("OGuildID cache hit for '{}:{}'", guild_name, world_name);
            return Ok(cached_id);
        }

        let res = self.get_id(guild_name, world_name).await?;
        let id = res.oguild_id.ok_or_else(|| {
            MapleStoryError::InvalidParameter(format!(
                "Guild '{}' not found in world '{}'",
                guild_name, world_name
            ))
        })?;

        self.client
            .identity_cache
            .set_oguild_id(guild_name, world_name, &id)
            .await;

        Ok(id)
    }

    /// Retrieve basic information of a guild using its `oguild_id`.
    pub async fn get_basic(
        &self,
        oguild_id: &str,
        date: impl Into<DateParam>,
    ) -> Result<GuildBasic> {
        let mut params = vec![("oguild_id", oguild_id.to_string())];
        let date_param = date.into();
        if let Some(d) = date_param.as_deref() {
            params.push(("date", MapleStoryClient::validate_date(d)?));
        }
        self.client.get("/maplestory/v1/guild/basic", &params).await
    }

    /// Retrieve basic information of a guild directly by its name and world.
    pub async fn get_by_name(
        &self,
        guild_name: &str,
        world_name: &str,
        date: impl Into<DateParam>,
    ) -> Result<GuildBasic> {
        let oguild_id = self.get_oguild_id(guild_name, world_name).await?;
        self.get_basic(&oguild_id, date).await
    }

    /// Retrieve an aggregated guild profile for the latest available date.
    pub async fn get_guild_profile(
        &self,
        guild_name: &str,
        world_name: &str,
    ) -> Result<crate::models::guild::GuildProfile> {
        self.get_profile(guild_name, world_name, None).await
    }

    /// Retrieve an aggregated guild profile including Noblesse skills and basic guild details.
    pub async fn get_profile(
        &self,
        guild_name: &str,
        world_name: &str,
        date: impl Into<DateParam>,
    ) -> Result<crate::models::guild::GuildProfile> {
        self.get_profile_with_members(guild_name, world_name, date, false)
            .await
    }

    /// Retrieve an aggregated guild profile, optionally fetching member profiles and combat powers concurrently.
    pub async fn get_profile_with_members(
        &self,
        guild_name: &str,
        world_name: &str,
        date: impl Into<DateParam>,
        fetch_members: bool,
    ) -> Result<crate::models::guild::GuildProfile> {
        let date_param = date.into();
        let date_ref = date_param.as_deref();
        let basic = self.get_by_name(guild_name, world_name, date_ref).await?;

        let mut members = Vec::new();
        let mut member_combat_powers = Vec::new();

        if fetch_members {
            if let Some(member_names) = basic.guild_member.as_deref() {
                let futures = member_names.iter().map(|name| {
                    let client = self.client;
                    async move {
                        let basic_res = client.get_character_by_name(name).await.ok();
                        let cp_res = client
                            .get_character_stat_by_name(name)
                            .await
                            .ok()
                            .and_then(|s| s.combat_power());
                        (basic_res, cp_res)
                    }
                });
                let results = futures_util::future::join_all(futures).await;
                for (m_basic, m_cp) in results {
                    if let Some(b) = m_basic {
                        members.push(b);
                    }
                    if let Some(cp) = m_cp {
                        member_combat_powers.push(cp);
                    }
                }
            }
        }

        Ok(crate::models::guild::GuildProfile::from_basic(
            basic,
            members,
            member_combat_powers,
        ))
    }
}

impl MapleStoryClient {
    /// Retrieve the guild identifier (oguild_id) by guild name and world name.
    ///
    /// Corresponds to `GET /maplestory/v1/guild/id`.
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
        self.guild().get_id(guild_name, world_name).await
    }

    /// Retrieve only the `oguild_id` string for a guild.
    pub async fn get_oguild_id(&self, guild_name: &str, world_name: &str) -> Result<String> {
        self.guild().get_oguild_id(guild_name, world_name).await
    }

    /// Retrieve basic information of a guild using its `oguild_id`.
    ///
    /// Corresponds to `GET /maplestory/v1/guild/basic`.
    pub async fn get_guild_basic(
        &self,
        oguild_id: &str,
        date: impl Into<DateParam>,
    ) -> Result<GuildBasic> {
        self.guild().get_basic(oguild_id, date).await
    }

    /// Convenience method: Retrieve basic information of a guild directly by its name and world.
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
        self.guild().get_by_name(guild_name, world_name, date).await
    }

    /// Retrieve an aggregated guild profile including Noblesse skills and basic guild details.
    pub async fn get_guild_profile(
        &self,
        guild_name: &str,
        world_name: &str,
        date: impl Into<DateParam>,
    ) -> Result<crate::models::guild::GuildProfile> {
        self.guild().get_profile(guild_name, world_name, date).await
    }

    /// Retrieve an aggregated guild profile, optionally fetching member profiles concurrently.
    pub async fn get_guild_profile_with_members(
        &self,
        guild_name: &str,
        world_name: &str,
        date: impl Into<DateParam>,
        fetch_members: bool,
    ) -> Result<crate::models::guild::GuildProfile> {
        self.guild()
            .get_profile_with_members(guild_name, world_name, date, fetch_members)
            .await
    }
}
