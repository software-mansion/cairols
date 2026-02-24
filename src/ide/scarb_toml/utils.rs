use lsp_types::Position;

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
