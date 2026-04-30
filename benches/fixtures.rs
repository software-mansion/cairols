//! Shared Cairo fixtures used across multiple benchmark binaries.

#![allow(dead_code)]

// ── Simple fixtures ───────────────────────────────────────────────────────────

/// Navigation-focused Cairo — a helper function called from two sites, plus
/// several more functions that chain calls to build a richer call graph.
///
/// Benchmark positions (0-indexed lines):
///   3: "fn helper(x: u32) -> u32 {"    ← `helper` defined at col 3
///   4: "    u32_sqrt(x)"                ← `u32_sqrt` at col 4
///   8: "    let x = helper(a);"         ← `helper` call at col 12
pub const NAV_CAIRO: &str = r#"
use core::integer::u32_sqrt;

fn helper(x: u32) -> u32 {
    u32_sqrt(x)
}

fn compute(a: u32, b: u32) -> u32 {
    let x = helper(a);
    let y = helper(b);
    x + y
}

fn clamp(val: u32, low: u32, high: u32) -> u32 {
    if val < low {
        low
    } else if val > high {
        high
    } else {
        val
    }
}

fn normalize(x: u32, scale: u32) -> u32 {
    let root = helper(x);
    compute(root, scale)
}

fn distance_sq(ax: u32, ay: u32, bx: u32, by: u32) -> u32 {
    let dx = if ax > bx { ax - bx } else { bx - ax };
    let dy = if ay > by { ay - by } else { by - ay };
    compute(dx, dy)
}

fn weighted_sum(a: u32, b: u32, c: u32) -> u32 {
    let ab = compute(a, b);
    let abc = compute(ab, c);
    helper(abc)
}

fn reduce_pair(x: u32, y: u32) -> u32 {
    let sum = compute(x, y);
    let root = helper(sum);
    clamp(root, 0_u32, sum)
}

fn pipeline_three(a: u32, b: u32, c: u32) -> u32 {
    let step1 = normalize(a, 2_u32);
    let step2 = normalize(b, 3_u32);
    let step3 = normalize(c, 5_u32);
    weighted_sum(step1, step2, step3)
}
"#;

// ── Macro fixtures ────────────────────────────────────────────────────────────

/// Struct-heavy Cairo with multiple derive macros, a `#[generate_trait]` impl,
/// and two additional geometry types (Circle, Triangle) with their own impls.
///
/// Benchmark positions (0-indexed lines):
///    1: "#[derive(Drop, Serde, Clone, PartialEq, Debug)]"  ← `Drop` at col 9
///    2: "struct Point {"                                    ← `Point` at col 7
///    9: "    top_left: Point,"                             ← `Point` at col 14
///   13: "#[generate_trait]"                                ← `generate_trait` at col 2
///   16: "        let width = …"                            ← `width` at col 12
///   20: ""                                                 ← blank line between methods
///   21: "    fn contains(self: @Rectangle, point: @Point) -> bool {"  ← `Point` at col 41
pub const DERIVE_CAIRO: &str = r#"
#[derive(Drop, Serde, Clone, PartialEq, Debug)]
struct Point {
    x: felt252,
    y: felt252,
}

#[derive(Drop, Serde)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

#[generate_trait]
impl RectangleImpl of RectangleTrait {
    fn area(self: @Rectangle) -> felt252 {
        let width = *self.bottom_right.x - *self.top_left.x;
        let height = *self.bottom_right.y - *self.top_left.y;
        width * height
    }

    fn contains(self: @Rectangle, point: @Point) -> bool {
        *point.x >= *self.top_left.x
            && *point.x <= *self.bottom_right.x
            && *point.y >= *self.top_left.y
            && *point.y <= *self.bottom_right.y
    }
}

#[derive(Drop, Clone, PartialEq)]
struct Circle {
    cx: u32,
    cy: u32,
    radius: u32,
}

#[generate_trait]
impl CircleImpl of CircleTrait {
    fn diameter(self: @Circle) -> u32 {
        *self.radius * 2
    }

    fn area_approx(self: @Circle) -> u32 {
        3 * *self.radius * *self.radius
    }

    fn contains(self: @Circle, px: u32, py: u32) -> bool {
        let dx = if px > *self.cx { px - *self.cx } else { *self.cx - px };
        let dy = if py > *self.cy { py - *self.cy } else { *self.cy - py };
        dx * dx + dy * dy <= *self.radius * *self.radius
    }

