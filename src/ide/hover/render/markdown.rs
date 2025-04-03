//! Utilities for building Markdown texts.
//!
//! Markdown is used because this is the format used by the LSP protocol for rich text.

/// Horizontal rule.
pub const RULE: &str = "---\n";

/// Surround the given code with `cairo` fenced code block.
pub fn fenced_code_block(code: &str) -> String {
    if code.is_empty() {
        return String::new();
    }
    format!("```cairo\n{code}\n```\n")
}
