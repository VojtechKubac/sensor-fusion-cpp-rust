cmake_minimum_required(VERSION 3.28)
project(fusion_cpp LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Pinned Release flags for fair comparisons (docs/toolchains.md).
# RelWithDebInfo / Debug may be used for development but not for published numbers.
add_library(fusion_cpp_flags INTERFACE)
target_compile_options(
  fusion_cpp_flags
  INTERFACE
    $<$<CONFIG:Release>:-O3;-DNDEBUG;-march=x86-64-v2>
    $<$<CONFIG:Release>:-Wall;-Wextra;-Wpedantic>
)

add_executable(fusion_cpp src/main.cpp)
target_link_libraries(fusion_cpp PRIVATE fusion_cpp_flags)

install(TARGETS fusion_cpp RUNTIME DESTINATION bin)
