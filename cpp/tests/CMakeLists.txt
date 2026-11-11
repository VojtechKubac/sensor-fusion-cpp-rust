# One executable per test .cpp (ADR 0005). Catch2 owns main via Catch2WithMain.
# Each TEST_CASE becomes its own CTest entry ("<exe>:<case>") for IDE runners.
list(APPEND CMAKE_MODULE_PATH ${catch2_SOURCE_DIR}/extras)
include(Catch)

function(fusion_cpp_add_test name label)
  add_executable(${name} ${ARGN})
  target_link_libraries(
    ${name}
    PRIVATE
      Catch2::Catch2WithMain
      fusion_core
  )
  catch_discover_tests(
    ${name}
    TEST_PREFIX "${name}:"
    PROPERTIES LABELS "${label}"
  )
endfunction()

fusion_cpp_add_test(test_harness unit unit/test_harness.cpp)
fusion_cpp_add_test(test_constant_velocity unit unit/test_constant_velocity.cpp)
fusion_cpp_add_test(test_linear_kf unit unit/test_linear_kf.cpp)
# integration/ — add targets here when replay/SPSC tests land.