# Friction log

Running notes on where each language slowed development or prevented bugs.
Entries are short and dated. Per-feature-loop reports may cite these.

| Date | Loop / area | Language | What happened | Takeaway |
| --- | --- | --- | --- | --- |
| | | | | |

## Conventions

- **Loop / area** — e.g. `1 / SPSC queue`, `sim / schema`.
- **Language** — `C++`, `Rust`, `both`, or `tooling`.
- Log C++ design choices that leaked into the Rust port (and the reverse).
- Log every justified `unsafe` (Rust) or raw owning pointer (C++) here as well as in the loop report.
