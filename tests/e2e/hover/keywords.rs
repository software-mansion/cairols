use lsp_types::Hover;

use crate::support::insta::test_transform_plain;

#[test]
fn kw_as() {
    test_transform_plain!(Hover, r#"
    pub mod fun_mod {
       pub fn add_one(x: u32) -> u32 { x + 1 }
    }

    fn using_aliased_fun() {
        use fun_mod::add_one <caret>as one_adder;
        assert_eq!(one_adder(1), 2);
    }
    "#, @r#"
    source_context = """
        use fun_mod::add_one <caret>as one_adder;
    """
    highlight = """
        use fun_mod::add_one <sel>as</sel> one_adder;
    """
    popover = """
    ```cairo
    as
    ```
    ---

    ### The `as` keyword.

    Allows to alias an item when importing it with `use` statement.
    ### Example
    ```cairo
     pub mod single_const {
        pub const A: u8 = 1;
     }

    fn using_aliased_const() {
        use single_const::A as B;
        assert_eq!(B, 1);
    }
    ```"""
    "#);
}

#[test]
fn kw_const() {
    test_transform_plain!(Hover, r#"
    <caret>const X: felt252 = 1;
    fn main() {}
    "#, @r#"
    source_context = """
    <caret>const X: felt252 = 1;
    """
    highlight = """
    <sel>const</sel> X: felt252 = 1;
    """
    popover = """
    ```cairo
    const
    ```
    ---

    ### The `const` keyword.

    Introduces a constant item or const generic parameter. Constants are evaluated at compile time.
    ### Examples
    - Module-level constant:
    ```cairo
    const TEN: u32 = 10;
    fn times_ten(x: u32) -> u32 { x * TEN }
    ```
    - Const-generic parameter used by functions:
    ```cairo
    fn pad_with<const P: felt252>(x: felt252) -> felt252 { x + P }
    ```"""
    "#);
}

#[test]
fn kw_else() {
    test_transform_plain!(Hover, r#"
    fn main() {
        if true {} <caret>else {}
    }
    "#, @r#"
    source_context = """
        if true {} <caret>else {}
    """
    highlight = """
        if true {} <sel>else</sel> {}
    """
    popover = """
    ```cairo
    else
    ```
    ---

    ### The `else` keyword.

    Specifies an alternative branch for `if` or a fallback with `match` guards.
    ### Example
    ```cairo
    fn is_positive(x: i32) -> bool {
        if x >= 0 { true } else { false }
    }
    ```"""
    "#);
}

#[test]
fn kw_enum() {
    test_transform_plain!(Hover, r#"
    <caret>enum E { A: felt252 }
    fn main() {}
    "#, @r#"
    source_context = """
    <caret>enum E { A: felt252 }
    """
    highlight = """
    <sel>enum</sel> E { A: felt252 }
    """
    popover = """
    ```cairo
    enum
    ```
    ---

    ### The `enum` keyword.

    Declares an enumeration type with named variants.
    ### Example
    ```cairo
    #[derive(Copy, Drop)]
    enum ResultU32 {
        Ok: u32,
        Err: felt252,
    }
    ```"""
    "#);
}

#[test]
fn kw_extern() {
    test_transform_plain!(Hover, r#"
    <caret>extern fn ext_fn() -> felt252;
    fn main() {}
    "#, @r#"
    source_context = """
    <caret>extern fn ext_fn() -> felt252;
    """
    highlight = """
    <sel>extern</sel> fn ext_fn() -> felt252;
    """
    popover = """
    ```cairo
    extern
    ```
    ---

    ### The `extern` keyword.

    Declares external libfuncs or types provided by the compiler.
    ### Example
    ```cairo
    extern fn array_new<T>() -> Array<T> nopanic;
    ```"""
    "#);
}

#[test]
fn kw_false() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let _x = <caret>false;
    }
    "#, @r#"
    source_context = """
        let _x = <caret>false;
    """
    highlight = """
        let _x = <sel>false</sel>;
    """
    popover = """
    ```cairo
    false
    ```
    ---

    ### The `false` keyword.

    Boolean enum value representing logical false.
    ### Example
    ```cairo
    fn default_bool() -> bool { false }
    ```"""
    "#);
}

#[test]
fn kw_fn() {
    test_transform_plain!(Hover, r#"
    <caret>fn main() {}
    "#, @r#"
    source_context = """
    <caret>fn main() {}
    """
    highlight = """
    <sel>fn</sel> main() {}
    """
    popover = """
    ```cairo
    fn
    ```
    ---

    ### The `fn` keyword.

    Declares a function. Functions may specify implicits, panic behavior, generics, and attrs.
    ### Example
    ```cairo
    fn add_u32(a: u32, b: u32) -> u32 { a + b }
    ```"""
    "#);
}

#[test]
fn kw_if() {
    test_transform_plain!(Hover, r#"
    fn main() {
        <caret>if true {} else {}
    }
    "#, @r#"
    source_context = """
        <caret>if true {} else {}
    """
    highlight = """
        <sel>if</sel> true {} else {}
    """
    popover = """
    ```cairo
    if
    ```
    ---

    ### The `if` keyword.

    Begins a conditional branch.
    ### Example
    ```cairo
    fn sign(x: felt252) -> felt252 {
        if x == 0 {
           0
        } else {
            if x > 0 { 1 } else { -1 }
        }
    }
    ```"""
    "#);
}

#[test]
fn kw_while() {
    test_transform_plain!(Hover, r#"
    fn main() {
        <caret>while true { break; }
    }
    "#, @r#"
    source_context = """
        <caret>while true { break; }
    """
    highlight = """
        <sel>while</sel> true { break; }
    """
    popover = """
    ```cairo
    while
    ```
    ---

    ### The `while` keyword.

    Starts a conditional loop that runs while the condition is true.
    ### Example
    ```cairo
    fn sum_first(n: u32) -> u32 {
        let mut i = 0;
        let mut acc = 0;
        while i < n { acc = acc + i; i = i + 1; }
        acc
    }
    ```"""
    "#);
}

#[test]
fn kw_for() {
    test_transform_plain!(Hover, r#"
    fn main() {
        <caret>for _i in 0..1 { }
    }
    "#, @r#"
    source_context = """
        <caret>for _i in 0..1 { }
    """
    highlight = """
        <sel>for</sel> _i in 0..1 { }
    """
    popover = """
    ```cairo
    for
    ```
    ---

    ### The `for` keyword.

    Iteration construct over ranges or iterables.
    ### Example
    ```cairo
    fn sum_range(n: u32) -> u32 {
        let mut acc = 0;
        for i in 0..n { acc = acc + i; }
        acc
    }
    ```"""
    "#);
}

#[test]
fn kw_loop() {
    test_transform_plain!(Hover, r#"
    fn main() {
        <caret>loop { break; }
    }
    "#, @r#"
    source_context = """
        <caret>loop { break; }
    """
    highlight = """
        <sel>loop</sel> { break; }
    """
    popover = """
    ```cairo
    loop
    ```
    ---

    ### The `loop` keyword.

    Starts an infinite loop, typically exited with `break` or `return`.
    ### Example
    ```cairo
    fn first_positive(xs: Array<felt252>) -> felt252 {
        let mut i = 0;
        loop {
            if i >= xs.len() { return 0; }
            let v = *xs.at(i);
            if v != 0 { return v; }
            i = i + 1;
        }
    }
    ```"""
    "#);
}

#[test]
fn kw_impl() {
    test_transform_plain!(Hover, r#"
    <caret>impl Foo {}
    "#, @r#"
    source_context = """
    <caret>impl Foo {}
    """
    highlight = """
    <sel>impl</sel> Foo {}
    """
    popover = """
    ```cairo
    impl
    ```
    ---

    ### The `impl` keyword.

    Introduces an implementation block for a trait or type.
    ### Example
    ```cairo
    trait Doubler { fn double(self: @u32) -> u32; }
    impl Doubling of Doubler {
        fn double(self: @u32) -> u32 { *self + *self }
    }
    ```"""
    "#);
}

#[test]
fn kw_implicits() {
    test_transform_plain!(Hover, r#"
    fn main() <caret>implicits() {}
    "#, @r#"
    source_context = """
    fn main() <caret>implicits() {}
    """
    highlight = """
    fn main() <sel>implicits</sel>() {}
    """
    popover = """
    ```cairo
    implicits
    ```
    ---

    ### The `implicits` keyword.

    Declares implicit parameters required by a function. These are usually passed automatically.
    ### Example
    ```cairo
    extern fn check_in_u32_range(value: u32) -> (bool,) implicits(RangeCheck) nopanic;
    ```"""
    "#);
}

#[test]
fn kw_let() {
    test_transform_plain!(Hover, r#"
    fn main() { <caret>let x = 1; }
    "#, @r#"
    source_context = """
    fn main() { <caret>let x = 1; }
    """
    highlight = """
    fn main() { <sel>let</sel> x = 1; }
    """
    popover = """
    ```cairo
    let
    ```
    ---

    ### The `let` keyword.

    Binds a new variable.
    ### Example
    ```cairo
    fn square(x: u32) -> u32 { let y = x * x; y }
    ```"""
    "#);
}

#[test]
fn kw_macro() {
    test_transform_plain!(Hover, r#"
    <caret>macro id {
       ($x:ident) => { $x };
    }
    "#, @r#"
    source_context = """
    <caret>macro id {
    """
    highlight = """
    <sel>macro</sel> id {
    """
    popover = """
    ```cairo
    macro
    ```
    ---

    ### The `macro` keyword.

    Creates a declarative macro.
    ### Example
    ```cairo
    macro add_one {
       ($x:ident) => { $x + 1 };
    }
    ```"""
    "#);
}

#[test]
fn kw_match() {
    test_transform_plain!(Hover, r#"
    fn main() {
        <caret>match 1 { _ => {} }
    }
    "#, @r#"
    source_context = """
        <caret>match 1 { _ => {} }
    """
    highlight = """
        <sel>match</sel> 1 { _ => {} }
    """
    popover = """
    ```cairo
    match
    ```
    ---

    ### The `match` keyword.

    Pattern matching construct that selects a branch based on a value.
    ### Example
    ```cairo
    fn to_bool(x: u32) -> bool {
        match x { 0 => false, _ => true }
    }
    ```"""
    "#);
}

#[test]
fn kw_mod() {
    test_transform_plain!(Hover, r#"
    <caret>mod m {}
    fn main() {}
    "#, @r#"
    source_context = """
    <caret>mod m {}
    """
    highlight = """
    <sel>mod</sel> m {}
    """
    popover = """
    ```cairo
    mod
    ```
    ---

    ### The `mod` keyword.

    Declares a module. Modules group items and control visibility.
    ### Example
    ```cairo
    mod math_utils { pub fn add(a: u32, b: u32) -> u32 { a + b } }
    ```"""
    "#);
}

#[test]
fn kw_mut() {
    test_transform_plain!(Hover, r#"
    fn main() { let <caret>mut x = 1; }
    "#, @r#"
    source_context = """
    fn main() { let <caret>mut x = 1; }
    """
    highlight = """
    fn main() { let <sel>mut</sel> x = 1; }
    """
    popover = """
    ```cairo
    mut
    ```
    ---

    ### The `mut` keyword.

    Marks a binding or reference as mutable.
    ### Example
    ```cairo
    fn count(n: u32) -> u32 {
        let mut i = 0; while i < n { i = i + 1; } i
    }
    ```"""
    "#);
}

#[test]
fn kw_nopanic() {
    test_transform_plain!(Hover, r#"
    fn main() <caret>nopanic {}
    "#, @r#"
    source_context = """
    fn main() <caret>nopanic {}
    """
    highlight = """
    fn main() <sel>nopanic</sel> {}
    """
    popover = """
    ```cairo
    nopanic
    ```
    ---

    ### The `nopanic` keyword.

    Marks a function as guaranteed not to panic. The compiler enforces no panicking paths.
    ### Example
    ```cairo
    extern fn bool_to_felt252(a: bool) -> felt252 nopanic;
    fn into_felt(b: bool) -> felt252 nopanic { bool_to_felt252(b) }
    ```"""
    "#);
}

#[test]
fn kw_of() {
    test_transform_plain!(Hover, r#"
    trait MyTrait {
        fn foo() -> felt252;
    }

    impl MyImpl <caret>of MyTrait {
        fn foo() -> felt252 {
            7
        }
    }
    "#, @r#"
    source_context = """
    impl MyImpl <caret>of MyTrait {
    """
    highlight = """
    impl MyImpl <sel>of</sel> MyTrait {
    """
    popover = """
    ```cairo
    of
    ```
    ---

    ### The `of` keyword.

    Used in `impl Type of Trait` headers.
    ### Example
    ```cairo
    trait Foo<T> { fn foo(self: @T) -> T; }
    impl FooU32 of Foo<u32> { fn foo(self: @u32) -> u32 { *self + 1 } }
    ```"""
    "#);
}

#[test]
fn kw_ref() {
    test_transform_plain!(Hover, r#"
    fn push_four(<caret>ref ary: Array<felt252>) {
        ary.append(4)
    }
    "#, @r#"
    source_context = """
    fn push_four(<caret>ref ary: Array<felt252>) {
    """
    highlight = """
    fn push_four(<sel>ref</sel> ary: Array<felt252>) {
    """
    popover = """
    ```cairo
    ref
    ```
    ---

    ### The `ref` keyword.

    Allows functions to mutate variables by passing them as a reference.
    The value is implicitly copied and passed back to the user.
    ### Example
    ```cairo
    fn push_four(ref ary: Array<felt252>) {
        ary.append(4)
    }

    fn main() {
        let mut ary = array![1, 2, 3];
        push_four(ref ary);
        assert!(ary == array![1, 2, 3, 4])
    }
    ```"""
    "#);
}

#[test]
fn kw_continue() {
    test_transform_plain!(Hover, r#"
    fn main() { loop { <caret>continue; } }
    "#, @r#"
    source_context = """
    fn main() { loop { <caret>continue; } }
    """
    highlight = """
    fn main() { loop { <sel>continue</sel>; } }
    """
    popover = """
    ```cairo
    continue
    ```
    ---

    ### The `continue` keyword.

    Skips to the next iteration of a loop.
    ### Example
    ```cairo
    fn skip_even(n: u32) -> u32 {
        let mut i = 0; let mut cnt = 0;
        while i < n {
            i = i + 1;
            if i % 2 == 0 { continue; }
            cnt = cnt + 1;
        }
        cnt
    }
    ```"""
    "#);
}

#[test]
fn kw_return() {
    test_transform_plain!(Hover, r#"
    fn main() { <caret>return; }
    "#, @r#"
    source_context = """
    fn main() { <caret>return; }
    """
    highlight = """
    fn main() { <sel>return</sel>; }
    """
    popover = """
    ```cairo
    return
    ```
    ---

    ### The `return` keyword.

    Exits a function, optionally returning a value.
    ### Example
    ```cairo
    fn clamp01(x: i32) -> i32    {
        if x < 0 { return 0; }
        if x > 1 { return 1; }
        x
    }
    ```"""
    "#);
}

#[test]
fn kw_break() {
    test_transform_plain!(Hover, r#"
    fn main() { loop { <caret>break; } }
    "#, @r#"
    source_context = """
    fn main() { loop { <caret>break; } }
    """
    highlight = """
    fn main() { loop { <sel>break</sel>; } }
    """
    popover = """
    ```cairo
    break
    ```
    ---

    ### The `break` keyword.

    Exits a loop early. Can only be used inside `loop`, `while` or `for` loop blocks.
    ### Example
    ```cairo
    fn first_gt(xs: Array<u32>, cmp: u32) -> Option<u32> {
        let mut i = 0;
        let v = loop {
            if i >= xs.len() {
                return None;
            }

            let v = *xs.at(i);
            if v > cmp { break Some(v); }
            i = i + 1;
        };
        v
    }
    ```"""
    "#);
}

#[test]
fn kw_struct() {
    test_transform_plain!(Hover, r#"
    <caret>struct S { a: felt252 }
    fn main() {}
    "#, @r#"
    source_context = """
    <caret>struct S { a: felt252 }
    """
    highlight = """
    <sel>struct</sel> S { a: felt252 }
    """
    popover = """
    ```cairo
    struct
    ```
    ---

    ### The `struct` keyword.

    Declares a structure type with named members.
    ### Example
    ```cairo
    struct Point { x: felt252, y: felt252 }
    fn origin() -> Point { Point { x: 0, y: 0 } }
    ```"""
    "#);
}

#[test]
fn kw_trait() {
    test_transform_plain!(Hover, r#"
    <caret>trait T {}
    fn main() {}
    "#, @r#"
    source_context = """
    <caret>trait T {}
    """
    highlight = """
    <sel>trait</sel> T {}
    """
    popover = """
    ```cairo
    trait
    ```
    ---

    ### The `trait` keyword.

    Declares a trait containing associated items to be implemented.
    ### Example
    ```cairo
    trait Volume<T> { fn volume(self: @T) -> usize; }
    impl ArrayVolume<T> of Volume<Array<T>> { fn volume(self: @Array<T>) -> usize { self.len() } }
    ```"""
    "#);
}

#[test]
fn kw_true() {
    test_transform_plain!(Hover, r#"
    fn main() { let _x = <caret>true; }
    "#, @r#"
    source_context = """
    fn main() { let _x = <caret>true; }
    """
    highlight = """
    fn main() { let _x = <sel>true</sel>; }
    """
    popover = """
    ```cairo
    true
    ```
    ---

    ### The `true` keyword.

    Boolean enum value representing logical true.
    ### Example
    ```cairo
    fn yes() -> bool { true }
    ```"""
    "#);
}

#[test]
fn kw_type() {
    test_transform_plain!(Hover, r#"
    <caret>type T = felt252;
    fn main() {}
    "#, @r#"
    source_context = """
    <caret>type T = felt252;
    """
    highlight = """
    <sel>type</sel> T = felt252;
    """
    popover = """
    ```cairo
    type
    ```
    ---

    ### The `type` keyword.

    Declares a type alias.
    ### Example
    ```cairo
    type usize = u32;
    ```"""
    "#);
}

#[test]
fn kw_use() {
    test_transform_plain!(Hover, r#"
    <caret>use core::array;
    fn main() {}
    "#, @r#"
    source_context = """
    <caret>use core::array;
    """
    highlight = """
    <sel>use</sel> core::array;
    """
    popover = """
    ```cairo
    use
    ```
    ---

    ### The `use` keyword.

    Imports items into the current scope.
    ### Examples
    ```cairo
    use core::panic_with_felt252;
    use super::traits::PartialEq;
    ```"""
    "#);
}

#[test]
fn kw_pub() {
    test_transform_plain!(Hover, r#"
    <caret>pub fn main() {}
    "#, @r#"
    source_context = """
    <caret>pub fn main() {}
    """
    highlight = """
    <sel>pub</sel> fn main() {}
    """
    popover = """
    ```cairo
    pub
    ```
    ---

    ### The `pub` keyword.

    Makes an item public within its parent module (or crate).
    ### Example
    ```cairo
    pub fn get_zero() -> u32 { 0 }
    pub(crate) struct Pair { pub a: u32, b: u32 }
    ```"""
    "#);
}
