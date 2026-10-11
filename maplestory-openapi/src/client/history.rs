// src/client/history.rs
//! Client methods for History tag API endpoints.

use crate::{
    client::MapleStoryClient,
    error::Result,
    models::history::{CubeHistory, PotentialHistory, SoulPotentialHistory, StarforceHistory},
};

impl MapleStoryClient {
    /// Retrieve starforce enhancement history.
    ///
    /// Corresponds to `GET /maplestory/v1/history/starforce`.
    ///
    /// # Arguments
    /// * `count` — number of results to fetch per page (10 to 1000)
    /// * `date` — optional reference date `"YYYY-MM-DD"` (KST)
    /// * `cursor` — pagination cursor from previous response
    pub async fn get_starforce_history(
        &self,
        count: u32,
        date: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<StarforceHistory> {
        let mut params = vec![("count", count.to_string())];
        if let Some(d) = date {
            params.push(("date", Self::validate_date(d)?));
        }
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        self.get("/maplestory/v1/history/starforce", &params).await
    }

    /// Retrieve potential reset history.
    ///
    /// Corresponds to `GET /maplestory/v1/history/potential`.
    ///
    /// # Arguments
    /// * `count` — number of results to fetch per page (10 to 1000)
    /// * `date` — optional reference date `"YYYY-MM-DD"` (KST)
    /// * `cursor` — pagination cursor from previous response
    pub async fn get_potential_history(
        &self,
        count: u32,
        date: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<PotentialHistory> {
        let mut params = vec![("count", count.to_string())];
        if let Some(d) = date {
            params.push(("date", Self::validate_date(d)?));
        }
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        self.get("/maplestory/v1/history/potential", &params).await
    }

    /// Retrieve cube and special item usage history.
    ///
    /// Corresponds to `GET /maplestory/v1/history/cube`.
    ///
    /// # Arguments
    /// * `count` — number of results to fetch per page (10 to 1000)
    /// * `date` — optional reference date `"YYYY-MM-DD"` (KST)
    /// * `cursor` — pagination cursor from previous response
    pub async fn get_cube_history(
        &self,
        count: u32,
        date: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<CubeHistory> {
        let mut params = vec![("count", count.to_string())];
        if let Some(d) = date {
            params.push(("date", Self::validate_date(d)?));
        }
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        self.get("/maplestory/v1/history/cube", &params).await
    }

    /// Retrieve soul potential reset history.
    ///
    /// Corresponds to `GET /maplestory/v1/history/soul-potential`.
    ///
    /// # Arguments
    /// * `count` — number of results to fetch per page (10 to 1000)
    /// * `date` — optional reference date `"YYYY-MM-DD"` (KST)
    /// * `cursor` — pagination cursor from previous response
    pub async fn get_soul_potential_history(
        &self,
        count: u32,
        date: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<SoulPotentialHistory> {
        let mut params = vec![("count", count.to_string())];
        if let Some(d) = date {
            params.push(("date", Self::validate_date(d)?));
        }
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        self.get("/maplestory/v1/history/soul-potential", &params)
            .await
    }
}

// ─── HistoryClient ───────────────────────────────────────────────────────────

/// Domain sub-client for History tag API endpoints.
#[derive(Clone, Copy)]
pub struct HistoryClient<'a> {
    pub(crate) client: &'a MapleStoryClient,
}

impl<'a> HistoryClient<'a> {
    /// Create a new `HistoryClient` referencing a [`MapleStoryClient`].
    pub fn new(client: &'a MapleStoryClient) -> Self {
        Self { client }
    }

    /// Retrieve starforce enhancement history.
    pub async fn get_starforce(
        &self,
        count: u32,
        date: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<StarforceHistory> {
        self.client.get_starforce_history(count, date, cursor).await
    }

    /// Retrieve potential reset history.
    pub async fn get_potential(
        &self,
        count: u32,
        date: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<PotentialHistory> {
        self.client.get_potential_history(count, date, cursor).await
    }

    /// Retrieve cube and special item usage history.
    pub async fn get_cube(
        &self,
        count: u32,
        date: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<CubeHistory> {
        self.client.get_cube_history(count, date, cursor).await
    }

    /// Retrieve soul potential reset history.
    pub async fn get_soul_potential(
        &self,
        count: u32,
        date: Option<&str>,
        cursor: Option<&str>,
    ) -> Result<SoulPotentialHistory> {
        self.client.get_soul_potential_history(count, date, cursor).await
    }
}
