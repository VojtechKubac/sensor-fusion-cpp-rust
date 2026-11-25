# Friction log

Running notes on where each language slowed development or prevented bugs.
Entries are short and dated. Per-feature-loop reports may cite these.

| Date | Loop / area | Language | What happened | Takeaway |
| --- | --- | --- | --- | --- |
| 2026-11-11 | 1 / linear KF | C++ | `std::expected` invisible to Clang 18 (`__cpp_concepts` 201907) with libstdc++ 13; clang-tidy/clangd/Mull-18 broke on any TU using it | Bumped LLVM toolchain to 20 ([ADR 0007](adr/0007-llvm-20-toolchain.md)); C++23 library support depends on compiler × stdlib pairing |
| 2026-11-25 | 1 / Rerun | tooling | `rerun` / `rerun-cli` after 0.40 need rustc 1.98; pinned 1.97.1 blocked the install | Bumped pin to 1.98.1; check a crate's `rust-version` before pinning the toolchain |

## Conventions

- **Loop / area** — e.g. `1 / SPSC queue`, `sim / schema`.
- **Language** — `C++`, `Rust`, `both`, or `tooling`.
- Log C++ design choices that leaked into the Rust port (and the reverse).
- Log every justified `unsafe` (Rust) or raw owning pointer (C++) here as well as in the loop report.
