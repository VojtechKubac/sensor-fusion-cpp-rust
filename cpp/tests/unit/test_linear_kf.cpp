#include "fusion/kf/linear_kf.hpp"
#include "fusion/model/constant_velocity.hpp"

#include <catch2/catch_test_macros.hpp>

namespace kf = fusion::kf;
namespace model = fusion::model;

namespace {

constexpr double Tolerance = 1e-12;

kf::StateCovariance diagonalCovariance() {
  return kf::StateVector{1.0, 2.0, 3.0, 4.0}.asDiagonal();
}

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
