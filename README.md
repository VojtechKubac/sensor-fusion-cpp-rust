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
| [`rust/`](rust/) | Rust pipeline (nalgebra, Cargo) + Rerun viewer (`fusion_viz`) |
| [`bench/`](bench/) | Experiment runners (to come) |
| [`report/`](report/) | Raw results, plots, per-loop writeups |
| [`docs/`](docs/) | ADRs, friction log, toolchain pins |

## Build

Pinned versions and flags: [`docs/toolchains.md`](docs/toolchains.md).

```bash
# Shared baseline input (2D constant-velocity, one position sensor)
python3 sim/generate_baseline.py --out sim/out/baseline

(cd cpp && cmake --workflow --preset release)   # presets: cpp/README.md
cargo build --release --manifest-path rust/Cargo.toml
```

## Watch a replay

```bash
cargo install rerun-cli --locked --version 0.43.2      # viewer, once
cargo run --release --manifest-path rust/Cargo.toml -p fusion_viz -- --realtime
```

Opens the [Rerun](https://rerun.io) viewer and streams the Rust linear KF over
the baseline replay; the run is also saved to `report/out/loop1/loop1.rrd`.
Details: [`rust/README.md`](rust/README.md#viewer-rerun).

## Worktrees

This checkout uses a bare repository (`.bare/`) with `main/` as the primary worktree.
Feature branches live as sibling directories next to `main/`.

```bash
./start-worktree.sh <feature-name>
./remove-worktree.sh <feature-name>
```
