#![allow(clippy::missing_safety_doc)]

use maplestory_openapi::{
    AnimationPreset, AvatarAction, AvatarCode, AvatarEmotion, AvatarLookQuery, MapleStoryClient,
    OverallRankingQuery, WeaponMotion,
};
use serde::Serialize;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use tokio::runtime::Runtime;

thread_local! {
    static LAST_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
}

fn set_last_error(msg: impl Into<String>) {
    LAST_ERROR.with(|slot| {
        *slot.borrow_mut() = Some(msg.into());
    });
}

fn clear_last_error() {
    LAST_ERROR.with(|slot| {
        *slot.borrow_mut() = None;
    });
}

/// Retrieve the last error message recorded on the current thread.
/// Returns a newly allocated UTF-8 C string that must be freed with `maplestory_free_string`,
/// or NULL if no error has occurred.
#[no_mangle]
pub unsafe extern "C" fn maplestory_last_error_message() -> *mut c_char {
    LAST_ERROR.with(|slot| match slot.borrow().as_deref() {
        Some(msg) => CString::new(msg)
            .map(|c| c.into_raw())
            .unwrap_or(ptr::null_mut()),
        None => ptr::null_mut(),
    })
}

pub struct ClientHandle {
    pub client: MapleStoryClient,
    pub runtime: Runtime,
}

unsafe fn parse_required_cstr<'a>(ptr: *const c_char, name: &str) -> Result<&'a str, i32> {
    if ptr.is_null() {
        set_last_error(format!("Null pointer passed for parameter '{name}'"));
        return Err(-1);
    }
    CStr::from_ptr(ptr).to_str().map_err(|e| {
        set_last_error(format!("Invalid UTF-8 in parameter '{name}': {e}"));
        -2
    })
}

unsafe fn parse_optional_cstr<'a>(ptr: *const c_char, name: &str) -> Result<Option<&'a str>, i32> {
    if ptr.is_null() {
        return Ok(None);
    }
    let s = CStr::from_ptr(ptr).to_str().map_err(|e| {
        set_last_error(format!("Invalid UTF-8 in parameter '{name}': {e}"));
        -2
    })?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}

unsafe fn write_c_string(s: String, out_ptr: *mut *mut c_char) -> i32 {
    match CString::new(s) {
        Ok(c_string) => {
            *out_ptr = c_string.into_raw();
            clear_last_error();
            0
        }
        Err(e) => {
            set_last_error(format!("Failed to convert response to CString: {e}"));
            -3
        }
    }
}

unsafe fn validate_handle<'a>(
    handle: *mut ClientHandle,
    out_json: *mut *mut c_char,
) -> Result<&'a ClientHandle, i32> {
    if handle.is_null() || out_json.is_null() {
        set_last_error("Null pointer passed for handle or out_json");
        return Err(-1);
    }
    Ok(&*handle)
}

unsafe fn serialize_json_result<T: Serialize>(
    res: maplestory_openapi::Result<T>,
    out_json: *mut *mut c_char,
) -> i32 {
    match res {
        Ok(val) => match serde_json::to_string(&val) {
            Ok(json_str) => write_c_string(json_str, out_json),
            Err(e) => {
                set_last_error(format!("Failed to serialize response to JSON: {e}"));
                -3
            }
        },
        Err(e) => {
            set_last_error(e.to_string());
            -3
        }
    }
}

fn create_runtime() -> Option<Runtime> {
    match Runtime::new() {
        Ok(rt) => Some(rt),
        Err(e) => {
            set_last_error(format!("Failed to initialize Tokio runtime: {e}"));
            None
        }
    }
}

fn build_client_handle(
    client_res: maplestory_openapi::Result<MapleStoryClient>,
) -> *mut ClientHandle {
    let client = match client_res {
        Ok(c) => c,
        Err(e) => {
            set_last_error(e.to_string());
            return ptr::null_mut();
        }
    };
    let runtime = match create_runtime() {
        Some(rt) => rt,
        None => return ptr::null_mut(),
    };
    clear_last_error();
    Box::into_raw(Box::new(ClientHandle { client, runtime }))
}

