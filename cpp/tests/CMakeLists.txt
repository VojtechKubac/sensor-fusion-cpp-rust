# One executable per test .cpp (ADR 0005). Catch2 owns main via Catch2WithMain.

function(fusion_cpp_add_test name label)
  add_executable(${name} ${ARGN})
  target_link_libraries(
    ${name}
    PRIVATE
      Catch2::Catch2WithMain
      fusion_core
  )
  add_test(NAME ${name} COMMAND ${name})
  set_tests_properties(${name} PROPERTIES LABELS "${label}")
endfunction()

fusion_cpp_add_test(test_harness unit unit/test_harness.cpp)
fusion_cpp_add_test(test_constant_velocity unit unit/test_constant_velocity.cpp)
# integration/ — add targets here when replay/SPSC tests land.