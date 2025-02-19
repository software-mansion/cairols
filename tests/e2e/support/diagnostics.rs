use lsp_types::{Diagnostic, Range, Url};

use crate::support::MockClient;

// Slices the chunk of given text at a given `Range`
fn text_chunk_at_range(text: String, range: Range) -> String {
    let lines: Vec<&str> = text.lines().collect();

    let start = range.start;
    let end = range.end;

    let start_char_idx = start.character as usize;
    let end_char_idx = end.character as usize;
    let start_line_idx = start.line as usize;
    let end_line_idx = end.line as usize;

    if start_line_idx == end_line_idx {
        // If the start and end positions are on the same line
        let line = lines[start_line_idx];

        line[start_char_idx..end_char_idx].to_string()
    } else {
        // Collect slices from the start, middle (if any), and end lines
        let mut result = String::new();
        // Start line
        let start_line = lines[start_line_idx];
        result.push_str(&start_line[start_char_idx..]);

        // Lines in between, if any
        for line in &lines[(start_line_idx + 1)..end_line_idx] {
            result.push_str(line);
            result.push('\n'); // Preserve line breaks
        }

        // End line
        let end_line = lines[end_line_idx];
        if !result.ends_with('\n') {
            result.push('\n');
        }
        result.push_str(&end_line[..end_char_idx]);

        result
    }
}

pub fn get_related_diagnostic_code(
    client: &MockClient,
    diagnostic: &Diagnostic,
    file_url: &Url,
) -> String {
    let fixture = client.as_ref();
    let file_content = fixture.read_file(file_url.path());
    text_chunk_at_range(file_content, diagnostic.range)
}
