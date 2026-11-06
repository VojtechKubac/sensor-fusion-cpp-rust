# C++ pipeline

Hand-written filter and SPSC queue; Eigen for fixed-size matrix math; CMake
Release builds with pinned flags (see [`docs/toolchains.md`](../docs/toolchains.md)).

```bash
cmake -S cpp -B build/cpp -DCMAKE_BUILD_TYPE=Release
cmake --build build/cpp -j
./build/cpp/fusion_cpp --help
```

Loop 1 will add the linear KF, one sensor consumer, and latency probes. Until
then this package builds a placeholder binary so the toolchain pin is exercisable.
