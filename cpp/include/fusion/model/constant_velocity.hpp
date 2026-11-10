#ifndef FUSION_MODEL_CONSTANT_VELOCITY_HPP
#define FUSION_MODEL_CONSTANT_VELOCITY_HPP

#include "fusion/kf/types.hpp"

// 2D constant-velocity motion model with a position-only sensor.
// State [x, y, vx, vy], measurement [zx, zy] (fusion/kf/types.hpp).
namespace fusion::model {

// F(dt): advances each position by velocity * dt; velocities unchanged.
// dtS: time step [s].
[[nodiscard]] kf::StateTransition transition(double dtS);

// Q(dt) for continuous white-noise acceleration, per axis:
//   q * [[dt^3/3, dt^2/2],
//        [dt^2/2, dt    ]]
// dtS: time step [s]; accelerationNoiseDensity: q [m^2/s^3].
[[nodiscard]] kf::StateCovariance processNoise(double dtS, double accelerationNoiseDensity);

// H: selects [x, y] from the state.
[[nodiscard]] kf::MeasurementMatrix measurementMatrix();

// R = sigma^2 * I, i.i.d. per axis. noiseStdM: sigma [m].
[[nodiscard]] kf::MeasurementCovariance measurementNoise(double noiseStdM);

} // namespace fusion::model

#endif // FUSION_MODEL_CONSTANT_VELOCITY_HPP
