#include <catch2/catch_test_macros.hpp>

// Placeholder so FetchContent Catch2 + CTest are exercisable before loop-1 code.
TEST_CASE("test harness links Catch2WithMain", "[harness][unit]") {
  REQUIRE(1 + 1 == 2);
}