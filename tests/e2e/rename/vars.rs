use crate::rename::rename;
use crate::support::insta::test_transform;

#[test]
fn var_via_binding() {
    test_transform!(rename, r#"
    fn main() {
        let foo<caret>bar = 1233; // good
        let x = foobar + 1; // good
    }
    fn bar() {
        let foobar = 42; // bad
    }
    "#, @r"
    fn main() {
        let RENAMED = 1233; // good
        let x = RENAMED + 1; // good
    }
    fn bar() {
        let foobar = 42; // bad
    }
    ")
}

#[test]
fn var_via_use() {
    test_transform!(rename, r#"
    fn main() {
        let foobar = 1233; // good
        let x = foo<caret>bar + 1; // good
    }
    fn bar() {
        let foobar = 42; // bad
    }
    "#, @r"
    fn main() {
        let RENAMED = 1233; // good
        let x = RENAMED + 1; // good
    }
    fn bar() {
        let foobar = 42; // bad
    }
    ")
}

#[test]
fn complex_binding() {
    test_transform!(rename, r#"
    fn main() {
        let (foobar, foobar2) = (1, 2); // good
        let x = foo<caret>bar + foobar2; // good
        let y = foobar2 * foobar2; // bad
        let z = foobar2 + foobar; // good
    }
    "#, @r"
    fn main() {
        let (RENAMED, foobar2) = (1, 2); // good
        let x = RENAMED + foobar2; // good
        let y = foobar2 * foobar2; // bad
        let z = foobar2 + RENAMED; // good
    }
    ")
}

#[test]
fn var_captured_by_closure_outside() {
    test_transform!(rename, r#"
    fn main() {
        let foobar = 1;
        let x = foo<caret>bar + 1;
        let f = |y| x + y + foobar;
    }
    "#, @r"
    fn main() {
        let RENAMED = 1;
        let x = RENAMED + 1;
        let f = |y| x + y + RENAMED;
    }
    ")
}

#[test]
fn var_captured_by_closure_inside() {
    test_transform!(rename, r#"
    fn main() {
        let foobar = 1;
        let x = foobar + 1;
        let f = |y| x + y + foo<caret>bar;
    }
    "#, @r"
    fn main() {
        let RENAMED = 1;
        let x = RENAMED + 1;
        let f = |y| x + y + RENAMED;
    }
    ")
}

#[test]
fn shadowing1() {
    test_transform!(rename, r#"
    fn main() {
        let foobar = 1;
        let x = foo<caret>bar + 1;
        let foobar = 2;
        let y = foobar + 1;
    }
    "#, @r"
    fn main() {
        let RENAMED = 1;
        let x = RENAMED + 1;
        let foobar = 2;
        let y = foobar + 1;
    }
    ")
}

#[test]
fn shadowing2() {
    test_transform!(rename, r#"
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
        let RENAMED = 2;
        let y = RENAMED + 1;
    }
    ")
}

#[test]
fn param_via_binding() {
    test_transform!(rename, r#"
    fn pow(nu<caret>m: felt252) -> felt252 {
        num * num
    }
    "#, @r"
    fn pow(RENAMED: felt252) -> felt252 {
        RENAMED * RENAMED
    }
    ")
}

#[test]
fn param_via_use() {
    test_transform!(rename, r#"
    fn pow(num: felt252) -> felt252 {
        nu<caret>m * num
    }
    "#, @r"
    fn pow(RENAMED: felt252) -> felt252 {
        RENAMED * RENAMED
    }
    ")
}

#[test]
fn param_captured_by_closure() {
    test_transform!(rename, r#"
    fn pow(num: felt252) -> felt252 {
        let f = |x| nu<caret>m * x;
        num * f(num)
    }
    "#, @r"
    fn pow(RENAMED: felt252) -> felt252 {
        let f = |x| RENAMED * x;
        RENAMED * f(RENAMED)
    }
    ")
}

#[test]
fn var_in_trait_function_default_body() {
    test_transform!(rename, r#"
    trait Foo<T> {
        fn foo() {
            let foobar = 42;
            let x = foo<caret>bar + 1;
        }
    }
    fn bar() {
        let foobar = 42;
    }
    "#, @r"
    trait Foo<T> {
        fn foo() {
            let RENAMED = 42;
            let x = RENAMED + 1;
        }
    }
    fn bar() {
        let foobar = 42;
    }
    ")
}

#[test]
fn closure_param_via_use() {
    test_transform!(rename, r#"
    fn main() {
        let f = |abc: felt252| a<caret>bc + 1;
    }
    "#, @r"
    fn main() {
        let f = |RENAMED: felt252| RENAMED + 1;
    }
    ")
}

#[test]
fn closure_param_via_binding() {
    test_transform!(rename, r#"
    fn main() {
        let f = |a<caret>bc: felt252| abc + 1;
    }
    "#, @r"
    fn main() {
        let f = |RENAMED: felt252| RENAMED + 1;
    }
    ")
}
