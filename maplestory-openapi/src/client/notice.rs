// src/client/notice.rs
//! Client methods for Notice tag API endpoints.

use crate::{
    client::MapleStoryClient,
    error::Result,
    models::notice::{
        CashshopNoticeDetail, CashshopNoticeList, EventNoticeDetail, EventNoticeList, NoticeDetail,
        NoticeList, UpdateNoticeDetail, UpdateNoticeList,
    },
};

impl MapleStoryClient {
    /// Retrieve standard notice list.
    ///
    /// Corresponds to `GET /maplestory/v1/notice`.
    pub async fn get_notice_list(&self) -> Result<NoticeList> {
        self.get("/maplestory/v1/notice", &[]).await
    }

    /// Retrieve standard notice detail.
    ///
    /// Corresponds to `GET /maplestory/v1/notice/detail`.
    ///
    /// # Arguments
    /// * `notice_id` — notice identifier
    pub async fn get_notice_detail(&self, notice_id: i64) -> Result<NoticeDetail> {
        let params = vec![("notice_id", notice_id.to_string())];
        self.get("/maplestory/v1/notice/detail", &params).await
    }

    /// Retrieve update notice list.
    ///
    /// Corresponds to `GET /maplestory/v1/notice-update`.
    pub async fn get_update_notice_list(&self) -> Result<UpdateNoticeList> {
        self.get("/maplestory/v1/notice-update", &[]).await
    }

    /// Retrieve update notice detail.
    ///
    /// Corresponds to `GET /maplestory/v1/notice-update/detail`.
    ///
    /// # Arguments
    /// * `notice_id` — notice identifier
    pub async fn get_update_notice_detail(&self, notice_id: i64) -> Result<UpdateNoticeDetail> {
        let params = vec![("notice_id", notice_id.to_string())];
        self.get("/maplestory/v1/notice-update/detail", &params)
            .await
    }

    /// Retrieve ongoing event notice list.
    ///
    /// Corresponds to `GET /maplestory/v1/notice-event`.
    pub async fn get_event_notice_list(&self) -> Result<EventNoticeList> {
        self.get("/maplestory/v1/notice-event", &[]).await
    }

    /// Retrieve ongoing event notice detail.
    ///
    /// Corresponds to `GET /maplestory/v1/notice-event/detail`.
    ///
    /// # Arguments
    /// * `notice_id` — notice identifier
    pub async fn get_event_notice_detail(&self, notice_id: i64) -> Result<EventNoticeDetail> {
        let params = vec![("notice_id", notice_id.to_string())];
        self.get("/maplestory/v1/notice-event/detail", &params)
            .await
    }

    /// Retrieve cashshop notice list.
    ///
    /// Corresponds to `GET /maplestory/v1/notice-cashshop`.
    pub async fn get_cashshop_notice_list(&self) -> Result<CashshopNoticeList> {
        self.get("/maplestory/v1/notice-cashshop", &[]).await
    }

    /// Retrieve cashshop notice detail.
    ///
    /// Corresponds to `GET /maplestory/v1/notice-cashshop/detail`.
    ///
    /// # Arguments
    /// * `notice_id` — notice identifier
    pub async fn get_cashshop_notice_detail(&self, notice_id: i64) -> Result<CashshopNoticeDetail> {
        let params = vec![("notice_id", notice_id.to_string())];
        self.get("/maplestory/v1/notice-cashshop/detail", &params)
            .await
    }
}

// ─── NoticeClient ────────────────────────────────────────────────────────────

/// Domain sub-client for Notice tag API endpoints.
#[derive(Clone, Copy)]
pub struct NoticeClient<'a> {
    pub(crate) client: &'a MapleStoryClient,
}

impl<'a> NoticeClient<'a> {
    /// Create a new `NoticeClient` referencing a [`MapleStoryClient`].
    pub fn new(client: &'a MapleStoryClient) -> Self {
        Self { client }
    }

    /// Retrieve standard notice list.
    pub async fn get_notice_list(&self) -> Result<NoticeList> {
        self.client.get_notice_list().await
    }

    /// Retrieve standard notice detail.
    pub async fn get_notice_detail(&self, notice_id: i64) -> Result<NoticeDetail> {
        self.client.get_notice_detail(notice_id).await
    }

    /// Retrieve update notice list.
    pub async fn get_update_notice_list(&self) -> Result<UpdateNoticeList> {
        self.client.get_update_notice_list().await
    }

    /// Retrieve update notice detail.
    pub async fn get_update_notice_detail(&self, notice_id: i64) -> Result<UpdateNoticeDetail> {
        self.client.get_update_notice_detail(notice_id).await
    }

    /// Retrieve ongoing event notice list.
    pub async fn get_event_notice_list(&self) -> Result<EventNoticeList> {
        self.client.get_event_notice_list().await
    }

    /// Retrieve ongoing event notice detail.
    pub async fn get_event_notice_detail(&self, notice_id: i64) -> Result<EventNoticeDetail> {
        self.client.get_event_notice_detail(notice_id).await
    }

    /// Retrieve cashshop notice list.
    pub async fn get_cashshop_notice_list(&self) -> Result<CashshopNoticeList> {
        self.client.get_cashshop_notice_list().await
    }

    /// Retrieve cashshop notice detail.
    pub async fn get_cashshop_notice_detail(&self, notice_id: i64) -> Result<CashshopNoticeDetail> {
        self.client.get_cashshop_notice_detail(notice_id).await
    }
}
