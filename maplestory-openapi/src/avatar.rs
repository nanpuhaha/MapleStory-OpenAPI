// src/avatar.rs
//! # MapleStory Character Avatar & Look URL Decoding Module
//!
//! Provides utilities for parsing, building, and inspecting MapleStory character avatar URLs
//! and decoding the underlying 1024-character hexadecimal/nibble avatar state blocks.
//!
//! ## Avatar Domains Comparison
//!
//! MapleStory character appearance images are served through two primary URL formats:
//!
//! | Category | Nexon Open API Static CDN (`character/look/*`) | Legacy / Web Avatar Server (`Character/*`) |
//! | :--- | :--- | :--- |
//! | **Base URL** | `https://open.api.nexon.com/static/maplestory/character/look/{CODE}` | `https://avatar.maplestory.nexon.com/Character/{CODE}.png` |
//! | **Origin** | Modern NEXON Open API official endpoint (`CharacterBasic.character_image`) | MapleStory official website rankings / character cards |
//! | **Query Params** | Supported: `action` (pose), `emotion` (face expression), `wmotion` (weapon), `width`/`height` | Limited or unsupported (fixed standard preview) |
//! | **Resolution** | Dynamic via `width`/`height` params (e.g. 180, 200, 300) | Fixed or path-based prefix (e.g., `/Character/180/{CODE}.png`) |
//! | **CORS** | Configured for Open API clients | Often restricted by CORS policies for third-party browser canvas |
//!
//! Both endpoints use the exact same **1024-character Base16 (A–P) Avatar Code** identifying the full coordination state.

