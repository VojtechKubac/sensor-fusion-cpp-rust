# Results and loop reports

Raw run data, plots, and short comparison writeups — one subdirectory per
feature loop once that loop is finished.

```text
report/
  loop-01-linear-kf/     # after loop 1
    runs/                # raw metrics
    plots/
    REPORT.md            # method, numbers, limits
```

Generated artefacts (`runs/`, `plots/`) stay gitignored; committed text is the
writeup and any tiny fixtures needed to regenerate figures.
