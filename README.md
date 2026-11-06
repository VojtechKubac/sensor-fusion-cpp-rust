# Real-Time Sensor Fusion in C++ and Rust

Build the same real-time multi-sensor fusion pipeline twice — once in C++ and once in
Rust — and report honestly what each language costs and buys in latency, jitter,
throughput and footprint.

A shared simulator and benchmark harness drive both implementations so measured
differences come from the languages and their ecosystems, not from the test setup.

License: [MIT](LICENSE).

## Layout

| Path | Role |
| --- | --- |
| [`sim/`](sim/) | Shared schemas + generators (identical replay for both ports) |
| [`cpp/`](cpp/) | C++ pipeline (Eigen, CMake) |
| [`rust/`](rust/) | Rust pipeline (nalgebra, Cargo) |
| [`bench/`](bench/) | Experiment runners (to come) |
| [`report/`](report/) | Raw results, plots, per-loop writeups |
| [`docs/`](docs/) | ADRs, friction log, toolchain pins |

## Build

Pinned versions and flags: [`docs/toolchains.md`](docs/toolchains.md).

```bash
# Shared baseline input (2D constant-velocity, one position sensor)
python3 sim/generate_baseline.py --out sim/out/baseline

cmake -S cpp -B build/cpp -DCMAKE_BUILD_TYPE=Release && cmake --build build/cpp -j
cargo build --release --manifest-path rust/Cargo.toml
```

Pipelines are stubs until feature loop 1 (linear KF). The commands above already
configure and build the placeholders.

## Worktrees

This checkout uses a bare repository (`.bare/`) with `main/` as the primary worktree.
Feature branches live as sibling directories next to `main/`.

```bash
./start-worktree.sh <feature-name>
./remove-worktree.sh <feature-name>
```
