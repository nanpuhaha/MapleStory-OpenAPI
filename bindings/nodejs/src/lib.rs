use maplestory_openapi::{
    AnimationPreset, AvatarAction, AvatarCode as CoreAvatarCode, AvatarEmotion, AvatarLookQuery,
    DojangRankingQuery, GuildRankingQuery, MapleStoryClient as CoreClient, OverallRankingQuery,
    UnionRankingQuery, WeaponMotion,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;

macro_rules! to_js_value {
    ($val:expr) => {
        serde_json::to_value($val).map_err(|e| Error::from_reason(e.to_string()))
    };
}

#[napi(js_name = "MapleStoryClient")]
pub struct JsMapleStoryClient {
    inner: Arc<CoreClient>,
}

#[napi]
impl JsMapleStoryClient {
    #[napi(constructor)]
    pub fn new(api_key: String) -> Result<Self> {
        let client = CoreClient::new(&api_key).map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Self {
            inner: Arc::new(client),
        })
    }

    /// Retrieve comprehensive all-in-one character dashboard as a parsed JSON object.
    #[napi]
    pub async fn get_character_dashboard(
        &self,
        character_name: String,
    ) -> Result<serde_json::Value> {
        let dash = self
            .inner
            .get_character_dashboard_by_name(&character_name)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&dash)
    }

    /// Retrieve character basic profile information.
    #[napi]
    pub async fn get_character_basic(&self, character_name: String) -> Result<serde_json::Value> {
        let basic = self
            .inner
            .get_character_by_name(&character_name)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&basic)
    }

    /// Retrieve character coordination look with synthesized visible items.
    #[napi]
    pub async fn get_character_coordination(
        &self,
        character_name: String,
    ) -> Result<serde_json::Value> {
        let look = self
            .inner
            .get_character_coordination_by_name(&character_name)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&look)
    }

    /// Render an animated GIF directly by character name.
    ///
    /// Available presets: "walk", "wink", "stand", "jump", "sit", "prone", "fly", "blink"
    #[napi]
    pub async fn render_character_gif(
        &self,
        character_name: String,
        preset: Option<String>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Buffer> {
        let preset_str = preset.unwrap_or_else(|| "walk".to_string());
        let anim_preset = match preset_str.to_lowercase().as_str() {
            "walk" => AnimationPreset::Walk,
            "wink" => AnimationPreset::Wink,
            "stand" => AnimationPreset::Stand,
            "jump" => AnimationPreset::Jump,
            "sit" => AnimationPreset::Sit,
            "prone" => AnimationPreset::Prone,
            "fly" => AnimationPreset::Fly,
            "blink" => AnimationPreset::Blink,
            _ => AnimationPreset::Walk,
        };

        let gif_bytes = self
            .inner
            .render_character_gif_by_name(
                &character_name,
                &anim_preset,
                Some(width.unwrap_or(128)),
                Some(height.unwrap_or(128)),
            )
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;

        Ok(gif_bytes.into())
    }

    // ─── Guild ───────────────────────────────────────────────────────────────

    /// Retrieve guild basic information by guild name and world name.
    #[napi]
    pub async fn get_guild_basic(
        &self,
        guild_name: String,
        world_name: String,
        date: Option<String>,
    ) -> Result<serde_json::Value> {
        let guild = self
            .inner
            .guild()
            .get_by_name(&guild_name, &world_name, date.as_deref())
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&guild)
    }

    /// Retrieve aggregated guild profile by guild name and world name.
    #[napi]
    pub async fn get_guild_profile(
        &self,
        guild_name: String,
        world_name: String,
        date: Option<String>,
    ) -> Result<serde_json::Value> {
        let profile = self
            .inner
            .guild()
            .get_profile(&guild_name, &world_name, date.as_deref())
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&profile)
    }

    // ─── Union ───────────────────────────────────────────────────────────────

    /// Retrieve Union information by character name.
    #[napi]
    pub async fn get_user_union(
        &self,
        character_name: String,
        date: Option<String>,
    ) -> Result<serde_json::Value> {
        let ocid = self
            .inner
            .get_ocid(&character_name)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let union_data = self
            .inner
            .union()
            .get_union(&ocid, date.as_deref())
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&union_data)
    }

    /// Retrieve Union artifact information by character name.
    #[napi]
    pub async fn get_user_union_artifact(
        &self,
        character_name: String,
        date: Option<String>,
    ) -> Result<serde_json::Value> {
        let ocid = self
            .inner
            .get_ocid(&character_name)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let artifact = self
            .inner
            .union()
            .get_artifact(&ocid, date.as_deref())
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&artifact)
    }

    /// Retrieve Union raider information by character name.
    #[napi]
    pub async fn get_user_union_raider(
        &self,
        character_name: String,
        date: Option<String>,
    ) -> Result<serde_json::Value> {
        let ocid = self
            .inner
            .get_ocid(&character_name)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let raider = self
            .inner
            .union()
            .get_raider(&ocid, date.as_deref())
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&raider)
    }

    // ─── Ranking ─────────────────────────────────────────────────────────────

    /// Retrieve overall ranking.
    #[napi]
    pub async fn get_overall_ranking(
        &self,
        date: Option<String>,
        world_name: Option<String>,
        world_type: Option<i32>,
        class_name: Option<String>,
        ocid: Option<String>,
        page: Option<u32>,
    ) -> Result<serde_json::Value> {
        let query = OverallRankingQuery {
            date: date.as_deref(),
            world_name: world_name.as_deref(),
            world_type,
            class: class_name.as_deref(),
            ocid: ocid.as_deref(),
            page,
        };
        let ranking = self
            .inner
            .ranking()
            .get_overall(query)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&ranking)
    }

    /// Retrieve Union ranking.
    #[napi]
    pub async fn get_union_ranking(
        &self,
        date: Option<String>,
        world_name: Option<String>,
        ocid: Option<String>,
        page: Option<u32>,
    ) -> Result<serde_json::Value> {
        let query = UnionRankingQuery {
            date: date.as_deref(),
            world_name: world_name.as_deref(),
            ocid: ocid.as_deref(),
            page,
        };
        let ranking = self
            .inner
            .ranking()
            .get_union(query)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&ranking)
    }

    /// Retrieve guild ranking.
    #[napi]
    pub async fn get_guild_ranking(
        &self,
        ranking_type: Option<u32>,
        date: Option<String>,
        world_name: Option<String>,
        guild_name: Option<String>,
        page: Option<u32>,
    ) -> Result<serde_json::Value> {
        let query = GuildRankingQuery {
            date: date.as_deref(),
            ranking_type: ranking_type.unwrap_or(0),
            world_name: world_name.as_deref(),
            guild_name: guild_name.as_deref(),
            page,
        };
        let ranking = self
            .inner
            .ranking()
            .get_guild(query)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&ranking)
    }

    /// Retrieve Mu Lung Dojang ranking.
    #[napi]
    pub async fn get_dojang_ranking(
        &self,
        difficulty: Option<u32>,
        date: Option<String>,
        world_name: Option<String>,
        class_name: Option<String>,
        ocid: Option<String>,
        page: Option<u32>,
    ) -> Result<serde_json::Value> {
        let query = DojangRankingQuery {
            date: date.as_deref(),
            difficulty: difficulty.unwrap_or(1),
            world_name: world_name.as_deref(),
            class: class_name.as_deref(),
            ocid: ocid.as_deref(),
            page,
        };
        let ranking = self
            .inner
            .ranking()
            .get_dojang(query)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&ranking)
    }

    // ─── History ─────────────────────────────────────────────────────────────

    /// Retrieve starforce enhancement history.
    #[napi]
    pub async fn get_starforce_history(
        &self,
        count: u32,
        date: Option<String>,
        cursor: Option<String>,
    ) -> Result<serde_json::Value> {
        let hist = self
            .inner
            .history()
            .get_starforce(count, date.as_deref(), cursor.as_deref())
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&hist)
    }

    /// Retrieve potential reset history.
    #[napi]
    pub async fn get_potential_history(
        &self,
        count: u32,
        date: Option<String>,
        cursor: Option<String>,
    ) -> Result<serde_json::Value> {
        let hist = self
            .inner
            .history()
            .get_potential(count, date.as_deref(), cursor.as_deref())
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&hist)
    }

    /// Retrieve cube usage history.
    #[napi]
    pub async fn get_cube_history(
        &self,
        count: u32,
        date: Option<String>,
        cursor: Option<String>,
    ) -> Result<serde_json::Value> {
        let hist = self
            .inner
            .history()
            .get_cube(count, date.as_deref(), cursor.as_deref())
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&hist)
    }

    /// Retrieve soul potential history.
    #[napi]
    pub async fn get_soul_potential_history(
        &self,
        count: u32,
        date: Option<String>,
        cursor: Option<String>,
    ) -> Result<serde_json::Value> {
        let hist = self
            .inner
            .history()
            .get_soul_potential(count, date.as_deref(), cursor.as_deref())
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&hist)
    }

    // ─── Notice & User & Scheduler ───────────────────────────────────────────

    /// Retrieve notice list.
    #[napi]
    pub async fn get_notice_list(&self) -> Result<serde_json::Value> {
        let notices = self
            .inner
            .notice()
            .get_notice_list()
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&notices)
    }

    /// Retrieve update notice list.
    #[napi]
    pub async fn get_update_notice_list(&self) -> Result<serde_json::Value> {
        let notices = self
            .inner
            .notice()
            .get_update_notice_list()
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&notices)
    }

    /// Retrieve event notice list.
    #[napi]
    pub async fn get_event_notice_list(&self) -> Result<serde_json::Value> {
        let notices = self
            .inner
            .notice()
            .get_event_notice_list()
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&notices)
    }

    /// Retrieve cashshop notice list.
    #[napi]
    pub async fn get_cashshop_notice_list(&self) -> Result<serde_json::Value> {
        let notices = self
            .inner
            .notice()
            .get_cashshop_notice_list()
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&notices)
    }

    /// Retrieve account summary across worlds.
    #[napi]
    pub async fn get_account_summary(&self) -> Result<serde_json::Value> {
        let summary = self
            .inner
            .user()
            .get_account_summary(None)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&summary)
    }

    /// Retrieve scheduler character state by character name.
    #[napi]
    pub async fn get_scheduler_character_state(
        &self,
        character_name: String,
    ) -> Result<serde_json::Value> {
        let state = self
            .inner
            .scheduler()
            .get_character_state_by_name(&character_name)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        to_js_value!(&state)
    }
}

#[napi(js_name = "AvatarCode")]
pub struct JsAvatarCode {
    inner: CoreAvatarCode,
}

#[napi]
impl JsAvatarCode {
    #[napi(constructor)]
    pub fn new(code_or_url: String) -> Result<Self> {
        let inner = CoreAvatarCode::from_url_or_code(&code_or_url)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    #[napi]
    pub fn code(&self) -> String {
        self.inner.as_str().to_string()
    }

    #[napi]
    pub fn to_bytes(&self) -> Buffer {
        self.inner.to_bytes().into()
    }

    #[napi]
    pub fn to_look_url(
        &self,
        action: Option<String>,
        emotion: Option<String>,
        wmotion: Option<String>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> String {
        let mut query = AvatarLookQuery::new();
        if let Some(act) = action {
            query.action = Some(AvatarAction::Custom(act));
        }
        if let Some(emo) = emotion {
            query.emotion = Some(AvatarEmotion::Custom(emo));
        }
        if let Some(wm) = wmotion {
            query.wmotion = Some(WeaponMotion::Custom(wm));
        }
        if let (Some(w), Some(h)) = (width, height) {
            query = query.size(w, h);
        }

        self.inner.to_look_url(&query)
    }
}
