# Granular Plugin Override Probe

- Repetitions per measurement: `25`
- `aggregate_*`: old whole-map plugin override rewrite path
- `granular_set_*`: direct per-crate suite replacement in the new granular DB model
- `granular_sync_*`: raw full-suite sync helper in the new granular DB model

## Results

| Existing crates | Aggregate update | Granular set update | Granular sync update | Set speedup | Sync/aggregate slowdown |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 82.3 us | 126.5 us | 169.0 us | 0.65x | 2.05x |
| 10 | 95.6 us | 124.5 us | 683.9 us | 0.77x | 7.15x |
| 100 | 140.5 us | 132.5 us | 5931.3 us | 1.06x | 42.22x |
| 1000 | 667.7 us | 135.7 us | 58964.3 us | 4.92x | 88.31x |
| 5000 | 2996.2 us | 168.2 us | 300325.2 us | 17.81x | 100.24x |

## Interpretation

- Direct per-crate plugin suite replacement is the main win of this migration.
- At `5000` crates, direct granular update is about `17.8x` faster than the old aggregate update path.
- The raw full-sync helper is still much slower than aggregate replacement because it walks the whole workspace and fingerprints every suite.
- CairoLS now avoids that raw full-sync path during ordinary project reloads: `ProjectModel` keeps per-crate plugin-suite fingerprints and only rebuilds/apply suites for changed crates.
- That means this probe is best read as: the underlying granular storage is good for incremental updates, but direct use of `sync_granular_crate_plugin_suites` is still not a good production reload strategy for large workspaces.
- A reduced Alexandria end-to-end smoke run was started and then intentionally stopped after it did not complete in a reasonable window, so this directory does not claim a trustworthy session-level benchmark yet.
