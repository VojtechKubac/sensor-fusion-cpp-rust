# Replay file schema (baseline)

UTF-8 CSV, comma-separated, LF line endings, header row required.
Numbers use `.` as the decimal mark. No quoting unless a future field needs it.

## `ground_truth.csv`

| Column | Type | Unit | Meaning |
| --- | --- | --- | --- |
| `t` | f64 | s | Sample time |
| `x` | f64 | m | Position X |
| `y` | f64 | m | Position Y |
| `vx` | f64 | m/s | Velocity X |
| `vy` | f64 | m/s | Velocity Y |

Rows are strictly increasing in `t`. This is the filter accuracy reference
(position RMSE, consistency checks).

## `sensor.csv`

| Column | Type | Unit | Meaning |
| --- | --- | --- | --- |
| `t` | f64 | s | Measurement timestamp (truth time + optional delay later) |
| `zx` | f64 | m | Measured position X |
| `zy` | f64 | m | Measured position Y |

One row per measurement from the single position sensor. For loop 1,
`t` matches a ground-truth sample time (no delay). Noise is i.i.d. Gaussian
per axis; parameters are in `meta.json`.

## `meta.json`

```json
{
  "scenario": "linear_kf_baseline",
  "schema_version": 1,
  "seed": 1,
  "duration_s": 20.0,
  "dt_s": 0.01,
  "sensor_period_s": 0.01,
  "noise_std_m": 0.5,
  "initial_state": { "x": 0.0, "y": 0.0, "vx": 1.0, "vy": 0.5 },
  "state_layout": ["x", "y", "vx", "vy"],
  "measurement_layout": ["zx", "zy"]
}
```

`state_layout` / `measurement_layout` document the order both ports must use in
memory so comparisons stay fair.
