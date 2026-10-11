// src/models/notice.rs
//! Models for Notice tag API endpoints.
//!
//! - `GET /maplestory/v1/notice` → [`NoticeList`]
//! - `GET /maplestory/v1/notice/detail` → [`NoticeDetail`]
//! - `GET /maplestory/v1/notice-update` → [`UpdateNoticeList`]
//! - `GET /maplestory/v1/notice-update/detail` → [`UpdateNoticeDetail`]
//! - `GET /maplestory/v1/notice-event` → [`EventNoticeList`]
//! - `GET /maplestory/v1/notice-event/detail` → [`EventNoticeDetail`]
//! - `GET /maplestory/v1/notice-cashshop` → [`CashshopNoticeList`]
//! - `GET /maplestory/v1/notice-cashshop/detail` → [`CashshopNoticeDetail`]

use serde::{Deserialize, Serialize};

// ─── Standard Notice ─────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/notice`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeList {
    /// Notice list
    pub notice: Option<Vec<NoticeListItem>>,
}

/// A single notice list item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeListItem {
    /// Notice title
    pub title: Option<String>,
    /// Notice link URL
    pub url: Option<String>,
    /// Notice identifier
    pub notice_id: Option<i64>,
    /// Notice registration timestamp (KST)
    pub date: Option<String>,
}

/// Response from `GET /maplestory/v1/notice/detail`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoticeDetail {
    /// Notice title
    pub title: Option<String>,
    /// Notice link URL
    pub url: Option<String>,
    /// Notice body content (HTML)
    pub contents: Option<String>,
    /// Notice registration timestamp (KST)
    pub date: Option<String>,
}

// ─── Update Notice ───────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/notice-update`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNoticeList {
    /// Update notice list
    pub update_notice: Option<Vec<NoticeListItem>>,
}

/// Response from `GET /maplestory/v1/notice-update/detail`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNoticeDetail {
    /// Notice title
    pub title: Option<String>,
    /// Notice link URL
    pub url: Option<String>,
    /// Notice body content (HTML)
    pub contents: Option<String>,
    /// Notice registration timestamp (KST)
    pub date: Option<String>,
}

// ─── Event Notice ────────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/notice-event`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventNoticeList {
    /// Event notice list
    pub event_notice: Option<Vec<EventNoticeListItem>>,
}

/// A single event notice list item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventNoticeListItem {
    /// Notice title
    pub title: Option<String>,
    /// Notice link URL
    pub url: Option<String>,
    /// Notice identifier
    pub notice_id: Option<i64>,
    /// Notice registration timestamp (KST)
    pub date: Option<String>,
    /// Event start timestamp (KST)
    pub date_event_start: Option<String>,
    /// Event end timestamp (KST)
    pub date_event_end: Option<String>,
}

/// Response from `GET /maplestory/v1/notice-event/detail`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventNoticeDetail {
    /// Notice title
    pub title: Option<String>,
    /// Notice link URL
    pub url: Option<String>,
    /// Notice body content (HTML)
    pub contents: Option<String>,
    /// Notice registration timestamp (KST)
    pub date: Option<String>,
    /// Event start timestamp (KST)
    pub date_event_start: Option<String>,
    /// Event end timestamp (KST)
    pub date_event_end: Option<String>,
}

// ─── Cashshop Notice ─────────────────────────────────────────────────────────

/// Response from `GET /maplestory/v1/notice-cashshop`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashshopNoticeList {
    /// Cashshop notice list
    pub cashshop_notice: Option<Vec<CashshopNoticeListItem>>,
}

/// A single cashshop notice list item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashshopNoticeListItem {
    /// Notice title
    pub title: Option<String>,
    /// Notice link URL
    pub url: Option<String>,
    /// Notice identifier
    pub notice_id: Option<i64>,
    /// Notice registration timestamp (KST)
    pub date: Option<String>,
    /// Sale start timestamp (KST)
    pub date_sale_start: Option<String>,
    /// Sale end timestamp (KST)
    pub date_sale_end: Option<String>,
    /// Ongoing sale flag ("true" if ongoing/permanent)
    pub ongoing_flag: Option<String>,
}

/// Response from `GET /maplestory/v1/notice-cashshop/detail`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashshopNoticeDetail {
    /// Notice title
    pub title: Option<String>,
    /// Notice link URL
    pub url: Option<String>,
    /// Notice body content (HTML)
    pub contents: Option<String>,
    /// Notice registration timestamp (KST)
    pub date: Option<String>,
    /// Sale start timestamp (KST)
    pub date_sale_start: Option<String>,
    /// Sale end timestamp (KST)
    pub date_sale_end: Option<String>,
    /// Ongoing sale flag ("true" if ongoing/permanent)
    pub ongoing_flag: Option<String>,
}
