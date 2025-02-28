use lsp_types::{Position, Range};

use super::cursors;

#[test]
fn test_cursors() {
    let (text, cursors) = cursors(
        r#"<caret>
He<caret>llo, <caret>world!
"#,
    );
    assert_eq!(text.find('<'), None);
    assert_eq!(cursors.caret(0), Position::new(0, 0));
    assert_eq!(cursors.caret(1), Position::new(1, 2));
    assert_eq!(cursors.caret(2), Position::new(1, 7));
}

#[test]
fn test_cursors_with_selections() {
    let (text, cursors) = cursors(
        r#"<sel><caret>
He<caret>llo, <caret>wor</sel>ld!
<sel> some more text</sel>, maybe more
"#,
    );
    assert_eq!(text.find('<'), None);
    assert_eq!(cursors.caret(0), Position::new(0, 0));
    assert_eq!(cursors.caret(1), Position::new(1, 2));
    assert_eq!(cursors.caret(2), Position::new(1, 7));
    assert_eq!(
        cursors.selection(0),
        Range { start: Position::new(0, 0), end: Position::new(1, 10) }
    );
    assert_eq!(
        cursors.selection(1),
        Range { start: Position::new(2, 0), end: Position::new(2, 15) }
    );
}
