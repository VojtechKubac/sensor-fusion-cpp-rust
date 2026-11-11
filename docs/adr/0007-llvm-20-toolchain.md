# ADR 0007: LLVM 20 toolchain (Clang, clang-tidy, clang-format, clangd, Mull)

- **Status:** Accepted
- **Date:** 2026-11-11
- **Supersedes:** the LLVM **18** pins in ADRs 0004, 0005, 0006

## Context

The linear KF reports a non-positive-definite innovation covariance through
C++23 `std::expected`. libstdc++ 13 (GCC 13, Ubuntu 24.04) only exposes
`std::expected` when `__cpp_concepts >= 202002L`; Clang 18 reports `201907L`.
Every Clang-18-based tool (compiler, clang-tidy-18, clangd-18, Mull-18) failed
on any translation unit using it. Clang 19+ reports `202002L`.

Ubuntu 24.04 `noble-updates` ships LLVM 19.1.1 and 20.1.2, and Mull 0.37.0
publishes plugins built against exactly those versions.

## Decision

- Pin the whole LLVM-based tool set to major **20** (20.1.2): `clang-20`,
  `clang-tidy-20`, `clang-format-20`, `clangd-20`, and Mull **0.37.0**
  with the `mull-20` plugin. GCC 13 stays the default compiler.
- Install through `tools/install-cpp-toolchain.sh`, used locally and in CI
  (`ubuntu-24.04` runners) so pins live in one place.

## Consequences

- C++23 library features gated on concepts (`std::expected`) work in all tools.
- clang-format output may differ slightly from 18; the tree is reformatted once.
- Future LLVM bumps change the script and this ADR family only.
