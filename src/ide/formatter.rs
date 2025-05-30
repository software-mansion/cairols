use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_formatter::get_formatted_file;
use cairo_lang_parser::db::ParserGroup;
use lsp_types::{DocumentFormattingParams, Position, Range, TextEdit};
use tracing::error;

use crate::lang::lsp::LsProtoGroup;
use crate::state::StateSnapshot;

/// Format a whole document.
pub fn format(params: DocumentFormattingParams, state: StateSnapshot) -> Option<Vec<TextEdit>> {
    let db = &*state.db;
    let file_uri = params.text_document.uri;
    let file = db.file_for_url(&file_uri)?;

    let path = file_uri.to_file_path().ok()?;

    let config = state.configs_registry.config_for_file(&path).unwrap_or_default();

    let Ok(node) = db.file_syntax(file) else {
        error!("formatting failed: file '{file_uri}' does not exist");
        return None;
    };

    if db.file_syntax_diagnostics(file).check_error_free().is_err() {
        error!("formatting failed: cannot properly parse '{file_uri}' exist");
        return None;
    }

    let new_text = get_formatted_file(db, &node, config.fmt);

    let Some(file_summary) = db.file_summary(file) else {
        error!("formatting failed: cannot get summary for file '{file_uri}'");
        return None;
    };

    let Ok(old_line_count) = file_summary.line_count().try_into() else {
        error!("formatting failed: line count out of bound in file '{file_uri}'");
        return None;
    };

    Some(vec![TextEdit {
        range: Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: old_line_count, character: 0 },
        },
        new_text,
    }])
}
