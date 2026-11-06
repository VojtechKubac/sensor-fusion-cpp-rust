# Benchmark harness

Scripts that rerun experiments and regenerate plots/tables from raw data under
[`report/`](../report/).

Not started yet. Target shape:

- One entrypoint command that rebuilds both ports (optional), runs the scenario,
  and writes raw JSON/CSV under `report/<loop>/<run_id>/`.
- Metrics: end-to-end latency percentiles, jitter, deadline misses, throughput,
  CPU/memory, stripped binary size (see project vision / loop reports).
- Machine, kernel, compiler, and scheduling config recorded next to every run.
