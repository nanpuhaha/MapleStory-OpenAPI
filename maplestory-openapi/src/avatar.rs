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

/// Metadata description of a packed spritesheet atlas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpritesheetAtlas {
    pub width: u32,
    pub height: u32,
    pub columns: u32,
    pub rows: u32,
    pub frame_count: usize,
    pub frames: Vec<SpritesheetFrame>,
}

/// Metadata description of a single frame entry within a spritesheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpritesheetFrame {
    pub index: usize,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub delay_ms: u32,
    pub action: AvatarAction,
    pub emotion: AvatarEmotion,
    pub wmotion: WeaponMotion,
}

/// Combined container holding spritesheet image bytes and atlas metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpritesheetOutput {
    pub atlas: SpritesheetAtlas,
    pub image_bytes: Vec<u8>,
}

impl SpritesheetOutput {
    /// Serialize the atlas metadata to a compact JSON string.
    pub fn to_json(&self) -> crate::Result<String> {
        serde_json::to_string(&self.atlas)
            .map_err(|e| crate::MapleStoryError::ParseError(e.to_string()))
    }

    /// Serialize the atlas metadata to pretty-printed JSON.
    pub fn to_json_pretty(&self) -> crate::Result<String> {
        serde_json::to_string_pretty(&self.atlas)
            .map_err(|e| crate::MapleStoryError::ParseError(e.to_string()))
    }
}

/// Packs multiple avatar animation frames into a single 2D grid spritesheet PNG image and atlas metadata.
///
/// Available when the `animation` feature is enabled.
#[cfg(feature = "animation")]
pub fn encode_spritesheet(
    frames: &[AnimationFrame],
    columns: Option<u32>,
) -> crate::Result<SpritesheetOutput> {
    if frames.is_empty() {
        return Err(crate::MapleStoryError::InvalidParameter(
            "Animation frames cannot be empty for spritesheet generation".to_string(),
        ));
    }

    let mut decoded_images = Vec::with_capacity(frames.len());
    for frame in frames {
        let img = image::load_from_memory(&frame.image_bytes).map_err(|e| {
            crate::MapleStoryError::Other(format!("Failed to decode frame for spritesheet: {e}"))
        })?;
        decoded_images.push(img.to_rgba8());
    }

    let max_frame_w = decoded_images.iter().map(|img| img.width()).max().unwrap_or(0);
    let max_frame_h = decoded_images.iter().map(|img| img.height()).max().unwrap_or(0);
    let frame_count = frames.len();
    let cols = columns
        .unwrap_or_else(|| (frame_count as f64).sqrt().ceil().max(1.0) as u32)
        .max(1);
    let rows = (frame_count as u32).div_ceil(cols);
    let sheet_w = cols * max_frame_w;
    let sheet_h = rows * max_frame_h;

    let mut sheet = image::RgbaImage::new(sheet_w, sheet_h);
    let mut atlas_frames = Vec::with_capacity(frame_count);

    for (i, (frame, img)) in frames.iter().zip(&decoded_images).enumerate() {
        let col = (i as u32) % cols;
        let row = (i as u32) / cols;
        let x = col * max_frame_w;
        let y = row * max_frame_h;
        image::imageops::overlay(&mut sheet, img, x as i64, y as i64);
        atlas_frames.push(SpritesheetFrame {
            index: i,
            x,
            y,
            width: img.width(),
            height: img.height(),
            delay_ms: frame.delay_ms,
            action: frame.action.clone(),
            emotion: frame.emotion.clone(),
            wmotion: frame.wmotion.clone(),
        });
    }

    let mut png_bytes = Vec::new();
    sheet
        .write_to(
            &mut std::io::Cursor::new(&mut png_bytes),
            image::ImageFormat::Png,
        )
        .map_err(|e| {
            crate::MapleStoryError::Other(format!("Failed to encode spritesheet PNG: {e}"))
        })?;

    Ok(SpritesheetOutput {
        atlas: SpritesheetAtlas {
            width: sheet_w,
            height: sheet_h,
            columns: cols,
            rows,
            frame_count,
            frames: atlas_frames,
        },
        image_bytes: png_bytes,
    })
}

