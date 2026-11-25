# Pinned toolchains

Record these versions (or any intentional deviation) with every benchmark result.

| Tool | Pin | Notes |
| --- | --- | --- |
| Rust | `1.98.1` via [`rust-toolchain.toml`](../rust-toolchain.toml) | `cargo build --release` in `rust/` |
| C++ compiler | GCC 13 (Ubuntu 24.04 default) or Clang **20** | Document which binary produced a result |
| C++ standard | C++23 | Set in `cpp/CMakeLists.txt` |
| CMake | ≥ 3.28 | |
| CMake build type | `Release` | |
| C++ warnings | `-Wall -Wextra -Wpedantic -Werror` | Every build config; third-party headers via `-isystem` |
| C++ release flags | `-O3 -DNDEBUG -march=x86-64-v2` | Not `-march=native` — results must be portable across similar desktops |
| clang-format | **20** (`clang-format-20`) | Ubuntu package `clang-format-20`. Do not use unversioned `clang-format` on PATH |
| clang-tidy | **20** (`clang-tidy-20`) | Ubuntu package `clang-tidy-20`. Style policy: [ADR 0004](adr/0004-cpp-style-and-qa-tools.md) |
| Catch2 | **3.5.4** | CMake `FetchContent` tag `v3.5.4` — not Noble apt `3.4.0`. See [ADR 0005](adr/0005-cpp-test-stack.md) |
| gcovr | **7.0** | Ubuntu apt `gcovr`; coverage reports only (no %-gate yet) |
| Mull | **0.37.0** / LLVM **20.1.2** (`mull-runner-20`) | GitHub `.deb`s — see below. Clang-20 builds only |
| Sanitizers | ASan+UBSan (Debug/tests); TSan later | CMake option; never on published Release benches. [ADR 0006](adr/0006-cpp-sanitizers-and-analyzers.md) |
| CPD | **PMD 7.29.0** (`pmd cpd`) | `cpp/` only; report-only until gated. [ADR 0006](adr/0006-cpp-sanitizers-and-analyzers.md) |
| IWYU | prefer Clang **20**-matched package | `-isystem` for third-party; project TUs only |
| Rust release profile | Cargo `[profile.release]` defaults (`opt-level = 3`, LTO off unless noted) | Any profile change is a report variable |
| Eigen | **3.4.0** | CMake `FetchContent` tag `3.4.0` (download only, header-only imported target). See [ADR 0001](adr/0001-matrix-libraries.md) |
| nalgebra | **0.35.0** (`default-features = false`, `std`) | Exact pin in `rust/Cargo.toml`. See [ADR 0001](adr/0001-matrix-libraries.md) |
| serde / serde_json | **1.0.229** (`derive`) / **1.0.154** | Replay metadata (`meta.json`) only, parsed once at startup |
| Rerun | **0.43.2** (`rerun` crate + `rerun-cli` viewer, versions must match) | Visualization only, never linked into the benchmarked `fusion_rust`. Needs rustc ≥ 1.98 |
| Python (sim generator) | ≥ 3.11, stdlib only | Used to write shared replay files |

## C++ format and tidy

```bash
# Install the whole pinned toolchain (Ubuntu 24.04)
./tools/install-cpp-toolchain.sh

# Format check (CI will use the same binary)
clang-format-20 --dry-run -Werror $(find cpp -type f \( -name '*.cpp' -o -name '*.hpp' \))

# Apply locally
clang-format-20 -i $(find cpp -type f \( -name '*.cpp' -o -name '*.hpp' \))
```

clang-tidy needs `compile_commands.json` (CMake `CMAKE_EXPORT_COMPILE_COMMANDS`);
exact invocation lands with the QA-gate CMake/CI batch. Config paths (authoritative):
`cpp/.clang-format`, `cpp/.clang-tidy`, `cpp/.clangd`.

## Mull (mutation testing)

Not in Ubuntu apt: `tools/install-cpp-toolchain.sh` installs the core `mull`
and `mull-20` `.deb`s from the GitHub release in one transaction (`mull-20`
depends on `mull (= 0.37.0)`). Requires `libclang-cpp20` / `libllvm20` (pulled
in with the `clang-20` tooling). Use the versioned binary `mull-runner-20`.
CMake/CI wiring lands after KF unit tests exist ([ADR 0005](adr/0005-cpp-test-stack.md)).

## CI

CI must run on `runs-on: ubuntu-24.04` (not `ubuntu-latest`) and install tools
with `tools/install-cpp-toolchain.sh`, so local and CI pins cannot drift.
LLVM major bump rationale: [ADR 0007](adr/0007-llvm-20-toolchain.md).

## PMD CPD (clone detection)

Needs a JDK on `PATH` (local: Temurin/OpenJDK 25 is fine). Install the pinned
binary distribution (not an apt snapshot):

```bash
curl -fsSL -o /tmp/pmd-dist-7.29.0-bin.zip \
  https://github.com/pmd/pmd/releases/download/pmd_releases%2F7.29.0/pmd-dist-7.29.0-bin.zip
sudo mkdir -p /opt/pmd
sudo unzip -q -o /tmp/pmd-dist-7.29.0-bin.zip -d /opt/pmd
# CLI: /opt/pmd/pmd-bin-7.29.0/bin/pmd
export PATH="/opt/pmd/pmd-bin-7.29.0/bin:$PATH"
pmd --version
```

Report-only scan of the C++ tree (tune `--minimum-tokens` when wiring CI):

```bash
pmd cpd --dir cpp --language cpp --minimum-tokens 100 --no-fail-on-violation
```

## How to build

```bash
# Shared baseline input (one sensor + ground truth)
python3 sim/generate_baseline.py --out sim/out/baseline

# C++ (cpp/CMakePresets.json: configure + build + test)
(cd cpp && cmake --workflow --preset release)

# Rust
cargo build --release --manifest-path rust/Cargo.toml
```

Until the pipelines exist, the CMake/Cargo projects are stubs that configure and build a placeholder binary (and a Catch2 harness test) so the commands above stay valid. Eigen **3.4.0** and Catch2 **v3.5.4** are pulled with FetchContent on first configure (needs network once).
