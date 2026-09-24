// src/client/union.rs
//! Client methods for Union tag API endpoints.

use crate::{
    client::MapleStoryClient,
    error::Result,
    models::union::{UserUnion, UserUnionArtifact, UserUnionRaider},
};

impl MapleStoryClient {
    /// Retrieve Union basic information for a character's account.
    ///
    /// Corresponds to `GET /maplestory/v1/user/union`.
    ///
    /// # Arguments
    /// * `ocid` — character identifier
    /// * `date` — optional reference date `"YYYY-MM-DD"` (KST)
    pub async fn get_user_union(&self, ocid: &str, date: Option<&str>) -> Result<UserUnion> {
        let mut params = vec![("ocid", ocid.to_string())];
        if let Some(d) = date {
            params.push(("date", Self::validate_date(d)?));
        }
        self.get("/maplestory/v1/user/union", &params).await
    }

    /// Retrieve Union basic information directly by character name.
    pub async fn get_user_union_by_name(&self, character_name: &str) -> Result<UserUnion> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_user_union(&ocid, None).await
    }

    /// Retrieve Union Raider (공격대) board and stat information.
    ///
    /// Corresponds to `GET /maplestory/v1/user/union-raider`.
    ///
    /// # Arguments
    /// * `ocid` — character identifier
    /// * `date` — optional reference date `"YYYY-MM-DD"` (KST)
    pub async fn get_user_union_raider(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<UserUnionRaider> {
        let mut params = vec![("ocid", ocid.to_string())];
        if let Some(d) = date {
            params.push(("date", Self::validate_date(d)?));
        }
        self.get("/maplestory/v1/user/union-raider", &params).await
    }

    /// Retrieve Union Raider information directly by character name.
    pub async fn get_user_union_raider_by_name(
        &self,
        character_name: &str,
    ) -> Result<UserUnionRaider> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_user_union_raider(&ocid, None).await
    }

    /// Retrieve Union Artifact information.
    ///
    /// Corresponds to `GET /maplestory/v1/user/union-artifact`.
    ///
    /// # Arguments
    /// * `ocid` — character identifier
    /// * `date` — optional reference date `"YYYY-MM-DD"` (KST)
    pub async fn get_user_union_artifact(
        &self,
        ocid: &str,
        date: Option<&str>,
    ) -> Result<UserUnionArtifact> {
        let mut params = vec![("ocid", ocid.to_string())];
        if let Some(d) = date {
            params.push(("date", Self::validate_date(d)?));
        }
        self.get("/maplestory/v1/user/union-artifact", &params)
            .await
    }

    /// Retrieve Union Artifact information directly by character name.
    pub async fn get_user_union_artifact_by_name(
        &self,
        character_name: &str,
    ) -> Result<UserUnionArtifact> {
        let ocid = self.get_ocid(character_name).await?;
        self.get_user_union_artifact(&ocid, None).await
    }
}
