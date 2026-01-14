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
    Title: Change it to be mutable
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
    Title: Change it to be mutable
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
    Title: Change it to be mutable
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
    Title: Change it to be mutable
    Add new text: "mut a"
    At: Range { start: Position { line: 1, character: 7 }, end: Position { line: 1, character: 8 } }
    "#);
}
