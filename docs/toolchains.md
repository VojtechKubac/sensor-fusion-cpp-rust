# Pinned toolchains

Record these versions (or any intentional deviation) with every benchmark result.

| Tool | Pin | Notes |
| --- | --- | --- |
| Rust | `1.97.1` via [`rust-toolchain.toml`](../rust-toolchain.toml) | `cargo build --release` in `rust/` |
| C++ compiler | GCC 13 (Ubuntu 24.04 default) or Clang 18+ | Document which binary produced a result |
| C++ standard | C++23 | Set in `cpp/CMakeLists.txt` |
| CMake | ≥ 3.28 | |
| CMake build type | `Release` | |
| C++ warnings | `-Wall -Wextra -Wpedantic -Werror` | Every build config; third-party headers via `-isystem` |
| C++ release flags | `-O3 -DNDEBUG -march=x86-64-v2` | Not `-march=native` — results must be portable across similar desktops |
| clang-format | **18** (`clang-format-18`) | Ubuntu package `clang-format-18`. Do not use unversioned `clang-format` on PATH |
| clang-tidy | **18** (`clang-tidy-18`) | Ubuntu package `clang-tidy-18`. Style policy: [ADR 0004](adr/0004-cpp-style-and-qa-tools.md) |
| Catch2 | **3.5.4** | CMake `FetchContent` tag `v3.5.4` — not Noble apt `3.4.0`. See [ADR 0005](adr/0005-cpp-test-stack.md) |
| gcovr | **7.0** | Ubuntu apt `gcovr`; coverage reports only (no %-gate yet) |
| Mull | **0.37.0** / LLVM **18** (`mull-runner-18`) | GitHub `.deb`s — see below. Clang-18 builds only |
| Sanitizers | ASan+UBSan (Debug/tests); TSan later | CMake option; never on published Release benches. [ADR 0006](adr/0006-cpp-sanitizers-and-analyzers.md) |
| CPD | **PMD 7.29.0** (`pmd cpd`) | `cpp/` only; report-only until gated. [ADR 0006](adr/0006-cpp-sanitizers-and-analyzers.md) |
| IWYU | prefer Clang **18**-matched package | `-isystem` for third-party; project TUs only |
| Rust release profile | Cargo `[profile.release]` defaults (`opt-level = 3`, LTO off unless noted) | Any profile change is a report variable |
| Eigen | **3.4.0** | CMake `FetchContent` tag `3.4.0` (download only, header-only imported target). See [ADR 0001](adr/0001-matrix-libraries.md) |
| nalgebra | TBD | See [ADR 0001](adr/0001-matrix-libraries.md) |
| Python (sim generator) | ≥ 3.11, stdlib only | Used to write shared replay files |

## C++ format and tidy

```bash
# Install if missing (Ubuntu 24.04)
sudo apt install clang-format-18 clang-tidy-18

# Format check (CI will use the same binary)
clang-format-18 --dry-run -Werror $(find cpp -type f \( -name '*.cpp' -o -name '*.hpp' \))

# Apply locally
clang-format-18 -i $(find cpp -type f \( -name '*.cpp' -o -name '*.hpp' \))
```

clang-tidy needs `compile_commands.json` (CMake `CMAKE_EXPORT_COMPILE_COMMANDS`);
exact invocation lands with the QA-gate CMake/CI batch. Config paths (authoritative):
`cpp/.clang-format`, `cpp/.clang-tidy`, `cpp/.clangd`.

## Mull (mutation testing)

Not in Ubuntu apt. Install **both** release packages in one transaction
(`mull-18` depends on `mull (= 0.37.0)`):

```bash
curl -fsSL -o /tmp/mull-base.deb \
  https://github.com/mull-project/mull/releases/download/0.37.0/Mull-0.37.0-ubuntu-amd64-24.04.deb
curl -fsSL -o /tmp/mull-18.deb \
  https://github.com/mull-project/mull/releases/download/0.37.0/Mull-18-0.37.0-LLVM-18.1.3-ubuntu-amd64-24.04.deb
sudo apt install /tmp/mull-base.deb /tmp/mull-18.deb
mull-runner-18 --version
```

Requires `libclang-cpp18` and `libllvm18` (pulled in with `clang-18` tooling).
Prefer the versioned binary `mull-runner-18`. CMake/CI wiring lands after KF
unit tests exist ([ADR 0005](adr/0005-cpp-test-stack.md)).

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

# C++ (after loop-1 code lands)
cmake -S cpp -B build/cpp -DCMAKE_BUILD_TYPE=Release
cmake --build build/cpp -j
ctest --test-dir build/cpp -L unit --output-on-failure

# Rust
cargo build --release --manifest-path rust/Cargo.toml
```

Until the pipelines exist, the CMake/Cargo projects are stubs that configure and build a placeholder binary (and a Catch2 harness test) so the commands above stay valid. Eigen **3.4.0** and Catch2 **v3.5.4** are pulled with FetchContent on first configure (needs network once).
