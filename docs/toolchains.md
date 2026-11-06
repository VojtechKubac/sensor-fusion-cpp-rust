# Pinned toolchains

Record these versions (or any intentional deviation) with every benchmark result.

| Tool | Pin | Notes |
| --- | --- | --- |
| Rust | `1.97.1` via [`rust-toolchain.toml`](../rust-toolchain.toml) | `cargo build --release` in `rust/` |
| C++ compiler | GCC 13 (Ubuntu 24.04 default) or Clang 18+ | Document which binary produced a result |
| C++ standard | C++20 | Set in `cpp/CMakeLists.txt` |
| CMake | ≥ 3.28 | |
| CMake build type | `Release` | |
| C++ release flags | `-O3 -DNDEBUG -march=x86-64-v2` | Not `-march=native` — results must be portable across similar desktops |
| Rust release profile | Cargo `[profile.release]` defaults (`opt-level = 3`, LTO off unless noted) | Any profile change is a report variable |
| Matrix libraries | Eigen (C++), nalgebra (Rust) | See [ADR 0001](adr/0001-matrix-libraries.md) |
| Python (sim generator) | ≥ 3.11, stdlib only | Used to write shared replay files |

## How to build

```bash
# Shared baseline input (one sensor + ground truth)
python3 sim/generate_baseline.py --out sim/out/baseline

# C++ (after loop-1 code lands)
cmake -S cpp -B build/cpp -DCMAKE_BUILD_TYPE=Release
cmake --build build/cpp -j

# Rust
cargo build --release --manifest-path rust/Cargo.toml
```

Until the pipelines exist, the CMake/Cargo projects are stubs that configure and build a placeholder binary so the commands above stay valid.
