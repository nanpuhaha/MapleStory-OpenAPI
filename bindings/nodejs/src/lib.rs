use maplestory_openapi::{
    AnimationPreset, AvatarAction, AvatarCode as CoreAvatarCode, AvatarEmotion, AvatarLookQuery,
    MapleStoryClient as CoreClient, WeaponMotion,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;

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
        serde_json::to_value(&dash).map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Retrieve character basic profile information.
    #[napi]
    pub async fn get_character_basic(&self, character_name: String) -> Result<serde_json::Value> {
        let basic = self
            .inner
            .get_character_by_name(&character_name)
            .await
            .map_err(|e| Error::from_reason(e.to_string()))?;
        serde_json::to_value(&basic).map_err(|e| Error::from_reason(e.to_string()))
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
        serde_json::to_value(&look).map_err(|e| Error::from_reason(e.to_string()))
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