    fn scale(self: @Circle, factor: u32) -> Circle {
        Circle { cx: *self.cx, cy: *self.cy, radius: *self.radius * factor }
    }
}

#[derive(Drop, Clone)]
struct Triangle {
    ax: u32,
    ay: u32,
    bx: u32,
    by: u32,
    cx: u32,
    cy: u32,
}

#[generate_trait]
impl TriangleImpl of TriangleTrait {
    fn centroid_x(self: @Triangle) -> u32 {
        (*self.ax + *self.bx + *self.cx) / 3
    }

    fn centroid_y(self: @Triangle) -> u32 {
        (*self.ay + *self.by + *self.cy) / 3
    }

    fn side_sq_ab(self: @Triangle) -> u32 {
        let dx = if *self.bx > *self.ax { *self.bx - *self.ax } else { *self.ax - *self.bx };
        let dy = if *self.by > *self.ay { *self.by - *self.ay } else { *self.ay - *self.by };
        dx * dx + dy * dy
    }

    fn side_sq_bc(self: @Triangle) -> u32 {
        let dx = if *self.cx > *self.bx { *self.cx - *self.bx } else { *self.bx - *self.cx };
        let dy = if *self.cy > *self.by { *self.cy - *self.by } else { *self.by - *self.cy };
        dx * dx + dy * dy
    }

    fn perimeter_sq(self: @Triangle) -> u32 {
        self.side_sq_ab() + self.side_sq_bc()
    }
}
"#;

/// Cairo code that uses inline macros (`array!`, `println!`), with several
/// additional utility functions to give the LSP a richer symbol table to search.
///
/// Benchmark positions (0-indexed lines):
///    1: "fn fibonacci(n: u32) -> Array<u32> {"
///    2: "    let mut arr = array![0_u32, 1_u32];"  ← `array!` at col 20
///    5: "        let len = arr.len();"              ← `arr` at col 18, `len` at col 12
///   11: ""                                          ← blank line for untyped completions
///   19: "        println!(\"  value: {}\", val);"   ← `println!` at col 8
///   18: "        let val = *arr.at(i);"             ← `val` at col 12
pub const INLINE_MACRO_CAIRO: &str = r#"
fn fibonacci(n: u32) -> Array<u32> {
    let mut arr = array![0_u32, 1_u32];
    let mut i: u32 = 2;
    while i < n {
        let len = arr.len();
        let a = *arr.at(len - 2);
        let b = *arr.at(len - 1);
        arr.append(a + b);
        i += 1;
    };

    arr
}

fn log_values(arr: @Array<u32>) {
    let mut i: usize = 0;
    while i < arr.len() {
        let val = *arr.at(i);
        println!("  value: {}", val);
        i += 1;
    };
}

fn sum_array(arr: @Array<u32>) -> u32 {
    let mut total: u32 = 0;
    let mut i: usize = 0;
    while i < arr.len() {
        total += *arr.at(i);
        i += 1;
    };
    total
}

fn max_array(arr: @Array<u32>) -> u32 {
    let mut max_val = *arr.at(0);
    let mut i: usize = 1;
    while i < arr.len() {
        let val = *arr.at(i);
        if val > max_val {
            max_val = val;
        }
        i += 1;
    };
    max_val
}

fn min_array(arr: @Array<u32>) -> u32 {
    let mut min_val = *arr.at(0);
    let mut i: usize = 1;
    while i < arr.len() {
        let val = *arr.at(i);
        if val < min_val {
            min_val = val;
        }
        i += 1;
    };
    min_val
}

fn range(start: u32, end: u32) -> Array<u32> {
    let mut arr: Array<u32> = array![];
    let mut i = start;
    while i < end {
        arr.append(i);
        i += 1;
    };
    arr
}

fn zip_sum(a: @Array<u32>, b: @Array<u32>) -> Array<u32> {
    let mut result: Array<u32> = array![];
    let mut i: usize = 0;
    while i < a.len() {
        result.append(*a.at(i) + *b.at(i));
        i += 1;
    };
    result
}

fn print_stats(arr: @Array<u32>) {
    println!("count: {}", arr.len());
    println!("sum:   {}", sum_array(arr));
    println!("max:   {}", max_array(arr));
    println!("min:   {}", min_array(arr));
}
"#;