/// Encodes multiple avatar animation frames into an animated WebP file with alpha transparency.
///
/// Available when the `animation` feature is enabled.
#[cfg(feature = "animation")]
pub fn encode_animated_webp(frames: &[AnimationFrame]) -> crate::Result<Vec<u8>> {
    if frames.is_empty() {
        return Err(crate::MapleStoryError::InvalidParameter(
            "Animation frames cannot be empty for WebP generation".to_string(),
        ));
    }

    let mut decoded_images = Vec::with_capacity(frames.len());
    for frame in frames {
        let img = image::load_from_memory(&frame.image_bytes).map_err(|e| {
            crate::MapleStoryError::Other(format!("Failed to decode frame for WebP: {e}"))
        })?;
        decoded_images.push(img.to_rgba8());
    }

    let canvas_width = decoded_images
        .iter()
        .map(|img| img.width())
        .max()
        .unwrap_or(0);
    let canvas_height = decoded_images
        .iter()
        .map(|img| img.height())
        .max()
        .unwrap_or(0);

    let mut body = Vec::new();

    // 1. VP8X Chunk (Extended header)
    body.extend_from_slice(b"VP8X");
    body.extend_from_slice(&10u32.to_le_bytes());
    body.push(0x02 | 0x10); // flags: Animation + Alpha
    body.extend_from_slice(&[0u8, 0u8, 0u8]); // reserved
    let w_minus_1 = (canvas_width.saturating_sub(1)) & 0x00FF_FFFF;
    body.push((w_minus_1 & 0xFF) as u8);
    body.push(((w_minus_1 >> 8) & 0xFF) as u8);
    body.push(((w_minus_1 >> 16) & 0xFF) as u8);
    let h_minus_1 = (canvas_height.saturating_sub(1)) & 0x00FF_FFFF;
    body.push((h_minus_1 & 0xFF) as u8);
    body.push(((h_minus_1 >> 8) & 0xFF) as u8);
    body.push(((h_minus_1 >> 16) & 0xFF) as u8);

    // 2. ANIM Chunk
    body.extend_from_slice(b"ANIM");
    body.extend_from_slice(&6u32.to_le_bytes());
    body.extend_from_slice(&[0u8, 0u8, 0u8, 0u8]); // background RGBA
    body.extend_from_slice(&0u16.to_le_bytes()); // loop count (0 = infinite)

    // 3. ANMF Chunks for each frame
    for (frame, img) in frames.iter().zip(&decoded_images) {
        let mut single_frame_webp = Vec::new();
        img.write_to(
            &mut std::io::Cursor::new(&mut single_frame_webp),
            image::ImageFormat::WebP,
        )
        .map_err(|e| {
            crate::MapleStoryError::Other(format!("Failed to encode WebP subframe: {e}"))
        })?;

        // Extract bitstream payload after RIFF header
        let subchunk = if single_frame_webp.len() > 12
            && &single_frame_webp[0..4] == b"RIFF"
            && &single_frame_webp[8..12] == b"WEBP"
        {
            &single_frame_webp[12..]
        } else {
            &single_frame_webp[..]
        };

        let mut anmf_payload = Vec::with_capacity(16 + subchunk.len() + 1);
        anmf_payload.extend_from_slice(&[0u8, 0u8, 0u8]); // X offset
        anmf_payload.extend_from_slice(&[0u8, 0u8, 0u8]); // Y offset
        let fw_minus_1 = (img.width().saturating_sub(1)) & 0x00FF_FFFF;
        anmf_payload.push((fw_minus_1 & 0xFF) as u8);
        anmf_payload.push(((fw_minus_1 >> 8) & 0xFF) as u8);
        anmf_payload.push(((fw_minus_1 >> 16) & 0xFF) as u8);
        let fh_minus_1 = (img.height().saturating_sub(1)) & 0x00FF_FFFF;
        anmf_payload.push((fh_minus_1 & 0xFF) as u8);
        anmf_payload.push(((fh_minus_1 >> 8) & 0xFF) as u8);
        anmf_payload.push(((fh_minus_1 >> 16) & 0xFF) as u8);
        let dur = frame.delay_ms & 0x00FF_FFFF;
        anmf_payload.push((dur & 0xFF) as u8);
        anmf_payload.push(((dur >> 8) & 0xFF) as u8);
        anmf_payload.push(((dur >> 16) & 0xFF) as u8);
        anmf_payload.push(0x02); // flags: do not blend

        anmf_payload.extend_from_slice(subchunk);
        if anmf_payload.len() % 2 != 0 {
            anmf_payload.push(0);
        }

        body.extend_from_slice(b"ANMF");
        body.extend_from_slice(&(anmf_payload.len() as u32).to_le_bytes());
        body.extend_from_slice(&anmf_payload);
    }

    let mut file_bytes = Vec::with_capacity(12 + body.len());
    file_bytes.extend_from_slice(b"RIFF");
    let riff_size = (4 + body.len()) as u32;
    file_bytes.extend_from_slice(&riff_size.to_le_bytes());
    file_bytes.extend_from_slice(b"WEBP");
    file_bytes.extend_from_slice(&body);

    Ok(file_bytes)
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub image_bytes: Vec<u8>,
}

