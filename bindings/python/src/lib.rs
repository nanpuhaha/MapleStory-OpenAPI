#![allow(clippy::useless_conversion, clippy::too_many_arguments)]

use maplestory_core::{
    AnimationPreset, AvatarAction, AvatarCode as CoreAvatarCode, AvatarEmotion, AvatarLookQuery,
    DojangRankingQuery, GuildRankingQuery, MapleStoryClient as CoreClient, OverallRankingQuery,
    UnionRankingQuery, WeaponMotion,
};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pythonize::pythonize;
use std::sync::Arc;

macro_rules! to_py_object {
    ($val:expr) => {
        Python::with_gil(|py| {
            pythonize(py, $val)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
                .map(|obj| obj.into_any().unbind())
        })
    };
}

/// Python wrapper for MapleStoryClient.
#[pyclass(name = "MapleStoryClient")]
#[derive(Clone)]
pub struct PyMapleStoryClient {
    inner: Arc<CoreClient>,
}

#[pymethods]
impl PyMapleStoryClient {
    #[new]
    #[pyo3(signature = (api_key))]
    pub fn new(api_key: &str) -> PyResult<Self> {
        let client = CoreClient::new(api_key)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(Self {
            inner: Arc::new(client),
        })
    }

    /// Initialize client from NEXON_API_KEY environment variable.
    #[staticmethod]
    pub fn from_env() -> PyResult<Self> {
        let client = CoreClient::from_env()
            .map_err(|e| pyo3::exceptions::PyEnvironmentError::new_err(e.to_string()))?;
        Ok(Self {
            inner: Arc::new(client),
        })
    }