use crate::error::{MapleStoryError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a validated 1024-character (or variable length) MapleStory Avatar Code.
///
/// The code is composed strictly of the 16 characters `A` through `P`, corresponding
/// to 4-bit nibbles (0x0 to 0xF) in the game engine's packed coordination payload.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AvatarCode(String);

impl AvatarCode {
    /// Validates and constructs an `AvatarCode` from a raw string.
    pub fn new(code: impl Into<String>) -> Result<Self> {
        let s = code.into();
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err(MapleStoryError::InvalidParameter(
                "Avatar code cannot be empty".to_string(),
            ));
        }

        // Validate that all characters are within 'A'..='P'
        if let Some(invalid_char) = trimmed.chars().find(|c| !matches!(c, 'A'..='P')) {
            return Err(MapleStoryError::InvalidParameter(format!(
                "Invalid character '{invalid_char}' in avatar code. Only 'A' through 'P' are allowed."
            )));
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Parses an avatar code from any MapleStory avatar URL or raw code.
    ///
    /// Supports:
    /// - `https://open.api.nexon.com/static/maplestory/character/look/{CODE}?wmotion=W00`
    /// - `https://avatar.maplestory.nexon.com/Character/{CODE}.png`
    /// - `https://avatar.maplestory.nexon.com/Character/180/{CODE}.png`
    /// - Raw code string (e.g., `"JFKCLCKKAMIBFP..."`)
    pub fn from_url_or_code(input: &str) -> Result<Self> {
        let trimmed = input.trim();

        // 1. Remove query parameters if present
        let without_query = match trimmed.split_once('?') {
            Some((path, _)) => path,
            None => trimmed,
        };

        // 2. Remove trailing .png extension if present
        let without_ext = without_query.strip_suffix(".png").unwrap_or(without_query);

        // 3. Extract the last path segment (or after /Character/ or /look/)
        let raw_code = if let Some(idx) = without_ext.rfind('/') {
            &without_ext[idx + 1..]
        } else {
            without_ext
        };

        Self::new(raw_code)
    }

    /// Returns the raw avatar code string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the length of the avatar code (typically 1024 characters).
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the code is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Decodes the A–P nibbles into 512 raw binary bytes.
    ///
    /// Each character 'A'..='P' represents a 4-bit nibble (0..15).
    /// Two consecutive characters form 1 byte (high nibble first).
    #[allow(clippy::chunks_exact_to_as_chunks)]
    pub fn to_bytes(&self) -> Vec<u8> {
        let nibbles: Vec<u8> = self.0.bytes().map(|b| b - b'A').collect();
        let mut bytes = Vec::with_capacity(nibbles.len() / 2);

        for chunk in nibbles.chunks_exact(2) {
            bytes.push((chunk[0] << 4) | chunk[1]);
        }

        bytes
    }

    /// Constructs an official NEXON Open API static look image URL.
    pub fn to_look_url(&self, query: &AvatarLookQuery) -> String {
        let base = format!(
            "https://open.api.nexon.com/static/maplestory/character/look/{}",
            self.0
        );
        let q = query.to_query_string();
        if q.is_empty() {
            base
        } else {
            format!("{base}?{q}")
        }
    }

    /// Constructs a legacy website avatar image URL.
    ///
    /// - `resolution`: Optional pixel size prefix (e.g. `Some(180)` produces `/Character/180/{CODE}.png`).
    pub fn to_legacy_avatar_url(&self, resolution: Option<u32>) -> String {
        match resolution {
            Some(res) => format!(
                "https://avatar.maplestory.nexon.com/Character/{}/{}.png",
                res, self.0
            ),
            None => format!(
                "https://avatar.maplestory.nexon.com/Character/{}.png",
                self.0
            ),
        }
    }

    /// Generates a list of frame URLs with their respective delay (in milliseconds)
    /// based on the provided animation preset and dimension options.
    pub fn animation_frame_urls(
        &self,
        preset: &AnimationPreset,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Vec<(String, u32)> {
        preset
            .frames()
            .into_iter()
            .map(|frame| {
                let mut query = AvatarLookQuery::new()
                    .action(frame.action)
                    .emotion(frame.emotion)
                    .weapon_motion(frame.wmotion);
                if let (Some(w), Some(h)) = (width, height) {
                    query = query.size(w, h);
                }
                (self.to_look_url(&query), frame.delay_ms)
            })
            .collect()
    }
}

/// Encodes a list of PNG image buffers and frame durations into an animated GIF.
///
/// Available when the `animation` feature is enabled.
#[cfg(feature = "animation")]
pub fn encode_animated_gif(frames_data: &[(Vec<u8>, u32)]) -> crate::Result<Vec<u8>> {
    use image::codecs::gif::{GifEncoder, Repeat};
    use image::{Delay, Frame};
    use std::time::Duration;

    let mut buffer = Vec::new();
    {
        let mut encoder = GifEncoder::new(&mut buffer);
        encoder.set_repeat(Repeat::Infinite).map_err(|e| {
            crate::MapleStoryError::Other(format!("Failed to set GIF repeat mode: {e}"))
        })?;

        for (png_bytes, delay_ms) in frames_data {
            let img = image::load_from_memory(png_bytes).map_err(|e| {
                crate::MapleStoryError::Other(format!("Failed to decode PNG frame: {e}"))
            })?;
            let rgba = img.to_rgba8();
            let delay = Delay::from_saturating_duration(Duration::from_millis(*delay_ms as u64));
            let frame = Frame::from_parts(rgba, 0, 0, delay);
            encoder.encode_frame(frame).map_err(|e| {
                crate::MapleStoryError::Other(format!("Failed to encode GIF frame: {e}"))
            })?;
        }
    }

    Ok(buffer)
}

impl fmt::Display for AvatarCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Character Action / Pose for avatar rendering.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum AvatarAction {
    #[default]
    Stand1,
    Stand2,
    Walk1,
    Walk2,
    Prone,
    Fly,
    Jump,
    Sit,
    Ladder,
    Rope,
    Heal,
    Alert,
    ProneStab,
    SwingO1,
    SwingO2,
    SwingO3,
    SwingOF,
    SwingT1,
    SwingT2,
    SwingT3,
    SwingTF,
    StabO1,
    StabO2,
    StabOF,
    StabT1,
    StabT2,
    StabTF,
    Shoot1,
    Shoot2,
    ShootF,
    Dead,
    GhostWalk,
    Custom(String),
}

impl AvatarAction {
    pub fn as_code(&self) -> &str {
        match self {
            Self::Stand1 => "A00",
            Self::Stand2 => "A01",
            Self::Walk1 => "A02",
            Self::Walk2 => "A03",
            Self::Prone => "A04",
            Self::Fly => "A05",
            Self::Jump => "A06",
            Self::Sit => "A07",
            Self::Ladder => "A08",
            Self::Rope => "A09",
            Self::Heal => "A10",
            Self::Alert => "A11",
            Self::ProneStab => "A12",
            Self::SwingO1 => "A13",
            Self::SwingO2 => "A14",
            Self::SwingO3 => "A15",
            Self::SwingOF => "A16",
            Self::SwingT1 => "A17",
            Self::SwingT2 => "A18",
            Self::SwingT3 => "A19",
            Self::SwingTF => "A20",
            Self::StabO1 => "A21",
            Self::StabO2 => "A22",
            Self::StabOF => "A23",
            Self::StabT1 => "A24",
            Self::StabT2 => "A25",
            Self::StabTF => "A26",
            Self::Shoot1 => "A27",
            Self::Shoot2 => "A28",
            Self::ShootF => "A29",
            Self::Dead => "A33",
            Self::GhostWalk => "A34",
            Self::Custom(c) => c.as_str(),
        }
    }
}

/// Character Facial Expression / Emotion for avatar rendering.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum AvatarEmotion {
    #[default]
    Default,
    Wink,
    Smile,
    Cry,
    Angry,
    Bewildered,
    Blink,
    Furious,
    Sleepy,
    Love,
    Despair,
    Pain,
    Custom(String),
}

impl AvatarEmotion {
    pub fn as_code(&self) -> &str {
        match self {
            Self::Default => "E00",
            Self::Wink => "E01",
            Self::Smile => "E02",
            Self::Cry => "E03",
            Self::Angry => "E04",
            Self::Bewildered => "E05",
            Self::Blink => "E06",
            Self::Furious => "E07",
            Self::Sleepy => "E08",
            Self::Love => "E17",
            Self::Despair => "E12",
            Self::Pain => "E19",
            Self::Custom(c) => c.as_str(),
        }
    }
}

/// Predefined character animation loops and frame sequences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationPreset {
    /// Idle breathing animation loop (Stand1 <-> Stand2, 500ms each)
    Stand,
    /// Walking cycle animation loop (Walk1 <-> Walk2, 250ms each)
    Walk,
    /// Jumping motion (Stand1 -> Jump -> Stand1)
    Jump,
    /// Sitting pose (Sit)
    Sit,
    /// Prone pose (Prone)
    Prone,
    /// Flying floating motion (Fly)
    Fly,
    /// Climbing ladder/rope (Ladder <-> Rope)
    Climb,
    /// Facial blinking loop (Default -> Blink -> Default)
    Blink,
    /// Winking animation loop (Default -> Wink -> Default)
    Wink,
    /// Custom frame sequence with arbitrary actions, emotions, and durations
    Custom(Vec<AnimationFrame>),
}

/// A single frame in an avatar animation sequence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnimationFrame {
    pub action: AvatarAction,
    pub emotion: AvatarEmotion,
    pub wmotion: WeaponMotion,
    pub delay_ms: u32,
}

