// src/client/scheduler.rs
//! Client methods for Scheduler tag API endpoints.

use crate::{
    client::MapleStoryClient, error::Result, models::common::DateParam,
    models::scheduler::CharacterStateResponse,
};

// ─── SchedulerClient ─────────────────────────────────────────────────────────

/// Domain sub-client for Scheduler tag API endpoints.
#[derive(Clone, Copy)]
pub struct SchedulerClient<'a> {
    pub(crate) client: &'a MapleStoryClient,
}

impl<'a> SchedulerClient<'a> {
    /// Create a new `SchedulerClient` referencing a [`MapleStoryClient`].
    pub fn new(client: &'a MapleStoryClient) -> Self {
        Self { client }
    }

    /// Retrieve scheduler status for a character.
    ///
    /// Corresponds to `GET /maplestory/v1/scheduler/character-state`.
    pub async fn get_character_state(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterStateResponse> {
        let mut params = vec![("ocid", ocid.to_string())];
        let date_param = date.into();
        if let Some(d) = date_param.as_deref() {
            params.push(("date", MapleStoryClient::validate_date(d)?));
        }
        self.client
            .get("/maplestory/v1/scheduler/character-state", &params)
            .await
    }

    /// Retrieve scheduler status directly by character name.
    pub async fn get_character_state_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterStateResponse> {
        let ocid = self.client.get_ocid(character_name).await?;
        self.get_character_state(&ocid, None).await
    }
}

impl MapleStoryClient {
    /// Retrieve scheduler status for a character.
    pub async fn get_scheduler_character_state(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<CharacterStateResponse> {
        self.scheduler().get_character_state(ocid, date).await
    }

    /// Retrieve scheduler status directly by character name.
    pub async fn get_scheduler_character_state_by_name(
        &self,
        character_name: &str,
    ) -> Result<CharacterStateResponse> {
        self.scheduler()
            .get_character_state_by_name(character_name)
            .await
    }
}
