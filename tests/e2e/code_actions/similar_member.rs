use crate::code_actions::quick_fix;
use crate::support::insta::test_transform;

#[test]
fn remove_1_char_typo() {
    test_transform!(quick_fix, "
    struct ElStructuro {
        membero: felt252
    }

    fn main() {
        let x = ElStructuro { membero: 1 };
        let _v = x.me<caret>mber + 1
    }
    ", @r#"
    Title: Use membero instead
    Add new text: "membero"
    At: Range { start: Position { line: 6, character: 15 }, end: Position { line: 6, character: 21 } }
    "#);
}

#[test]
fn add_1_char_typo() {
    test_transform!(quick_fix, "
    struct ElStructuro {
        membero: felt252
    }

    fn main() {
        let x = ElStructuro { membero: 1 };
        let _v = x.membe<caret>roo + 1
    }
    ", @r#"
    Title: Use membero instead
    Add new text: "membero"
    At: Range { start: Position { line: 6, character: 15 }, end: Position { line: 6, character: 23 } }
    "#);
}

#[test]
fn add_1_char_typo_multiple_members() {
    test_transform!(quick_fix, "
    struct S {
        some_member: felt252,
        some_member_2: felt252,
    }

    fn main() {
        let x = S { some_member: 1, some_member_2: 2 };
        let _v = x.some_membe<caret>er + 1
    }
    ", @r#"
    Title: Use some_member instead
    Add new text: "some_member"
    At: Range { start: Position { line: 7, character: 15 }, end: Position { line: 7, character: 27 } }
    Title: Use some_member_2 instead
    Add new text: "some_member_2"
    At: Range { start: Position { line: 7, character: 15 }, end: Position { line: 7, character: 27 } }
    "#);
}

#[test]
fn no_similar_member_found() {
    test_transform!(quick_fix, "
    struct ElStructuro {
        membero: felt252
    }

    fn main() {
        let x = ElStructuro { membero: 1 };
        let _v = x.completely_different_na<caret>me + 1
    }
    ", @"No code actions.");
}

// FIXME
#[test]
fn trait_function_body_member_typo() {
    test_transform!(quick_fix, "
    struct S {
        membero: felt252
    }

    trait Foo<T> {
        fn foo() {
            let s = S { membero: 1 };
            let _v = s.memb<caret>er + 1;
        }
    }
    ", @r#"
    Title: Use membero instead
    Add new text: "membero"
    At: Range { start: Position { line: 7, character: 19 }, end: Position { line: 7, character: 25 } }
    "#);
}

#[test]
fn impl_function_body_member_typo() {
    test_transform!(quick_fix, "
    struct S { membero: felt252 }

    trait Foo<T> { fn foo(self: T) -> felt252; }

    impl FeltFoo of Foo<S> {
        fn foo(self: S) -> felt252 {
            let _v = self.membe<caret>r + 1;
            0
        }
    }
    ", @r#"
    Title: Use membero instead
    Add new text: "membero"
    At: Range { start: Position { line: 6, character: 22 }, end: Position { line: 6, character: 28 } }
    "#);
}

#[test]
fn mutable_variable_member_typo() {
    test_transform!(quick_fix, "
    struct S { membero: felt252 }

    fn main() {
        let mut x = S { membero: 1 };
        let _v = x.membe<caret>r + 1;
    }
    ", @r#"
    Title: Use membero instead
    Add new text: "membero"
    At: Range { start: Position { line: 4, character: 15 }, end: Position { line: 4, character: 21 } }
    "#);
}

#[test]
fn nested_member_typo() {
    test_transform!(quick_fix, "
    struct Inner { foo: felt252 }
    struct Outer { inner: Inner }

    fn main() {
        let o = Outer { inner: Inner { foo: 1 } };
        let _v = o.inner.fo<caret>p + 1
    }
    ", @r#"
    Title: Use foo instead
    Add new text: "foo"
    At: Range { start: Position { line: 5, character: 21 }, end: Position { line: 5, character: 24 } }
    "#);
}

#[test]
fn typo_with_trivia() {
    test_transform!(quick_fix, "
    struct ElStructuro {
        membero: felt252
    }

    fn main() {
        let x = ElStructuro { membero: 1 };
        let _v = x   .    me<caret>mber      + 1
    }
    ", @r#"
    Title: Use membero instead
    Add new text: "membero"
    At: Range { start: Position { line: 6, character: 22 }, end: Position { line: 6, character: 28 } }
    "#);
}
