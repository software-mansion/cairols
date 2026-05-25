use crate::inlay_hints::inlay_hint;
use crate::support::insta::test_transform;

#[test]
fn simple_function_call() {
    test_transform!(inlay_hint, r#"
    fn foo(x: felt252, y: felt252) {}

    fn main() {
        <sel>foo(1, 2);</sel>
    }
    "#, @r#"
    fn foo(x: felt252, y: felt252) {}

    fn main() {
        foo(x: 1, y: 2);
    }
    "#)
}

#[test]
fn single_parameter() {
    test_transform!(inlay_hint, r#"
    fn foo(x: felt252) {}

    fn main() {
        <sel>foo(42);</sel>
    }
    "#, @r#"
    fn foo(x: felt252) {}

    fn main() {
        foo(x: 42);
    }
    "#)
}

#[test]
fn no_parameters() {
    test_transform!(inlay_hint, r#"
    fn foo() {}

    fn main() {
        <sel>foo();</sel>
    }
    "#, @r#"
    fn foo() {}

    fn main() {
        foo();
    }
    "#)
}

#[test]
fn named_argument_skipped() {
    test_transform!(inlay_hint, r#"
    fn foo(x: felt252, y: felt252) {}

    fn main() {
        <sel>foo(x: 1, y: 2);</sel>
    }
    "#, @r#"
    fn foo(x: felt252, y: felt252) {}

    fn main() {
        foo(x: 1, y: 2);
    }
    "#)
}

#[test]
fn argument_name_matches_param() {
    test_transform!(inlay_hint, r#"
    fn foo(x: felt252, y: felt252) {}

    fn main() {
        let x = 1;
        let y = 2;
        <sel>foo(x, y);</sel>
    }
    "#, @r#"
    fn foo(x: felt252, y: felt252) {}

    fn main() {
        let x = 1;
        let y = 2;
        foo(x, y);
    }
    "#)
}

#[test]
fn mixed_matching_and_non_matching() {
    test_transform!(inlay_hint, r#"
    fn foo(x: felt252, y: felt252) {}

    fn main() {
        let x = 1;
        <sel>foo(x, 42);</sel>
    }
    "#, @r#"
    fn foo(x: felt252, y: felt252) {}

    fn main() {
        let x = 1;
        foo(x, y: 42);
    }
    "#)
}

#[test]
fn with_type_hint_on_let() {
    test_transform!(inlay_hint, r#"
    fn foo(x: felt252, y: felt252) -> felt252 { x + y }

    fn main() {
        <sel>let result = foo(1, 2);</sel>
    }
    "#, @r#"
    fn foo(x: felt252, y: felt252) -> felt252 { x + y }

    fn main() {
        let result<hint>: </hint><hint tooltip="```cairo\nfelt252\n```\n">felt252</hint> = foo(x: 1, y: 2);
    }
    "#)
}

#[test]
fn generic_function() {
    test_transform!(inlay_hint, r#"
    fn identity<T>(value: T) -> T { value }

    fn main() {
        <sel>identity(42);</sel>
    }
    "#, @r#"
    fn identity<T>(value: T) -> T { value }

    fn main() {
        identity(value: 42);
    }
    "#)
}

#[test]
fn method_call() {
    test_transform!(inlay_hint, r#"
    #[derive(Drop)]
    struct Counter {
        value: felt252,
    }

    trait CounterTrait {
        fn add(self: @Counter, amount: felt252) -> felt252;
    }

    impl CounterImpl of CounterTrait {
        fn add(self: @Counter, amount: felt252) -> felt252 {
            *self.value + amount
        }
    }

    fn main() {
        let c = Counter { value: 10 };
        <sel>c.add(5);</sel>
    }
    "#, @r#"
    #[derive(Drop)]
    struct Counter {
        value: felt252,
    }

    trait CounterTrait {
        fn add(self: @Counter, amount: felt252) -> felt252;
    }

    impl CounterImpl of CounterTrait {
        fn add(self: @Counter, amount: felt252) -> felt252 {
            *self.value + amount
        }
    }

    fn main() {
        let c = Counter { value: 10 };
        c.add(amount: 5);
    }
    "#)
}

#[test]
fn nested_calls() {
    test_transform!(inlay_hint, r#"
    fn add(a: felt252, b: felt252) -> felt252 { a + b }

    fn double(n: felt252) -> felt252 { n + n }

    fn main() {
        <sel>add(double(1), double(2));</sel>
    }
    "#, @r#"
    fn add(a: felt252, b: felt252) -> felt252 { a + b }

    fn double(n: felt252) -> felt252 { n + n }

    fn main() {
        add(a: double(n: 1), b: double(n: 2));
    }
    "#)
}

#[test]
fn arity_overflow_no_hints() {
    test_transform!(inlay_hint, r#"
    fn foo(a: felt252, b: felt252) -> felt252 { a + b }

    fn main() {
        <sel>foo(1, 2, 3);</sel>
    }
    "#, @r#"
    fn foo(a: felt252, b: felt252) -> felt252 { a + b }

    fn main() {
        foo(1, 2, 3);
    }
    "#)
}

#[test]
fn arity_underflow_no_hints() {
    test_transform!(inlay_hint, r#"
    fn foo(a: felt252, b: felt252) -> felt252 { a + b }

    fn main() {
        <sel>foo(1);</sel>
    }
    "#, @r#"
    fn foo(a: felt252, b: felt252) -> felt252 { a + b }

    fn main() {
        foo(1);
    }
    "#)
}
