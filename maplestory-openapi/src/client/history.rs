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
#[cfg(not(target_arch = "wasm32"))]
use futures_util::stream::{self, BoxStream, StreamExt};
#[cfg(target_arch = "wasm32")]
use futures_util::stream::{self, LocalBoxStream as BoxStream, StreamExt};

fn build_history_params(
    count: u32,
    date: DateParam,
    cursor: Option<&str>,
) -> Result<Vec<(&'static str, String)>> {
    let mut params = vec![("count", count.to_string())];
    if let Some(d) = date.as_deref() {
        params.push(("date", MapleStoryClient::validate_date(d)?));
    }
    if let Some(c) = cursor {
        params.push(("cursor", c.to_string()));
    }
    Ok(params)
}

fn history_unfold_stream<'a, T, F, Fut>(
    date: impl Into<DateParam>,
    fetch_page: F,
) -> impl futures_util::Stream<Item = Result<T>> + 'a
where
    T: 'a,
    F: Fn(Option<String>, Option<String>) -> Fut + 'a,
    Fut: std::future::Future<Output = Result<(Option<String>, Vec<T>)>> + 'a,
{
    let date_str = date.into().as_deref().map(|s| s.to_string());

    struct State<T, F> {
        cursor: Option<String>,
        buffer: std::vec::IntoIter<T>,
        finished: bool,
        fetch_page: F,
        date_str: Option<String>,
    }

    let initial_state = State {
        cursor: None,
        buffer: Vec::new().into_iter(),
        finished: false,
        fetch_page,
        date_str,
    };

    stream::unfold(initial_state, move |mut state| async move {
        loop {
            if let Some(item) = state.buffer.next() {
                return Some((Ok(item), state));
            }
            if state.finished {
                return None;
            }

            let res = (state.fetch_page)(state.date_str.clone(), state.cursor.clone()).await;
            match res {
                Ok((next_cursor, entries)) => {
                    let next_cursor = next_cursor.filter(|c| !c.is_empty());
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
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn paginate_history_stream<'a, T, F, Fut>(
    date: impl Into<DateParam>,
    fetch_page: F,
) -> BoxStream<'a, Result<T>>
where
    T: Send + 'a,
    F: Fn(Option<String>, Option<String>) -> Fut + Send + Sync + 'a,
    Fut: std::future::Future<Output = Result<(Option<String>, Vec<T>)>> + Send + 'a,
{
    history_unfold_stream(date, fetch_page).boxed()
}

#[cfg(target_arch = "wasm32")]
fn paginate_history_stream<'a, T, F, Fut>(
    date: impl Into<DateParam>,
    fetch_page: F,
) -> BoxStream<'a, Result<T>>
where
    T: 'a,
    F: Fn(Option<String>, Option<String>) -> Fut + 'a,
    Fut: std::future::Future<Output = Result<(Option<String>, Vec<T>)>> + 'a,
{
    history_unfold_stream(date, fetch_page).boxed_local()
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
        let params = build_history_params(count, date.into(), cursor)?;
        self.client
            .get("/maplestory/v1/history/starforce", &params)
            .await
    }

    /// Stream starforce enhancement history entries with automatic cursor pagination.
    pub fn get_starforce_history_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'a, Result<StarforceHistoryEntry>> {
        let history = *self;
        paginate_history_stream(date, move |d, c| async move {
            let page = history
                .get_starforce(count, d.as_deref(), c.as_deref())
                .await?;
            Ok((page.next_cursor, page.starforce_history.unwrap_or_default()))
        })
    }

    /// Alias for [`get_starforce_history_stream`](Self::get_starforce_history_stream).
    #[inline]
    pub fn get_starforce_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'a, Result<StarforceHistoryEntry>> {
        self.get_starforce_history_stream(count, date)
    }

    /// Retrieve potential reset history.
    pub async fn get_potential(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<PotentialHistory> {
        let params = build_history_params(count, date.into(), cursor)?;
        self.client
            .get("/maplestory/v1/history/potential", &params)
            .await
    }

    /// Stream potential reset history entries with automatic cursor pagination.
    pub fn get_potential_history_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'a, Result<PotentialHistoryEntry>> {
        let history = *self;
        paginate_history_stream(date, move |d, c| async move {
            let page = history
                .get_potential(count, d.as_deref(), c.as_deref())
                .await?;
            Ok((page.next_cursor, page.potential_history.unwrap_or_default()))
        })
    }

    /// Alias for [`get_potential_history_stream`](Self::get_potential_history_stream).
    #[inline]
    pub fn get_potential_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'a, Result<PotentialHistoryEntry>> {
        self.get_potential_history_stream(count, date)
    }

    /// Retrieve cube and special item usage history.
    pub async fn get_cube(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<CubeHistory> {
        let params = build_history_params(count, date.into(), cursor)?;
        self.client
            .get("/maplestory/v1/history/cube", &params)
            .await
    }

    /// Stream cube usage history entries with automatic cursor pagination.
    pub fn get_cube_history_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'a, Result<CubeHistoryEntry>> {
        let history = *self;
        paginate_history_stream(date, move |d, c| async move {
            let page = history.get_cube(count, d.as_deref(), c.as_deref()).await?;
            Ok((page.next_cursor, page.cube_history.unwrap_or_default()))
        })
    }

    /// Alias for [`get_cube_history_stream`](Self::get_cube_history_stream).
    #[inline]
    pub fn get_cube_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'a, Result<CubeHistoryEntry>> {
        self.get_cube_history_stream(count, date)
    }

    /// Retrieve soul potential reset history.
    pub async fn get_soul_potential(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<SoulPotentialHistory> {
        let params = build_history_params(count, date.into(), cursor)?;
        self.client
            .get("/maplestory/v1/history/soul-potential", &params)
            .await
    }

    /// Stream soul potential reset history entries with automatic cursor pagination.
    pub fn get_soul_potential_history_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'a, Result<SoulPotentialHistoryEntry>> {
        let history = *self;
        paginate_history_stream(date, move |d, c| async move {
            let page = history
                .get_soul_potential(count, d.as_deref(), c.as_deref())
                .await?;
            Ok((
                page.next_cursor,
                page.soul_potential_history.unwrap_or_default(),
            ))
        })
    }

    /// Alias for [`get_soul_potential_history_stream`](Self::get_soul_potential_history_stream).
    #[inline]
    pub fn get_soul_potential_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'a, Result<SoulPotentialHistoryEntry>> {
        self.get_soul_potential_history_stream(count, date)
    }
}

