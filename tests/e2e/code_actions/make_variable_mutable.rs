use crate::code_actions::{quick_fix, quick_fix_with_macros};
use crate::support::insta::test_transform;

#[test]
fn change_to_mutable_let_definition() {
    test_transform!(quick_fix, "
    fn abc() {
        let a: felt252 = 1;
        a = 25<caret>;
    }
    ", @r#"
    Title: Change "a" to be mutable
    Add new text: "mut a"
    At: Range { start: Position { line: 1, character: 8 }, end: Position { line: 1, character: 9 } }
    "#);
}

#[test]
fn change_to_mutable_param_definition() {
    test_transform!(quick_fix, "
    fn abc(a: felt252) {
        a = 25<caret>;
    }
    ", @r#"
    Title: Change "a" to be mutable
    Add new text: "mut a"
    At: Range { start: Position { line: 0, character: 7 }, end: Position { line: 0, character: 8 } }
    "#);
}

#[test]
fn change_to_mutable_let_definition_with_macros() {
    test_transform!(quick_fix_with_macros, "
    #[test]
    fn abc() {
        let a: felt252 = 1;
        a = 25<caret>;
    }
    ", @r#"
    Title: Change "a" to be mutable
    Add new text: "mut a"
    At: Range { start: Position { line: 2, character: 8 }, end: Position { line: 2, character: 9 } }
    "#);
}

#[test]
fn change_to_mutable_param_definition_with_macros() {
    test_transform!(quick_fix_with_macros, "
    #[test]
    fn abc(a: felt252) {
        a = 25<caret>;
    }
    ", @r#"
    Title: Change "a" to be mutable
    Add new text: "mut a"
    At: Range { start: Position { line: 1, character: 7 }, end: Position { line: 1, character: 8 } }
    "#);
}

#[test]
fn change_ref_argument_to_mutable() {
    test_transform!(quick_fix, "
    #[derive(Copy, Drop)]
    struct X {}

    trait Abc {
        fn abc(self: X, ref a: felt252) {}
    }


    impl AbcImpl of Abc {
        fn abc(self: X, ref a: felt252) {
            a = 5;
        }
    }

    fn main() {
        let x = X {};
        let a = 15;
        x.abc(ref a<caret>);
    }
    ", @r#"
    Title: Change "a" to be mutable
    Add new text: "mut a"
    At: Range { start: Position { line: 16, character: 8 }, end: Position { line: 16, character: 9 } }
    "#);
}

#[test]
fn change_ref_argument_to_mutable_with_macros() {
    test_transform!(quick_fix_with_macros, "
    #[derive(Copy, Drop)]
    struct X {}

    trait Abc {
        fn abc(self: X, ref a: felt252) {}
    }


    impl AbcImpl of Abc {
        fn abc(self: X, ref a: felt252) {
            a = 5;
        }
    }

    #[test]
    fn maintest() {
        let x = X {};
        let a = 15;
        x.abc(ref a<caret>);
    }
    ", @r#"
    Title: Change "a" to be mutable
    Add new text: "mut a"
    At: Range { start: Position { line: 17, character: 8 }, end: Position { line: 17, character: 9 } }
    "#);
}
