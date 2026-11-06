# Real-Time Sensor Fusion in C++ and Rust

Build the same real-time multi-sensor fusion pipeline twice — once in C++ and once in
Rust — and report honestly what each language costs and buys in latency, jitter,
throughput and footprint.

A shared simulator and benchmark harness drive both implementations so measured
differences come from the languages and their ecosystems, not from the test setup.

## Repository layout

```text
sim/      shared simulator, message formats, fault replay files
cpp/      C++ pipeline (Eigen, CMake)
rust/     Rust pipeline (nalgebra, Cargo)
bench/    harness, metrics, one-command experiment runner
report/   raw results, plots, per-feature-loop writeups
docs/     ADRs, friction log, architecture notes
```

This checkout uses a bare repository (`.bare/`) with `main/` as the primary worktree.
Feature branches live as sibling worktrees next to `main/`.

From the directory that contains `.bare`:

```bash
./start-worktree.sh <feature-name>   # create/reuse worktree + tmux (Claude | shell)
./remove-worktree.sh <feature-name>  # kill tmux, remove worktree and local branch
```
