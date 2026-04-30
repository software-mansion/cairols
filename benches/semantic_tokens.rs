//! LSP semantic-tokens benchmarks — simple code and macro-heavy code.
//!
//! Groups:
//!   - `semantic_tokens_simple`        — navigation fixture with imports and two functions
//!   - `semantic_tokens_derive_macros` — struct definitions with derives and a trait impl
//!   - `semantic_tokens_inline_macros` — functions using `array!` and `println!`
//!
//! Run with:
//!   cargo bench --bench semantic_tokens
//! or for a single group:
//!   cargo bench --bench semantic_tokens semantic_tokens_simple

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

#[path = "client.rs"]
mod client;
use client::BenchClient;

#[path = "fixtures.rs"]
mod fixtures;
use fixtures::{DERIVE_CAIRO, INLINE_MACRO_CAIRO, NAV_CAIRO};

// ── Benchmarks: simple ────────────────────────────────────────────────────────

fn bench_simple(c: &mut Criterion) {
    let mut client = BenchClient::new(NAV_CAIRO);
    let mut group = c.benchmark_group("semantic_tokens_simple");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("full_file", |b| b.iter(|| client.semantic_tokens()));

    group.finish();
}

// ── Benchmarks: derive macros ─────────────────────────────────────────────────

fn bench_derive_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(DERIVE_CAIRO);
    let mut group = c.benchmark_group("semantic_tokens_derive_macros");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("full_file", |b| b.iter(|| client.semantic_tokens()));

    group.finish();
}

// ── Benchmarks: inline macros ─────────────────────────────────────────────────

fn bench_inline_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(INLINE_MACRO_CAIRO);
    let mut group = c.benchmark_group("semantic_tokens_inline_macros");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("full_file", |b| b.iter(|| client.semantic_tokens()));

    group.finish();
}

criterion_group!(benches, bench_simple, bench_derive_macros, bench_inline_macros);
criterion_main!(benches);
