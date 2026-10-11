#![allow(clippy::useless_conversion)]

use maplestory_core::{
    AnimationPreset, AvatarAction, AvatarCode as CoreAvatarCode, AvatarEmotion, AvatarLookQuery,
    MapleStoryClient as CoreClient, WeaponMotion,
};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pythonize::pythonize;
use std::sync::Arc;

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

            Python::with_gil(|py| {
                pythonize(py, &dash)
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
                    .map(|obj| obj.into_any().unbind())
            })
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

            Python::with_gil(|py| {
                pythonize(py, &basic)
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
                    .map(|obj| obj.into_any().unbind())
            })
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

            Python::with_gil(|py| {
                pythonize(py, &look)
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
                    .map(|obj| obj.into_any().unbind())
            })
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

        let w = width.unwrap_or(128);
        let h = height.unwrap_or(128);

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let gif_bytes = client
                .render_character_gif_by_name(&character_name, &anim_preset, Some(w), Some(h))
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Python::with_gil(|py| Ok(PyBytes::new_bound(py, &gif_bytes).into_any().unbind()))
        })
    }

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
                .get_guild_by_name(&guild_name, &world_name, date.as_deref())
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Python::with_gil(|py| {
                pythonize(py, &guild)
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
                    .map(|obj| obj.into_any().unbind())
            })
        })
    }

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
                .get_user_union(&ocid, date.as_deref())
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Python::with_gil(|py| {
                pythonize(py, &union_data)
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
                    .map(|obj| obj.into_any().unbind())
            })
        })
    }

    /// Retrieve notice list.
    pub fn get_notice_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let notices = client
                .get_notice_list()
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Python::with_gil(|py| {
                pythonize(py, &notices)
                    .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
                    .map(|obj| obj.into_any().unbind())
            })
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
