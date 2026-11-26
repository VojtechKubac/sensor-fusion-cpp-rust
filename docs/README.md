# Project documentation

| Path | Purpose |
| --- | --- |
| [toolchains.md](toolchains.md) | Pinned compilers, flags, and build commands |
| [friction-log.md](friction-log.md) | Ongoing C++ / Rust friction and bug-prevention notes |
| [adr/](adr/) | Architecture decision records |

## Visualization

[Rerun](https://rerun.io) shows replays on a `sim_time` timeline: truth,
measurements, the estimate with its 2σ ellipse, and error / σ / NIS plots.
Only the `rust/fusion_viz` crate links it; benchmarked binaries do not. The
viewer (`rerun-cli`) and the `rerun` crate must have the same version, pinned
in [toolchains.md](toolchains.md). Usage: [`../rust/README.md`](../rust/README.md#viewer-rerun).

Feature-loop comparison writeups live under [`../report/`](../report/), next to the raw data and plots that produced them.
