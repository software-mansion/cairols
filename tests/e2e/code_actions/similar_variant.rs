use crate::code_actions::{quick_fix, quick_fix_with_macros};
use crate::support::insta::test_transform;

#[test]
fn variant_typo() {
    test_transform!(quick_fix, "
    enum Action {
        Deposit: felt252,
        Withdraw: felt252,
    }

    fn main() {
        let _x = Action::Deposi<caret>(1);
    }
    ", @r#"
    Title: Did you mean `Deposit`?
    Add new text: "Deposit"
    At: Range { start: Position { line: 6, character: 21 }, end: Position { line: 6, character: 27 } }
    "#);
}

#[test]
fn variant_typo_multiple_variants() {
    test_transform!(quick_fix, "
    enum Action {
        Deposit: felt252,
        Deposited: felt252,
        Withdraw: felt252,
    }

    fn main() {
        let _x = Action::Deposi<caret>(1);
    }
    ", @r#"
    Title: Did you mean `Deposit`?
    Add new text: "Deposit"
    At: Range { start: Position { line: 7, character: 21 }, end: Position { line: 7, character: 27 } }
    Title: Did you mean `Deposited`?
    Add new text: "Deposited"
    At: Range { start: Position { line: 7, character: 21 }, end: Position { line: 7, character: 27 } }
    "#);
}

#[test]
fn no_similar_variant_found() {
    test_transform!(quick_fix, "
    enum Action {
        Deposit: felt252,
        Withdraw: felt252,
    }

    fn main() {
        let _x = Action::Completely_differen<caret>t(1);
    }
    ", @"No code actions.");
}

#[test]
fn variant_typo_in_macro_call() {
    test_transform!(quick_fix_with_macros, "
    enum Action {
        Deposit: felt252,
        Withdraw: felt252,
    }

    #[test]
    fn test_action() {
        let _x = Action::Deposi<caret>(1);
    }
    ", @r#"
    Title: Did you mean `Deposit`?
    Add new text: "Deposit"
    At: Range { start: Position { line: 7, character: 21 }, end: Position { line: 7, character: 27 } }
    "#);
}
