#include "fusion/kf/linear_kf.hpp"

#include <Eigen/Cholesky>

#include <utility>

namespace fusion::kf {

namespace {

// P = (I - KH) P (I - KH)^T + K R K^T.
StateCovariance josephCovariance(const StateCovariance& p,
                                 const KalmanGain& k,
                                 const MeasurementMatrix& h,
                                 const MeasurementCovariance& r) {
  const StateCovariance a = StateCovariance::Identity() - k * h;
  return a * p * a.transpose() + k * r * k.transpose();
}

// P = (I - KH) P.
StateCovariance
simpleCovariance(const StateCovariance& p, const KalmanGain& k, const MeasurementMatrix& h) {
  return (StateCovariance::Identity() - k * h) * p;
}

} // namespace

LinearKalmanFilter::LinearKalmanFilter(const StateVector& initialState,
                                       const StateCovariance& initialCovariance,
                                       CovarianceUpdate covarianceUpdate)
    : x_(initialState), p_(initialCovariance), covarianceUpdate_(covarianceUpdate) {}

void LinearKalmanFilter::predict(const StateTransition& f, const StateCovariance& q) {
  x_ = f * x_;
  p_ = f * p_ * f.transpose() + q;
}

std::expected<UpdateResult, UpdateError> LinearKalmanFilter::update(
    const MeasurementVector& z, const MeasurementMatrix& h, const MeasurementCovariance& r) {
  const MeasurementVector y = z - h * x_;
  const MeasurementCovariance s = h * p_ * h.transpose() + r;
  const Eigen::LLT<MeasurementCovariance> sCholesky(s);
  if (sCholesky.info() != Eigen::Success) {
    return std::unexpected(UpdateError::InnovationCovarianceNotPositiveDefinite);
  }

  // K = P H^T S^-1, computed as K^T = S^-1 (H P) (P and S symmetric).
  const KalmanGain k = sCholesky.solve(h * p_).transpose();
  x_ += k * y;
  p_ = updatedCovariance(k, h, r);
  return UpdateResult{.innovation = y, .innovationCovariance = s, .nis = y.dot(sCholesky.solve(y))};
}

StateCovariance LinearKalmanFilter::updatedCovariance(const KalmanGain& k,
                                                      const MeasurementMatrix& h,
                                                      const MeasurementCovariance& r) const {
  switch (covarianceUpdate_) {
  case CovarianceUpdate::Joseph:
    return josephCovariance(p_, k, h, r);
  case CovarianceUpdate::Simple:
    return simpleCovariance(p_, k, h);
  }
  std::unreachable();
}

} // namespace fusion::kf