    /// Retrieve comprehensive all-in-one character dashboard.
    pub fn get_character_dashboard<'py>(
        &self,
        py: Python<'py>,
        character_name: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let dash = client
                .get_character_dashboard_by_name(&character_name)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&dash)
        })
    }

    /// Retrieve character basic information.
    pub fn get_character_basic<'py>(
        &self,
        py: Python<'py>,
        character_name: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let basic = client
                .get_character_by_name(&character_name)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&basic)
        })
    }

    /// Retrieve character coordination look information.
    pub fn get_character_coordination<'py>(
        &self,
        py: Python<'py>,
        character_name: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let look = client
                .get_character_coordination_by_name(&character_name)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&look)
        })
    }

    /// Generate an animated GIF for the character look.
    ///
    /// Presets: "walk", "wink", "stand", "jump", "sit", "prone", "fly", "blink"
    #[pyo3(signature = (character_name, preset = None, width = None, height = None))]
    pub fn render_character_gif<'py>(
        &self,
        py: Python<'py>,
        character_name: String,
        preset: Option<String>,
        width: Option<u32>,
        height: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        let preset_str = preset.unwrap_or_else(|| "walk".to_string());
        let anim_preset = AnimationPreset::parse_or_default(&preset_str, AnimationPreset::Walk);

        let w = width.unwrap_or(128);
        let h = height.unwrap_or(128);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let gif_bytes = client
                .avatar()
                .render_gif_by_name(&character_name, &anim_preset, Some(w), Some(h))
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Python::with_gil(|py| Ok(PyBytes::new_bound(py, &gif_bytes).into_any().unbind()))
        })
    }

    // ─── Guild ───────────────────────────────────────────────────────────────

    /// Retrieve guild basic information by guild name and world name.
    #[pyo3(signature = (guild_name, world_name, date = None))]
    pub fn get_guild_basic<'py>(
        &self,
        py: Python<'py>,
        guild_name: String,
        world_name: String,
        date: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let guild = client
                .guild()
                .get_by_name(&guild_name, &world_name, date.as_deref())
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&guild)
        })
    }

    /// Retrieve aggregated guild profile by guild name and world name.
    #[pyo3(signature = (guild_name, world_name, date = None))]
    pub fn get_guild_profile<'py>(
        &self,
        py: Python<'py>,
        guild_name: String,
        world_name: String,
        date: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let profile = client
                .guild()
                .get_profile(&guild_name, &world_name, date.as_deref())
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&profile)
        })
    }

    // ─── Union ───────────────────────────────────────────────────────────────

    /// Retrieve Union information by character name.
    #[pyo3(signature = (character_name, date = None))]
    pub fn get_user_union<'py>(
        &self,
        py: Python<'py>,
        character_name: String,
        date: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let ocid = client
                .get_ocid(&character_name)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            let union_data = client
                .union()
                .get_union(&ocid, date.as_deref())
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&union_data)
        })
    }

    /// Retrieve Union artifact information by character name.
    #[pyo3(signature = (character_name, date = None))]
    pub fn get_user_union_artifact<'py>(
        &self,
        py: Python<'py>,
        character_name: String,
        date: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let ocid = client
                .get_ocid(&character_name)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            let artifact = client
                .union()
                .get_artifact(&ocid, date.as_deref())
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&artifact)
        })
    }

    /// Retrieve Union raider information by character name.
    #[pyo3(signature = (character_name, date = None))]
    pub fn get_user_union_raider<'py>(
        &self,
        py: Python<'py>,
        character_name: String,
        date: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let ocid = client
                .get_ocid(&character_name)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            let raider = client
                .union()
                .get_raider(&ocid, date.as_deref())
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&raider)
        })
    }

    // ─── Ranking ─────────────────────────────────────────────────────────────

    /// Retrieve overall ranking.
    #[pyo3(signature = (date = None, world_name = None, world_type = None, class_name = None, ocid = None, page = None))]
    pub fn get_overall_ranking<'py>(
        &self,
        py: Python<'py>,
        date: Option<String>,
        world_name: Option<String>,
        world_type: Option<i32>,
        class_name: Option<String>,
        ocid: Option<String>,
        page: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let query = OverallRankingQuery {
                date: date.as_deref(),
                world_name: world_name.as_deref(),
                world_type,
                class: class_name.as_deref(),
                ocid: ocid.as_deref(),
                page,
            };
            let ranking = client
                .ranking()
                .get_overall(query)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&ranking)
        })
    }

    /// Retrieve Union ranking.
    #[pyo3(signature = (date = None, world_name = None, ocid = None, page = None))]
    pub fn get_union_ranking<'py>(
        &self,
        py: Python<'py>,
        date: Option<String>,
        world_name: Option<String>,
        ocid: Option<String>,
        page: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let query = UnionRankingQuery {
                date: date.as_deref(),
                world_name: world_name.as_deref(),
                ocid: ocid.as_deref(),
                page,
            };
            let ranking = client
                .ranking()
                .get_union(query)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&ranking)
        })
    }

    /// Retrieve guild ranking.
    #[pyo3(signature = (ranking_type = 0, date = None, world_name = None, guild_name = None, page = None))]
    pub fn get_guild_ranking<'py>(
        &self,
        py: Python<'py>,
        ranking_type: u32,
        date: Option<String>,
        world_name: Option<String>,
        guild_name: Option<String>,
        page: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let query = GuildRankingQuery {
                date: date.as_deref(),
                ranking_type,
                world_name: world_name.as_deref(),
                guild_name: guild_name.as_deref(),
                page,
            };
            let ranking = client
                .ranking()
                .get_guild(query)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&ranking)
        })
    }

    /// Retrieve Mu Lung Dojang ranking.
    #[pyo3(signature = (difficulty = 1, date = None, world_name = None, class_name = None, ocid = None, page = None))]
    pub fn get_dojang_ranking<'py>(
        &self,
        py: Python<'py>,
        difficulty: u32,
        date: Option<String>,
        world_name: Option<String>,
        class_name: Option<String>,
        ocid: Option<String>,
        page: Option<u32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let query = DojangRankingQuery {
                date: date.as_deref(),
                difficulty,
                world_name: world_name.as_deref(),
                class: class_name.as_deref(),
                ocid: ocid.as_deref(),
                page,
            };
            let ranking = client
                .ranking()
                .get_dojang(query)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&ranking)
        })
    }

    // ─── History ─────────────────────────────────────────────────────────────

    /// Retrieve starforce enhancement history.
    #[pyo3(signature = (count, date = None, cursor = None))]
    pub fn get_starforce_history<'py>(
        &self,
        py: Python<'py>,
        count: u32,
        date: Option<String>,
        cursor: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let hist = client
                .history()
                .get_starforce(count, date.as_deref(), cursor.as_deref())
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&hist)
        })
    }

    /// Retrieve potential reset history.
    #[pyo3(signature = (count, date = None, cursor = None))]
    pub fn get_potential_history<'py>(
        &self,
        py: Python<'py>,
        count: u32,
        date: Option<String>,
        cursor: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let hist = client
                .history()
                .get_potential(count, date.as_deref(), cursor.as_deref())
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&hist)
        })
    }

    /// Retrieve cube usage history.
    #[pyo3(signature = (count, date = None, cursor = None))]
    pub fn get_cube_history<'py>(
        &self,
        py: Python<'py>,
        count: u32,
        date: Option<String>,
        cursor: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let hist = client
                .history()
                .get_cube(count, date.as_deref(), cursor.as_deref())
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&hist)
        })
    }

    /// Retrieve soul potential history.
    #[pyo3(signature = (count, date = None, cursor = None))]
    pub fn get_soul_potential_history<'py>(
        &self,
        py: Python<'py>,
        count: u32,
        date: Option<String>,
        cursor: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let hist = client
                .history()
                .get_soul_potential(count, date.as_deref(), cursor.as_deref())
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&hist)
        })
    }

    // ─── Notice & User & Scheduler ───────────────────────────────────────────

    /// Retrieve notice list.
    pub fn get_notice_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let notices = client
                .notice()
                .get_notice_list()
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&notices)
        })
    }

    /// Retrieve update notice list.
    pub fn get_update_notice_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let notices = client
                .notice()
                .get_update_notice_list()
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&notices)
        })
    }

    /// Retrieve event notice list.
    pub fn get_event_notice_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let notices = client
                .notice()
                .get_event_notice_list()
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&notices)
        })
    }

    /// Retrieve cashshop notice list.
    pub fn get_cashshop_notice_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let notices = client
                .notice()
                .get_cashshop_notice_list()
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&notices)
        })
    }

    /// Retrieve account summary across worlds.
    pub fn get_account_summary<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let summary = client
                .user()
                .get_account_summary(None)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&summary)
        })
    }

    /// Retrieve scheduler character state by character name.
    pub fn get_scheduler_character_state<'py>(
        &self,
        py: Python<'py>,
        character_name: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let state = client
                .scheduler()
                .get_character_state_by_name(&character_name)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            to_py_object!(&state)
        })
    }
}

/// Python wrapper for 1024-character AvatarCode.
#[pyclass(name = "AvatarCode")]
pub struct PyAvatarCode {
    inner: CoreAvatarCode,
}

#[pymethods]
impl PyAvatarCode {
    #[new]
    pub fn new(code_or_url: &str) -> PyResult<Self> {
        let inner = CoreAvatarCode::from_url_or_code(code_or_url)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(Self { inner })
    }

    /// Returns the raw 1024-character string.
    pub fn code(&self) -> String {
        self.inner.as_str().to_string()
    }

    /// Returns the 512-byte binary decoded representation.
    pub fn to_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let bytes = self.inner.to_bytes();
        PyBytes::new_bound(py, &bytes)
    }

    /// Constructs an official NEXON Look static image URL.
    #[pyo3(signature = (action = None, emotion = None, wmotion = None, width = None, height = None))]
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

/// MapleStory OpenAPI Python Module powered by Rust
#[pymodule]
fn maplestory_openapi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMapleStoryClient>()?;
    m.add_class::<PyAvatarCode>()?;
    Ok(())
}
