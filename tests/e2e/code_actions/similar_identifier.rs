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
