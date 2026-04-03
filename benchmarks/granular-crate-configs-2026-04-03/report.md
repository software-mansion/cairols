# Granular Crate Config Probe

- Repetitions per measurement: `25`
- `aggregate_*`: old whole-map `crate_configs` replacement path
- `granular_set_*`: direct per-crate setter path
- `granular_sync_*`: production-style `sync_granular_crate_configs` path

## Results

| Existing crates | Aggregate update | Granular set update | Granular sync update | Sync/aggregate slowdown |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 18.7 us | 45.9 us | 49.6 us | 2.66x |
| 10 | 21.2 us | 44.6 us | 60.1 us | 2.83x |
| 100 | 48.8 us | 49.4 us | 178.3 us | 3.65x |
| 1000 | 318.8 us | 66.0 us | 1452.2 us | 4.56x |
| 5000 | 1552.4 us | 160.8 us | 7130.2 us | 4.59x |

## Interpretation

- The direct per-crate setter stays much flatter than the aggregate map and overtakes it once the workspace is moderately large.
- The production sync path is still O(n) because it diffs the desired crate set across the full workspace, but it is no longer dominated by Salsa handle reads for unchanged crates.
- At 5000 crates, direct granular update is about 9.7x faster than aggregate update.
- At 5000 crates, full granular sync is about 4.6x slower than aggregate whole-map replacement, which means crate-config migration helps structural invalidation and memory shape more than raw reload throughput at this stage.
