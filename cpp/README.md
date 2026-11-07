# C++ pipeline

Hand-written filter and SPSC queue; Eigen for fixed-size matrix math; CMake
Release builds with pinned flags (see [`docs/toolchains.md`](../docs/toolchains.md)).
C++ naming, includes, `.hpp`, and format/tidy pins: [ADR 0004](../docs/adr/0004-cpp-style-and-qa-tools.md).
Test stack (Catch2, coverage, Mull): [ADR 0005](../docs/adr/0005-cpp-test-stack.md).

```bash
cmake -S cpp -B build/cpp -DCMAKE_BUILD_TYPE=Release
cmake --build build/cpp -j
./build/cpp/fusion_cpp --help

# Unit tests (Catch2 v3.5.4 via FetchContent)
ctest --test-dir build/cpp -L unit --output-on-failure
# Single exe during development:
cmake --build build/cpp --target test_harness && ./build/cpp/tests/test_harness
```

Loop 1 will add the linear KF, one sensor consumer, and latency probes. Until
then this package builds a placeholder binary so the toolchain pin is exercisable.
