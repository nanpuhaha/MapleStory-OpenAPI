// src/client/user.rs
//! Client methods for User tag API endpoints.

use crate::{
    client::{MapleStoryClient, LEGACY_API_KEY_HEADER},
    error::Result,
    models::user::{Achievement, CharacterList, User},
};

impl MapleStoryClient {
    /// Retrieve the character list for the authenticated account.
    ///
    /// Corresponds to `GET /maplestory/v1/character/list`.
    ///
    /// > **Note**: Requires the account to be authenticated. Uses the API key
    /// > attached to an account (not a character-specific key).
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
        self.get("/maplestory/v1/character/list", &[]).await
    }

    /// Retrieve achievement information for the authenticated account.
    ///
    /// Corresponds to `GET /maplestory/v1/user/achievement`.
    ///
    /// # Arguments
    /// * `date` — optional reference date `"YYYY-MM-DD"` (KST). Uses latest if `None`.
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
    pub async fn get_achievement(&self, date: impl Into<crate::models::common::DateParam>) -> Result<Achievement> {
        let mut params: Vec<(&str, String)> = Vec::new();
        let date_param = date.into();
        if let Some(d) = date_param.as_deref() {
            params.push(("date", Self::validate_date(d)?));
        }
        self.get("/maplestory/v1/user/achievement", &params).await
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
        self.get("/maplestory/v1/ouid", &[]).await
    }

    /// Retrieve the account identifier (ouid) using a legacy API key.
    ///
    /// Corresponds to `GET /maplestory/legacy/ouid`.
    ///
    /// # Arguments
    /// * `legacy_api_key` — previous Nexon OpenAPI key
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
        self.get_with_header(
            "/maplestory/legacy/ouid",
            &[],
            LEGACY_API_KEY_HEADER,
            legacy_api_key,
        )
        .await
    }

    /// Retrieve an aggregated summary of the account (roster and achievements).
    pub async fn get_account_summary(
        &self,
        date: impl Into<crate::models::common::DateParam>,
    ) -> Result<crate::models::user::AccountSummary> {
        let (character_list_res, achievement_res) = futures_util::future::join(
            self.get_character_list(),
            self.get_achievement(date),
        )
        .await;

        let character_list = character_list_res?;
        let achievement = achievement_res.ok();

        Ok(crate::models::user::AccountSummary {
            character_list,
            achievement,
        })
    }
}

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
        self.client.get_character_list().await
    }

    /// Retrieve achievement information for the authenticated account.
    pub async fn get_achievement(&self, date: impl Into<crate::models::common::DateParam>) -> Result<Achievement> {
        self.client.get_achievement(date).await
    }

    /// Retrieve the account identifier (ouid) for the authenticated account.
    pub async fn get_ouid(&self) -> Result<User> {
        self.client.get_ouid().await
    }

    /// Retrieve the account identifier (ouid) using a legacy API key.
    pub async fn get_ouid_with_legacy_key(&self, legacy_api_key: &str) -> Result<User> {
        self.client.get_ouid_with_legacy_key(legacy_api_key).await
    }

    /// Retrieve an aggregated summary of the account (roster and achievements).
    pub async fn get_account_summary(
        &self,
        date: impl Into<crate::models::common::DateParam>,
    ) -> Result<crate::models::user::AccountSummary> {
        self.client.get_account_summary(date).await
    }
}
