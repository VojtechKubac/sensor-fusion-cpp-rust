//! Typed rows of `sensor.csv` and `ground_truth.csv`. Vector element order
//! follows the schema's state_layout / measurement_layout.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use super::csv::read_rows;
use super::ReplayError;
use crate::kf::types::{MeasurementVector, StateVector};
use crate::time::Seconds;

/// sensor.csv: t, zx, zy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorSample {
    pub timestamp: Seconds,
    pub measurement: MeasurementVector,
}

/// ground_truth.csv: t, x, y, vx, vy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TruthSample {
    pub timestamp: Seconds,
    pub state: StateVector,
}

/// Loads `<dir>/sensor.csv`.
pub fn load_sensor(dir: &Path) -> Result<Vec<SensorSample>, ReplayError> {
    let rows = load_rows(&dir.join("sensor.csv"), ["t", "zx", "zy"])?;
    Ok(rows
        .iter()
        .map(|&[t, zx, zy]| SensorSample {
            timestamp: Seconds(t),
            measurement: MeasurementVector::new(zx, zy),
        })
        .collect())
}

/// Loads `<dir>/ground_truth.csv`.
pub fn load_ground_truth(dir: &Path) -> Result<Vec<TruthSample>, ReplayError> {
    let rows = load_rows(&dir.join("ground_truth.csv"), ["t", "x", "y", "vx", "vy"])?;
    Ok(rows
        .iter()
        .map(|&[t, x, y, vx, vy]| TruthSample {
            timestamp: Seconds(t),
            state: StateVector::new(x, y, vx, vy),
        })
        .collect())
}

fn load_rows<const N: usize>(path: &Path, header: [&str; N]) -> Result<Vec<[f64; N]>, ReplayError> {
    let file = File::open(path).map_err(|source| ReplayError::Io {
        path: path.to_owned(),
        source,
    })?;
    read_rows(BufReader::new(file), header).map_err(|error| ReplayError::Csv {
        path: path.to_owned(),
        error,
    })
}
