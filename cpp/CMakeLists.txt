cmake_minimum_required(VERSION 3.28)
project(fusion_cpp LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

include(FetchContent)

# Eigen 3.4.0 (docs/toolchains.md). SOURCE_SUBDIR points at a missing dir so
# FetchContent only downloads: Eigen's own CMake project (tests, BLAS probes,
# uninstall target) stays out of our build. Header-only, so an imported
# INTERFACE target is enough; imported include dirs are -isystem (ADR 0006).
FetchContent_Declare(
  Eigen
  GIT_REPOSITORY https://gitlab.com/libeigen/eigen.git
  GIT_TAG 3.4.0
  GIT_SHALLOW TRUE
  SOURCE_SUBDIR do-not-configure
)
FetchContent_MakeAvailable(Eigen)
add_library(Eigen3::Eigen INTERFACE IMPORTED)
target_include_directories(Eigen3::Eigen INTERFACE ${eigen_SOURCE_DIR})

# Warnings are errors in every config. Optimisation flags are pinned for
# Release only (docs/toolchains.md); other configs are for development.
add_library(fusion_cpp_flags INTERFACE)
target_compile_options(
  fusion_cpp_flags
  INTERFACE
    -Wall
    -Wextra
    -Wpedantic
    -Werror
    $<$<CONFIG:Release>:-O3;-DNDEBUG;-march=x86-64-v2>
)

# Filter, queue and replay code. INTERFACE until the first .cpp lands; switch
# to STATIC then. Headers live under include/fusion/ ("fusion/…", ADR 0004).
add_library(fusion_core INTERFACE)
target_include_directories(fusion_core INTERFACE include)
target_link_libraries(fusion_core INTERFACE Eigen3::Eigen fusion_cpp_flags)

add_executable(fusion_cpp src/main.cpp)
target_link_libraries(fusion_cpp PRIVATE fusion_core)

install(TARGETS fusion_cpp RUNTIME DESTINATION bin)

include(CTest)
if(BUILD_TESTING)
  FetchContent_Declare(
    Catch2
    GIT_REPOSITORY https://github.com/catchorg/Catch2.git
    GIT_TAG v3.5.4
    GIT_SHALLOW TRUE
    SYSTEM
  )
  FetchContent_MakeAvailable(Catch2)
  add_subdirectory(tests)
endif()
