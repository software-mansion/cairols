//! LSP document-highlight benchmarks — simple code and macro-heavy code.
//!
//! Groups:
//!   - `document_highlight_simple`        — local function with two call sites
//!   - `document_highlight_derive_macros` — struct referenced across field declarations and a method
//!
//! Run with:
//!   cargo bench --bench document_highlight
//! or for a single group:
//!   cargo bench --bench document_highlight document_highlight_simple

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
    let mut group = c.benchmark_group("document_highlight_simple");
    group.measurement_time(Duration::from_secs(10));

    // Highlight all occurrences of a function with two call sites in the file.
    group.bench_function("local_function", |b| {
        b.iter(|| client.document_highlight(3, 3)) // line: "fn helper(x: u32) -> u32 {"
    });

    group.finish();
}

// ── Benchmarks: derive macros ─────────────────────────────────────────────────

fn bench_derive_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(DERIVE_CAIRO);
    let mut group = c.benchmark_group("document_highlight_derive_macros");
    group.measurement_time(Duration::from_secs(10));

    // Highlight all occurrences of `Point` — definition, two field uses, and a method param.
    group.bench_function("struct_with_derives", |b| {
        b.iter(|| client.document_highlight(2, 7)) // line: "struct Point {"
    });

    group.finish();
}

criterion_group!(benches, bench_simple, bench_derive_macros);
criterion_main!(benches);
