// src/client/ranking.rs
//! Client methods for Ranking tag API endpoints.

use crate::{
    client::MapleStoryClient,
    error::Result,
    models::ranking::{
        AchievementRanking, AchievementRankingQuery, DojangRanking, DojangRankingQuery,
        GuildRanking, GuildRankingQuery, OverallRanking, OverallRankingQuery, TheSeedRanking,
        TheSeedRankingQuery, UnionRanking, UnionRankingQuery,
    },
};

// ─── RankingClient ───────────────────────────────────────────────────────────

/// Domain sub-client for Ranking tag API endpoints.
#[derive(Clone, Copy)]
pub struct RankingClient<'a> {
    pub(crate) client: &'a MapleStoryClient,
}

impl<'a> RankingClient<'a> {
    /// Create a new `RankingClient` referencing a [`MapleStoryClient`].
    pub fn new(client: &'a MapleStoryClient) -> Self {
        Self { client }
    }

    /// Retrieve overall ranking.
    pub async fn get_overall(&self, query: OverallRankingQuery<'_>) -> Result<OverallRanking> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(date) = query.date {
            params.push(("date", MapleStoryClient::validate_date(date)?));
        }
        if let Some(world_name) = query.world_name {
            params.push(("world_name", world_name.to_string()));
        }
        if let Some(world_type) = query.world_type {
            params.push(("world_type", world_type.to_string()));
        }
        if let Some(class) = query.class {
            params.push(("class", class.to_string()));
        }
        if let Some(ocid) = query.ocid {
            params.push(("ocid", ocid.to_string()));
        }
        if let Some(page) = query.page {
            params.push(("page", page.to_string()));
        }
        self.client
            .get("/maplestory/v1/ranking/overall", &params)
            .await
    }

    /// Retrieve Union ranking.
    pub async fn get_union(&self, query: UnionRankingQuery<'_>) -> Result<UnionRanking> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(date) = query.date {
            params.push(("date", MapleStoryClient::validate_date(date)?));
        }
        if let Some(world_name) = query.world_name {
            params.push(("world_name", world_name.to_string()));
        }
        if let Some(ocid) = query.ocid {
            params.push(("ocid", ocid.to_string()));
        }
        if let Some(page) = query.page {
            params.push(("page", page.to_string()));
        }
        self.client
            .get("/maplestory/v1/ranking/union", &params)
            .await
    }

    /// Retrieve guild ranking.
    pub async fn get_guild(&self, query: GuildRankingQuery<'_>) -> Result<GuildRanking> {
        let mut params: Vec<(&str, String)> =
            vec![("ranking_type", query.ranking_type.to_string())];
        if let Some(date) = query.date {
            params.push(("date", MapleStoryClient::validate_date(date)?));
        }
        if let Some(world_name) = query.world_name {
            params.push(("world_name", world_name.to_string()));
        }
        if let Some(guild_name) = query.guild_name {
            params.push(("guild_name", guild_name.to_string()));
        }
        if let Some(page) = query.page {
            params.push(("page", page.to_string()));
        }
        self.client
            .get("/maplestory/v1/ranking/guild", &params)
            .await
    }

    /// Retrieve Mu Lung Dojang ranking.
    pub async fn get_dojang(&self, query: DojangRankingQuery<'_>) -> Result<DojangRanking> {
        let mut params: Vec<(&str, String)> = vec![("difficulty", query.difficulty.to_string())];
        if let Some(date) = query.date {
            params.push(("date", MapleStoryClient::validate_date(date)?));
        }
        if let Some(world_name) = query.world_name {
            params.push(("world_name", world_name.to_string()));
        }
        if let Some(class) = query.class {
            params.push(("class", class.to_string()));
        }
        if let Some(ocid) = query.ocid {
            params.push(("ocid", ocid.to_string()));
        }
        if let Some(page) = query.page {
            params.push(("page", page.to_string()));
        }
        self.client
            .get("/maplestory/v1/ranking/dojang", &params)
            .await
    }

    /// Retrieve The Seed ranking.
    pub async fn get_the_seed(&self, query: TheSeedRankingQuery<'_>) -> Result<TheSeedRanking> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(date) = query.date {
            params.push(("date", MapleStoryClient::validate_date(date)?));
        }
        if let Some(world_name) = query.world_name {
            params.push(("world_name", world_name.to_string()));
        }
        if let Some(ocid) = query.ocid {
            params.push(("ocid", ocid.to_string()));
        }
        if let Some(page) = query.page {
            params.push(("page", page.to_string()));
        }
        self.client
            .get("/maplestory/v1/ranking/theseed", &params)
            .await
    }

    /// Retrieve achievement ranking.
    pub async fn get_achievement(
        &self,
        query: AchievementRankingQuery<'_>,
    ) -> Result<AchievementRanking> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(date) = query.date {
            params.push(("date", MapleStoryClient::validate_date(date)?));
        }
        if let Some(ocid) = query.ocid {
            params.push(("ocid", ocid.to_string()));
        }
        if let Some(page) = query.page {
            params.push(("page", page.to_string()));
        }
        self.client
            .get("/maplestory/v1/ranking/achievement", &params)
            .await
    }
}

impl MapleStoryClient {
    /// Retrieve overall ranking.
    ///
    /// Corresponds to `GET /maplestory/v1/ranking/overall`.
    pub async fn get_overall_ranking(
        &self,
        query: OverallRankingQuery<'_>,
    ) -> Result<OverallRanking> {
        self.ranking().get_overall(query).await
    }

    /// Retrieve Union ranking.
    ///
    /// Corresponds to `GET /maplestory/v1/ranking/union`.
    #[deprecated(since = "0.1.0", note = "Please use client.ranking().get_union(...) instead")]
    pub async fn get_union_ranking(&self, query: UnionRankingQuery<'_>) -> Result<UnionRanking> {
        self.ranking().get_union(query).await
    }

    /// Retrieve guild ranking.
    ///
    /// Corresponds to `GET /maplestory/v1/ranking/guild`.
    #[deprecated(since = "0.1.0", note = "Please use client.ranking().get_guild(...) instead")]
    pub async fn get_guild_ranking(&self, query: GuildRankingQuery<'_>) -> Result<GuildRanking> {
        self.ranking().get_guild(query).await
    }

    /// Retrieve dojang ranking.
    ///
    /// Corresponds to `GET /maplestory/v1/ranking/dojang`.
    #[deprecated(since = "0.1.0", note = "Please use client.ranking().get_dojang(...) instead")]
    pub async fn get_dojang_ranking(&self, query: DojangRankingQuery<'_>) -> Result<DojangRanking> {
        self.ranking().get_dojang(query).await
    }

    /// Retrieve the seed ranking.
    ///
    /// Corresponds to `GET /maplestory/v1/ranking/theseed`.
    #[deprecated(since = "0.1.0", note = "Please use client.ranking().get_the_seed(...) instead")]
    pub async fn get_theseed_ranking(
        &self,
        query: TheSeedRankingQuery<'_>,
    ) -> Result<TheSeedRanking> {
        self.ranking().get_the_seed(query).await
    }

    /// Retrieve achievement ranking.
    ///
    /// Corresponds to `GET /maplestory/v1/ranking/achievement`.
    #[deprecated(since = "0.1.0", note = "Please use client.ranking().get_achievement(...) instead")]
    pub async fn get_achievement_ranking(
        &self,
        query: AchievementRankingQuery<'_>,
    ) -> Result<AchievementRanking> {
        self.ranking().get_achievement(query).await
    }
}
