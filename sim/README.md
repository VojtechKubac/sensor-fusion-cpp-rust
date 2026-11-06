# Shared simulator inputs

Both the C++ and Rust pipelines consume the **same** files. The generator here
writes them once; the ports only replay.

## Baseline scenario (feature loop 1)

Constant-velocity motion in 2D, one position sensor, Gaussian noise:

```bash
python3 sim/generate_baseline.py --out sim/out/baseline
```

Produces:

| File | Role |
| --- | --- |
| `meta.json` | Seed, rates, noise, units — record with results |
| `ground_truth.csv` | Exact trajectory at the simulation step rate |
| `sensor.csv` | Noisy position measurements (one sensor stream) |

Schema: [SCHEMA.md](SCHEMA.md).

## Rules

- Deterministic given `--seed` (default `1`). Do not rely on C++ or Rust RNGs
  for shared fault/noise sequences later — generate once here, replay everywhere.
- Times are seconds, `f64`, starting at `0`. Positions are metres; velocities
  are metres per second.
- Later stages add GPS-like and camera-like streams, then fault replay files,
  without breaking these column names for the baseline sensor.
