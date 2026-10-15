# Ticket 20: Dedicated Avatar Client (`AvatarClient`)

## Background & Motivation
Avatar GIF rendering (`render_avatar_gif`, `render_custom_avatar_gif`) currently lives inside `CharacterClient`, mixing character profile API operations with multi-frame image fetching and binary animated GIF encoding.

## Requirements
1. Create `AvatarClient<'a>` in `src/client/avatar.rs`.
2. Expose `client.avatar()` on `MapleStoryClient`.
3. Provide high-level methods on `AvatarClient`:
   - `render_gif(character_look: &CharacterLookQuery, preset: AnimationPreset) -> Result<Vec<u8>>`
   - `render_custom_gif(character_look: &CharacterLookQuery, frames: &[FrameConfig]) -> Result<Vec<u8>>`
   - `fetch_frame(character_look: &CharacterLookQuery, frame: &FrameConfig) -> Result<Vec<u8>>`
4. Preserve backward compatibility on `CharacterClient`:
   - Provide forwarding helper on `CharacterClient` with deprecation note or seamless delegation to `AvatarClient`.
5. Unit and integration tests for avatar pipeline.
