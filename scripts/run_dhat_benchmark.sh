#!/usr/bin/env bash
set -euo pipefail

if ! command -v valgrind >/dev/null 2>&1; then
  echo "valgrind is required to run DHAT profiling" >&2
  exit 1
fi

if [ "$#" -lt 1 ]; then
  echo "usage: $0 <cargo-run-args...>" >&2
  echo "example: $0 -- --project-root /path/to/project --package-manifests 0" >&2
  exit 1
fi

exec valgrind \
  --tool=dhat \
  --num-callers=30 \
  --time-stamp=yes \
  cargo run --features testing --bin cairols-bench "$@"
