//! LSP completion benchmarks — simple code and macro-heavy code.
//!
//! Groups:
//!   - `completions_simple`        — basic function body, typed and untyped triggers
//!   - `completions_derive_macros` — inside a `#[generate_trait]` impl method
//!   - `completions_inline_macros` — inside a function that uses `array!`
//!
//! Run with:
//!   cargo bench --bench completions
//! or for a single group:
//!   cargo bench --bench completions completions_simple

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

#[path = "client.rs"]
mod client;
use client::BenchClient;

#[path = "fixtures.rs"]
mod fixtures;
use fixtures::{DERIVE_CAIRO, INLINE_MACRO_CAIRO};

// ── Simple fixture ────────────────────────────────────────────────────────────

/// Benchmark positions (0-indexed lines):
///   1: "fn foo(value: u32) -> u32 {"        ← untyped trigger at col 0
///   4: "    result"                          ← `result` prefix at col 10
const COMPLETION_CAIRO: &str = r#"
fn foo(value: u32) -> u32 {
    let doubled = value * 2;
    let result = doubled + value;
    result
}

fn scale(value: u32, factor: u32) -> u32 {
    let product = value * factor;
    let clamped = if product > 1000_u32 { 1000_u32 } else { product };
    clamped
}

fn accumulate(a: u32, b: u32, c: u32) -> u32 {
    let ab = foo(a) + foo(b);
    let abc = ab + scale(c, 2_u32);
    abc
}

fn apply_twice(value: u32) -> u32 {
    let first = foo(value);
    let second = foo(first);
    accumulate(first, second, value)
}

fn normalize(value: u32, total: u32) -> u32 {
    if total == 0_u32 {
        0_u32
    } else {
        scale(value, 100_u32) / total
    }
}
"#;

// ── Benchmarks: simple ────────────────────────────────────────────────────────

fn bench_simple(c: &mut Criterion) {
    let mut client = BenchClient::new(COMPLETION_CAIRO);
    let mut group = c.benchmark_group("completions_simple");
    group.measurement_time(Duration::from_secs(10));

    // Typed completions — `res` prefix on the `result` return expression.
    group.bench_function("typed_prefix", |b| {
        b.iter(|| client.completion(4, 10)) // line: "    result"
    });

    // Untyped completions — cursor at the beginning of the `fn` line (empty trigger).
    group.bench_function("untyped", |b| {
        b.iter(|| client.completion(1, 0)) // line: "fn foo(value: u32) -> u32 {"
    });

    group.finish();
}

// ── Benchmarks: derive macros ─────────────────────────────────────────────────

fn bench_derive_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(DERIVE_CAIRO);
    let mut group = c.benchmark_group("completions_derive_macros");
    group.measurement_time(Duration::from_secs(10));

    // Untyped completions on the blank line between `area` and `contains` methods.
    group.bench_function("untyped_in_trait_method", |b| {
        b.iter(|| client.completion(20, 0)) // line: ""  (blank between methods)
    });

    // Typed completions — `wid` prefix inside a method that references derived fields.
    group.bench_function("typed_prefix_in_trait_method", |b| {
        b.iter(|| client.completion(16, 17)) // line: "        let width = …", cursor on `width`
    });

    group.finish();
}

// ── Benchmarks: inline macros ─────────────────────────────────────────────────

fn bench_inline_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(INLINE_MACRO_CAIRO);
    let mut group = c.benchmark_group("completions_inline_macros");
    group.measurement_time(Duration::from_secs(10));

    // Untyped completions on the blank line inside the `while` body of `fibonacci`.
    group.bench_function("untyped_in_function", |b| {
        b.iter(|| client.completion(11, 4)) // line: ""  (blank line inside fibonacci body)
    });

    // Typed completions — `arr` prefix inside the loop that uses `array!`.
    group.bench_function("typed_prefix_in_loop", |b| {
        b.iter(|| client.completion(5, 16)) // line: "        let len = arr.len();"
    });

    group.finish();
}

criterion_group!(benches, bench_simple, bench_derive_macros, bench_inline_macros);
criterion_main!(benches);
