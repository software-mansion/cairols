# Diagnostics Optimization Comparison

Configuration: direct child benchmark, root manifest only, hot_files=3, edit_files=2, edit_iterations=4, mixed_rounds=2, idle_ms=100.

## Summary Table

| Project | Metric | Before | After | Delta |
|---|---:|---:|---:|---:|
| alexandria | first diagnostics (ms) | 45012 | 14120 | -30892 |
| alexandria | edit loop (ms) | 28878 | 31992 | +3114 |
| alexandria | mixed loop (ms) | 270 | 299 | +29 |
| alexandria | force swap (ms) | 54060 | 16442 | -37618 |
| alexandria | RSS after first diagnostics (MB) | 3812.0 | 789.4 | -3022.6 |
| alexandria | RSS after edit loop (MB) | 3561.7 | 1214.7 | -2347.0 |
| alexandria | RSS after mixed loop (MB) | 3731.8 | 1395.3 | -2336.5 |
| alexandria | RSS after forced swap (MB) | 4963.4 | 1504.4 | -3459.0 |
| alexandria | Salsa after first diagnostics (MB) | 1401.2 | 264.0 | -1137.2 |
| alexandria | Salsa after edit loop (MB) | 1441.5 | 375.1 | -1066.4 |
| alexandria | Salsa after mixed loop (MB) | 1443.2 | 377.0 | -1066.2 |
| alexandria | Salsa after forced swap (MB) | 1401.2 | 334.9 | -1066.3 |
| | | | | |
| openzeppelin | first diagnostics (ms) | 70240 | 39109 | -31131 |
| openzeppelin | edit loop (ms) | 111833 | 82529 | -29304 |
| openzeppelin | mixed loop (ms) | 824 | 899 | +75 |
| openzeppelin | force swap (ms) | 77100 | 48337 | -28763 |
| openzeppelin | RSS after first diagnostics (MB) | 5508.9 | 2026.1 | -3482.8 |
| openzeppelin | RSS after edit loop (MB) | 6892.9 | 4179.8 | -2713.1 |
| openzeppelin | RSS after mixed loop (MB) | 6783.7 | 4301.6 | -2482.1 |
| openzeppelin | RSS after forced swap (MB) | 6249.3 | 4133.2 | -2116.1 |
| openzeppelin | Salsa after first diagnostics (MB) | 2467.7 | 785.9 | -1681.8 |
| openzeppelin | Salsa after edit loop (MB) | 2532.4 | 1258.3 | -1274.1 |
| openzeppelin | Salsa after mixed loop (MB) | 2535.6 | 1264.4 | -1271.2 |
| openzeppelin | Salsa after forced swap (MB) | 2467.7 | 1193.6 | -1274.1 |
| | | | | |