/// Create a new MapleStory client handle with an API key.
/// Returns NULL on failure (inspect `maplestory_last_error_message()`).
#[no_mangle]
pub unsafe extern "C" fn maplestory_client_new(api_key: *const c_char) -> *mut ClientHandle {
    let key = match parse_required_cstr(api_key, "api_key") {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    build_client_handle(MapleStoryClient::new(key))
}

/// Create a new MapleStory client handle with an API key and a custom base URL.
/// Returns NULL on failure (inspect `maplestory_last_error_message()`).
#[no_mangle]
pub unsafe extern "C" fn maplestory_client_with_base_url(
    api_key: *const c_char,
    base_url: *const c_char,
) -> *mut ClientHandle {
    let key = match parse_required_cstr(api_key, "api_key") {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    let url = match parse_required_cstr(base_url, "base_url") {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    build_client_handle(
        MapleStoryClient::builder()
            .api_key(key)
            .base_url(url)
            .build(),
    )
}

/// Create a new MapleStory client handle from NEXON_API_KEY environment variable.
#[no_mangle]
pub unsafe extern "C" fn maplestory_client_from_env() -> *mut ClientHandle {
    build_client_handle(MapleStoryClient::from_env())
}

/// Free a MapleStory client handle.
#[no_mangle]
pub unsafe extern "C" fn maplestory_client_free(handle: *mut ClientHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapleStoryCombatSummary {
    pub combat_power: i64,
    pub boss_damage: f64,
    pub ignore_monster_armor: f64,
    pub damage: f64,
    pub critical_rate: f64,
    pub critical_damage: f64,
    pub total_starforce: i32,
    pub arcane_force: i32,
    pub authentic_force: i32,
    pub dojang_best_floor: i32,
    pub estimated_dojang_floor: i32,
    pub effective_damage_multiplier_300: f64,
    pub effective_damage_multiplier_380: f64,
}

impl MapleStoryCombatSummary {
    fn from_analytics(
        analytics: &maplestory_openapi::models::character::CharacterCombatAnalytics,
    ) -> Self {
        Self {
            combat_power: analytics.combat_power,
            boss_damage: analytics.boss_damage,
            ignore_monster_armor: analytics.ignore_monster_armor,
            damage: analytics.damage,
            critical_rate: analytics.critical_rate,
            critical_damage: analytics.critical_damage,
            total_starforce: analytics.total_starforce as i32,
            arcane_force: analytics.arcane_force as i32,
            authentic_force: analytics.authentic_force as i32,
            dojang_best_floor: analytics.dojang_best_floor.unwrap_or(0) as i32,
            estimated_dojang_floor: analytics.estimated_dojang_floor() as i32,
            effective_damage_multiplier_300: analytics.effective_damage_multiplier_300(),
            effective_damage_multiplier_380: analytics.effective_damage_multiplier_380(),
        }
    }
}

/// Fetch character combat analytics summary by OCID directly into a typed C struct.
/// Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn maplestory_character_get_combat_summary(
    handle: *mut ClientHandle,
    ocid: *const c_char,
    out_summary: *mut MapleStoryCombatSummary,
) -> i32 {
    let ocid_str = match parse_required_cstr(ocid, "ocid") {
        Ok(s) => s,
        Err(code) => return code,
    };
    if handle.is_null() || out_summary.is_null() {
        set_last_error("Null pointer passed for handle or out_summary");
        return -1;
    }
    let h = &*handle;

    let res = h
        .runtime
        .block_on(h.client.character().get_combat_analytics(ocid_str, None));

    match res {
        Ok(analytics) => {
            *out_summary = MapleStoryCombatSummary::from_analytics(&analytics);
            clear_last_error();
            0
        }
        Err(e) => {
            set_last_error(e.to_string());
            -3
        }
    }
}

/// Fetch character combat analytics summary directly by character name into a typed C struct.
/// Fetch character combat analytics summary directly by character name into a typed C struct.
/// Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn maplestory_character_get_combat_summary_by_name(
    handle: *mut ClientHandle,
    character_name: *const c_char,
    out_summary: *mut MapleStoryCombatSummary,
) -> i32 {
    maplestory_get_character_combat_summary(handle, character_name, out_summary)
}

/// Legacy alias for `maplestory_character_get_combat_summary_by_name`.
/// Returns 0 on success.
#[no_mangle]
pub unsafe extern "C" fn maplestory_get_character_combat_summary(
    handle: *mut ClientHandle,
    character_name: *const c_char,
    out_summary: *mut MapleStoryCombatSummary,
) -> i32 {
    let name = match parse_required_cstr(character_name, "character_name") {
        Ok(s) => s,
        Err(code) => return code,
    };
    if handle.is_null() || out_summary.is_null() {
        set_last_error("Null pointer passed for handle or out_summary");
        return -1;
    }
    let h = &*handle;

    let res = h
        .runtime
        .block_on(h.client.character().get_combat_analytics_by_name(name));

    match res {
        Ok(analytics) => {
            *out_summary = MapleStoryCombatSummary::from_analytics(&analytics);
            clear_last_error();
            0
        }
        Err(e) => {
            set_last_error(e.to_string());
            -3
        }
    }
}

/// Fetch synthesized character coordination (visible equipment and appearance) by OCID as a JSON string.
/// On success, returns 0 and sets `*out_json` to a heap-allocated C string (must be freed with `maplestory_free_string`).
#[no_mangle]
pub unsafe extern "C" fn maplestory_character_get_coordination(
    handle: *mut ClientHandle,
    ocid: *const c_char,
    out_json: *mut *mut c_char,
) -> i32 {
    let ocid_str = match parse_required_cstr(ocid, "ocid") {
        Ok(s) => s,
        Err(code) => return code,
    };
    let h = match validate_handle(handle, out_json) {
        Ok(h) => h,
        Err(code) => return code,
    };

    let res = h
        .runtime
        .block_on(h.client.character().get_coordination(ocid_str, None));
    serialize_json_result(res, out_json)
}

/// Fetch synthesized character coordination (visible equipment and appearance) directly by character name as a JSON string.
/// On success, returns 0 and sets `*out_json` to a heap-allocated C string (must be freed with `maplestory_free_string`).
#[no_mangle]
pub unsafe extern "C" fn maplestory_get_character_coordination(
    handle: *mut ClientHandle,
    character_name: *const c_char,
    out_json: *mut *mut c_char,
) -> i32 {
    let name = match parse_required_cstr(character_name, "character_name") {
        Ok(s) => s,
        Err(code) => return code,
    };
    let h = match validate_handle(handle, out_json) {
        Ok(h) => h,
        Err(code) => return code,
    };

    let res = h
        .runtime
        .block_on(h.client.character().get_coordination_by_name(name));
    serialize_json_result(res, out_json)
}

/// Fetch character dashboard JSON string.
/// On success, returns 0 and sets `*out_json` to a heap-allocated C string (must be freed with `maplestory_free_string`).
#[no_mangle]
pub unsafe extern "C" fn maplestory_get_character_dashboard(
    handle: *mut ClientHandle,
    character_name: *const c_char,
    out_json: *mut *mut c_char,
) -> i32 {
    let name = match parse_required_cstr(character_name, "character_name") {
        Ok(s) => s,
        Err(code) => return code,
    };
    let h = match validate_handle(handle, out_json) {
        Ok(h) => h,
        Err(code) => return code,
    };

    let res = h
        .runtime
        .block_on(h.client.get_character_dashboard_by_name(name));
    serialize_json_result(res, out_json)
}

/// Fetch a complete guild profile (guild basic + member basic profiles) as a JSON string.
/// `date` may be NULL or empty for the latest available data.
#[no_mangle]
pub unsafe extern "C" fn maplestory_get_guild_profile(
    handle: *mut ClientHandle,
    guild_name: *const c_char,
    world_name: *const c_char,
    date: *const c_char,
    out_json: *mut *mut c_char,
) -> i32 {
    let g_name = match parse_required_cstr(guild_name, "guild_name") {
        Ok(s) => s,
        Err(code) => return code,
    };
    let w_name = match parse_required_cstr(world_name, "world_name") {
        Ok(s) => s,
        Err(code) => return code,
    };
    let date_opt = match parse_optional_cstr(date, "date") {
        Ok(d) => d,
        Err(code) => return code,
    };
    let h = match validate_handle(handle, out_json) {
        Ok(h) => h,
        Err(code) => return code,
    };

    let res = h
        .runtime
        .block_on(h.client.guild().get_profile(g_name, w_name, date_opt));
    serialize_json_result(res, out_json)
}

/// Fetch user union level/grade information by character name as a JSON string.
/// `date` may be NULL or empty for the latest available data.
#[no_mangle]
pub unsafe extern "C" fn maplestory_get_user_union(
    handle: *mut ClientHandle,
    character_name: *const c_char,
    date: *const c_char,
    out_json: *mut *mut c_char,
) -> i32 {
    let name = match parse_required_cstr(character_name, "character_name") {
        Ok(s) => s,
        Err(code) => return code,
    };
    let date_opt = match parse_optional_cstr(date, "date") {
        Ok(d) => d,
        Err(code) => return code,
    };
    let h = match validate_handle(handle, out_json) {
        Ok(h) => h,
        Err(code) => return code,
    };

    let res = h.runtime.block_on(async {
        let ocid = h.client.get_ocid(name).await?;
        h.client.union().get_union(&ocid, date_opt).await
    });
    serialize_json_result(res, out_json)
}

/// Fetch overall ranking information as a JSON string.
/// Optional string parameters (`world_name`, `character_class`, `ocid`) may be NULL or empty.
/// Pass `world_type < 0` to omit world_type filter (or `0` for regular, `1` for Reboot).
/// Pass `page <= 0` to omit page filter.
#[no_mangle]
pub unsafe extern "C" fn maplestory_get_overall_ranking(
    handle: *mut ClientHandle,
    date: *const c_char,
    world_name: *const c_char,
    world_type: i32,
    character_class: *const c_char,
    ocid: *const c_char,
    page: i32,
    out_json: *mut *mut c_char,
) -> i32 {
    let date_opt = match parse_optional_cstr(date, "date") {
        Ok(s) => s,
        Err(code) => return code,
    };
    let world_opt = match parse_optional_cstr(world_name, "world_name") {
        Ok(w) => w,
        Err(code) => return code,
    };
    let class_opt = match parse_optional_cstr(character_class, "character_class") {
        Ok(c) => c,
        Err(code) => return code,
    };
    let ocid_opt = match parse_optional_cstr(ocid, "ocid") {
        Ok(o) => o,
        Err(code) => return code,
    };
    let wt_opt = if world_type >= 0 {
        Some(world_type)
    } else {
        None
    };
    let page_opt = if page > 0 { Some(page as u32) } else { None };
    let h = match validate_handle(handle, out_json) {
        Ok(h) => h,
        Err(code) => return code,
    };

    let query = OverallRankingQuery {
        date: date_opt,
        world_name: world_opt,
        world_type: wt_opt,
        class: class_opt,
        ocid: ocid_opt,
        page: page_opt,
    };
    let res = h.runtime.block_on(h.client.ranking().get_overall(query));
    serialize_json_result(res, out_json)
}

/// Fetch the latest official in-game notices list as a JSON string.
#[no_mangle]
pub unsafe extern "C" fn maplestory_get_notice_list(
    handle: *mut ClientHandle,
    out_json: *mut *mut c_char,
) -> i32 {
    let h = match validate_handle(handle, out_json) {
        Ok(h) => h,
        Err(code) => return code,
    };

    let res = h.runtime.block_on(h.client.notice().get_notice_list());
    serialize_json_result(res, out_json)
}

/// Fetch a complete account summary (roster + achievements + union) as a JSON string.
/// `character_name` and `date` may be NULL or empty. If `character_name` is provided and the
/// account roster has no union attached, union information for `character_name` is included.
#[no_mangle]
pub unsafe extern "C" fn maplestory_get_account_summary(
    handle: *mut ClientHandle,
    character_name: *const c_char,
    date: *const c_char,
    out_json: *mut *mut c_char,
) -> i32 {
    let name_opt = match parse_optional_cstr(character_name, "character_name") {
        Ok(s) => s,
        Err(code) => return code,
    };
    let date_opt = match parse_optional_cstr(date, "date") {
        Ok(d) => d,
        Err(code) => return code,
    };
    let h = match validate_handle(handle, out_json) {
        Ok(h) => h,
        Err(code) => return code,
    };

    let res = h.runtime.block_on(
        h.client
            .user()
            .get_account_summary_with_character(name_opt, date_opt),
    );
    serialize_json_result(res, out_json)
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
    if handle.is_null() || out_data.is_null() || out_len.is_null() {
        set_last_error("Null pointer passed to maplestory_render_character_gif");
        return -1;
    }

    let name = match parse_required_cstr(character_name, "character_name") {
        Ok(s) => s,
        Err(code) => return code,
    };

    let preset_str = if preset.is_null() {
        "walk"
    } else {
        CStr::from_ptr(preset).to_str().unwrap_or("walk")
    };

    let anim_preset = AnimationPreset::parse_or_default(preset_str, AnimationPreset::Walk);

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
            clear_last_error();
            0
        }
        Err(e) => {
            set_last_error(e.to_string());
            -3
        }
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
    if out_url.is_null() {
        set_last_error("Null pointer passed for out_url");
        return -1;
    }

    let raw = match parse_required_cstr(code_or_url, "code_or_url") {
        Ok(s) => s,
        Err(code) => return code,
    };

    let avatar_code = match AvatarCode::from_url_or_code(raw) {
        Ok(c) => c,
        Err(e) => {
            set_last_error(e.to_string());
            return -3;
        }
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
    write_c_string(url, out_url)
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

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    unsafe fn take_c_string(ptr: *mut c_char) -> String {
        assert!(!ptr.is_null());
        let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        maplestory_free_string(ptr);
        s
    }

    #[test]
    fn test_null_pointer_records_diagnostic_error_message() {
        unsafe {
            let mut out: *mut c_char = ptr::null_mut();
            let code = maplestory_get_character_dashboard(ptr::null_mut(), ptr::null(), &mut out);
            assert_eq!(code, -1);

            let err_ptr = maplestory_last_error_message();
            let err_msg = take_c_string(err_ptr);
            assert!(
                err_msg.contains("character_name"),
                "Unexpected error message: {err_msg}"
            );
        }
    }

    #[test]
    fn test_c_abi_notice_union_ranking_guild_and_account_summary() {
        let rt = Runtime::new().unwrap();
        let mock_server = rt.block_on(async {
            let server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/notice"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "notice": [
                        {
                            "title": "패치 안내",
                            "url": "https://maplestory.nexon.com/News/Notice/1",
                            "notice_id": 1,
                            "date": "2024-05-01T10:00+09:00"
                        }
                    ]
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/id"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "ocid": "ocid_123"
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/user/union"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "date": "2024-05-01T00:00+09:00",
                    "union_level": 9000,
                    "union_grade": "그랜드 마스터 유니온 3",
                    "union_artifact_level": 45,
                    "union_artifact_exp": 1000,
                    "union_artifact_point": 5000
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/user/union-artifact"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "date": "2024-05-01T00:00+09:00",
                    "union_artifact_effect": [],
                    "union_artifact_crystal": [],
                    "union_artifact_remain_ap": 10
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/user/union-champion"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "date": "2024-05-01T00:00+09:00",
                    "union_champion": [],
                    "champion_badge_total_info": []
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/character/list"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "account_list": [
                        {
                            "account_id": "acc_1",
                            "character_list": [
                                {
                                    "ocid": "ocid_123",
                                    "character_name": "아크",
                                    "world_name": "스카니아",
                                    "character_class": "배틀메이지",
                                    "character_level": 295
                                }
                            ]
                        }
                    ]
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/user/achievement"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "account_list": []
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/ranking/overall"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "ranking": [
                        {
                            "date": "2024-05-01",
                            "ranking": 1,
                            "character_name": "아크",
                            "world_name": "스카니아",
                            "class_name": "레지스탕스",
                            "sub_class_name": "배틀메이지",
                            "character_level": 295,
                            "character_exp": 123456789,
                            "character_popularity": 500,
                            "character_guildname": "명예길드"
                        }
                    ]
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/guild/id"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "oguild_id": "guild_999"
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/guild/basic"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "date": "2024-05-01T00:00+09:00",
                    "world_name": "스카니아",
                    "guild_name": "명예길드",
                    "guild_level": 30,
                    "guild_fame": 100000,
                    "guild_point": 500000,
                    "guild_master_name": "아크",
                    "guild_member_count": 1,
                    "guild_member": ["아크"],
                    "guild_skill": [],
                    "guild_noblesse_skill": []
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/character/basic"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "date": "2024-05-01T00:00+09:00",
                    "character_name": "아크",
                    "world_name": "스카니아",
                    "character_gender": "남",
                    "character_class": "배틀메이지",
                    "character_class_level": "6",
                    "character_level": 295,
                    "character_exp": 1000,
                    "character_exp_rate": "12.5",
                    "character_guild_name": "명예길드",
                    "character_image": "https://open.api.nexon.com/static/maplestory/character/look/A",
                    "character_date_create": "2015-01-01T00:00+09:00",
                    "access_flag": "true",
                    "liberation_quest_clear": "1"
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/character/stat"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "date": "2024-05-01T00:00+09:00",
                    "character_class": "배틀메이지",
                    "final_stat": [
                        {"stat_name": "전투력", "stat_value": "200000000"},
                        {"stat_name": "보스 몬스터 데미지", "stat_value": "350"},
                        {"stat_name": "방어율 무시", "stat_value": "95"},
                        {"stat_name": "데미지", "stat_value": "50"},
                        {"stat_name": "크리티컬 확률", "stat_value": "100"},
                        {"stat_name": "크리티컬 데미지", "stat_value": "80"}
                    ],
                    "remain_ap": 0
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/character/item-equipment"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "date": "2024-05-01T00:00+09:00",
                    "character_gender": "남",
                    "character_class": "배틀메이지",
                    "preset_no": 1,
                    "item_equipment": [
                        {
                            "item_equipment_part": "모자",
                            "item_equipment_slot": "모자",
                            "item_name": "하이네스 던위치햇",
                            "item_icon": "https://example.com/hat.png",
                            "starforce": "22"
                        }
                    ]
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/character/symbol-equipment"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "date": "2024-05-01T00:00+09:00",
                    "character_class": "배틀메이지",
                    "symbol": [
                        {
                            "symbol_name": "아케인심볼 : 여로",
                            "symbol_force": "1320"
                        },
                        {
                            "symbol_name": "어센틱심볼 : 세르니움",
                            "symbol_force": "660"
                        }
                    ]
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/character/dojang"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "date": "2024-05-01T00:00+09:00",
                    "character_class": "배틀메이지",
                    "world_name": "스카니아",
                    "dojang_best_floor": 65,
                    "date_dojang_record": "2024-05-01T00:00+09:00",
                    "dojang_best_time": 800
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/character/beauty-equipment"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "date": "2024-05-01T00:00+09:00",
                    "character_gender": "남",
                    "character_class": "배틀메이지",
                    "character_hair": {
                        "hair_name": "빅 멍뭉이 헤어",
                        "base_color": "검정"
                    },
                    "character_face": {
                        "face_name": "차분한 헤헤 얼굴",
                        "base_color": "자주"
                    }
                })))
                .mount(&server)
                .await;

            Mock::given(method("GET"))
                .and(path("/maplestory/v1/character/cashitem-equipment"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "date": "2024-05-01T00:00+09:00",
                    "character_gender": "남",
                    "character_class": "배틀메이지",
                    "preset_no": 1,
                    "cash_item_equipment_base": [
                        {
                            "cash_item_equipment_part": "모자",
                            "cash_item_equipment_slot": "모자",
                            "cash_item_name": "토끼귀",
                            "cash_item_icon": "https://example.com/ears.png"
                        }
                    ]
                })))
                .mount(&server)
                .await;

            server
        });

        let api_key = CString::new("test_key").unwrap();
        let base_url = CString::new(mock_server.uri()).unwrap();

        unsafe {
            let handle = maplestory_client_with_base_url(api_key.as_ptr(), base_url.as_ptr());
            assert!(!handle.is_null());

            // 1. Notice list
            let mut out_json: *mut c_char = ptr::null_mut();
            let rc = maplestory_get_notice_list(handle, &mut out_json);
            assert_eq!(rc, 0);
            let notice_json = take_c_string(out_json);
            assert!(notice_json.contains("패치 안내"));

            // 2. User union
            let char_name = CString::new("아크").unwrap();
            let mut out_union: *mut c_char = ptr::null_mut();
            let rc =
                maplestory_get_user_union(handle, char_name.as_ptr(), ptr::null(), &mut out_union);
            assert_eq!(rc, 0);
            let union_json = take_c_string(out_union);
            assert!(union_json.contains("9000"));

            // 3. Overall ranking
            let date_str = CString::new("2024-05-01").unwrap();
            let mut out_ranking: *mut c_char = ptr::null_mut();
            let rc = maplestory_get_overall_ranking(
                handle,
                date_str.as_ptr(),
                ptr::null(),
                -1,
                ptr::null(),
                ptr::null(),
                1,
                &mut out_ranking,
            );
            assert_eq!(rc, 0);
            let ranking_json = take_c_string(out_ranking);
            assert!(ranking_json.contains("배틀메이지"));

            // 4. Guild profile
            let guild_name = CString::new("명예길드").unwrap();
            let world_name = CString::new("스카니아").unwrap();
            let mut out_guild: *mut c_char = ptr::null_mut();
            let rc = maplestory_get_guild_profile(
                handle,
                guild_name.as_ptr(),
                world_name.as_ptr(),
                ptr::null(),
                &mut out_guild,
            );
            assert_eq!(rc, 0);
            let guild_json = take_c_string(out_guild);
            assert!(guild_json.contains("명예길드"));

            // 5. Account summary
            let mut out_summary: *mut c_char = ptr::null_mut();
            let rc = maplestory_get_account_summary(
                handle,
                char_name.as_ptr(),
                ptr::null(),
                &mut out_summary,
            );
            assert_eq!(rc, 0);
            let summary_json = take_c_string(out_summary);
            assert!(summary_json.contains("그랜드 마스터 유니온 3"));

            // 6. Invalid date error surfaces through maplestory_last_error_message
            let bad_date = CString::new("2010-01-01").unwrap();
            let mut out_err_json: *mut c_char = ptr::null_mut();
            let rc = maplestory_get_user_union(
                handle,
                char_name.as_ptr(),
                bad_date.as_ptr(),
                &mut out_err_json,
            );
            assert_eq!(rc, -3);
            let err_msg = take_c_string(maplestory_last_error_message());
            assert!(
                err_msg.contains("2023-12-21"),
                "Expected date error: {err_msg}"
            );

            // 7. Typed Combat Summary (by name and by OCID)
            let mut summary = MapleStoryCombatSummary {
                combat_power: 0,
                boss_damage: 0.0,
                ignore_monster_armor: 0.0,
                damage: 0.0,
                critical_rate: 0.0,
                critical_damage: 0.0,
                total_starforce: 0,
                arcane_force: 0,
                authentic_force: 0,
                dojang_best_floor: 0,
                estimated_dojang_floor: 0,
                effective_damage_multiplier_300: 0.0,
                effective_damage_multiplier_380: 0.0,
            };
            let rc = maplestory_get_character_combat_summary(handle, char_name.as_ptr(), &mut summary);
            assert_eq!(rc, 0);
            assert_eq!(summary.combat_power, 200_000_000);
            assert_eq!(summary.arcane_force, 1320);
            assert_eq!(summary.authentic_force, 660);
            assert_eq!(summary.dojang_best_floor, 65);
            assert!(summary.effective_damage_multiplier_300 > 0.0);

            // Also test maplestory_character_get_combat_summary by OCID
            let ocid = CString::new("ocid_123").unwrap();
            let mut summary_by_ocid = summary;
            let rc = maplestory_character_get_combat_summary(handle, ocid.as_ptr(), &mut summary_by_ocid);
            assert_eq!(rc, 0);
            assert_eq!(summary_by_ocid.combat_power, 200_000_000);

            // 8. Coordination Endpoint (by name and by OCID)
            let mut out_coord_json: *mut c_char = ptr::null_mut();
            let rc = maplestory_get_character_coordination(handle, char_name.as_ptr(), &mut out_coord_json);
            assert_eq!(rc, 0);
            let coord_json = take_c_string(out_coord_json);
            assert!(coord_json.contains("visible_items") || coord_json.contains("character_name"));

            let mut out_coord_by_ocid: *mut c_char = ptr::null_mut();
            let rc = maplestory_character_get_coordination(handle, ocid.as_ptr(), &mut out_coord_by_ocid);
            assert_eq!(rc, 0);
            let coord_by_ocid = take_c_string(out_coord_by_ocid);
            assert!(coord_by_ocid.contains("visible_items"));

            maplestory_client_free(handle);
        }
    }
}
