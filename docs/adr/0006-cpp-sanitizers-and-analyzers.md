# ADR 0006: C++ sanitizers, CPD, and IWYU

- **Status:** Accepted
- **Date:** 2026-11-07

## Context

Beyond format/tidy/tests (ADRs 0004–0005), the C++ port should catch memory/UB
issues early and keep includes and duplication honest. Heavy platforms
(cppcheck as a second linter, CodeChecker, SonarQube) add little on top of
clang-tidy for this size of tree and are deferred.

## Decision

**Sanitizers:** Support a non-Release CMake option (name TBD when wired) that
builds and runs tests with **ASan + UBSan**. Add **TSan** when SPSC/threading
tests exist (separate preset or option — TSan does not mix with ASan). Never
enable sanitizers on published benchmark Release builds.

**clang-tidy complexity:** Part of ADR 0004’s `cpp/.clang-tidy` — not a separate
tool.

**CPD (copy-paste / DRY):** **PMD CPD** (not jscpd), scoped to **`cpp/` only**
(never `rust/` in the same job — intentional dual ports would look like
clones). Pin **PMD 7.29.0** (see [toolchains.md](../toolchains.md)).
Report-only at first; failing the gate is a later switch. jscpd stays out of
scope unless PMD proves painful.

**IWYU (include-what-you-use):** Adopt once the include root and Eigen wiring
exist. Steer noise down by:
- compiling third-party with **`-isystem`** (Eigen, Catch2),
- project mapping / IWYU pragmas only where needed,
- analyzing **project** TUs under `cpp/include` and `cpp/src` (not deps).

**Explicitly not now:** cppcheck, CodeChecker/scan-build UI, SonarQube.
Mull remains ADR 0005 — wire after KF unit tests exist.

## Consequences

- CI will need at least one sanitizer test job distinct from Release.
- CPD/IWYU jobs can start report-only; failing the gate is a later switch.
- IWYU version should track Clang 18 where packages allow (LLVM major superseded by [ADR 0007](0007-llvm-20-toolchain.md): **20**).
