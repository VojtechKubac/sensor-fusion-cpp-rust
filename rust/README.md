# Rust pipeline

Hand-written filter and SPSC queue; nalgebra for fixed-size matrix math; Cargo
release profile with the toolchain in [`rust-toolchain.toml`](../rust-toolchain.toml).

```bash
cargo run --release --manifest-path rust/Cargo.toml -- sim/out/baseline
```

Replays the simulator files through the linear KF and prints position RMSE
and mean NIS. Same design as `cpp/`; for loop 1 the Rust port leads.
