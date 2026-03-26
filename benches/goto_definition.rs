//! LSP go-to-definition benchmarks — simple code and macro-heavy code.
//!
//! Groups:
//!   - `goto_definition_simple`        — local function and stdlib function
//!   - `goto_definition_derive_macros` — struct used as a field type / method param
//!   - `goto_definition_inline_macros` — variable from `array!`, function returning `Array`
//!
//! Run with:
//!   cargo bench --bench goto_definition
//! or for a single group:
//!   cargo bench --bench goto_definition goto_definition_simple

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
    let mut group = c.benchmark_group("goto_definition_simple");
    group.measurement_time(Duration::from_secs(10));

    // Go to definition of a local function — resolves within the same file.
    group.bench_function("local_function", |b| {
        b.iter(|| client.goto_definition(8, 12)) // line: "    let x = helper(a);"
    });

    // Go to definition of a stdlib function — resolves to a virtual file.
    group.bench_function("stdlib_function", |b| {
        b.iter(|| client.goto_definition(4, 4)) // line: "    u32_sqrt(x)"
    });

    group.finish();
}

// ── Benchmarks: derive macros ─────────────────────────────────────────────────

fn bench_derive_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(DERIVE_CAIRO);
    let mut group = c.benchmark_group("goto_definition_derive_macros");
    group.measurement_time(Duration::from_secs(10));

    // Go to definition of `Point` from a struct field — crosses the derive boundary.
    group.bench_function("struct_used_as_field_type", |b| {
        b.iter(|| client.goto_definition(9, 14)) // line: "    top_left: Point,"
    });

    // Go to definition of `Point` from inside a method parameter list.
    group.bench_function("struct_used_in_method_param", |b| {
        b.iter(|| client.goto_definition(21, 41)) // line: "    fn contains(…, point: @Point) -> bool {"
    });

    group.finish();
}

// ── Benchmarks: inline macros ─────────────────────────────────────────────────

fn bench_inline_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(INLINE_MACRO_CAIRO);
    let mut group = c.benchmark_group("goto_definition_inline_macros");
    group.measurement_time(Duration::from_secs(10));

    // Go to definition of `arr` (defined via `array!`) from a later use in the loop.
    group.bench_function("array_var_from_use", |b| {
        b.iter(|| client.goto_definition(5, 14)) // line: "        let len = arr.len();"
    });

    // Go to definition of `fibonacci` from within the same file — function defined with Array.
    group.bench_function("function_returning_array", |b| {
        b.iter(|| client.goto_definition(1, 3)) // line: "fn fibonacci(n: u32) -> Array<u32> {"
    });

    group.finish();
}

criterion_group!(benches, bench_simple, bench_derive_macros, bench_inline_macros);
criterion_main!(benches);
