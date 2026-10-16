# 20: Dedicated Avatar Client (`AvatarClient`)

**What to build:** Introduce a dedicated `AvatarClient` sub-client accessed via `client.avatar()`, separating multi-frame avatar HTTP fetching and animated GIF encoding from `CharacterClient` while maintaining backward compatibility.

**Blocked by:** None

**Status:** done

- [x] Created `AvatarClient<'a>` in `src/client/avatar.rs` with `render_gif`, `render_custom_gif`, `fetch_frame`, and `fetch_frame_bytes`.
- [x] Registered `client.avatar()` on `MapleStoryClient`.
- [x] Added `FrameConfig` type alias in `src/avatar.rs` and builder methods on `AvatarLookQuery`.
- [x] Preserved backward-compatible forwarding on `CharacterClient::render_avatar_gif` and `render_character_gif_by_name`.
- [x] Added automated pipeline tests in `test_avatar_client_pipeline_and_custom_frames`.
