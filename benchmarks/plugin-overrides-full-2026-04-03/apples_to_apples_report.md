# Apples-To-Apples Comparison

Compared against the pre-plugin diagnostics-optimization benchmark using the same reduced workload: root manifest only, `hot_files=3`, `edit_files=2`, `edit_iterations=4`, `mixed_rounds=2`, `idle_ms=100`.

| Project | Metric | Old | Current | Delta |
| --- | --- | ---: | ---: | ---: |
| alexandria | first diagnostics (ms) | 14120.0 | 12969.0 | -1151.0 |
| alexandria | edit loop (ms) | 31992.0 | 31588.0 | -404.0 |
| alexandria | mixed loop (ms) | 299.0 | 280.0 | -19.0 |
| alexandria | RSS after first diagnostics (MB) | 789.4 | 790.1 | +0.7 |
| alexandria | RSS after edit loop (MB) | 1214.7 | 1172.8 | -42.0 |
| alexandria | RSS after mixed loop (MB) | 1395.3 | 1325.5 | -69.8 |
| alexandria | Salsa after first diagnostics (MB) | 264.0 | 264.0 | +0.0 |
| alexandria | Salsa after edit loop (MB) | 375.1 | 375.1 | +0.0 |
| alexandria | Salsa after mixed loop (MB) | 377.0 | 377.0 | +0.0 |
| openzeppelin | first diagnostics (ms) | 39109.0 | 36946.0 | -2163.0 |
| openzeppelin | edit loop (ms) | 82529.0 | 76120.0 | -6409.0 |
| openzeppelin | mixed loop (ms) | 899.0 | 927.0 | +28.0 |
| openzeppelin | RSS after first diagnostics (MB) | 2026.1 | 2026.8 | +0.7 |
| openzeppelin | RSS after edit loop (MB) | 4179.8 | 3929.5 | -250.3 |
| openzeppelin | RSS after mixed loop (MB) | 4301.6 | 4018.7 | -282.9 |
| openzeppelin | Salsa after first diagnostics (MB) | 785.9 | 785.9 | -0.0 |
| openzeppelin | Salsa after edit loop (MB) | 1258.3 | 1258.3 | -0.0 |
| openzeppelin | Salsa after mixed loop (MB) | 1264.4 | 1264.3 | -0.1 |
