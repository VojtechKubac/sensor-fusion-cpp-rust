//! Replay loop: one predict + update per sensor sample, shared by the
//! benchmark binary (`fusion_rust`) and the viewer (`fusion_viz`).

use crate::kf::linear_kf::{CovarianceUpdate, LinearKalmanFilter, UpdateError, UpdateResult};
use crate::kf::types::{StateCovariance, StateVector};
use crate::model::constant_velocity::{
    measurement_matrix, measurement_noise, process_noise, transition,
};
use crate::replay::{SensorSample, TruthSample};
use crate::time::Seconds;

/// Filter choices that are not facts about the simulation (not in meta.json).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FilterTuning {
    /// q [m^2/s^3] for `process_noise`.
    pub acceleration_noise_density: f64,
    /// Prior std of each velocity component [m/s]; velocity starts at 0.
    pub initial_velocity_std: f64,
}

impl Default for FilterTuning {
    fn default() -> Self {
        Self {
            acceleration_noise_density: 0.01,
            initial_velocity_std: 2.0,
        }
    }
}

/// Posterior after fusing one sensor sample.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Estimate {
    pub timestamp: Seconds,
    pub state: StateVector,
    pub covariance: StateCovariance,
    pub update: UpdateResult,
}

/// Runs the filter over `samples`, calling `on_estimate` after every update.
/// The filter starts at the first measurement with zero velocity, so no
/// ground truth leaks into the estimate.
pub fn run_filter(
    samples: &[SensorSample],
    noise_std_m: f64,
    tuning: &FilterTuning,
    mut on_estimate: impl FnMut(&SensorSample, &Estimate),
) -> Result<(), UpdateError> {
    let Some(first) = samples.first() else {
        return Ok(());
    };
    let mut filter = LinearKalmanFilter::new(
        StateVector::new(first.measurement.x, first.measurement.y, 0.0, 0.0),
        initial_covariance(noise_std_m, tuning.initial_velocity_std),
        CovarianceUpdate::Joseph,
    );
    let (h, r) = (measurement_matrix(), measurement_noise(noise_std_m));
    let mut previous_time = first.timestamp;
    for sample in samples {
        let dt = Seconds(sample.timestamp.0 - previous_time.0);
        filter.predict(
            &transition(dt),
            &process_noise(dt, tuning.acceleration_noise_density),
        );
        let update = filter.update(&sample.measurement, &h, &r)?;
        previous_time = sample.timestamp;
        let estimate = Estimate {
            timestamp: sample.timestamp,
            state: *filter.state(),
            covariance: *filter.covariance(),
            update,
        };
        on_estimate(sample, &estimate);
    }
    Ok(())
}

fn initial_covariance(position_std: f64, velocity_std: f64) -> StateCovariance {
    let (p, v) = (position_std * position_std, velocity_std * velocity_std);
    StateCovariance::from_diagonal(&StateVector::new(p, p, v, v))
}

/// Ground-truth sample at `timestamp` (within 1 µs), if any. `truth` must be
/// sorted by time, as the CSV reader guarantees.
#[must_use]
pub fn truth_at(truth: &[TruthSample], timestamp: Seconds) -> Option<&TruthSample> {
    const TOLERANCE_S: f64 = 1e-6;
    let index = truth.partition_point(|sample| sample.timestamp.0 < timestamp.0 - TOLERANCE_S);
    truth
        .get(index)
        .filter(|sample| (sample.timestamp.0 - timestamp.0).abs() <= TOLERANCE_S)
}
