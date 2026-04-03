# Plugin Override Full Benchmark

## Child Hang Root Cause

The benchmark child process used to stall in the `force_swap` phase because the testing-only `ForceDatabaseSwap` request swapped the database and emitted `DatabaseSwapped`, but did not schedule diagnostics refresh or emit the mutation signal that drives `AnalysisStarted`/`AnalysisFinished`. The benchmark then waited forever for a post-swap `AnalysisFinished` event that could never arrive.

The fix is in `/Users/jsmolka/Work/cairo-language/cairols/src/server/routing/handlers.rs`: after a forced swap, CairoLS now reconciles open-file overlays, refreshes diagnostics, and emits the mutation signal, so the force-swap phase behaves like a real swap path and terminates normally.

## Results

| Manifest | afterFirstDiagnostics RSS | afterFirstDiagnostics Salsa | afterEditLoop RSS | afterEditLoop Salsa | afterMixedLoop RSS | afterMixedLoop Salsa | afterForcedSwap RSS | afterForcedSwap Salsa |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| /Users/jsmolka/Work/cairo-projects/ecosystem/alexandria/Scarb.toml | 792.0 MB | 264.0 MB | 2779.0 MB | 1015.2 MB | 3273.7 MB | 1018.7 MB | 3469.8 MB | 861.3 MB |
| /Users/jsmolka/Work/cairo-projects/ecosystem/alexandria/packages/ascii/Scarb.toml | 324.8 MB | 88.3 MB | 476.8 MB | 113.6 MB | 551.6 MB | 114.9 MB | 602.6 MB | 97.4 MB |
| /Users/jsmolka/Work/cairo-projects/ecosystem/alexandria/packages/btc/Scarb.toml | 790.7 MB | 264.0 MB | 1204.7 MB | 375.1 MB | 1330.9 MB | 379.1 MB | 1408.1 MB | 269.5 MB |
| /Users/jsmolka/Work/cairo-projects/ecosystem/open-zeppelin/Scarb.toml | 2027.3 MB | 785.9 MB | 5003.8 MB | 1560.4 MB | 5186.1 MB | 1569.9 MB | 5628.8 MB | 1318.6 MB |
| /Users/jsmolka/Work/cairo-projects/ecosystem/open-zeppelin/packages/access/Scarb.toml | 1221.9 MB | 450.6 MB | 1951.0 MB | 674.9 MB | 2254.1 MB | 679.3 MB | 2340.7 MB | 459.0 MB |
| /Users/jsmolka/Work/cairo-projects/ecosystem/open-zeppelin/packages/account/Scarb.toml | 1107.0 MB | 389.1 MB | 1477.4 MB | 489.8 MB | 1805.6 MB | 500.6 MB | 1946.8 MB | 398.2 MB |

## Scenario Times

| Manifest | first_diagnostics | edit_loop | mixed_loop | force_swap |
| --- | ---: | ---: | ---: | ---: |
| /Users/jsmolka/Work/cairo-projects/ecosystem/alexandria/Scarb.toml | 13295 ms | 153442 ms | 603 ms | 31579 ms |
| /Users/jsmolka/Work/cairo-projects/ecosystem/alexandria/packages/ascii/Scarb.toml | 8638 ms | 77195 ms | 157 ms | 8770 ms |
| /Users/jsmolka/Work/cairo-projects/ecosystem/alexandria/packages/btc/Scarb.toml | 12566 ms | 113572 ms | 769 ms | 13985 ms |
| /Users/jsmolka/Work/cairo-projects/ecosystem/open-zeppelin/Scarb.toml | 36568 ms | 305097 ms | 2149 ms | 50198 ms |
| /Users/jsmolka/Work/cairo-projects/ecosystem/open-zeppelin/packages/access/Scarb.toml | 27039 ms | 151206 ms | 1028 ms | 27064 ms |
| /Users/jsmolka/Work/cairo-projects/ecosystem/open-zeppelin/packages/account/Scarb.toml | 25435 ms | 114374 ms | 1793 ms | 25631 ms |
