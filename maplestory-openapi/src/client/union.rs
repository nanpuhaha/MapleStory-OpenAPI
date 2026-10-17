// src/client/union.rs
//! Client methods for Union tag API endpoints.

use crate::{
    client::MapleStoryClient,
    error::Result,
    models::common::DateParam,
    models::union::{UserUnion, UserUnionArtifact, UserUnionChampion, UserUnionRaider},
};

fn build_union_params(ocid: &str, date: DateParam) -> Result<Vec<(&'static str, String)>> {
    let mut params = vec![("ocid", ocid.to_string())];
    if let Some(d) = date.as_deref() {
        params.push(("date", MapleStoryClient::validate_date(d)?));
    }
    Ok(params)
}

// ─── UnionClient ─────────────────────────────────────────────────────────────

/// Domain sub-client for Union tag API endpoints.
#[derive(Clone, Copy)]
pub struct UnionClient<'a> {
    pub(crate) client: &'a MapleStoryClient,
}

impl<'a> UnionClient<'a> {
    /// Create a new `UnionClient` referencing a [`MapleStoryClient`].
    pub fn new(client: &'a MapleStoryClient) -> Self {
        Self { client }
    }

    /// Retrieve Union basic information for a character's account.
    pub async fn get_union(&self, ocid: &str, date: impl Into<DateParam>) -> Result<UserUnion> {
        let params = build_union_params(ocid, date.into())?;
        self.client.get("/maplestory/v1/user/union", &params).await
    }

    /// Retrieve Union basic information directly by character name.
    pub async fn get_by_name(&self, character_name: &str) -> Result<UserUnion> {
        self.get_by_name_and_date(character_name, None).await
    }

    /// Retrieve Union basic information directly by character name and query date.
    pub async fn get_by_name_and_date(
        &self,
        character_name: &str,
        date: impl Into<DateParam>,
    ) -> Result<UserUnion> {
        let ocid = self.client.get_ocid(character_name).await?;
        self.get_union(&ocid, date).await
    }

    /// Retrieve Union Raider board and stat information.
    pub async fn get_raider(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<UserUnionRaider> {
        let params = build_union_params(ocid, date.into())?;
        self.client
            .get("/maplestory/v1/user/union-raider", &params)
            .await
    }

    /// Retrieve Union Raider information directly by character name.
    pub async fn get_raider_by_name(&self, character_name: &str) -> Result<UserUnionRaider> {
        let ocid = self.client.get_ocid(character_name).await?;
        self.get_raider(&ocid, None).await
    }

    /// Retrieve Union Artifact information.
    pub async fn get_artifact(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<UserUnionArtifact> {
        let params = build_union_params(ocid, date.into())?;
        self.client
            .get("/maplestory/v1/user/union-artifact", &params)
            .await
    }

    /// Retrieve Union Artifact information directly by character name.
    pub async fn get_artifact_by_name(&self, character_name: &str) -> Result<UserUnionArtifact> {
        let ocid = self.client.get_ocid(character_name).await?;
        self.get_artifact(&ocid, None).await
    }

    /// Retrieve Union Champion information.
    pub async fn get_champion(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<UserUnionChampion> {
        let params = build_union_params(ocid, date.into())?;
        self.client
            .get("/maplestory/v1/user/union-champion", &params)
            .await
    }

    /// Retrieve Union Champion information directly by character name.
    pub async fn get_champion_by_name(&self, character_name: &str) -> Result<UserUnionChampion> {
        let ocid = self.client.get_ocid(character_name).await?;
        self.get_champion(&ocid, None).await
    }
}

impl MapleStoryClient {
    /// Retrieve Union basic information for a character's account.
    ///
    /// Corresponds to `GET /maplestory/v1/user/union`.
    pub async fn get_user_union(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<UserUnion> {
        self.union().get_union(ocid, date).await
    }

    /// Retrieve Union basic information directly by character name.
    pub async fn get_user_union_by_name(&self, character_name: &str) -> Result<UserUnion> {
        self.union().get_by_name(character_name).await
    }

    /// Retrieve Union Raider (공격대) board and stat information.
    ///
    /// Corresponds to `GET /maplestory/v1/user/union-raider`.
    pub async fn get_user_union_raider(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<UserUnionRaider> {
        self.union().get_raider(ocid, date).await
    }

    /// Retrieve Union Raider information directly by character name.
    pub async fn get_user_union_raider_by_name(
        &self,
        character_name: &str,
    ) -> Result<UserUnionRaider> {
        self.union().get_raider_by_name(character_name).await
    }

    /// Retrieve Union Artifact information.
    ///
    /// Corresponds to `GET /maplestory/v1/user/union-artifact`.
    pub async fn get_user_union_artifact(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<UserUnionArtifact> {
        self.union().get_artifact(ocid, date).await
    }

    /// Retrieve Union Artifact information directly by character name.
    pub async fn get_user_union_artifact_by_name(
        &self,
        character_name: &str,
    ) -> Result<UserUnionArtifact> {
        self.union().get_artifact_by_name(character_name).await
    }

    /// Retrieve Union Champion information.
    ///
    /// Corresponds to `GET /maplestory/v1/user/union-champion`.
    pub async fn get_user_union_champion(
        &self,
        ocid: &str,
        date: impl Into<DateParam>,
    ) -> Result<UserUnionChampion> {
        self.union().get_champion(ocid, date).await
    }

    /// Retrieve Union Champion information directly by character name.
    pub async fn get_user_union_champion_by_name(
        &self,
        character_name: &str,
    ) -> Result<UserUnionChampion> {
        self.union().get_champion_by_name(character_name).await
    }
}