impl MapleStoryClient {
    /// Retrieve starforce enhancement history.
    ///
    /// Corresponds to `GET /maplestory/v1/history/starforce`.
    #[inline]
    pub async fn get_starforce_history(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<StarforceHistory> {
        self.history().get_starforce(count, date, cursor).await
    }

    /// Stream starforce enhancement history entries with automatic cursor pagination.
    #[inline]
    pub fn get_starforce_history_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'_, Result<StarforceHistoryEntry>> {
        self.history().get_starforce_history_stream(count, date)
    }

    /// Retrieve potential reset history.
    ///
    /// Corresponds to `GET /maplestory/v1/history/potential`.
    #[inline]
    pub async fn get_potential_history(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<PotentialHistory> {
        self.history().get_potential(count, date, cursor).await
    }

    /// Stream potential reset history entries with automatic cursor pagination.
    #[inline]
    pub fn get_potential_history_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'_, Result<PotentialHistoryEntry>> {
        self.history().get_potential_history_stream(count, date)
    }

    /// Retrieve cube and special item usage history.
    ///
    /// Corresponds to `GET /maplestory/v1/history/cube`.
    #[inline]
    pub async fn get_cube_history(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<CubeHistory> {
        self.history().get_cube(count, date, cursor).await
    }

    /// Stream cube usage history entries with automatic cursor pagination.
    #[inline]
    pub fn get_cube_history_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'_, Result<CubeHistoryEntry>> {
        self.history().get_cube_history_stream(count, date)
    }

    /// Retrieve soul potential reset history.
    ///
    /// Corresponds to `GET /maplestory/v1/history/soul-potential`.
    #[inline]
    pub async fn get_soul_potential_history(
        &self,
        count: u32,
        date: impl Into<DateParam>,
        cursor: Option<&str>,
    ) -> Result<SoulPotentialHistory> {
        self.history().get_soul_potential(count, date, cursor).await
    }

    /// Stream soul potential reset history entries with automatic cursor pagination.
    #[inline]
    pub fn get_soul_potential_history_stream(
        &self,
        count: u32,
        date: impl Into<DateParam>,
    ) -> BoxStream<'_, Result<SoulPotentialHistoryEntry>> {
        self.history()
            .get_soul_potential_history_stream(count, date)
    }
}
