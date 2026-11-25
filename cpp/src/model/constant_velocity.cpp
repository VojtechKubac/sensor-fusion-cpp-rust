#include "fusion/model/constant_velocity.hpp"

#include <array>

namespace fusion::model {

namespace {

struct AxisIndices {
  Eigen::Index position;
  Eigen::Index velocity;
};

constexpr std::array<AxisIndices, 2> Axes{{
    {.position = kf::StateX, .velocity = kf::StateVx},
    {.position = kf::StateY, .velocity = kf::StateVy},
}};

} // namespace

kf::StateTransition transition(Seconds dt) {
  kf::StateTransition f = kf::StateTransition::Identity();
  for (const AxisIndices& axis : Axes) {
    f(axis.position, axis.velocity) = dt.count();
  }
  return f;
}

kf::StateCovariance processNoise(Seconds dt, double accelerationNoiseDensity) {
  const double dt1 = dt.count();
  const double dt2 = dt1 * dt1;
  const double dt3 = dt2 * dt1;
  kf::StateCovariance q = kf::StateCovariance::Zero();
  for (const AxisIndices& axis : Axes) {
    q(axis.position, axis.position) = dt3 / 3.0;
    q(axis.position, axis.velocity) = dt2 / 2.0;
    q(axis.velocity, axis.position) = dt2 / 2.0;
    q(axis.velocity, axis.velocity) = dt1;
  }
  return accelerationNoiseDensity * q;
}

kf::MeasurementMatrix measurementMatrix() {
  kf::MeasurementMatrix h = kf::MeasurementMatrix::Zero();
  h(kf::MeasurementX, kf::StateX) = 1.0;
  h(kf::MeasurementY, kf::StateY) = 1.0;
  return h;
}

kf::MeasurementCovariance measurementNoise(double noiseStdM) {
  return noiseStdM * noiseStdM * kf::MeasurementCovariance::Identity();
}

} // namespace fusion::model
