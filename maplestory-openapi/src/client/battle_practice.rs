// src/client/battle_practice.rs
//! Client methods for Battle Practice (연무장) tag API endpoints.

use crate::{
    client::MapleStoryClient,
    error::Result,
    models::battle_practice::{
        BattlePracticeCharacterInfo, BattlePracticeResult, ReplayId, SkillTimeline,
    },
};

impl MapleStoryClient {
    /// Retrieve replay identifiers for a character.
    ///
    /// Corresponds to `GET /maplestory/v1/battle-practice/replay-id`.
    ///
    /// # Arguments
    /// * `ocid` — character identifier
    pub async fn get_battle_practice_replay_id(&self, ocid: &str) -> Result<ReplayId> {
        let params = vec![("ocid", ocid.to_string())];
        self.get("/maplestory/v1/battle-practice/replay-id", &params)
            .await
    }

    /// Retrieve replay identifiers directly by character name.
    pub async fn get_battle_practice_replay_id_by_name(
        &self,
        character_name: &str,
    ) -> Result<ReplayId> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_battle_practice_replay_id(&ocid).await
    }

    /// Retrieve battle practice measurement result.
    ///
    /// Corresponds to `GET /maplestory/v1/battle-practice/result`.
    ///
    /// # Arguments
    /// * `replay_id` — battle practice replay identifier
    pub async fn get_battle_practice_result(
        &self,
        replay_id: &str,
    ) -> Result<BattlePracticeResult> {
        let params = vec![("replay_id", replay_id.to_string())];
        self.get("/maplestory/v1/battle-practice/result", &params)
            .await
    }

    /// Retrieve battle practice skill timeline.
    ///
    /// Corresponds to `GET /maplestory/v1/battle-practice/skill-timeline`.
    ///
    /// # Arguments
    /// * `replay_id` — battle practice replay identifier
    /// * `page_no` — optional page number
    pub async fn get_battle_practice_skill_timeline(
        &self,
        replay_id: &str,
        page_no: Option<i64>,
    ) -> Result<SkillTimeline> {
        let mut params = vec![("replay_id", replay_id.to_string())];
        if let Some(p) = page_no {
            params.push(("page_no", p.to_string()));
        }
        self.get(
            "/maplestory/v1/battle-practice/skill-timeline",
            &params,
        )
        .await
    }

    /// Retrieve character stats and equipment at battle practice entry.
    ///
    /// Corresponds to `GET /maplestory/v1/battle-practice/character-info`.
    ///
    /// # Arguments
    /// * `replay_id` — battle practice replay identifier
    pub async fn get_battle_practice_character_info(
        &self,
        replay_id: &str,
    ) -> Result<BattlePracticeCharacterInfo> {
        let params = vec![("replay_id", replay_id.to_string())];
        self.get(
            "/maplestory/v1/battle-practice/character-info",
            &params,
        )
        .await
    }
}

// ─── BattlePracticeClient ────────────────────────────────────────────────────

/// Domain sub-client for Battle Practice (연무장) tag API endpoints.
#[derive(Clone, Copy)]
pub struct BattlePracticeClient<'a> {
    pub(crate) client: &'a MapleStoryClient,
}

impl<'a> BattlePracticeClient<'a> {
    /// Create a new `BattlePracticeClient` referencing a [`MapleStoryClient`].
    pub fn new(client: &'a MapleStoryClient) -> Self {
        Self { client }
    }

    /// Retrieve replay identifiers for a character.
    pub async fn get_replay_id(&self, ocid: &str) -> Result<ReplayId> {
        self.client.get_battle_practice_replay_id(ocid).await
    }

    /// Retrieve replay identifiers directly by character name.
    pub async fn get_replay_id_by_name(&self, character_name: &str) -> Result<ReplayId> {
        self.client.get_battle_practice_replay_id_by_name(character_name).await
    }

    /// Retrieve battle practice measurement result.
    pub async fn get_result(&self, replay_id: &str) -> Result<BattlePracticeResult> {
        self.client.get_battle_practice_result(replay_id).await
    }

    /// Retrieve battle practice skill timeline.
    pub async fn get_skill_timeline(
        &self,
        replay_id: &str,
        page_no: Option<i64>,
    ) -> Result<SkillTimeline> {
        self.client.get_battle_practice_skill_timeline(replay_id, page_no).await
    }

    /// Retrieve character stats and equipment at battle practice entry.
    pub async fn get_character_info(&self, replay_id: &str) -> Result<BattlePracticeCharacterInfo> {
        self.client.get_battle_practice_character_info(replay_id).await
    }
}
