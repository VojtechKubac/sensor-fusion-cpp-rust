# ADR 0005: C++ test stack

- **Status:** Accepted
- **Date:** 2026-11-07

## Context

Loop-1 needs a fixed C++ test policy before the skeleton fills in: unit vs
integration, framework pin, coverage, mutation testing, and (later) property
tests. Benchmarks stay out of this ADR until a working pipeline exists.

## Decision

**Framework:** Catch2 **v3.5.4**, obtained via CMake `FetchContent` (pinned tag).
Do not rely on Ubuntu’s `catch2` package (Noble ships 3.4.0). Each test
translation unit is its own executable, linked with `Catch2::Catch2WithMain`
(Catch owns `main`; tests are `TEST_CASE`s only). No `catch_discover_tests` for
now — one CTest entry per executable. Filter inside an exe with Catch’s CLI.

**Layout:** `cpp/tests/unit/` and `cpp/tests/integration/`, CTest labels `unit`
and `integration`. Integration tests also use Catch2; they exercise boundaries
(replay files, SPSC/threads). Default local focus: `ctest -L unit`; CI runs
both. Replay input: generate from `sim/` (shared schema); tiny committed
fixtures only if a hermetic parser test needs them.

**Property tests:** Catch2 generators first; RapidCheck (pinned) when real
invariants exist. Not wired in the first harness batch.

**Coverage:** `gcovr` **7.0** (Ubuntu apt), optional CMake coverage build;
report-only in CI at first (no minimum-% gate). Use a non-Release build — never
mix with published bench flags.

**Mutation testing:** Mull **0.37.0** with the **LLVM 18** plugin
(`mull-runner-18`) (LLVM major superseded by [ADR 0007](0007-llvm-20-toolchain.md): **20**). Mutate Clang-18 builds only. Wire runners after KF unit
tests exist; until then the pin and install steps live in
[toolchains.md](../toolchains.md).

**Benchmarks:** deferred to `bench/` — not Catch2.

## Consequences

- System Catch2 3.4.0 must not silently satisfy `find_package` when 3.5.4 is
  required — FetchContent pin is authoritative.
- Mull needs two GitHub `.deb`s (`mull` + `mull-18`); it is not in Ubuntu apt.
- Coverage (GCC/gcovr) and Mull (Clang 18) are separate toolchains paths.
