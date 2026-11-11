#include "fusion/kf/linear_kf.hpp"

namespace fusion::kf {

LinearKalmanFilter::LinearKalmanFilter(const StateVector& initialState,
                                       const StateCovariance& initialCovariance,
                                       CovarianceUpdate covarianceUpdate)
    : x_(initialState), p_(initialCovariance), covarianceUpdate_(covarianceUpdate) {}

void LinearKalmanFilter::predict(const StateTransition& f, const StateCovariance& q) {
  x_ = f * x_;
  p_ = f * p_ * f.transpose() + q;
}

// TODO(update): innovation, Cholesky of S, gain, Joseph/Simple covariance, NIS.

} // namespace fusion::kf
