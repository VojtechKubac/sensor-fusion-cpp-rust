//! 2D constant-velocity motion model with a position-only sensor.
//! State [x, y, vx, vy], measurement [zx, zy] (see `kf::types`).

use crate::kf::types::{
    MeasurementCovariance, MeasurementMatrix, StateCovariance, StateTransition, MEASUREMENT_X,
    MEASUREMENT_Y, STATE_VX, STATE_VY, STATE_X, STATE_Y,
};
use crate::time::Seconds;

struct AxisIndices {
    position: usize,
    velocity: usize,
}

const AXES: [AxisIndices; 2] = [
    AxisIndices {
        position: STATE_X,
        velocity: STATE_VX,
    },
    AxisIndices {
        position: STATE_Y,
        velocity: STATE_VY,
    },
];

/// F(dt): advances each position by velocity * dt; velocities unchanged.
#[must_use]
pub fn transition(dt: Seconds) -> StateTransition {
    let mut f = StateTransition::identity();
    for axis in &AXES {
        f[(axis.position, axis.velocity)] = dt.0;
    }
    f
}

/// Q(dt) for continuous white-noise acceleration, per axis:
/// `q * [[dt^3/3, dt^2/2], [dt^2/2, dt]]`.
/// `acceleration_noise_density`: q [m^2/s^3].
#[must_use]
pub fn process_noise(dt: Seconds, acceleration_noise_density: f64) -> StateCovariance {
    let dt1 = dt.0;
    let dt2 = dt1 * dt1;
    let dt3 = dt2 * dt1;
    let mut q = StateCovariance::zeros();
    for axis in &AXES {
        q[(axis.position, axis.position)] = dt3 / 3.0;
        q[(axis.position, axis.velocity)] = dt2 / 2.0;
        q[(axis.velocity, axis.position)] = dt2 / 2.0;
        q[(axis.velocity, axis.velocity)] = dt1;
    }
    q * acceleration_noise_density
}

/// H: selects [x, y] from the state.
#[must_use]
pub fn measurement_matrix() -> MeasurementMatrix {
    let mut h = MeasurementMatrix::zeros();
    h[(MEASUREMENT_X, STATE_X)] = 1.0;
    h[(MEASUREMENT_Y, STATE_Y)] = 1.0;
    h
}

/// R = sigma^2 * I, i.i.d. per axis. `noise_std_m`: sigma [m].
#[must_use]
pub fn measurement_noise(noise_std_m: f64) -> MeasurementCovariance {
    MeasurementCovariance::identity() * (noise_std_m * noise_std_m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kf::types::{MeasurementVector, StateVector};

    const TOLERANCE: f64 = 1e-12;

    #[test]
    fn transition_advances_position_by_velocity_times_dt() {
        let state = StateVector::new(1.0, 2.0, 3.0, 4.0);
        let expected = StateVector::new(2.5, 4.0, 3.0, 4.0);

        let actual = transition(Seconds(0.5)) * state;

        assert!(
            actual.relative_eq(&expected, TOLERANCE, TOLERANCE),
            "{actual}"
        );
    }

    #[test]
    fn process_noise_matches_white_noise_acceleration_closed_form() {
        // dt = 0.5, q = 2: dt^3/3 * q = 1/12, dt^2/2 * q = 1/4, dt * q = 1.
        #[rustfmt::skip]
        let expected = StateCovariance::new(
            1.0 / 12.0, 0.0,        0.25, 0.0,
            0.0,        1.0 / 12.0, 0.0,  0.25,
            0.25,       0.0,        1.0,  0.0,
            0.0,        0.25,       0.0,  1.0,
        );

        let actual = process_noise(Seconds(0.5), 2.0);

        assert!(
            actual.relative_eq(&expected, TOLERANCE, TOLERANCE),
            "{actual}"
        );
    }

    #[test]
    fn process_noise_is_consistent_when_a_step_is_split_in_two() {
        // Continuous-time model: Q(2dt) = F(dt) Q(dt) F(dt)^T + Q(dt).
        let dt = Seconds(0.1);
        let q = 3.0;
        let f = transition(dt);
        let half_step = process_noise(dt, q);

        let composed = f * half_step * f.transpose() + half_step;
        let full_step = process_noise(Seconds(2.0 * dt.0), q);

        assert!(
            composed.relative_eq(&full_step, TOLERANCE, TOLERANCE),
            "{composed}{full_step}"
        );
    }

    #[test]
    fn measurement_matrix_selects_position() {
        let state = StateVector::new(1.0, 2.0, 3.0, 4.0);
        let expected = MeasurementVector::new(1.0, 2.0);

        let actual = measurement_matrix() * state;

        assert!(
            actual.relative_eq(&expected, TOLERANCE, TOLERANCE),
            "{actual}"
        );
    }
}
