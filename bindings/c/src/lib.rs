use maplestory_openapi::{
    AnimationPreset, AvatarAction, AvatarCode, AvatarEmotion, AvatarLookQuery, MapleStoryClient,
    WeaponMotion,
};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use tokio::runtime::Runtime;

pub struct ClientHandle {
    pub client: MapleStoryClient,
    pub runtime: Runtime,
}

/// Create a new MapleStory client handle with an API key.
/// Returns NULL on failure.
#[no_mangle]
pub unsafe extern "C" fn maplestory_client_new(api_key: *const c_char) -> *mut ClientHandle {
    if api_key.is_null() {
        return ptr::null_mut();
    }

    let c_str = match CStr::from_ptr(api_key).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let client = match MapleStoryClient::new(c_str) {
        Ok(c) => c,
        Err(_) => return ptr::null_mut(),
    };

    let runtime = match Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(ClientHandle { client, runtime }))
}

/// Create a new MapleStory client handle from NEXON_API_KEY environment variable.
#[no_mangle]
pub unsafe extern "C" fn maplestory_client_from_env() -> *mut ClientHandle {
    let client = match MapleStoryClient::from_env() {
        Ok(c) => c,
        Err(_) => return ptr::null_mut(),
    };

    let runtime = match Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(ClientHandle { client, runtime }))
}

/// Free a MapleStory client handle.
#[no_mangle]
pub unsafe extern "C" fn maplestory_client_free(handle: *mut ClientHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

/// Fetch character dashboard JSON string.
/// On success, returns 0 and sets `*out_json` to a heap-allocated C string (must be freed with `maplestory_free_string`).
#[no_mangle]
pub unsafe extern "C" fn maplestory_get_character_dashboard(
    handle: *mut ClientHandle,
    character_name: *const c_char,
    out_json: *mut *mut c_char,
) -> i32 {
    if handle.is_null() || character_name.is_null() || out_json.is_null() {
        return -1;
    }

    let name = match CStr::from_ptr(character_name).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };

    let h = &*handle;
    let res = h.runtime.block_on(async {
        h.client.get_character_dashboard_by_name(name).await
    });

    match res {
        Ok(dash) => match serde_json::to_string(&dash) {
            Ok(json_str) => match CString::new(json_str) {
                Ok(c_string) => {
                    *out_json = c_string.into_raw();
                    0
                }
                Err(_) => -3,
            },
            Err(_) => -3,
        },
        Err(_) => -3,
    }
}

/// Render an animated GIF directly by character name.
/// On success, returns 0 and sets `*out_data` and `*out_len` (must be freed with `maplestory_free_bytes`).
#[no_mangle]
pub unsafe extern "C" fn maplestory_render_character_gif(
    handle: *mut ClientHandle,
    character_name: *const c_char,
    preset: *const c_char,
    width: u32,
    height: u32,
    out_data: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    if handle.is_null() || character_name.is_null() || out_data.is_null() || out_len.is_null() {
        return -1;
    }

    let name = match CStr::from_ptr(character_name).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };

    let preset_str = if preset.is_null() {
        "walk"
    } else {
        match CStr::from_ptr(preset).to_str() {
            Ok(s) => s,
            Err(_) => "walk",
        }
    };

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

    let w = if width == 0 { 128 } else { width };
    let h = if height == 0 { 128 } else { height };

    let h_ref = &*handle;
    let res = h_ref.runtime.block_on(async {
        h_ref
            .client
            .render_character_gif_by_name(name, &anim_preset, Some(w), Some(h))
            .await
    });

    match res {
        Ok(bytes) => {
            let len = bytes.len();
            let mut boxed = bytes.into_boxed_slice();
            *out_data = boxed.as_mut_ptr();
            *out_len = len;
            std::mem::forget(boxed);
            0
        }
        Err(_) => -3,
    }
}

/// Convert 1024-character AvatarCode to a dynamic Look URL.
#[no_mangle]
pub unsafe extern "C" fn maplestory_avatar_to_look_url(
    code_or_url: *const c_char,
    action: *const c_char,
    emotion: *const c_char,
    wmotion: *const c_char,
    width: u32,
    height: u32,
    out_url: *mut *mut c_char,
) -> i32 {
    if code_or_url.is_null() || out_url.is_null() {
        return -1;
    }

    let raw = match CStr::from_ptr(code_or_url).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };

    let avatar_code = match AvatarCode::from_url_or_code(raw) {
        Ok(c) => c,
        Err(_) => return -3,
    };

    let mut query = AvatarLookQuery::new();
    if !action.is_null() {
        if let Ok(act) = CStr::from_ptr(action).to_str() {
            query.action = Some(AvatarAction::Custom(act.to_string()));
        }
    }
    if !emotion.is_null() {
        if let Ok(emo) = CStr::from_ptr(emotion).to_str() {
            query.emotion = Some(AvatarEmotion::Custom(emo.to_string()));
        }
    }
    if !wmotion.is_null() {
        if let Ok(wm) = CStr::from_ptr(wmotion).to_str() {
            query.wmotion = Some(WeaponMotion::Custom(wm.to_string()));
        }
    }
    if width > 0 && height > 0 {
        query = query.size(width, height);
    }

    let url = avatar_code.to_look_url(&query);
    match CString::new(url) {
        Ok(c_str) => {
            *out_url = c_str.into_raw();
            0
        }
        Err(_) => -3,
    }
}

/// Free a string allocated by this library.
#[no_mangle]
pub unsafe extern "C" fn maplestory_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

/// Free a byte buffer allocated by this library.
#[no_mangle]
pub unsafe extern "C" fn maplestory_free_bytes(ptr: *mut u8, len: usize) {
    if !ptr.is_null() {
        drop(Vec::from_raw_parts(ptr, len, len));
    }
}
