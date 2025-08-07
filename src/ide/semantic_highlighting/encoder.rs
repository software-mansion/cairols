/// LSP protocol is using a differential position encoding to report the tokens.
/// This encoder outputs this encoding.
#[derive(Default)]
pub struct TokenEncoder {
    /// Line number of the last encoded token.
    last_line: u32,
    /// Column number of the last encoded token.
    last_character: u32,
    /// Current line number.
    line: u32,
    /// Current column number.
    character: u32,
}
pub struct EncodedToken {
    pub delta_line: u32,
    pub delta_start: u32,
}
impl TokenEncoder {
    /// Skip a non newline token.
    pub fn skip(&mut self, width: u32) {
        self.character += width;
    }

    /// Moves to the next line.
    pub fn next_line(&mut self) {
        self.line += 1;
        self.character = 0;
    }

    /// Creates an encoded token based on the current position and the width of the token.
    /// This updates the state of the encoder text positions.
    pub fn encode(&mut self, width: u32) -> EncodedToken {
        let delta_line = self.line - self.last_line;
        let prev_col = if delta_line > 0 { 0 } else { self.last_character };
        let delta_start = self.character - prev_col;
        self.last_line = self.line;
        self.last_character = self.character;
        self.skip(width);
        EncodedToken { delta_line, delta_start }
    }
}
