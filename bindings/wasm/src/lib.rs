use maplestory_openapi::{
    AnimationPreset, AvatarAction, AvatarCode as CoreAvatarCode, AvatarEmotion, AvatarLookQuery,
    WeaponMotion,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmAvatarCode {
    inner: CoreAvatarCode,
}

#[derive(Serialize, Deserialize)]
pub struct WasmAnimationFrame {
    pub url: String,
    pub delay_ms: u32,
}

#[wasm_bindgen]
impl WasmAvatarCode {
    #[wasm_bindgen(constructor)]
    pub fn new(code_or_url: &str) -> Result<WasmAvatarCode, JsValue> {
        let inner = CoreAvatarCode::from_url_or_code(code_or_url)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(Self { inner })
    }

    /// Returns the raw 1024-character Base16 string.
    #[wasm_bindgen]
    pub fn code(&self) -> String {
        self.inner.as_str().to_string()
    }

    /// Returns the 512-byte binary decoded buffer.
    #[wasm_bindgen(js_name = toBytes)]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.to_bytes()
    }

    /// Constructs an official NEXON Look static image URL.
    #[wasm_bindgen(js_name = toLookUrl)]
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

    /// Constructs an animation frame sequence with URLs and delay durations for web rendering.
    ///
    /// Presets: "walk", "wink", "stand", "jump", "sit", "prone", "fly", "blink"
    #[wasm_bindgen(js_name = getAnimationFrames)]
    pub fn get_animation_frames(
        &self,
        preset: &str,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<JsValue, JsValue> {
        let anim_preset = match preset.to_lowercase().as_str() {
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

        let frames: Vec<WasmAnimationFrame> = self
            .inner
            .animation_frame_urls(&anim_preset, width, height)
            .into_iter()
            .map(|(url, delay_ms)| WasmAnimationFrame { url, delay_ms })
            .collect();

        serde_wasm_bindgen::to_value(&frames).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
