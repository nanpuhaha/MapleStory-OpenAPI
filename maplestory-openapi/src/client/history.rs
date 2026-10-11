// src/client/history.rs
//! Client methods for History tag API endpoints.

use crate::{
    client::MapleStoryClient,
    error::Result,
    models::{
        common::DateParam,
        history::{
            CubeHistory, CubeHistoryEntry, PotentialHistory, PotentialHistoryEntry,
            SoulPotentialHistory, SoulPotentialHistoryEntry, StarforceHistory,
            StarforceHistoryEntry,
        },
    },
};
use futures_util::stream::{self, BoxStream, StreamExt};

impl MapleStoryClient {
    /// Retrieve starforce enhancement history.
    ///
    /// Corresponds to `GET /maplestory/v1/history/starforce`.
    ///
    /// # Arguments
    /// * `count` — number of results to fetch per page (10 to 1000)
    /// * `date` — optional reference date (KST)
    /// * `cursor` — pagination cursor from previous response
    pub async fn get_starforce_history(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<StarforceHistory> {
        let mut params = vec![("count", count.to_string())];
        let date_param = date.into();
        if let Some(d) = date_param.as_deref() {
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
    /// * `date` — optional reference date (KST)
    /// * `cursor` — pagination cursor from previous response
    pub async fn get_potential_history(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<PotentialHistory> {
        let mut params = vec![("count", count.to_string())];
        let date_param = date.into();
        if let Some(d) = date_param.as_deref() {
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
    /// * `date` — optional reference date (KST)
    /// * `cursor` — pagination cursor from previous response
    pub async fn get_cube_history(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<CubeHistory> {
        let mut params = vec![("count", count.to_string())];
        let date_param = date.into();
        if let Some(d) = date_param.as_deref() {
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
    /// * `date` — optional reference date (KST)
    /// * `cursor` — pagination cursor from previous response
    pub async fn get_soul_potential_history(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<SoulPotentialHistory> {
        let mut params = vec![("count", count.to_string())];
        let date_param = date.into();
        if let Some(d) = date_param.as_deref() {
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
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<StarforceHistory> {
        self.client.get_starforce_history(count, date, cursor).await
    }

    /// Stream starforce enhancement history entries with automatic cursor pagination.
    pub fn get_starforce_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'a, Result<StarforceHistoryEntry>> {
        let client = self.client;
        let date_str = date.into().as_deref().map(|s| s.to_string());

        struct State {
            cursor: Option<String>,
            buffer: std::vec::IntoIter<StarforceHistoryEntry>,
            finished: bool,
        }

        let initial_state = State {
            cursor: None,
            buffer: Vec::new().into_iter(),
            finished: false,
        };

        stream::unfold(initial_state, move |mut state| {
            let date_str = date_str.clone();
            async move {
                loop {
                    if let Some(item) = state.buffer.next() {
                        return Some((Ok(item), state));
                    }

                    if state.finished {
                        return None;
                    }

                    let res = client
                        .get_starforce_history(count, date_str.as_deref(), state.cursor.as_deref())
                        .await;

                    match res {
                        Ok(page) => {
                            let next_cursor = page.next_cursor.filter(|c| !c.is_empty());
                            let entries = page.starforce_history.unwrap_or_default();

                            if entries.is_empty() || next_cursor.is_none() {
                                state.finished = true;
                            }
                            state.cursor = next_cursor;
                            state.buffer = entries.into_iter();
                        }
                        Err(e) => {
                            state.finished = true;
                            return Some((Err(e), state));
                        }
                    }
                }
            }
        })
        .boxed()
    }

    /// Retrieve potential reset history.
    pub async fn get_potential(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<PotentialHistory> {
        self.client.get_potential_history(count, date, cursor).await
    }

    /// Stream potential reset history entries with automatic cursor pagination.
    pub fn get_potential_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'a, Result<PotentialHistoryEntry>> {
        let client = self.client;
        let date_str = date.into().as_deref().map(|s| s.to_string());

        struct State {
            cursor: Option<String>,
            buffer: std::vec::IntoIter<PotentialHistoryEntry>,
            finished: bool,
        }

        let initial_state = State {
            cursor: None,
            buffer: Vec::new().into_iter(),
            finished: false,
        };

        stream::unfold(initial_state, move |mut state| {
            let date_str = date_str.clone();
            async move {
                loop {
                    if let Some(item) = state.buffer.next() {
                        return Some((Ok(item), state));
                    }

                    if state.finished {
                        return None;
                    }

                    let res = client
                        .get_potential_history(count, date_str.as_deref(), state.cursor.as_deref())
                        .await;

                    match res {
                        Ok(page) => {
                            let next_cursor = page.next_cursor.filter(|c| !c.is_empty());
                            let entries = page.potential_history.unwrap_or_default();

                            if entries.is_empty() || next_cursor.is_none() {
                                state.finished = true;
                            }
                            state.cursor = next_cursor;
                            state.buffer = entries.into_iter();
                        }
                        Err(e) => {
                            state.finished = true;
                            return Some((Err(e), state));
                        }
                    }
                }
            }
        })
        .boxed()
    }

    /// Retrieve cube and special item usage history.
    pub async fn get_cube(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<CubeHistory> {
        self.client.get_cube_history(count, date, cursor).await
    }

    /// Stream cube usage history entries with automatic cursor pagination.
    pub fn get_cube_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'a, Result<CubeHistoryEntry>> {
        let client = self.client;
        let date_str = date.into().as_deref().map(|s| s.to_string());

        struct State {
            cursor: Option<String>,
            buffer: std::vec::IntoIter<CubeHistoryEntry>,
            finished: bool,
        }

        let initial_state = State {
            cursor: None,
            buffer: Vec::new().into_iter(),
            finished: false,
        };

        stream::unfold(initial_state, move |mut state| {
            let date_str = date_str.clone();
            async move {
                loop {
                    if let Some(item) = state.buffer.next() {
                        return Some((Ok(item), state));
                    }

                    if state.finished {
                        return None;
                    }

                    let res = client
                        .get_cube_history(count, date_str.as_deref(), state.cursor.as_deref())
                        .await;

                    match res {
                        Ok(page) => {
                            let next_cursor = page.next_cursor.filter(|c| !c.is_empty());
                            let entries = page.cube_history.unwrap_or_default();

                            if entries.is_empty() || next_cursor.is_none() {
                                state.finished = true;
                            }
                            state.cursor = next_cursor;
                            state.buffer = entries.into_iter();
                        }
                        Err(e) => {
                            state.finished = true;
                            return Some((Err(e), state));
                        }
                    }
                }
            }
        })
        .boxed()
    }

    /// Retrieve soul potential reset history.
    pub async fn get_soul_potential(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<SoulPotentialHistory> {
        self.client.get_soul_potential_history(count, date, cursor).await
    }

    /// Stream soul potential reset history entries with automatic cursor pagination.
    pub fn get_soul_potential_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'a, Result<SoulPotentialHistoryEntry>> {
        let client = self.client;
        let date_str = date.into().as_deref().map(|s| s.to_string());

        struct State {
            cursor: Option<String>,
            buffer: std::vec::IntoIter<SoulPotentialHistoryEntry>,
            finished: bool,
        }

        let initial_state = State {
            cursor: None,
            buffer: Vec::new().into_iter(),
            finished: false,
        };

        stream::unfold(initial_state, move |mut state| {
            let date_str = date_str.clone();
            async move {
                loop {
                    if let Some(item) = state.buffer.next() {
                        return Some((Ok(item), state));
                    }

                    if state.finished {
                        return None;
                    }

                    let res = client
                        .get_soul_potential_history(count, date_str.as_deref(), state.cursor.as_deref())
                        .await;

                    match res {
                        Ok(page) => {
                            let next_cursor = page.next_cursor.filter(|c| !c.is_empty());
                            let entries = page.soul_potential_history.unwrap_or_default();

                            if entries.is_empty() || next_cursor.is_none() {
                                state.finished = true;
                            }
                            state.cursor = next_cursor;
                            state.buffer = entries.into_iter();
                        }
                        Err(e) => {
                            state.finished = true;
                            return Some((Err(e), state));
                        }
                    }
                }
            }
        })
        .boxed()
    }
}
