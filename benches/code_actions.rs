//! LSP code-actions benchmarks — simple code and macro-heavy code.
//!
//! Groups:
//!   - `code_actions_simple`        — unused variable in a plain function
//!   - `code_actions_derive_macros` — code actions inside a `#[generate_trait]` method body
//!
//! Run with:
//!   cargo bench --bench code_actions
//! or for a single group:
//!   cargo bench --bench code_actions code_actions_simple

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

#[path = "client.rs"]
mod client;
use client::BenchClient;

#[path = "fixtures.rs"]
mod fixtures;
use fixtures::DERIVE_CAIRO;

// ── Simple fixture ────────────────────────────────────────────────────────────

/// Benchmark positions (0-indexed lines):
///   2: "    let unused = 42_u32;"    ← `unused` at col 8
const CODE_ACTION_CAIRO: &str = r#"
fn foo(value: u32) -> u32 {
    let unused = 42_u32;
    value
}

fn bar(a: u32, b: u32) -> u32 {
    let ignored = a * 10_u32;
    a + b
}

fn scale(value: u32, factor: u32) -> u32 {
    let doubled = value * 2_u32;
    let result = doubled * factor;
    result
}

fn pipeline(x: u32, y: u32) -> u32 {
    let a = foo(x);
    let b = bar(x, y);
    scale(a, b)
}
"#;

// ── Benchmarks: simple ────────────────────────────────────────────────────────

fn bench_simple(c: &mut Criterion) {
    let mut client = BenchClient::new(CODE_ACTION_CAIRO);
    let mut group = c.benchmark_group("code_actions_simple");
    group.measurement_time(Duration::from_secs(10));

    // Code actions on an unused variable — may suggest a rename-to-underscore fix.
    group.bench_function("unused_variable", |b| {
        b.iter(|| client.code_actions(2, 8)) // line: "    let unused = 42_u32;"
    });

    group.finish();
}

// ── Benchmarks: derive macros ─────────────────────────────────────────────────

fn bench_derive_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(DERIVE_CAIRO);
    let mut group = c.benchmark_group("code_actions_derive_macros");
    group.measurement_time(Duration::from_secs(10));

    // Code actions on a local variable inside a `#[generate_trait]` method.
    group.bench_function("local_var_in_trait_method", |b| {
        b.iter(|| client.code_actions(16, 12)) // line: "        let width = …"
    });

    group.finish();
}

criterion_group!(benches, bench_simple, bench_derive_macros);
criterion_main!(benches);
