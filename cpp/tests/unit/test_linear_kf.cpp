#include "fusion/kf/linear_kf.hpp"
#include "fusion/model/constant_velocity.hpp"

#include <catch2/catch_test_macros.hpp>
#include <catch2/generators/catch_generators.hpp>
#include <Eigen/Eigenvalues>

#include <cmath>

namespace kf = fusion::kf;
namespace model = fusion::model;

namespace {

constexpr double Tolerance = 1e-12;

kf::StateCovariance diagonalCovariance() {
  return kf::StateVector{1.0, 2.0, 3.0, 4.0}.asDiagonal();
}

// Filter after one CV predict, so P has position-velocity correlation.
kf::LinearKalmanFilter predictedFilter(kf::CovarianceUpdate form) {
  kf::LinearKalmanFilter filter{kf::StateVector{1.0, 2.0, 3.0, 4.0}, diagonalCovariance(), form};
  filter.predict(model::transition(0.1), model::processNoise(0.1, 1.0));
  return filter;
}

const kf::MeasurementVector SampleMeasurement{1.2, 1.8};

} // namespace

TEST_CASE("predict without process noise propagates state and covariance", "[kf][unit]") {
  kf::LinearKalmanFilter filter{kf::StateVector{1.0, 2.0, 3.0, 4.0}, diagonalCovariance()};
  const kf::StateVector expectedState{2.5, 4.0, 3.0, 4.0};
  // Per axis F P F^T with F = [[1, 0.5], [0, 1]], P = diag(p, v):
  //   [[p + 0.25 v, 0.5 v], [0.5 v, v]]  ->  x: (1, 3), y: (2, 4).
  kf::StateCovariance expectedCovariance;
  expectedCovariance << 1.75, 0.0, 1.5, 0.0, //
      0.0, 3.0, 0.0, 2.0,                    //
      1.5, 0.0, 3.0, 0.0,                    //
      0.0, 2.0, 0.0, 4.0;

  filter.predict(model::transition(0.5), kf::StateCovariance::Zero());

  CAPTURE(filter.state().transpose(), filter.covariance());
  REQUIRE(filter.state().isApprox(expectedState, Tolerance));
  REQUIRE(filter.covariance().isApprox(expectedCovariance, Tolerance));
}

TEST_CASE("predict adds process noise to the covariance", "[kf][unit]") {
  const kf::StateVector initialState{1.0, 2.0, 3.0, 4.0};
  kf::LinearKalmanFilter filter{initialState, diagonalCovariance()};
  const kf::StateCovariance q = 0.1 * kf::StateCovariance::Identity();

  filter.predict(kf::StateTransition::Identity(), q);

  CAPTURE(filter.covariance());
  REQUIRE(filter.state().isApprox(initialState, Tolerance));
  REQUIRE(filter.covariance().isApprox(diagonalCovariance() + q, Tolerance));
}

TEST_CASE("update with equal prior and sensor variance halves position variance", "[kf][unit]") {
  kf::LinearKalmanFilter filter{kf::StateVector::Zero(), kf::StateCovariance::Identity()};
  const kf::StateVector expectedState{0.5, 0.5, 0.0, 0.0};
  const kf::StateCovariance expectedCovariance = kf::StateVector{0.5, 0.5, 1.0, 1.0}.asDiagonal();

  const auto result =
      filter.update({1.0, 1.0}, model::measurementMatrix(), model::measurementNoise(1.0));

  REQUIRE(result.has_value());
  CAPTURE(filter.state().transpose(), filter.covariance());
  REQUIRE(filter.state().isApprox(expectedState, Tolerance));
  REQUIRE(filter.covariance().isApprox(expectedCovariance, Tolerance));
}

TEST_CASE("update reports innovation, its covariance and NIS", "[kf][unit]") {
  // P = 0, so S = R and the state must not move.
  kf::LinearKalmanFilter filter{kf::StateVector::Zero(), kf::StateCovariance::Zero()};
  const kf::MeasurementCovariance r = kf::MeasurementVector{4.0, 1.0}.asDiagonal();

  const auto result = filter.update({2.0, 3.0}, model::measurementMatrix(), r);

  REQUIRE(result.has_value());
  REQUIRE(result->innovation.isApprox(kf::MeasurementVector{2.0, 3.0}, Tolerance));
  REQUIRE(result->innovationCovariance.isApprox(r, Tolerance));
  REQUIRE(std::abs(result->nis - 10.0) < Tolerance); // 2^2/4 + 3^2/1
  REQUIRE(filter.state().isZero());
}

TEST_CASE("update never increases uncertainty and keeps P symmetric", "[kf][unit]") {
  const auto form = GENERATE(kf::CovarianceUpdate::Joseph, kf::CovarianceUpdate::Simple);
  kf::LinearKalmanFilter filter = predictedFilter(form);
  const kf::StateCovariance prior = filter.covariance();

  REQUIRE(
      filter.update(SampleMeasurement, model::measurementMatrix(), model::measurementNoise(0.5)));

  const kf::StateCovariance posterior = filter.covariance();
  const Eigen::SelfAdjointEigenSolver<kf::StateCovariance> shrink(prior - posterior);
  CAPTURE(prior, posterior);
  REQUIRE(shrink.eigenvalues().minCoeff() > -Tolerance); // prior - posterior is PSD
  REQUIRE((posterior - posterior.transpose()).norm() < Tolerance);
}

TEST_CASE("Joseph and Simple covariance forms agree on a well-conditioned update", "[kf][unit]") {
  kf::LinearKalmanFilter joseph = predictedFilter(kf::CovarianceUpdate::Joseph);
  kf::LinearKalmanFilter simple = predictedFilter(kf::CovarianceUpdate::Simple);
  const kf::MeasurementMatrix h = model::measurementMatrix();
  const kf::MeasurementCovariance r = model::measurementNoise(0.5);

  REQUIRE(joseph.update(SampleMeasurement, h, r));
  REQUIRE(simple.update(SampleMeasurement, h, r));

  CAPTURE(joseph.covariance(), simple.covariance());
  REQUIRE(joseph.state().isApprox(simple.state(), Tolerance));
  REQUIRE(joseph.covariance().isApprox(simple.covariance(), Tolerance));
}

TEST_CASE("update rejects a non-positive-definite S and leaves the estimate", "[kf][unit]") {
  const kf::StateVector initialState{1.0, 2.0, 3.0, 4.0};
  kf::LinearKalmanFilter filter{initialState, kf::StateCovariance::Zero()};

  const auto result = filter.update(
      SampleMeasurement, model::measurementMatrix(), -kf::MeasurementCovariance::Identity());

  REQUIRE_FALSE(result.has_value());
  REQUIRE(result.error() == kf::UpdateError::InnovationCovarianceNotPositiveDefinite);
  REQUIRE(filter.state() == initialState);
  REQUIRE(filter.covariance().isZero());
}
