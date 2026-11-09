use crate::code_actions::{quick_fix, quick_fix_with_macros};
use crate::support::insta::test_transform;

#[test]
fn single_segment() {
    test_transform!(quick_fix, "
    struct Mariusz {
        whatever: felt252
    }

    fn main() {
        let _x = Mar<caret>ius { whatever: 1 };
    }
    ", @r#"
    Title: Did you mean `Mariusz`?
    Add new text: "Mariusz"
    At: Range { start: Position { line: 5, character: 13 }, end: Position { line: 5, character: 19 } }
    "#);
}

#[test]
fn multi_segment_first_bad() {
    test_transform!(quick_fix, "
    fn main() {
        sra<caret>rknet::syscalls::emit_event_syscall(array![1].span(), array![2].span()).unwrap_syscall();
    }
    ", @r#"
    Title: Did you mean `starknet`?
    Add new text: "starknet"
    At: Range { start: Position { line: 1, character: 4 }, end: Position { line: 1, character: 12 } }
    "#);
}

#[test]
fn multi_segment_second_bad() {
    test_transform!(quick_fix, "
    fn main() {
        starknet::sysk<caret>ells::emit_event_syscall(array![1].span(), array![2].span()).unwrap_syscall();
    }
    ", @r#"
    Title: Did you mean `syscalls`?
    Add new text: "syscalls"
    At: Range { start: Position { line: 1, character: 14 }, end: Position { line: 1, character: 22 } }
    "#);
}

#[test]
fn multi_segment_third_bad() {
    test_transform!(quick_fix, "
    fn main() {
        starknet::syscalls::emil_event<caret>_syscal(array![1].span(), array![2].span()).unwrap_syscall();
    }
    ", @r#"
    Title: Did you mean `emit_event_syscall`?
    Add new text: "emit_event_syscall"
    At: Range { start: Position { line: 1, character: 24 }, end: Position { line: 1, character: 41 } }
    Title: Did you mean `storage_write_syscall`?
    Add new text: "storage_write_syscall"
    At: Range { start: Position { line: 1, character: 24 }, end: Position { line: 1, character: 41 } }
    Title: Did you mean `meta_tx_v0_syscall`?
    Add new text: "meta_tx_v0_syscall"
    At: Range { start: Position { line: 1, character: 24 }, end: Position { line: 1, character: 41 } }
    "#);
}

#[test]
fn multi_segment_with_trivia_comment() {
    test_transform!(quick_fix, "
    fn main() {
        starknet
        // this is a comment
        ::sysk<caret>alls::emit_event_syscall(array![1].span(), array![2].span()).unwrap_syscall();
    }
    ", @r#"
    Title: Did you mean `syscalls`?
    Add new text: "syscalls"
    At: Range { start: Position { line: 3, character: 6 }, end: Position { line: 3, character: 14 } }
    "#);
}

#[test]
fn multi_segment_with_trivia_whitespace() {
    test_transform!(quick_fix, "
    fn main() {
        starknet::    sysk<caret>ells    ::emit_event_syscall(array![1].span(), array![2].span()).unwrap_syscall();
    }
    ", @r#"
    Title: Did you mean `syscalls`?
    Add new text: "syscalls"
    At: Range { start: Position { line: 1, character: 18 }, end: Position { line: 1, character: 26 } }
    "#);
}

#[test]
fn in_proc_macro_controlled_code() {
    test_transform!(quick_fix_with_macros, "
    #[test]
    fn test_similar_identifier() {
        snforge_std::generate_<caret>rando_felt();
    }
    ", @r#"
    Title: Did you mean `generate_random_felt`?
    Add new text: "generate_random_felt"
    At: Range { start: Position { line: 2, character: 17 }, end: Position { line: 2, character: 36 } }
    "#);
}

#[test]
fn function_call_suggests_only_functions() {
    test_transform!(quick_fix, "
    struct Balance {
        amount: felt252
    }

    mod balances {}

    fn balance() -> felt252 {
        1
    }

    fn main() {
        let _x = balanc<caret>();
    }
    ", @r#"
    Title: Did you mean `panic`?
    Add new text: "panic"
    At: Range { start: Position { line: 11, character: 13 }, end: Position { line: 11, character: 19 } }
    Title: Did you mean `balance`?
    Add new text: "balance"
    At: Range { start: Position { line: 11, character: 13 }, end: Position { line: 11, character: 19 } }
    "#);
}

#[test]
fn function_call_suggests_enum_variants() {
    test_transform!(quick_fix, "
    enum Action {
        Deposit: felt252,
        Withdraw: felt252,
    }

    fn main() {
        let _x = Action::Deposi<caret>(1);
    }
    ", @"No code actions.");
}

#[test]
fn struct_constructor_suggests_only_structs() {
    test_transform!(quick_fix, "
    struct Balance {
        amount: felt252
    }

    mod Balances {}

    fn Balanced() -> felt252 {
        1
    }

    fn main() {
        let _x = Balanc<caret> { amount: 1 };
    }
    ", @r#"
    Title: Did you mean `Panic`?
    Add new text: "Panic"
    At: Range { start: Position { line: 11, character: 13 }, end: Position { line: 11, character: 19 } }
    Title: Did you mean `Balance`?
    Add new text: "Balance"
    At: Range { start: Position { line: 11, character: 13 }, end: Position { line: 11, character: 19 } }
    "#);
}

#[test]
fn type_clause_suggests_only_types() {
    test_transform!(quick_fix, "
    struct Balance {
        amount: felt252
    }

    mod Balances {}

    fn Balanced() -> felt252 {
        1
    }

    fn main() {
        let _x: Balanc<caret> = Balance { amount: 1 };
    }
    ", @r#"
    Title: Did you mean `Panic`?
    Add new text: "Panic"
    At: Range { start: Position { line: 11, character: 12 }, end: Position { line: 11, character: 18 } }
    Title: Did you mean `Balance`?
    Add new text: "Balance"
    At: Range { start: Position { line: 11, character: 12 }, end: Position { line: 11, character: 18 } }
    "#);
}

#[test]
fn generic_argument_suggests_only_types() {
    test_transform!(quick_fix, "
    struct Balance {
        amount: felt252
    }

    mod Balances {}

    fn Balanced() -> felt252 {
        1
    }

    fn main() {
        let _x: Array<Balanc<caret>> = array![];
    }
    ", @r#"
    Title: Did you mean `Panic`?
    Add new text: "Panic"
    At: Range { start: Position { line: 11, character: 18 }, end: Position { line: 11, character: 24 } }
    Title: Did you mean `Balance`?
    Add new text: "Balance"
    At: Range { start: Position { line: 11, character: 18 }, end: Position { line: 11, character: 24 } }
    "#);
}

#[test]
fn struct_pattern_suggests_only_structs() {
    test_transform!(quick_fix, "
    struct Balance {
        amount: felt252
    }

    mod Balances {}

    fn Balanced() -> felt252 {
        1
    }

    fn main() {
        let Balanc<caret> { amount: _ } = Balance { amount: 1 };
    }
    ", @r#"
    Title: Did you mean `Panic`?
    Add new text: "Panic"
    At: Range { start: Position { line: 11, character: 8 }, end: Position { line: 11, character: 14 } }
    Title: Did you mean `Balance`?
    Add new text: "Balance"
    At: Range { start: Position { line: 11, character: 8 }, end: Position { line: 11, character: 14 } }
    "#);
}

#[test]
fn enum_pattern_suggests_only_variants() {
    test_transform!(quick_fix, "
    enum Action {
        Deposit: felt252,
        Withdraw: felt252,
    }

    fn main() {
        let action = Action::Deposit(1);
        match action {
            Action::Deposi<caret>(_) => {},
            Action::Withdraw(_) => {},
        }
    }
    ", @"No code actions.");
}

#[test]
fn value_suggests_only_constants_and_variants() {
    test_transform!(quick_fix, "
    const LIMIT: felt252 = 1;

    mod LIMITS {}

    fn main() {
        let _x = LIMI<caret>;
    }
    ", @r#"
    Title: Did you mean `LIMITS`?
    Add new text: "LIMITS"
    At: Range { start: Position { line: 5, character: 13 }, end: Position { line: 5, character: 17 } }
    Title: Did you mean `LIMIT`?
    Add new text: "LIMIT"
    At: Range { start: Position { line: 5, character: 13 }, end: Position { line: 5, character: 17 } }
    "#);
}

#[test]
fn inline_macro_suggests_only_macros() {
    test_transform!(quick_fix, "
    fn arrays() -> felt252 {
        1
    }

    fn main() {
        let _x = arra<caret>![1];
    }
    ", @"Title: Expand macro recursively at caret");
}
