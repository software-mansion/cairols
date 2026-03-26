//! LSP inlay-hints benchmarks — simple code and macro-heavy code.
//!
//! Groups:
//!   - `inlay_hints_simple`        — navigation fixture with imports and two functions
//!   - `inlay_hints_derive_macros` — struct definitions with derives and a trait impl
//!   - `inlay_hints_inline_macros` — functions using `array!` and `println!`
//!
//! Run with:
//!   cargo bench --bench inlay_hints
//! or for a single group:
//!   cargo bench --bench inlay_hints inlay_hints_simple

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};
use lsp_types::{Position, Range};

#[path = "client.rs"]
mod client;
use client::BenchClient;

#[path = "fixtures.rs"]
mod fixtures;
use fixtures::{DERIVE_CAIRO, INLINE_MACRO_CAIRO, NAV_CAIRO};

const WHOLE_FILE: Range =
    Range { start: Position { line: 0, character: 0 }, end: Position { line: 999, character: 0 } };

// ── Benchmarks: simple ────────────────────────────────────────────────────────

fn bench_simple(c: &mut Criterion) {
    let mut client = BenchClient::new(NAV_CAIRO);
    let mut group = c.benchmark_group("inlay_hints_simple");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("full_file", |b| b.iter(|| client.inlay_hints(WHOLE_FILE)));

    group.finish();
}

// ── Benchmarks: derive macros ─────────────────────────────────────────────────

fn bench_derive_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(DERIVE_CAIRO);
    let mut group = c.benchmark_group("inlay_hints_derive_macros");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("full_file", |b| b.iter(|| client.inlay_hints(WHOLE_FILE)));

    group.finish();
}

// ── Benchmarks: inline macros ─────────────────────────────────────────────────

fn bench_inline_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(INLINE_MACRO_CAIRO);
    let mut group = c.benchmark_group("inlay_hints_inline_macros");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("full_file", |b| b.iter(|| client.inlay_hints(WHOLE_FILE)));

    group.finish();
}

criterion_group!(benches, bench_simple, bench_derive_macros, bench_inline_macros);
criterion_main!(benches);
