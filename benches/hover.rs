//! LSP hover benchmarks — simple code and macro-heavy code.
//!
//! Groups:
//!   - `hover_simple`        — stdlib function and local variable
//!   - `hover_derive_macros` — derive attribute names, struct field types, trait method locals
//!   - `hover_inline_macros` — `array!`, `println!`, method calls, locals inside macro loops
//!
//! Run with:
//!   cargo bench --bench hover
//! or for a single group:
//!   cargo bench --bench hover hover_simple

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
///   4: "    let sum = x + y;"                ← `sum` at col 8
///   5: "    u32_sqrt(sum)"                   ← `u32_sqrt` at col 4
const HOVER_CAIRO: &str = r#"
use core::integer::u32_sqrt;

fn compute(x: u32, y: u32) -> u32 {
    let sum = x + y;
    u32_sqrt(sum)
}

fn magnitude(ax: u32, ay: u32, bx: u32, by: u32) -> u32 {
    let dx = if ax > bx { ax - bx } else { bx - ax };
    let dy = if ay > by { ay - by } else { by - ay };
    u32_sqrt(dx * dx + dy * dy)
}

fn normalize_vec(x: u32, y: u32) -> (u32, u32) {
    let len = magnitude(0_u32, 0_u32, x, y);
    if len == 0_u32 {
        (0_u32, 0_u32)
    } else {
        (x / len, y / len)
    }
}

fn dot_product(ax: u32, ay: u32, bx: u32, by: u32) -> u32 {
    ax * bx + ay * by
}

fn cross_product(ax: u32, ay: u32, bx: u32, by: u32) -> u32 {
    let forward = ax * by;
    let backward = ay * bx;
    if forward > backward { forward - backward } else { backward - forward }
}

fn pipeline(x: u32, y: u32, z: u32) -> u32 {
    let ab = compute(x, y);
    let bc = compute(y, z);
    let combined = compute(ab, bc);
    u32_sqrt(combined)
}
"#;

// ── Benchmarks: simple ────────────────────────────────────────────────────────

fn bench_simple(c: &mut Criterion) {
    let mut client = BenchClient::new(HOVER_CAIRO);
    let mut group = c.benchmark_group("hover_simple");
    group.measurement_time(Duration::from_secs(10));

    // Hover over the `u32_sqrt` function call — resolves to a stdlib function.
    group.bench_function("stdlib_function", |b| {
        b.iter(|| client.hover(5, 4)) // line: "    u32_sqrt(sum)"
    });

    // Hover over a local variable.
    group.bench_function("local_variable", |b| {
        b.iter(|| client.hover(4, 8)) // line: "    let sum = x + y;"
    });

    group.finish();
}

// ── Benchmarks: derive macros ─────────────────────────────────────────────────

fn bench_derive_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(DERIVE_CAIRO);
    let mut group = c.benchmark_group("hover_derive_macros");
    group.measurement_time(Duration::from_secs(10));

    // Hover over `Drop` inside `#[derive(Drop, Serde, …)]` — resolves a built-in derive.
    group.bench_function("derive_name", |b| {
        b.iter(|| client.hover(1, 9)) // line: "#[derive(Drop, Serde, Clone, PartialEq, Debug)]"
    });

    // Hover over `Point` used as a field type in another struct with derives.
    group.bench_function("struct_field_type", |b| {
        b.iter(|| client.hover(9, 14)) // line: "    top_left: Point,"
    });

    // Hover over `generate_trait` attribute on the impl block.
    group.bench_function("generate_trait_attribute", |b| {
        b.iter(|| client.hover(13, 2)) // line: "#[generate_trait]"
    });

    // Hover over a local variable inside a `#[generate_trait]` method.
    group.bench_function("local_var_in_trait_method", |b| {
        b.iter(|| client.hover(16, 12)) // line: "        let width = …"
    });

    group.finish();
}

// ── Benchmarks: inline macros ─────────────────────────────────────────────────

fn bench_inline_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(INLINE_MACRO_CAIRO);
    let mut group = c.benchmark_group("hover_inline_macros");
    group.measurement_time(Duration::from_secs(10));

    // Hover over the `array!` macro invocation.
    group.bench_function("array_macro", |b| {
        b.iter(|| client.hover(2, 20)) // line: "    let mut arr = array![0_u32, 1_u32];"
    });

    // Hover over the `println!` macro invocation.
    group.bench_function("println_macro", |b| {
        b.iter(|| client.hover(19, 8)) // line: "        println!(\"  value: {}\", val);"
    });

    // Hover over `arr.len()` — a method call on `Array<u32>` inside a macro-heavy function.
    group.bench_function("array_method_call", |b| {
        b.iter(|| client.hover(5, 18)) // line: "        let len = arr.len();"
    });

    // Hover over the local variable `val` introduced inside the `println!` loop.
    group.bench_function("local_var_near_macro", |b| {
        b.iter(|| client.hover(18, 12)) // line: "        let val = *arr.at(i);"
    });

    group.finish();
}

criterion_group!(benches, bench_simple, bench_derive_macros, bench_inline_macros);
criterion_main!(benches);