impl AnimationPreset {
    /// Returns the sequence of frames comprising this animation preset.
    pub fn frames(&self) -> Vec<AnimationFrame> {
        match self {
            Self::Stand => vec![
                AnimationFrame {
                    action: AvatarAction::Stand1,
                    emotion: AvatarEmotion::Default,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 500,
                },
                AnimationFrame {
                    action: AvatarAction::Stand2,
                    emotion: AvatarEmotion::Default,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 500,
                },
            ],
            Self::Walk => vec![
                AnimationFrame {
                    action: AvatarAction::Walk1,
                    emotion: AvatarEmotion::Default,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 250,
                },
                AnimationFrame {
                    action: AvatarAction::Walk2,
                    emotion: AvatarEmotion::Default,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 250,
                },
            ],
            Self::Jump => vec![
                AnimationFrame {
                    action: AvatarAction::Stand1,
                    emotion: AvatarEmotion::Default,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 300,
                },
                AnimationFrame {
                    action: AvatarAction::Jump,
                    emotion: AvatarEmotion::Default,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 600,
                },
            ],
            Self::Sit => vec![AnimationFrame {
                action: AvatarAction::Sit,
                emotion: AvatarEmotion::Default,
                wmotion: WeaponMotion::Default,
                delay_ms: 1000,
            }],
            Self::Prone => vec![AnimationFrame {
                action: AvatarAction::Prone,
                emotion: AvatarEmotion::Default,
                wmotion: WeaponMotion::Default,
                delay_ms: 1000,
            }],
            Self::Fly => vec![
                AnimationFrame {
                    action: AvatarAction::Fly,
                    emotion: AvatarEmotion::Default,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 500,
                },
                AnimationFrame {
                    action: AvatarAction::Stand1,
                    emotion: AvatarEmotion::Default,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 500,
                },
            ],
            Self::Climb => vec![
                AnimationFrame {
                    action: AvatarAction::Ladder,
                    emotion: AvatarEmotion::Default,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 300,
                },
                AnimationFrame {
                    action: AvatarAction::Rope,
                    emotion: AvatarEmotion::Default,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 300,
                },
            ],
            Self::Blink => vec![
                AnimationFrame {
                    action: AvatarAction::Stand1,
                    emotion: AvatarEmotion::Default,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 800,
                },
                AnimationFrame {
                    action: AvatarAction::Stand1,
                    emotion: AvatarEmotion::Blink,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 200,
                },
            ],
            Self::Wink => vec![
                AnimationFrame {
                    action: AvatarAction::Stand1,
                    emotion: AvatarEmotion::Default,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 600,
                },
                AnimationFrame {
                    action: AvatarAction::Stand1,
                    emotion: AvatarEmotion::Wink,
                    wmotion: WeaponMotion::Default,
                    delay_ms: 400,
                },
            ],
            Self::Custom(frames) => frames.clone(),
        }
    }
}

