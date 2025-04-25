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
    Title: Fix lint
    Add new text: "        break;
    "
    At: Range { start: Position { line: 2, character: 0 }, end: Position { line: 3, character: 0 } }
    "#
    );
}
