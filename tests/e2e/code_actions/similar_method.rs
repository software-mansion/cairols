use crate::code_actions::quick_fix;
use crate::support::insta::test_transform;

#[test]
fn add_1_char_typo() {
    test_transform!(quick_fix, "
    trait ATrait<T> {
        fn some_method(self: @T);
    }
    impl Felt252ATraitImpl of ATrait<felt252> {
        fn some_method(self: @felt252) {}
    }

    fn main() {
        let x = 5_felt252;
        x.some_me<caret>thood();
    }
    ", @r#"
    Title: Use some_method instead
    Add new text: "some_method"
    At: Range { start: Position { line: 9, character: 6 }, end: Position { line: 9, character: 18 } }
    "#);
}

#[test]
fn add_1_char_typo_multiple_methods_different_traits() {
    test_transform!(quick_fix, "
    trait ATrait<T> {
        fn some_method(self: @T);
    }
    impl Felt252ATraitImpl of ATrait<felt252> {
        fn some_method(self: @felt252) {}
    }

    trait BTrait<T> {
        fn some_method_2(self: @T);
    }
    impl Felt252BTraitImpl of BTrait<felt252> {
        fn some_method_2(self: @felt252) {}
    }

    fn main() {
        let x = 5_felt252;
        x.some_me<caret>thood();
    }
    ", @r#"
    Title: Use some_method instead
    Add new text: "some_method"
    At: Range { start: Position { line: 16, character: 6 }, end: Position { line: 16, character: 18 } }
    Title: Use some_method_2 instead
    Add new text: "some_method_2"
    At: Range { start: Position { line: 16, character: 6 }, end: Position { line: 16, character: 18 } }
    "#);
}

#[test]
fn add_1_char_typo_nested_module() {
    test_transform!(quick_fix, "
    trait ATrait<T> {
        fn some_method(self: @T);
    }
    impl Felt252ATraitImpl of ATrait<felt252> {
        fn some_method(self: @felt252) {}
    }

    mod nested {
        fn main() {
            let x = 5_felt252;
            x.some_me<caret>thood();
        }
    }
    ", @r#"
    Title: Use some_method instead
    Add new text: "some_method"
    At: Range { start: Position { line: 10, character: 10 }, end: Position { line: 10, character: 22 } }
    Add new text: "use crate::ATrait;

    "
    At: Range { start: Position { line: 8, character: 4 }, end: Position { line: 8, character: 4 } }
    "#);
}

#[test]
fn no_similar_method_found() {
    test_transform!(quick_fix, "
    trait ATrait<T> {
        fn some_method(self: @T);
    }
    impl Felt252ATraitImpl of ATrait<felt252> {
        fn some_method(self: @felt252) {}
    }

    fn main() {
        let x = 5_felt252;
        x.completely_different_na<caret>me();
    }
    ", @"No code actions.");
}