/// Weapon Motion rendering mode.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum WeaponMotion {
    #[default]
    Default,
    OneHand,
    TwoHand,
    Gun,
    HideWeapon,
    Custom(String),
}

impl WeaponMotion {
    pub fn as_code(&self) -> &str {
        match self {
            Self::Default => "W00",
            Self::OneHand => "W01",
            Self::TwoHand => "W02",
            Self::Gun => "W03",
            Self::HideWeapon => "W04",
            Self::Custom(c) => c.as_str(),
        }
    }
}

/// Builder for constructing dynamic `character/look` queries.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvatarLookQuery {
    pub action: Option<AvatarAction>,
    pub emotion: Option<AvatarEmotion>,
    pub wmotion: Option<WeaponMotion>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl AvatarLookQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn action(mut self, action: AvatarAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn emotion(mut self, emotion: AvatarEmotion) -> Self {
        self.emotion = Some(emotion);
        self
    }

    pub fn weapon_motion(mut self, wmotion: WeaponMotion) -> Self {
        self.wmotion = Some(wmotion);
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn to_query_string(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref action) = self.action {
            parts.push(format!("action={}", action.as_code()));
        }
        if let Some(ref emotion) = self.emotion {
            parts.push(format!("emotion={}", emotion.as_code()));
        }
        if let Some(ref wmotion) = self.wmotion {
            parts.push(format!("wmotion={}", wmotion.as_code()));
        }
        if let Some(width) = self.width {
            parts.push(format!("width={}", width));
        }
        if let Some(height) = self.height {
            parts.push(format!("height={}", height));
        }

        parts.join("&")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_code_parsing_and_normalization() {
        let openapi_url = "https://open.api.nexon.com/static/maplestory/character/look/JFKCLCKKAMIBFPCLOJLPFLPENKBMBCDC?wmotion=W00";
        let code1 = AvatarCode::from_url_or_code(openapi_url).unwrap();
        assert_eq!(code1.as_str(), "JFKCLCKKAMIBFPCLOJLPFLPENKBMBCDC");

        let legacy_url = "https://avatar.maplestory.nexon.com/Character/180/JFKCLCKKAMIBFPCLOJLPFLPENKBMBCDC.png";
        let code2 = AvatarCode::from_url_or_code(legacy_url).unwrap();
        assert_eq!(code1, code2);
    }

    #[test]
    fn test_avatar_to_bytes() {
        // 'A' = 0, 'B' = 1, 'P' = 15
        let code = AvatarCode::new("AB").unwrap();
        let bytes = code.to_bytes();
        assert_eq!(bytes, vec![0x01]);

        let code2 = AvatarCode::new("BA").unwrap();
        assert_eq!(code2.to_bytes(), vec![0x10]);

        let code_full = AvatarCode::new("PP").unwrap();
        assert_eq!(code_full.to_bytes(), vec![0xFF]);
    }

    #[test]
    fn test_avatar_look_query_builder() {
        let code = AvatarCode::new("ABCD").unwrap();
        let query = AvatarLookQuery::new()
            .action(AvatarAction::Sit)
            .emotion(AvatarEmotion::Love)
            .weapon_motion(WeaponMotion::HideWeapon)
            .size(200, 200);

        let url = code.to_look_url(&query);
        assert_eq!(
            url,
            "https://open.api.nexon.com/static/maplestory/character/look/ABCD?action=A07&emotion=E17&wmotion=W04&width=200&height=200"
        );

        let legacy = code.to_legacy_avatar_url(Some(180));
        assert_eq!(
            legacy,
            "https://avatar.maplestory.nexon.com/Character/180/ABCD.png"
        );
    }

    #[test]
    fn test_invalid_avatar_code() {
        assert!(AvatarCode::new("").is_err());
        assert!(AvatarCode::new("ABCDZ").is_err()); // 'Z' is not in 'A'..='P'
        assert!(AvatarCode::new("ABCD1").is_err()); // '1' is not in 'A'..='P'
    }
}
