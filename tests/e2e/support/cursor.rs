use std::cmp::min;
use std::str::Chars;

use itertools::{Itertools, MultiPeek};
use lsp_types::{Position, Range, TextEdit};

#[path = "cursor_test.rs"]
mod test;

/// Extracts cursor markers from the text.
///
/// A cursor is a marker in the text that can be one of the following:
/// 1. `<caret>` specifies the position where the caret should be placed.
/// 2. `<sel>` and `</sel>` specify the start and end of the selected text range.
pub fn cursors(text: &str) -> (String, Cursors) {
    // Trim the whitespace, because usually there is one for code clarity.
    let text = text.trim();

    let mut cursors = Cursors::new();
    let mut output_text = String::with_capacity(text.len());
    let mut position = Position::new(0, 0);
    let mut selection = Option::<SelectionElement>::None;

    let mut it = text.chars().multipeek();
    while let Some(ch) = it.next() {
        match ch {
            // Handle the '<caret>' marker.
            '<' if peek(&mut it, "caret") => {
                eat(&mut it, "caret>");
                cursors.add_caret(position);
            }
            '<' if peek(&mut it, "sel") => {
                eat(&mut it, "sel>");
                handle_selection(&mut selection, &mut cursors, SelectionElement::Open(position));
            }
            '<' if peek(&mut it, "/sel") => {
                eat(&mut it, "/sel>");
                handle_selection(&mut selection, &mut cursors, SelectionElement::Close(position));
            }
            _ => {
                // Add the character to the output text.
                output_text.push(ch);

                // Increment line and character.
                if ch == '\n' {
                    position.line += 1;
                    position.character = 0;
                } else {
                    position.character += 1;
                }
            }
        }
    }

    assert!(selection.is_none());

    return (output_text, cursors);

    #[derive(Copy, Clone)]
    enum SelectionElement {
        Open(Position),
        Close(Position),
    }

    fn handle_selection(
        selection: &mut Option<SelectionElement>,
        cursors: &mut Cursors,
        right: SelectionElement,
    ) {
        match (selection.take(), right) {
            (Some(SelectionElement::Open(start)), SelectionElement::Close(end)) => {
                cursors.add_selection(Range { start, end });
            }
            (None, SelectionElement::Open(_)) => *selection = Some(right),
            _ => panic!("selections should not overlap"),
        }
    }

    /// Peek multiple characters at once, check if they match the needle, and reset the peek.
    fn peek(it: &mut MultiPeek<Chars<'_>>, needle: &str) -> bool {
        let mut matched = true;
        for needle_ch in needle.chars() {
            let Some(&haystack_ch) = it.peek() else {
                matched = false;
                break;
            };
            if needle_ch != haystack_ch {
                matched = false;
                break;
            }
        }
        it.reset_peek();
        matched
    }

    /// Consume multiple characters at once, asserting they match the needle.
    fn eat(it: &mut MultiPeek<Chars<'_>>, needle: &str) {
        for needle_ch in needle.chars() {
            let haystack_ch = it.next();
            assert_eq!(haystack_ch, Some(needle_ch));
        }
    }
}

/// A collection of cursors in a text document.
///
/// See [`cursors`] docs for more information.
pub struct Cursors {
    carets: Vec<Position>,
    selections: Vec<Range>,
}

impl Cursors {
    fn new() -> Self {
        Self { carets: Vec::new(), selections: Vec::new() }
    }

    fn add_caret(&mut self, cursor: Position) {
        self.carets.push(cursor);
    }

    fn add_selection(&mut self, selection: Range) {
        self.selections.push(selection);
    }

    /// Get specified caret.
    pub fn caret(&self, idx: usize) -> Position {
        *self.carets.get(idx).unwrap_or_else(|| panic!("cursor not found: {idx}"))
    }

    /// Get all carets.
    pub fn carets(&self) -> Vec<Position> {
        self.carets.clone()
    }

    /// Get specified selection.
    pub fn selection(&self, idx: usize) -> Range {
        *self.selections.get(idx).unwrap_or_else(|| panic!("cursor not found: {idx}"))
    }

    pub fn assert_single(&self) -> Cursor {
        match (&self.carets[..], &self.selections[..]) {
            ([caret], []) => Cursor::Caret(*caret),
            ([], [selection]) => Cursor::Selection(*selection),
            _ => panic!("there should be exacly one caret or selection"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Cursor {
    Caret(Position),
    Selection(Range),
}

/// Creates a snippet (lines of interest) of the source text showing a caret at the specified
/// position.
pub fn peek_caret(text: &str, position: Position) -> String {
    let mut snippet = text.to_owned();
    snippet.insert_str(index_in_text(text, position), "<caret>");
    snippet.lines().nth(position.line as usize).unwrap().to_owned() + "\n"
}

/// Creates a snippet (lines of interest) of the source text showing a selection of the specified
/// range.
pub fn peek_selection(text: &str, range: &Range) -> String {
    let mut snippet = text.to_owned();
    assert!(range.start <= range.end);
    snippet.insert_str(index_in_text(text, range.start), "<sel>");
    snippet.insert_str(index_in_text(text, range.end) + "<sel>".len(), "</sel>");
    snippet
        .lines()
        .skip(range.start.line as usize)
        .take(range.end.line as usize - range.start.line as usize + 1)
        .join("\n")
        + "\n"
}

/// Adds selection markers for all ranges to the source text.
pub fn render_selections(text: &str, ranges: &[Range]) -> String {
    render_selections_with_attrs(
        text,
        &ranges.iter().map(|range| (*range, None)).collect::<Vec<_>>(),
    )
}

/// Adds selection markers for all ranges to the source text with optional attributes to attach.
pub fn render_selections_with_attrs(text: &str, ranges: &[(Range, Option<String>)]) -> String {
    let mut text = text.to_owned();
    ranges
        .iter()
        .flat_map(|(range, attr)| {
            assert!(range.start <= range.end);
            [
                (
                    index_in_text(&text, range.start),
                    format!(
                        "<sel{attr}>",
                        attr = attr.as_ref().map(|val| format!("={val}")).unwrap_or_default()
                    ),
                ),
                (index_in_text(&text, range.end), "</sel>".to_owned()),
            ]
        })
        .sorted_by_key(|(idx, _)| *idx)
        .fold(0, |offset, (idx, marker)| {
            text.insert_str(idx + offset, &marker);
            offset + marker.len()
        });
    text
}

/// Applies text edits to the code.
pub fn render_text_edits(text_edits: Vec<TextEdit>, mut cairo_code: String) -> String {
    for edit in text_edits {
        let start_idx = index_in_text(&cairo_code, edit.range.start);
        let stop_idx = index_in_text(&cairo_code, edit.range.end);
        cairo_code.replace_range(start_idx..stop_idx, &edit.new_text);
    }
    cairo_code
}

/// Converts a [`Position`] to a char-bounded index in the text.
///
/// This function assumes UTF-8 position encoding.
fn index_in_text(text: &str, position: Position) -> usize {
    let mut offset = 0;
    let mut lines = text.lines();
    for line in lines.by_ref().take(position.line as usize) {
        offset += line.len() + "\n".len();
    }
    if let Some(line) = lines.next() {
        offset += min(position.character as usize, line.len());
    }
    offset
}

pub fn text_chunk_at_range(text: String, range: Range) -> String {
    let start_idx = index_in_text(&text, range.start);
    let stop_idx = index_in_text(&text, range.end);

    text[start_idx..stop_idx].to_string()
}
