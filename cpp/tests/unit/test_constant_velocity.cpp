#include "fusion/model/constant_velocity.hpp"

#include <catch2/catch_test_macros.hpp>

namespace kf = fusion::kf;
namespace model = fusion::model;

namespace {

constexpr double Tolerance = 1e-12;

} // namespace

TEST_CASE("transition advances position by velocity * dt", "[model][unit]") {
  const kf::StateVector state{1.0, 2.0, 3.0, 4.0};
  const kf::StateVector expected{2.5, 4.0, 3.0, 4.0};

  const kf::StateVector actual = model::transition(0.5) * state;

  CAPTURE(actual.transpose());
  REQUIRE(actual.isApprox(expected, Tolerance));
}

TEST_CASE("processNoise matches the white-noise-acceleration closed form", "[model][unit]") {
  // dt = 0.5, q = 2: dt^3/3 * q = 1/12, dt^2/2 * q = 1/4, dt * q = 1.
  kf::StateCovariance expected;
  expected << 1.0 / 12.0, 0.0, 0.25, 0.0, //
      0.0, 1.0 / 12.0, 0.0, 0.25,         //
      0.25, 0.0, 1.0, 0.0,                //
      0.0, 0.25, 0.0, 1.0;

  const kf::StateCovariance actual = model::processNoise(0.5, 2.0);

  CAPTURE(actual);
  REQUIRE(actual.isApprox(expected, Tolerance));
}

TEST_CASE("processNoise is consistent when a step is split in two", "[model][unit]") {
  // Continuous-time model: Q(2dt) = F(dt) Q(dt) F(dt)^T + Q(dt).
  const double dtS = 0.1;
  const double q = 3.0;
  const kf::StateTransition f = model::transition(dtS);
  const kf::StateCovariance halfStep = model::processNoise(dtS, q);

  const kf::StateCovariance composed = f * halfStep * f.transpose() + halfStep;
  const kf::StateCovariance fullStep = model::processNoise(2.0 * dtS, q);

  CAPTURE(composed, fullStep);
  REQUIRE(composed.isApprox(fullStep, Tolerance));
}

TEST_CASE("measurementMatrix selects position", "[model][unit]") {
  const kf::StateVector state{1.0, 2.0, 3.0, 4.0};
  const kf::MeasurementVector expected{1.0, 2.0};

  const kf::MeasurementVector actual = model::measurementMatrix() * state;

  CAPTURE(actual.transpose());
  REQUIRE(actual.isApprox(expected, Tolerance));
}

TEST_CASE("measurementNoise is sigma squared on the diagonal", "[model][unit]") {
  const kf::MeasurementCovariance expected = 0.25 * kf::MeasurementCovariance::Identity();

  const kf::MeasurementCovariance actual = model::measurementNoise(0.5);

  CAPTURE(actual);
  REQUIRE(actual.isApprox(expected, Tolerance));
}
