# ADR 0003: Monorepo layout

- **Status:** Accepted
- **Date:** 2026-11-06

## Context

C++ and Rust must share one simulator, one message format, and one benchmark
harness. Separate repositories would drift.

## Decision

One monorepo:

| Directory | Role |
| --- | --- |
| `sim/` | Shared schemas and generators for replay input |
| `cpp/` | C++ pipeline (CMake, Eigen) |
| `rust/` | Rust pipeline (Cargo, nalgebra) |
| `bench/` | Experiment runners and metric collection |
| `report/` | Raw results, plots, per-loop writeups |
| `docs/` | ADRs, friction log, toolchain pins |

## Consequences

- Format changes land once and both ports update in the same change set.
- CI can build both ports and regenerate plots from one checkout.
