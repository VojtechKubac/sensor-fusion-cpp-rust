#ifndef FUSION_KF_LINEAR_KF_HPP
#define FUSION_KF_LINEAR_KF_HPP

#include "fusion/kf/types.hpp"

#include <expected>

// Model-agnostic linear Kalman filter: owns the estimate (x, P); the caller
// supplies F, Q, H, R (e.g. from fusion/model/constant_velocity.hpp) and dt.
namespace fusion::kf {

// How update() recomputes P after the gain is applied.
enum class CovarianceUpdate {
  // P = (I - KH) P (I - KH)^T + K R K^T: stays symmetric PSD under rounding.
  Joseph,
  // P = (I - KH) P: cheaper, can lose symmetry / PSD over long runs.
  Simple,
};

struct UpdateResult {
  MeasurementVector innovation;               // y = z - H x (prior x)
  MeasurementCovariance innovationCovariance; // S = H P H^T + R
  double nis;                                 // y^T S^-1 y
};

enum class UpdateError {
  // Cholesky of S failed (e.g. R not positive-definite); state is unchanged.
  InnovationCovarianceNotPositiveDefinite,
};

class LinearKalmanFilter {
public:
  LinearKalmanFilter(const StateVector& initialState,
                     const StateCovariance& initialCovariance,
                     CovarianceUpdate covarianceUpdate = CovarianceUpdate::Joseph);

  // x = F x;  P = F P F^T + Q.
  void predict(const StateTransition& f, const StateCovariance& q);

  // Fuses measurement z. On error the estimate is left untouched.
  [[nodiscard]] std::expected<UpdateResult, UpdateError>
  update(const MeasurementVector& z, const MeasurementMatrix& h, const MeasurementCovariance& r);

  [[nodiscard]] const StateVector& state() const { return x_; }
  [[nodiscard]] const StateCovariance& covariance() const { return p_; }

private:
  StateVector x_;
  StateCovariance p_;
  CovarianceUpdate covarianceUpdate_;
};

} // namespace fusion::kf

#endif // FUSION_KF_LINEAR_KF_HPP
