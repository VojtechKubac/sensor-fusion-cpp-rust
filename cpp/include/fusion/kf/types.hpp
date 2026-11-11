#ifndef FUSION_KF_TYPES_HPP
#define FUSION_KF_TYPES_HPP

#include <Eigen/Core>

// Fixed-size types for the 2D constant-velocity filter.
// Element order follows sim/SCHEMA.md (state_layout / measurement_layout) so
// both ports share one memory layout.
namespace fusion::kf {

inline constexpr Eigen::Index StateDim = 4;
inline constexpr Eigen::Index MeasurementDim = 2;

// State indices: [x, y, vx, vy].
inline constexpr Eigen::Index StateX = 0;
inline constexpr Eigen::Index StateY = 1;
inline constexpr Eigen::Index StateVx = 2;
inline constexpr Eigen::Index StateVy = 3;

// Measurement indices: [zx, zy].
inline constexpr Eigen::Index MeasurementX = 0;
inline constexpr Eigen::Index MeasurementY = 1;

using StateVector = Eigen::Matrix<double, StateDim, 1>;
using StateCovariance = Eigen::Matrix<double, StateDim, StateDim>;
using StateTransition = Eigen::Matrix<double, StateDim, StateDim>;
using MeasurementVector = Eigen::Matrix<double, MeasurementDim, 1>;
using MeasurementCovariance = Eigen::Matrix<double, MeasurementDim, MeasurementDim>;
using MeasurementMatrix = Eigen::Matrix<double, MeasurementDim, StateDim>;
using KalmanGain = Eigen::Matrix<double, StateDim, MeasurementDim>;

} // namespace fusion::kf

#endif // FUSION_KF_TYPES_HPP
