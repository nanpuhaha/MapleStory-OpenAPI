// src/client/user.rs
//! Client methods for User tag API endpoints.

use crate::{
    client::{MapleStoryClient, LEGACY_API_KEY_HEADER},
    error::Result,
    models::user::{Achievement, CharacterList, User},
};

// ─── UserClient ──────────────────────────────────────────────────────────────

/// Domain sub-client for User and account tag API endpoints.
#[derive(Clone, Copy)]
pub struct UserClient<'a> {
    pub(crate) client: &'a MapleStoryClient,
}

impl<'a> UserClient<'a> {
    /// Create a new `UserClient` referencing a [`MapleStoryClient`].
    pub fn new(client: &'a MapleStoryClient) -> Self {
        Self { client }
    }

    /// Retrieve the character list for the authenticated account.
    pub async fn get_character_list(&self) -> Result<CharacterList> {
        self.client.get("/maplestory/v1/character/list", &[]).await
    }

    /// Retrieve achievement information for the authenticated account.
    pub async fn get_achievement(
        &self,
        date: impl Into<crate::models::common::DateParam>,
    ) -> Result<Achievement> {
        let mut params: Vec<(&str, String)> = Vec::new();
        let date_param = date.into();
        if let Some(d) = date_param.as_deref() {
            params.push(("date", MapleStoryClient::validate_date(d)?));
        }
        self.client
            .get("/maplestory/v1/user/achievement", &params)
            .await
    }

    /// Retrieve the account identifier (ouid) for the authenticated account.
    pub async fn get_ouid(&self) -> Result<User> {
        self.client.get("/maplestory/v1/ouid", &[]).await
    }

    /// Retrieve the account identifier (ouid) using a legacy API key.
    pub async fn get_ouid_with_legacy_key(&self, legacy_api_key: &str) -> Result<User> {
        self.client
            .get_with_header(
                "/maplestory/legacy/ouid",
                &[],
                LEGACY_API_KEY_HEADER,
                legacy_api_key,
            )
            .await
    }

    /// Retrieve an aggregated summary of the account (roster, achievements, and main character Union).
    pub async fn get_account_summary(
        &self,
        date: impl Into<crate::models::common::DateParam>,
    ) -> Result<crate::models::user::AccountSummary> {
        let (character_list_res, achievement_res) =
            futures_util::future::join(self.get_character_list(), self.get_achievement(date)).await;

        let character_list = character_list_res?;
        let achievement = achievement_res.ok();

        let highest_char_name = character_list
            .account_list
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter_map(|acc| acc.character_list.as_deref())
            .flatten()
            .max_by_key(|c| c.character_level.unwrap_or(0))
            .and_then(|c| c.character_name.clone());

        let union = if let Some(main_name) = highest_char_name.as_deref() {
            self.client.union().get_by_name(main_name).await.ok()
        } else {
            None
        };

        Ok(crate::models::user::AccountSummary::from_parts(
            character_list,
            achievement,
            union,
        ))
    }
}

impl MapleStoryClient {
    /// Retrieve the character list for the authenticated account.
    ///
    /// Corresponds to `GET /maplestory/v1/character/list`.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use maplestory_openapi::MapleStoryClient;
    /// # tokio_test::block_on(async {
    /// let client = MapleStoryClient::new("YOUR_API_KEY").unwrap();
    /// let list = client.get_character_list().await.unwrap();
    /// for account in list.account_list.unwrap_or_default() {
    ///     for ch in account.character_list.unwrap_or_default() {
    ///         println!("{:?} - Lv.{:?}", ch.character_name, ch.character_level);
    ///     }
    /// }
    /// # });
    /// ```
    pub async fn get_character_list(&self) -> Result<CharacterList> {
        self.user().get_character_list().await
    }

    /// Retrieve achievement information for the authenticated account.
    ///
    /// Corresponds to `GET /maplestory/v1/user/achievement`.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use maplestory_openapi::MapleStoryClient;
    /// # tokio_test::block_on(async {
    /// let client = MapleStoryClient::new("YOUR_API_KEY").unwrap();
    /// let achievement = client.get_achievement(None).await.unwrap();
    /// println!("Score: {:?}", achievement.achievement_point);
    /// # });
    /// ```
    pub async fn get_achievement(
        &self,
        date: impl Into<crate::models::common::DateParam>,
    ) -> Result<Achievement> {
        self.user().get_achievement(date).await
    }

    /// Retrieve the account identifier (ouid) for the authenticated account.
    ///
    /// Corresponds to `GET /maplestory/v1/ouid`.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use maplestory_openapi::MapleStoryClient;
    /// # tokio_test::block_on(async {
    /// let client = MapleStoryClient::new("YOUR_API_KEY").unwrap();
    /// let user = client.get_ouid().await.unwrap();
    /// println!("ouid: {:?}", user.ouid);
    /// # });
    /// ```
    pub async fn get_ouid(&self) -> Result<User> {
        self.user().get_ouid().await
    }

    /// Retrieve the account identifier (ouid) using a legacy API key.
    ///
    /// Corresponds to `GET /maplestory/legacy/ouid`.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use maplestory_openapi::MapleStoryClient;
    /// # tokio_test::block_on(async {
    /// let client = MapleStoryClient::new("YOUR_API_KEY").unwrap();
    /// let user = client.get_ouid_with_legacy_key("YOUR_LEGACY_KEY").await.unwrap();
    /// println!("ouid: {:?}", user.ouid);
    /// # });
    /// ```
    pub async fn get_ouid_with_legacy_key(&self, legacy_api_key: &str) -> Result<User> {
        self.user().get_ouid_with_legacy_key(legacy_api_key).await
    }

    /// Retrieve an aggregated summary of the account (roster, achievements, and main character Union).
    pub async fn get_account_summary(
        &self,
        date: impl Into<crate::models::common::DateParam>,
    ) -> Result<crate::models::user::AccountSummary> {
        self.user().get_account_summary(date).await
    }
}
