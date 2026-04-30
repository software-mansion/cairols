//! LSP find-references benchmarks — simple code and macro-heavy code.
//!
//! Groups:
//!   - `references_simple`        — local function called from two sites
//!   - `references_derive_macros` — struct referenced in field declarations and a method param
//!
//! Run with:
//!   cargo bench --bench references
//! or for a single group:
//!   cargo bench --bench references references_simple

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

#[path = "client.rs"]
mod client;
use client::BenchClient;

#[path = "fixtures.rs"]
mod fixtures;
use fixtures::{DERIVE_CAIRO, NAV_CAIRO};

// ── Benchmarks: simple ────────────────────────────────────────────────────────

fn bench_simple(c: &mut Criterion) {
    let mut client = BenchClient::new(NAV_CAIRO);
    let mut group = c.benchmark_group("references_simple");
    group.measurement_time(Duration::from_secs(10));

    // Find all references of a local function called from two call sites.
    group.bench_function("local_function", |b| {
        b.iter(|| client.references(3, 3)) // line: "fn helper(x: u32) -> u32 {"
    });

    group.finish();
}

// ── Benchmarks: derive macros ─────────────────────────────────────────────────

fn bench_derive_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(DERIVE_CAIRO);
    let mut group = c.benchmark_group("references_derive_macros");
    group.measurement_time(Duration::from_secs(10));

    // Find all references of `Point` — appears in two field declarations and one method param.
    group.bench_function("struct_with_derives", |b| {
        b.iter(|| client.references(2, 7)) // line: "struct Point {"
    });

    group.finish();
}

criterion_group!(benches, bench_simple, bench_derive_macros);
criterion_main!(benches);
