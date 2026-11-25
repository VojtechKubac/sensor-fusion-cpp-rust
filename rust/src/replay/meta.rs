use std::path::Path;

use serde::Deserialize;

use super::ReplayError;

const SUPPORTED_SCHEMA_VERSION: u32 = 1;
const STATE_LAYOUT: [&str; 4] = ["x", "y", "vx", "vy"];
const MEASUREMENT_LAYOUT: [&str; 2] = ["zx", "zy"];

/// The `meta.json` fields this port uses; other fields are ignored.
#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    pub schema_version: u32,
    pub noise_std_m: f64,
    pub initial_state: InitialState,
    pub state_layout: Vec<String>,
    pub measurement_layout: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct InitialState {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
}

/// Loads `<dir>/meta.json` and checks it matches the layout in `kf::types`.
pub fn load_meta(dir: &Path) -> Result<Meta, ReplayError> {
    let path = dir.join("meta.json");
    let text = std::fs::read_to_string(&path).map_err(|source| ReplayError::Io {
        path: path.clone(),
        source,
    })?;
    let meta: Meta =
        serde_json::from_str(&text).map_err(|source| ReplayError::Json { path, source })?;
    check_supported(&meta)?;
    Ok(meta)
}

fn check_supported(meta: &Meta) -> Result<(), ReplayError> {
    if meta.schema_version != SUPPORTED_SCHEMA_VERSION {
        return Err(ReplayError::Unsupported(format!(
            "schema_version {} (expected {SUPPORTED_SCHEMA_VERSION})",
            meta.schema_version
        )));
    }
    if meta.state_layout != STATE_LAYOUT || meta.measurement_layout != MEASUREMENT_LAYOUT {
        return Err(ReplayError::Unsupported(format!(
            "layout {:?} / {:?}",
            meta.state_layout, meta.measurement_layout
        )));
    }
    Ok(())
}
