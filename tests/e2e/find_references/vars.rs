use crate::find_references::find_references;
use crate::support::insta::test_transform;

#[test]
fn var_via_binding() {
    test_transform!(find_references, r#"
    fn main() {
        let foo<caret>bar = 1233; // good
        let x = foobar + 1; // good
    }
    fn bar() {
        let foobar = 42; // bad
    }
    "#, @"none response")
}

#[test]
fn var_via_use() {
    test_transform!(find_references, r#"
    fn main() {
        let foobar = 1233; // good
        let x = foo<caret>bar + 1; // good
    }
    fn bar() {
        let foobar = 42; // bad
    }
    "#, @r"
    fn main() {
        let <sel=declaration>foobar</sel> = 1233; // good
        let x = <sel>foobar</sel> + 1; // good
    }
    fn bar() {
        let foobar = 42; // bad
    }
    ")
}

#[test]
fn complex_binding() {
    test_transform!(find_references, r#"
    fn main() {
        let (foobar, foobar2) = (1, 2); // good
        let x = foo<caret>bar + foobar2; // good
        let y = foobar2 * foobar2; // bad
        let z = foobar2 + foobar; // good
    }
    "#, @r"
    fn main() {
        let (<sel=declaration>foobar</sel>, foobar2) = (1, 2); // good
        let x = <sel>foobar</sel> + foobar2; // good
        let y = foobar2 * foobar2; // bad
        let z = foobar2 + <sel>foobar</sel>; // good
    }
    ")
}

#[test]
fn var_captured_by_closure_outside() {
    test_transform!(find_references, r#"
    fn main() {
        let foobar = 1;
        let x = foo<caret>bar + 1;
        let f = |y| x + y + foobar;
    }
    "#, @r"
    fn main() {
        let <sel=declaration>foobar</sel> = 1;
        let x = <sel>foobar</sel> + 1;
        let f = |y| x + y + <sel>foobar</sel>;
    }
    ")
}

#[test]
fn var_captured_by_closure_inside() {
    test_transform!(find_references, r#"
    fn main() {
        let foobar = 1;
        let x = foobar + 1;
        let f = |y| x + y + foo<caret>bar;
    }
    "#, @r"
    fn main() {
        let <sel=declaration>foobar</sel> = 1;
        let x = <sel>foobar</sel> + 1;
        let f = |y| x + y + <sel>foobar</sel>;
    }
    ")
}

#[test]
fn shadowing1() {
    test_transform!(find_references, r#"
    fn main() {
        let foobar = 1;
        let x = foo<caret>bar + 1;
        let foobar = 2;
        let y = foobar + 1;
    }
    "#, @r"
    fn main() {
        let <sel=declaration>foobar</sel> = 1;
        let x = <sel>foobar</sel> + 1;
        let foobar = 2;
        let y = foobar + 1;
    }
    ")
}

#[test]
fn shadowing2() {
    test_transform!(find_references, r#"
    fn main() {
        let foobar = 1;
        let x = foobar + 1;
        let foobar = 2;
        let y = foo<caret>bar + 1;
    }
    "#, @r"
    fn main() {
        let foobar = 1;
        let x = foobar + 1;
        let <sel=declaration>foobar</sel> = 2;
        let y = <sel>foobar</sel> + 1;
    }
    ")
}

#[test]
fn param_via_binding() {
    test_transform!(find_references, r#"
    fn pow(nu<caret>m: felt252) -> felt252 {
        num * num
    }
    "#, @r"
    <sel=declaration>fn <sel>pow</sel>(num: felt252) -> felt252 {
        num * num
    }</sel>
    ")
}

#[test]
fn param_via_use() {
    test_transform!(find_references, r#"
    fn pow(num: felt252) -> felt252 {
        nu<caret>m * num
    }
    "#, @r"
    fn pow(<sel=declaration>num: felt252</sel>) -> felt252 {
        <sel>num</sel> * <sel>num</sel>
    }
    ")
}

#[test]
fn param_captured_by_closure() {
    test_transform!(find_references, r#"
    fn pow(num: felt252) -> felt252 {
        let f = |x| nu<caret>m * x;
        num * f(num)
    }
    "#, @r"
    fn pow(<sel=declaration>num: felt252</sel>) -> felt252 {
        let f = |x| <sel>num</sel> * x;
        <sel>num</sel> * f(<sel>num</sel>)
    }
    ")
}
