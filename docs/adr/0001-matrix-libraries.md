# ADR 0001: Idiomatic matrix libraries (Eigen and nalgebra)

- **Status:** Accepted
- **Date:** 2026-11-06

## Context

Both ports need dense linear algebra for Kalman predict/update. Hand-rolling
matrix multiply and decompositions would dominate the work and would not reflect
how a competent developer writes either language. Idiomatic choices are Eigen
(C++) and nalgebra (Rust), both used with fixed-size matrices on the hot path.

Library quality and performance then become part of the C++ vs Rust comparison.

## Decision

- Use **Eigen** in C++ and **nalgebra** in Rust for all filter math.
- Prefer **fixed-size** matrices/vectors matching the filter state dimension.
- Treat library differences as an explicit experimental variable: reports must
  name Eigen/nalgebra effects instead of attributing them to the language alone.
- Core filter and queue logic remains hand-written; only matrix primitives come
  from the libraries.

## Consequences

- Faster to reach a correct KF/EKF/UKF, at the cost of a less “pure” language duel.
- Version pins for Eigen and nalgebra belong in toolchain/result metadata.
- If a result hinges on a library quirk, the loop report says so.
