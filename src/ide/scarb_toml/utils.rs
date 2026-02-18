use lsp_types::Position;

/// Translates a UTF-8 byte offset into an LSP [`Position`] (line and column).
pub fn byte_offset_to_lsp_position(text: &str, offset: usize) -> Position {
    let mut current_line: u32 = 0;
    let mut current_column: u32 = 0;
    let mut current_byte: usize = 0;

    for ch in text.chars() {
        if current_byte >= offset {
            break;
        }

        if ch == '\n' {
            current_line += 1;
            current_column = 0;
        } else {
            current_column += ch.len_utf16() as u32;
        }
        current_byte += ch.len_utf8();
    }
    Position { line: current_line, character: current_column }
}

/// Translates an LSP [`Position`] into a UTF-8 byte offset within the given text.
pub fn lsp_position_to_byte_offset(text: &str, pos: Position) -> usize {
    let mut offset = 0;
    for (i, line) in text.split_inclusive('\n').enumerate() {
        if i == pos.line as usize {
            let mut char_offset = 0;
            for ch in line.chars() {
                if char_offset >= pos.character as usize {
                    break;
                }
                char_offset += ch.len_utf16();
                offset += ch.len_utf8();
            }
            return offset;
        }
        offset += line.len();
    }
    offset
}
