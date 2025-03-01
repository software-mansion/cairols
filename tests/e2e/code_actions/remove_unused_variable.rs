use crate::code_actions::quick_fix;
use crate::support::insta::test_transform;

#[test]
fn on_let_keyword() {
    test_transform!(quick_fix, "
    fn a() {
        le<caret>t b = 1234;
    }
    ", @"No code actions.");
}

#[test]
fn before_name() {
    test_transform!(quick_fix, "
    fn a() {
        let <caret>b = 1234;
    }
    ", @r#"
    Title: Rename to `_b`
    Add new text: "_"
    At: Range { start: Position { line: 1, character: 8 }, end: Position { line: 1, character: 9 } }
    Title: Fix All
    Add new text: "_"
    At: Range { start: Position { line: 1, character: 8 }, end: Position { line: 1, character: 9 } }
    "#);
}

#[test]
fn after_name() {
    test_transform!(quick_fix, "
    fn a() {
        let b<caret> = 1234;
    }
    ", @r#"
    Title: Rename to `_b`
    Add new text: "_"
    At: Range { start: Position { line: 1, character: 8 }, end: Position { line: 1, character: 9 } }
    Title: Fix All
    Add new text: "_"
    At: Range { start: Position { line: 1, character: 8 }, end: Position { line: 1, character: 9 } }
    "#);
}

#[test]
fn before_value() {
    test_transform!(quick_fix, "
    fn a() {
        let b = <caret>1234;
    }
    ", @"No code actions.");
}

#[test]
fn on_value() {
    test_transform!(quick_fix, "
    fn a() {
        let b = 12<caret>34;
    }
    ", @"No code actions.");
}

#[test]
fn after_value() {
    test_transform!(quick_fix, "
    fn a() {
        let b = 1234<caret>;
    }
    ", @"No code actions.");
}

#[test]
fn after_let_statement() {
    test_transform!(quick_fix, "
    fn a() {
        let b = 1234;<caret>
    }
    ", @"No code actions.");
}

#[test]
fn on_let_keyword_when_mut() {
    test_transform!(quick_fix, "
    fn a() {
        le<caret>t mut b = 1234;
    }
    ", @"No code actions.");
}

#[test]
fn before_name_when_mut() {
    test_transform!(quick_fix, "
    fn a() {
        let mut <caret>b = 1234;
    }
    ", @r#"
    Title: Rename to `_b`
    Add new text: "_"
    At: Range { start: Position { line: 1, character: 12 }, end: Position { line: 1, character: 13 } }
    Title: Fix All
    Add new text: "_"
    At: Range { start: Position { line: 1, character: 12 }, end: Position { line: 1, character: 13 } }
    "#);
}

#[test]
fn after_name_when_mut() {
    test_transform!(quick_fix, "
    fn a() {
        let mut b<caret> = 1234;
    }
    ", @r#"
    Title: Rename to `_b`
    Add new text: "_"
    At: Range { start: Position { line: 1, character: 12 }, end: Position { line: 1, character: 13 } }
    Title: Fix All
    Add new text: "_"
    At: Range { start: Position { line: 1, character: 12 }, end: Position { line: 1, character: 13 } }
    "#);
}

#[test]
fn before_value_when_mut() {
    test_transform!(quick_fix, "
    fn a() {
        let mut b = <caret>1234;
    }
    ", @"No code actions.");
}

#[test]
fn on_value_when_mut() {
    test_transform!(quick_fix, "
    fn a() {
        let mut b = 12<caret>34;
    }
    ", @"No code actions.");
}

#[test]
fn after_value_when_mut() {
    test_transform!(quick_fix, "
    fn a() {
        let mut b = 1234<caret>;
    }
    ", @"No code actions.");
}

#[test]
fn after_let_statement_when_mut() {
    test_transform!(quick_fix, "
    fn a() {
        let mut b = 1234;<caret>
    }
    ", @"No code actions.");
}
