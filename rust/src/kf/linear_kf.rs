//! Model-agnostic linear Kalman filter: owns the estimate (x, P); the caller
//! supplies F, Q, H, R (e.g. from `model::constant_velocity`) and dt.

use super::types::{
    KalmanGain, MeasurementCovariance, MeasurementMatrix, MeasurementVector, StateCovariance,
    StateTransition, StateVector,
};

/// How `update` recomputes P after the gain is applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CovarianceUpdate {
    /// P = (I - KH) P (I - KH)^T + K R K^T: stays symmetric PSD under rounding.
    #[default]
    Joseph,
    /// P = (I - KH) P: cheaper, can lose symmetry / PSD over long runs.
    Simple,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UpdateResult {
    /// y = z - H x (prior x).
    pub innovation: MeasurementVector,
    /// S = H P H^T + R.
    pub innovation_covariance: MeasurementCovariance,
    /// y^T S^-1 y.
    pub nis: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateError {
    /// Cholesky of S failed (e.g. R not positive-definite); state is unchanged.
    InnovationCovarianceNotPositiveDefinite,
}

#[derive(Debug, Clone)]
pub struct LinearKalmanFilter {
    x: StateVector,
    p: StateCovariance,
    covariance_update: CovarianceUpdate,
}

impl LinearKalmanFilter {
    #[must_use]
    pub fn new(
        initial_state: StateVector,
        initial_covariance: StateCovariance,
        covariance_update: CovarianceUpdate,
    ) -> Self {
        Self {
            x: initial_state,
            p: initial_covariance,
            covariance_update,
        }
    }

    /// x = F x;  P = F P F^T + Q.
    pub fn predict(&mut self, f: &StateTransition, q: &StateCovariance) {
        self.x = f * self.x;
        self.p = f * self.p * f.transpose() + q;
    }

    /// Fuses measurement z. On error the estimate is left untouched.
    pub fn update(
        &mut self,
        z: &MeasurementVector,
        h: &MeasurementMatrix,
        r: &MeasurementCovariance,
    ) -> Result<UpdateResult, UpdateError> {
        let y = z - h * self.x;
        let s = h * self.p * h.transpose() + r;
        let s_cholesky = s
            .cholesky()
            .ok_or(UpdateError::InnovationCovarianceNotPositiveDefinite)?;

        // K = P H^T S^-1, computed as K^T = S^-1 (H P) (P and S symmetric).
        let k: KalmanGain = s_cholesky.solve(&(h * self.p)).transpose();
        self.x += k * y;
        self.p = self.updated_covariance(&k, h, r);
        Ok(UpdateResult {
            innovation: y,
            innovation_covariance: s,
            nis: y.dot(&s_cholesky.solve(&y)),
        })
    }

    #[must_use]
    pub fn state(&self) -> &StateVector {
        &self.x
    }

    #[must_use]
    pub fn covariance(&self) -> &StateCovariance {
        &self.p
    }

    /// Posterior P from the prior `self.p` using the configured form.
    fn updated_covariance(
        &self,
        k: &KalmanGain,
        h: &MeasurementMatrix,
        r: &MeasurementCovariance,
    ) -> StateCovariance {
        let a = StateCovariance::identity() - k * h;
        match self.covariance_update {
            CovarianceUpdate::Joseph => a * self.p * a.transpose() + k * r * k.transpose(),
            CovarianceUpdate::Simple => a * self.p,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::constant_velocity::{
        measurement_matrix, measurement_noise, process_noise, transition,
    };
    use crate::time::Seconds;

    const TOLERANCE: f64 = 1e-12;
    const FORMS: [CovarianceUpdate; 2] = [CovarianceUpdate::Joseph, CovarianceUpdate::Simple];

    fn sample_measurement() -> MeasurementVector {
        MeasurementVector::new(1.2, 1.8)
    }

    fn diagonal_covariance() -> StateCovariance {
        StateCovariance::from_diagonal(&StateVector::new(1.0, 2.0, 3.0, 4.0))
    }

    /// Filter after one CV predict, so P has position-velocity correlation.
    fn predicted_filter(form: CovarianceUpdate) -> LinearKalmanFilter {
        let mut filter = LinearKalmanFilter::new(
            StateVector::new(1.0, 2.0, 3.0, 4.0),
            diagonal_covariance(),
            form,
        );
        let dt = Seconds(0.1);
        filter.predict(&transition(dt), &process_noise(dt, 1.0));
        filter
    }

    #[test]
    fn predict_without_process_noise_propagates_state_and_covariance() {
        let mut filter = LinearKalmanFilter::new(
            StateVector::new(1.0, 2.0, 3.0, 4.0),
            diagonal_covariance(),
            CovarianceUpdate::default(),
        );
        let expected_state = StateVector::new(2.5, 4.0, 3.0, 4.0);
        // Per axis F P F^T with F = [[1, 0.5], [0, 1]], P = diag(p, v):
        //   [[p + 0.25 v, 0.5 v], [0.5 v, v]]  ->  x: (1, 3), y: (2, 4).
        #[rustfmt::skip]
        let expected_covariance = StateCovariance::new(
            1.75, 0.0, 1.5, 0.0,
            0.0,  3.0, 0.0, 2.0,
            1.5,  0.0, 3.0, 0.0,
            0.0,  2.0, 0.0, 4.0,
        );

        filter.predict(&transition(Seconds(0.5)), &StateCovariance::zeros());

        assert!(filter
            .state()
            .relative_eq(&expected_state, TOLERANCE, TOLERANCE));
        assert!(filter
            .covariance()
            .relative_eq(&expected_covariance, TOLERANCE, TOLERANCE));
    }

    #[test]
    fn predict_adds_process_noise_to_the_covariance() {
        let initial_state = StateVector::new(1.0, 2.0, 3.0, 4.0);
        let mut filter = LinearKalmanFilter::new(
            initial_state,
            diagonal_covariance(),
            CovarianceUpdate::default(),
        );
        let q = StateCovariance::identity() * 0.1;

        filter.predict(&StateTransition::identity(), &q);

        assert!(filter
            .state()
            .relative_eq(&initial_state, TOLERANCE, TOLERANCE));
        assert!(filter.covariance().relative_eq(
            &(diagonal_covariance() + q),
            TOLERANCE,
            TOLERANCE
        ));
    }

    #[test]
    fn update_with_equal_prior_and_sensor_variance_halves_position_variance() {
        let mut filter = LinearKalmanFilter::new(
            StateVector::zeros(),
            StateCovariance::identity(),
            CovarianceUpdate::default(),
        );
        let expected_state = StateVector::new(0.5, 0.5, 0.0, 0.0);
        let expected_covariance =
            StateCovariance::from_diagonal(&StateVector::new(0.5, 0.5, 1.0, 1.0));

        let result = filter.update(
            &MeasurementVector::new(1.0, 1.0),
            &measurement_matrix(),
            &measurement_noise(1.0),
        );

        assert!(result.is_ok());
        assert!(filter
            .state()
            .relative_eq(&expected_state, TOLERANCE, TOLERANCE));
        assert!(filter
            .covariance()
            .relative_eq(&expected_covariance, TOLERANCE, TOLERANCE));
    }

    #[test]
    fn update_reports_innovation_its_covariance_and_nis() {
        // P = 0, so S = R and the state must not move.
        let mut filter = LinearKalmanFilter::new(
            StateVector::zeros(),
            StateCovariance::zeros(),
            CovarianceUpdate::default(),
        );
        let r = MeasurementCovariance::from_diagonal(&MeasurementVector::new(4.0, 1.0));
        let z = MeasurementVector::new(2.0, 3.0);

        let result = filter
            .update(&z, &measurement_matrix(), &r)
            .expect("S = R is PD");

        assert!(result.innovation.relative_eq(&z, TOLERANCE, TOLERANCE));
        assert!(result
            .innovation_covariance
            .relative_eq(&r, TOLERANCE, TOLERANCE));
        assert!((result.nis - 10.0).abs() < TOLERANCE); // 2^2/4 + 3^2/1
        assert_eq!(*filter.state(), StateVector::zeros());
    }

    #[test]
    fn update_never_increases_uncertainty_and_keeps_p_symmetric() {
        for form in FORMS {
            let mut filter = predicted_filter(form);
            let prior = *filter.covariance();

            filter
                .update(
                    &sample_measurement(),
                    &measurement_matrix(),
                    &measurement_noise(0.5),
                )
                .expect("well-conditioned update");

            let posterior = *filter.covariance();
            let shrink = (prior - posterior).symmetric_eigenvalues();
            assert!(
                shrink.min() > -TOLERANCE,
                "{form:?}: prior - posterior is PSD"
            );
            assert!(
                (posterior - posterior.transpose()).norm() < TOLERANCE,
                "{form:?}"
            );
        }
    }

    #[test]
    fn joseph_and_simple_covariance_forms_agree_on_a_well_conditioned_update() {
        let [mut joseph, mut simple] = FORMS.map(predicted_filter);
        let h = measurement_matrix();
        let r = measurement_noise(0.5);

        assert!(joseph.update(&sample_measurement(), &h, &r).is_ok());
        assert!(simple.update(&sample_measurement(), &h, &r).is_ok());

        assert!(joseph
            .state()
            .relative_eq(simple.state(), TOLERANCE, TOLERANCE));
        assert!(joseph
            .covariance()
            .relative_eq(simple.covariance(), TOLERANCE, TOLERANCE));
    }

    #[test]
    fn update_rejects_a_non_positive_definite_s_and_leaves_the_estimate() {
        let initial_state = StateVector::new(1.0, 2.0, 3.0, 4.0);
        let mut filter = LinearKalmanFilter::new(
            initial_state,
            StateCovariance::zeros(),
            CovarianceUpdate::default(),
        );

        let result = filter.update(
            &sample_measurement(),
            &measurement_matrix(),
            &-MeasurementCovariance::identity(),
        );

        assert_eq!(
            result,
            Err(UpdateError::InnovationCovarianceNotPositiveDefinite)
        );
        assert_eq!(*filter.state(), initial_state);
        assert_eq!(*filter.covariance(), StateCovariance::zeros());
    }
}
