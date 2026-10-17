// src/client/avatar.rs
//! Dedicated sub-client for MapleStory Character Avatar Look & Animation rendering.

#[cfg(feature = "animation")]
use crate::avatar::{
    encode_animated_gif, encode_animated_webp, encode_spritesheet, AnimationFrame, AnimationPreset,
    SpritesheetOutput,
};
use crate::{
    avatar::AvatarCode,
    client::MapleStoryClient,
    error::{MapleStoryError, Result},
};

/// Sub-client for Character Avatar Look & Animation rendering.
#[derive(Clone, Copy)]
pub struct AvatarClient<'a> {
    pub(crate) client: &'a MapleStoryClient,
}

impl<'a> AvatarClient<'a> {
    /// Create a new `AvatarClient` referencing a [`MapleStoryClient`].
    pub fn new(client: &'a MapleStoryClient) -> Self {
        Self { client }
    }

    #[cfg(feature = "animation")]
    async fn fetch_frames(
        &self,
        avatar_code: &AvatarCode,
        preset: &AnimationPreset,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Vec<AnimationFrame>> {
        let frames = preset.frames();

        let fetch_futures = frames.into_iter().map(|frame| {
            let mut query = crate::avatar::AvatarLookQuery::new()
                .action(frame.action.clone())
                .emotion(frame.emotion.clone())
                .weapon_motion(frame.wmotion.clone());
            if let (Some(w), Some(h)) = (width, height) {
                query = query.size(w, h);
            }
            let url = avatar_code.to_look_url(&query);
            let http = self.client.http.clone();
            async move {
                let resp = http.get(&url).send().await?;
                let status = resp.status();
                if !status.is_success() {
                    return Err(MapleStoryError::Other(format!(
                        "Failed to fetch avatar frame from {}: HTTP {}",
                        url, status
                    )));
                }
                let bytes = resp.bytes().await?.to_vec();
                Ok::<_, MapleStoryError>(frame.with_image(bytes))
            }
        });

        let results = futures_util::future::join_all(fetch_futures).await;
        let mut loaded_frames = Vec::with_capacity(results.len());
        for res in results {
            loaded_frames.push(res?);
        }
        Ok(loaded_frames)
    }

    /// Fetches all frames of an animation preset for the given avatar code,
    /// and renders them into an animated GIF byte buffer.
    #[cfg(feature = "animation")]
    pub async fn render_gif(
        &self,
        avatar_code: &AvatarCode,
        preset: &AnimationPreset,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Vec<u8>> {
        let frames = self.fetch_frames(avatar_code, preset, width, height).await?;
        let frames_data: Vec<(Vec<u8>, u32)> = frames
            .into_iter()
            .map(|f| (f.image_bytes, f.delay_ms))
            .collect();
        encode_animated_gif(&frames_data)
    }

    /// Fetches all frames of an animation preset and renders them into an animated WebP byte buffer.
    #[cfg(feature = "animation")]
    pub async fn render_webp(
        &self,
        avatar_code: &AvatarCode,
        preset: &AnimationPreset,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Vec<u8>> {
        let frames = self.fetch_frames(avatar_code, preset, width, height).await?;
        encode_animated_webp(&frames)
    }

    /// Fetches all frames of an animation preset and packs them into a 2D spritesheet atlas with PNG output.
    #[cfg(feature = "animation")]
    pub async fn render_spritesheet(
        &self,
        avatar_code: &AvatarCode,
        preset: &AnimationPreset,
        columns: Option<u32>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<SpritesheetOutput> {
        let frames = self.fetch_frames(avatar_code, preset, width, height).await?;
        encode_spritesheet(&frames, columns)
    }

    /// Fetches the character's current look and renders an animated WebP directly by character name.
    #[cfg(feature = "animation")]
    pub async fn render_webp_by_name(
        &self,
        character_name: &str,
        preset: &AnimationPreset,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Vec<u8>> {
        let basic = self.client.character().get_by_name(character_name).await?;
        let img_url = basic.character_image.ok_or_else(|| {
            MapleStoryError::Other(format!(
                "Character '{}' does not have a valid character image URL",
                character_name
            ))
        })?;
        let avatar_code = AvatarCode::from_url_or_code(&img_url)?;
        self.render_webp(&avatar_code, preset, width, height).await
    }

    /// Fetches the character's current look and packs animation frames into a spritesheet atlas directly by character name.
    #[cfg(feature = "animation")]
    pub async fn render_spritesheet_by_name(
        &self,
        character_name: &str,
        preset: &AnimationPreset,
        columns: Option<u32>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<SpritesheetOutput> {
        let basic = self.client.character().get_by_name(character_name).await?;
        let img_url = basic.character_image.ok_or_else(|| {
            MapleStoryError::Other(format!(
                "Character '{}' does not have a valid character image URL",
                character_name
            ))
        })?;
        let avatar_code = AvatarCode::from_url_or_code(&img_url)?;
        self.render_spritesheet(&avatar_code, preset, columns, width, height).await
    }

    /// Fetches the character's current look and renders an animated GIF directly by character name.
    #[cfg(feature = "animation")]
    pub async fn render_gif_by_name(
        &self,
        character_name: &str,
        preset: &AnimationPreset,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Vec<u8>> {
        let basic = self.client.character().get_by_name(character_name).await?;
        let img_url = basic.character_image.ok_or_else(|| {
            MapleStoryError::Other(format!(
                "Character '{}' does not have a valid character image URL",
                character_name
            ))
        })?;
        let avatar_code = AvatarCode::from_url_or_code(&img_url)?;
        self.render_gif(&avatar_code, preset, width, height).await
    }

    /// Fetches raw bytes for a single frame of an avatar given look query parameters.
    pub async fn fetch_frame_bytes(
        &self,
        avatar_code: &AvatarCode,
        query: &crate::avatar::AvatarLookQuery,
    ) -> Result<Vec<u8>> {
        let url = avatar_code.to_look_url(query);
        let resp = self.client.http.get(&url).send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(MapleStoryError::Other(format!(
                "Failed to fetch avatar frame from {}: HTTP {}",
                url, status
            )));
        }
        Ok(resp.bytes().await?.to_vec())
    }

    /// Fetches and renders a custom sequence of [`crate::avatar::AnimationFrame`] into an animated GIF.
    #[cfg(feature = "animation")]
    pub async fn render_custom_gif(
        &self,
        avatar_code: &AvatarCode,
        frames: &[crate::avatar::AnimationFrame],
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Vec<u8>> {
        let preset = AnimationPreset::Custom(frames.to_vec());
        self.render_gif(avatar_code, &preset, width, height).await
    }

    /// Fetches a single frame of an avatar given frame configuration.
    pub async fn fetch_frame(
        &self,
        avatar_code: &AvatarCode,
        frame: &crate::avatar::AnimationFrame,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Vec<u8>> {
        let mut query = crate::avatar::AvatarLookQuery::default()
            .action(frame.action.clone())
            .emotion(frame.emotion.clone())
            .weapon_motion(frame.wmotion.clone());
        if let Some(w) = width {
            query = query.width(w);
        }
        if let Some(h) = height {
            query = query.height(h);
        }
        self.fetch_frame_bytes(avatar_code, &query).await
    }
}
