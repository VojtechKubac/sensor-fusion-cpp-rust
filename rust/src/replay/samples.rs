//! Typed rows of `sensor.csv` and `ground_truth.csv`.

use std::path::Path;

use super::csv::read_rows;
use super::ReplayError;
use crate::kf::types::{MeasurementVector, StateVector};
use crate::time::Seconds;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorSample {
    pub t: Seconds,
    pub z: MeasurementVector,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TruthSample {
    pub t: Seconds,
    pub state: StateVector,
}

/// Loads `<dir>/sensor.csv` (`t, zx, zy`).
pub fn load_sensor(dir: &Path) -> Result<Vec<SensorSample>, ReplayError> {
    let rows = read_rows(&dir.join("sensor.csv"), ["t", "zx", "zy"])?;
    Ok(rows
        .iter()
        .map(|&[t, zx, zy]| SensorSample {
            t: Seconds(t),
            z: MeasurementVector::new(zx, zy),
        })
        .collect())
}

/// Loads `<dir>/ground_truth.csv` (`t, x, y, vx, vy`).
pub fn load_ground_truth(dir: &Path) -> Result<Vec<TruthSample>, ReplayError> {
    let rows = read_rows(&dir.join("ground_truth.csv"), ["t", "x", "y", "vx", "vy"])?;
    Ok(rows
        .iter()
        .map(|&[t, x, y, vx, vy]| TruthSample {
            t: Seconds(t),
            state: StateVector::new(x, y, vx, vy),
        })
        .collect())
}
