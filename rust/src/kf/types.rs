//! Fixed-size types for the 2D constant-velocity filter.
//! Element order follows sim/SCHEMA.md (state_layout / measurement_layout) so
//! both ports share one memory layout.

use nalgebra::{SMatrix, SVector};

pub const STATE_DIM: usize = 4;
pub const MEASUREMENT_DIM: usize = 2;

// State indices: [x, y, vx, vy].
pub const STATE_X: usize = 0;
pub const STATE_Y: usize = 1;
pub const STATE_VX: usize = 2;
pub const STATE_VY: usize = 3;

// Measurement indices: [zx, zy].
pub const MEASUREMENT_X: usize = 0;
pub const MEASUREMENT_Y: usize = 1;

pub type StateVector = SVector<f64, STATE_DIM>;
pub type StateCovariance = SMatrix<f64, STATE_DIM, STATE_DIM>;
pub type StateTransition = SMatrix<f64, STATE_DIM, STATE_DIM>;
pub type MeasurementVector = SVector<f64, MEASUREMENT_DIM>;
pub type MeasurementCovariance = SMatrix<f64, MEASUREMENT_DIM, MEASUREMENT_DIM>;
pub type MeasurementMatrix = SMatrix<f64, MEASUREMENT_DIM, STATE_DIM>;
pub type KalmanGain = SMatrix<f64, STATE_DIM, MEASUREMENT_DIM>;
