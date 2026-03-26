//! LSP formatting benchmarks — poorly-formatted simple code and macro-heavy code.
//!
//! Groups:
//!   - `formatting_simple`        — compact, poorly-spaced function definitions
//!   - `formatting_derive_macros` — struct definitions with derives and a trait impl
//!
//! Run with:
//!   cargo bench --bench formatting
//! or for a single group:
//!   cargo bench --bench formatting formatting_simple

use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

#[path = "client.rs"]
mod client;
use client::BenchClient;

#[path = "fixtures.rs"]
mod fixtures;
use fixtures::DERIVE_CAIRO;

// ── Simple fixture ────────────────────────────────────────────────────────────

/// Poorly formatted Cairo for formatting benchmarks.
const FORMAT_CAIRO: &str = r#"
fn  add( a:u32,b:u32 )->u32{
    let   result=a+b;
    result
}
fn  mul( a:u32,b:u32 )->u32{
    let   result=a*b;
    result
}
fn  sub(a :u32 ,b :u32)->u32{
    if a>b{a-b}else{0_u32}
}
fn  clamp(val:u32,lo:u32,hi:u32)->u32{
    if val<lo{lo}else if val>hi{hi}else{val}
}
fn  accumulate(a:u32,b:u32,c:u32)->u32{
    let ab=add(a,b);
    let abc=add(ab,c);
    mul(abc,2_u32)
}
fn  pipeline(x:u32,y:u32,z:u32)->u32{
    let s=add(x,y);
    let p=mul(s,z);
    let r=sub(p,x);
    clamp(r,0_u32,1000_u32)
}
fn  normalize(value:u32,total:u32)->u32{
    if total==0_u32{0_u32}else{mul(value,100_u32)/total}
}
"#;

// ── Benchmarks: simple ────────────────────────────────────────────────────────

fn bench_simple(c: &mut Criterion) {
    let mut client = BenchClient::new(FORMAT_CAIRO);
    let mut group = c.benchmark_group("formatting_simple");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("file", |b| b.iter(|| client.formatting()));

    group.finish();
}

// ── Benchmarks: derive macros ─────────────────────────────────────────────────

fn bench_derive_macros(c: &mut Criterion) {
    let mut client = BenchClient::new(DERIVE_CAIRO);
    let mut group = c.benchmark_group("formatting_derive_macros");
    group.measurement_time(Duration::from_secs(10));

    // Format a file with multiple derive attributes and a `#[generate_trait]` impl.
    group.bench_function("file", |b| b.iter(|| client.formatting()));

    group.finish();
}

criterion_group!(benches, bench_simple, bench_derive_macros);
criterion_main!(benches);
