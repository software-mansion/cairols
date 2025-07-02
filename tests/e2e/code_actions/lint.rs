use crate::code_actions::quick_fix_with_linter;
use crate::support::insta::test_transform;

#[test]
fn check_for_lint() {
    test_transform!(quick_fix_with_linter, "
    fn main() {
        loop {
            brea<caret>k ();
        }
    }
    ", @r#"
    Title: Remove unnecessary parentheses from break
    Add new text: "        break;
    "
    At: Range { start: Position { line: 2, character: 0 }, end: Position { line: 3, character: 0 } }
    "#
    );
}

#[test]
fn check_for_complex_lint() {
    test_transform!(quick_fix_with_linter, "
    use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
    use starknet::syscalls::storage_read_syscall;

    fn main() {
        let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
        let result = storage_read_syscall(0, storage_address_from_base(storage_address));
        resul<caret>t.unwrap();
    }
    ", @r#"
    Title: Replace with `unwrap_syscall()` for syscall results
    Add new text: "    result.unwrap_syscall()"
    At: Range { start: Position { line: 6, character: 0 }, end: Position { line: 6, character: 19 } }
    Add new text: "use starknet::SyscallResultTrait;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    "#);
}
