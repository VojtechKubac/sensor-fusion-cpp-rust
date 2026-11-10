# ADR 0004: C++ style and QA tool pins

- **Status:** Accepted
- **Date:** 2026-11-07

## Context

The C++ port needs a mandatory format gate and a curated clang-tidy set before
the loop-1 skeleton lands. Local PATH may expose newer unversioned
`clang-format` / `clang-tidy` binaries than CI will run, so the pin must be
explicit. Naming and include rules should be fixed once so headers and tests do
not churn later.

## Decision

**Tools (see also [toolchains.md](../toolchains.md)):**

- Pin **clang-format-18** and **clang-tidy-18** (Ubuntu 24.04 packages).
- Always invoke the versioned binaries; do not rely on unversioned names on PATH.
- Format style: **LLVM**-based, details live in `cpp/.clang-format` (to land with
  the QA gate).
- Tidy: curated bug/perf/analyzer set with WarningsAsErrors; Eigen treated as
  third-party (not linted as project code). Config in `cpp/.clang-tidy`.
- Enable **complexity** checks in that config:
  `readability-function-cognitive-complexity` (threshold **25**, clang-tidy
  default) and `readability-function-size` (defaults). Retune when hot paths
  need it.

**Headers and names:**

- Header extension: **`.hpp`** (C++-only tree; no C API).
- File names: **`snake_case`** (e.g. `linear_kf.hpp`).
- Types: `CamelCase`. Functions / methods: `camelBack`. Members: `camelBack_`
  with trailing underscore; public members of plain aggregates: `camelBack`.
  Named constants: **PascalCase** (e.g. `ProcessNoise`). Macros only:
  `SCREAMING_SNAKE`.

**Includes:**

- `#include <…>` — C++ standard library and third-party (Eigen, Catch2, …).
- `#include "…"` — project / local headers (prefer `"fusion/…"` from an include
  root; no `"../…"`).
- Headers use **classic include guards** (not `#pragma once`), named from the
  path under `cpp/include/`: `fusion/kf/types.hpp` → `FUSION_KF_TYPES_HPP`;
  close with `#endif // FUSION_KF_TYPES_HPP`. Enforced by clang-tidy
  `llvm-header-guard`.

## Consequences

- CI and local format/tidy must use the same major; install `clang-tidy-18` if
  missing (`clang-format-18` is commonly already present on Noble).
- `.h` and hyphenated file names are out of scope unless a C boundary appears.
- Config files under `cpp/` are authoritative for the exact format/tidy knobs;
  this ADR records the policy, not every YAML key.
