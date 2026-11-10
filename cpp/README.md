# C++ pipeline

Hand-written filter and SPSC queue; Eigen for fixed-size matrix math; CMake
Release builds with pinned flags (see [`docs/toolchains.md`](../docs/toolchains.md)).
C++ naming, includes, `.hpp`, and format/tidy pins: [ADR 0004](../docs/adr/0004-cpp-style-and-qa-tools.md).
Test stack (Catch2, coverage, Mull): [ADR 0005](../docs/adr/0005-cpp-test-stack.md).
Sanitizers, CPD, IWYU: [ADR 0006](../docs/adr/0006-cpp-sanitizers-and-analyzers.md).

```bash
# Presets live in cpp/CMakePresets.json; run from cpp/.
cd cpp
cmake --workflow --preset debug     # configure + build + test (build/cpp/debug)
cmake --workflow --preset release   # same with pinned Release flags (build/cpp/release)
cmake --build --preset debug        # rebuild only
ctest --preset debug                # tests only
../build/cpp/release/fusion_cpp --help

# Single test exe during development:
cmake --build --preset debug --target test_constant_velocity \
  && ../build/cpp/debug/tests/test_constant_velocity
```

Builds are serial with Makefiles unless `CMAKE_BUILD_PARALLEL_LEVEL` is set
(e.g. `export CMAKE_BUILD_PARALLEL_LEVEL=$(nproc)` in your shell profile).
clangd reads the Debug compile DB (`cpp/.clangd`).

Targets: `fusion_core` (library; headers under `include/fusion/`, links Eigen
3.4.0 via FetchContent) and `fusion_cpp` (thin `main`). Tests link `fusion_core`.
Warnings are errors in every build config.

Loop 1 will add the linear KF, one sensor consumer, and latency probes. Until
then this package builds a placeholder binary so the toolchain pin is exercisable.