impl AnimationFrame {
    /// Create a new animation frame configuration without image payload.
    pub fn new(
        action: AvatarAction,
        emotion: AvatarEmotion,
        wmotion: WeaponMotion,
        delay_ms: u32,
    ) -> Self {
        Self {
            action,
            emotion,
            wmotion,
            delay_ms,
            image_bytes: Vec::new(),
        }
    }

    /// Attach raw PNG/RGBA image bytes to this frame.
    pub fn with_image(mut self, bytes: Vec<u8>) -> Self {
        self.image_bytes = bytes;
        self
    }
}

/// Alias for [`AnimationFrame`] used in frame configuration.
pub type FrameConfig = AnimationFrame;

impl AnimationPreset {
    /// Returns the sequence of frames comprising this animation preset.
    pub fn frames(&self) -> Vec<AnimationFrame> {
        match self {
            Self::Stand => vec![
                AnimationFrame::new(AvatarAction::Stand1, AvatarEmotion::Default, WeaponMotion::Default, 500),
                AnimationFrame::new(AvatarAction::Stand2, AvatarEmotion::Default, WeaponMotion::Default, 500),
            ],
            Self::Walk => vec![
                AnimationFrame::new(AvatarAction::Walk1, AvatarEmotion::Default, WeaponMotion::Default, 250),
                AnimationFrame::new(AvatarAction::Walk2, AvatarEmotion::Default, WeaponMotion::Default, 250),
            ],
            Self::Jump => vec![
                AnimationFrame::new(AvatarAction::Stand1, AvatarEmotion::Default, WeaponMotion::Default, 300),
                AnimationFrame::new(AvatarAction::Jump, AvatarEmotion::Default, WeaponMotion::Default, 600),
            ],
            Self::Sit => vec![AnimationFrame::new(AvatarAction::Sit, AvatarEmotion::Default, WeaponMotion::Default, 1000)],
            Self::Prone => vec![AnimationFrame::new(AvatarAction::Prone, AvatarEmotion::Default, WeaponMotion::Default, 1000)],
            Self::Fly => vec![
                AnimationFrame::new(AvatarAction::Fly, AvatarEmotion::Default, WeaponMotion::Default, 500),
                AnimationFrame::new(AvatarAction::Stand1, AvatarEmotion::Default, WeaponMotion::Default, 500),
            ],
            Self::Climb => vec![
                AnimationFrame::new(AvatarAction::Ladder, AvatarEmotion::Default, WeaponMotion::Default, 300),
                AnimationFrame::new(AvatarAction::Rope, AvatarEmotion::Default, WeaponMotion::Default, 300),
            ],
            Self::Blink => vec![
                AnimationFrame::new(AvatarAction::Stand1, AvatarEmotion::Default, WeaponMotion::Default, 800),
                AnimationFrame::new(AvatarAction::Stand1, AvatarEmotion::Blink, WeaponMotion::Default, 200),
            ],
            Self::Wink => vec![
                AnimationFrame::new(AvatarAction::Stand1, AvatarEmotion::Default, WeaponMotion::Default, 600),
                AnimationFrame::new(AvatarAction::Stand1, AvatarEmotion::Wink, WeaponMotion::Default, 400),
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

    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: u32) -> Self {
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
