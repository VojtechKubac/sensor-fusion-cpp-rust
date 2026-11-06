# Rust pipeline

Hand-written filter and SPSC queue; nalgebra for fixed-size matrix math; Cargo
release profile with the toolchain in [`rust-toolchain.toml`](../rust-toolchain.toml).

```bash
cargo build --release --manifest-path rust/Cargo.toml
./rust/target/release/fusion_rust --help
```

Same design as `cpp/`, ported after each C++ feature loop — not before.
