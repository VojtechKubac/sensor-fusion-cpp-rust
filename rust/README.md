# Rust pipeline

Hand-written filter and SPSC queue; nalgebra for fixed-size matrix math; Cargo
release profile with the toolchain in [`rust-toolchain.toml`](../rust-toolchain.toml).

```bash
cargo run --release --manifest-path rust/Cargo.toml -- sim/out/baseline
```

Replays the simulator files through the linear KF and prints position RMSE
and mean NIS. Same design, state layout and replay files as `cpp/`.

## Viewer (Rerun)

```bash
cargo install rerun-cli --locked --version 0.43.2   # once; must match the crate
cargo run --release --manifest-path rust/Cargo.toml -p fusion_viz -- --realtime
rerun report/out/loop1/loop1.rrd                    # reopen a saved run
```

`fusion_viz` spawns a viewer (or reuses one on port 9876), streams each filter
step on the `sim_time` timeline, and saves the same stream to
`report/out/loop1/loop1.rrd`. Without `--realtime` the 20 s replay is sent at
once. Only this crate links Rerun; `fusion_rust` (benchmarked) does not.
