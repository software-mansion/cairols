# CairoLS 05.2026

This document aims to serve as a place where high level information about upcoming, planned work for CairoLS lives.
It is basically a list of main features and tasks that are planned to be added in the next ~6 months.

The list is sorted from top to bottom according to the priority the given task has (first thing having the top priority,
ie being more likely to be done first). Size of the task is ranked `1-5`, where `1` is smallest and `5` biggest.

A more detailed breakdown can be found on the [Github board](https://github.com/orgs/software-mansion/projects/33/views/6),
typically in the `Todo` and `Backlog LS` columns.

## Q2 2026 + Q3 2026

### 1. Memory consumption optimizations

LS uses too much memory, causing slowdowns and crashes in larger workspaces.
The work involves researching ideas that can reduce memory usage and validating them through profiling and testing.

Size: 5

### 2. Scarb.toml support improvements

Expand LS understanding of `Scarb.toml` files: reject unknown/extra keys with diagnostics,
add quick-fixes for common manifest issues, and improve hover quality for manifest fields.

Size: 4

### 3. Gas cost & tooling code lenses

Add code lenses for displaying gas cost per function (including L2 gas and Starknet syscalls from top-level snforge functions),
and add commands for running `cairo-profiler` and `cairo-coverage` directly from the editor.

Size: 4

### 4. Proc-macro & declarative macro improvements

Fix bugs in proc-macro-controlled code where LSP features (goto, references, hover) do not work correctly,
and add code lens support for declarative macros.

Size: 4

### 5. Fix LS not being killed after closing VS Code window

Sometimes after closing all VS Code windows a hanging LS process remains with ever-growing memory usage.
The work involves researching the root cause and fixing the process lifecycle management.

Size: 2

### 6. Completions improvements

Make completions smarter and more context-aware: suggest identifiers based on their kind,
filter out already-imported items, propose only correct items for impl aliases,
and ensure consistent filtering with `text_matches` across all completion providers.

Size: 3

### 7. Hover improvements

Expand hover coverage: keyword documentation expansion, hovers for closure parameters,
and showing generic type parameter constraints in hover info.

Size: 3

### 8. Inlay hints improvements

Improve inlay hint robustness and coverage: handle parameter hints on arity mismatches,
keep semantic highlighting and inlay hints in sync with server-side state changes.

Size: 3
