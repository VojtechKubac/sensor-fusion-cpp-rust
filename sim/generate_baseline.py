#!/usr/bin/env python3
"""Generate shared baseline replay files: 2D constant-velocity + one position sensor.

Stdlib only. Both C++ and Rust ports must replay these files unchanged.
"""

from __future__ import annotations

import argparse
import csv
import json
import math
import random
from pathlib import Path


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument(
        "--out",
        type=Path,
        default=Path("sim/out/baseline"),
        help="Output directory for meta.json, ground_truth.csv, sensor.csv",
    )
    p.add_argument("--seed", type=int, default=1)
    p.add_argument("--duration-s", type=float, default=20.0)
    p.add_argument("--dt-s", type=float, default=0.01, help="Ground-truth step")
    p.add_argument(
        "--sensor-period-s",
        type=float,
        default=0.01,
        help="Position sensor period (loop 1: same as dt)",
    )
    p.add_argument("--noise-std-m", type=float, default=0.5)
    p.add_argument("--x0", type=float, default=0.0)
    p.add_argument("--y0", type=float, default=0.0)
    p.add_argument("--vx0", type=float, default=1.0)
    p.add_argument("--vy0", type=float, default=0.5)
    return p.parse_args()


def main() -> None:
    args = parse_args()
    if args.dt_s <= 0 or args.sensor_period_s <= 0 or args.duration_s <= 0:
        raise SystemExit("duration, dt, and sensor period must be positive")
    if args.noise_std_m < 0:
        raise SystemExit("noise-std-m must be >= 0")

    out = args.out
    out.mkdir(parents=True, exist_ok=True)

    rng = random.Random(args.seed)

    truth_path = out / "ground_truth.csv"
    sensor_path = out / "sensor.csv"
    meta_path = out / "meta.json"

    n_steps = int(math.floor(args.duration_s / args.dt_s)) + 1
    next_sensor_t = 0.0

    with truth_path.open("w", newline="", encoding="utf-8") as tf, sensor_path.open(
        "w", newline="", encoding="utf-8"
    ) as sf:
        truth_w = csv.writer(tf, lineterminator="\n")
        sensor_w = csv.writer(sf, lineterminator="\n")
        truth_w.writerow(["t", "x", "y", "vx", "vy"])
        sensor_w.writerow(["t", "zx", "zy"])

        for i in range(n_steps):
            t = i * args.dt_s
            x = args.x0 + args.vx0 * t
            y = args.y0 + args.vy0 * t
            vx = args.vx0
            vy = args.vy0
            truth_w.writerow(
                [
                    f"{t:.9f}",
                    f"{x:.9f}",
                    f"{y:.9f}",
                    f"{vx:.9f}",
                    f"{vy:.9f}",
                ]
            )

            # Emit a measurement when the sensor clock catches up (tolerant compare).
            if t + 1e-12 >= next_sensor_t:
                zx = x + rng.gauss(0.0, args.noise_std_m)
                zy = y + rng.gauss(0.0, args.noise_std_m)
                sensor_w.writerow([f"{t:.9f}", f"{zx:.9f}", f"{zy:.9f}"])
                next_sensor_t += args.sensor_period_s

    meta = {
        "scenario": "linear_kf_baseline",
        "schema_version": 1,
        "seed": args.seed,
        "duration_s": args.duration_s,
        "dt_s": args.dt_s,
        "sensor_period_s": args.sensor_period_s,
        "noise_std_m": args.noise_std_m,
        "initial_state": {
            "x": args.x0,
            "y": args.y0,
            "vx": args.vx0,
            "vy": args.vy0,
        },
        "state_layout": ["x", "y", "vx", "vy"],
        "measurement_layout": ["zx", "zy"],
        "files": {
            "ground_truth": "ground_truth.csv",
            "sensor": "sensor.csv",
        },
    }
    meta_path.write_text(json.dumps(meta, indent=2) + "\n", encoding="utf-8")

    print(f"Wrote {truth_path}")
    print(f"Wrote {sensor_path}")
    print(f"Wrote {meta_path}")


if __name__ == "__main__":
    main()
