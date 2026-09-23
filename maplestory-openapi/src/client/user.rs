// src/client/user.rs
//! Client methods for User tag API endpoints.

use crate::{
    client::MapleStoryClient,
    error::Result,
    models::user::{Achievement, CharacterList},
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
    pub async fn get_achievement(&self, date: Option<&str>) -> Result<Achievement> {
        let mut params: Vec<(&str, String)> = Vec::new();
        if let Some(d) = date {
            params.push(("date", Self::validate_date(d)?));
        }
        self.get("/maplestory/v1/user/achievement", &params).await
    }
}
