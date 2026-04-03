use std::sync::Arc;

use cairo_lang_diagnostics::DiagnosticsBuilder;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::{FileKind, FileLongId, SmolStrId, VirtualFile};
use cairo_lang_formatter::get_formatted_file;
use cairo_lang_parser::parser::Parser;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_utils::Intern;
use lsp_types::{DocumentFormattingParams, Position, Range, TextEdit};
use tracing::error;

use crate::lang::lsp::LsProtoGroup;
use crate::state::StateSnapshot;

/// Format a whole document.
pub fn format_document(
    params: DocumentFormattingParams,
    state: StateSnapshot,
) -> Option<Vec<TextEdit>> {
    let db = &state.db;
    let file_uri = params.text_document.uri;
    let file = db.file_for_url(&file_uri)?;
    let content = state
        .open_file_texts
        .get(&file_uri)
        .map(|text| text.to_string())
        .or_else(|| db.file_content(file).map(str::to_owned))?;

    let path = file_uri.to_file_path().ok()?;

    let config = state.configs_registry.config_for_file(&path).unwrap_or_default();
    let virtual_file = FileLongId::Virtual(VirtualFile {
        parent: None,
        name: file.file_name(db),
        content: SmolStrId::from(db, content.as_str()),
        code_mappings: Arc::from([]),
        kind: file.kind(db),
        original_item_removed: false,
    })
    .intern(db);

    let mut diagnostics = DiagnosticsBuilder::default();
    let node = match file.kind(db) {
        FileKind::Module => {
            Parser::parse_file(db, &mut diagnostics, virtual_file, &content).as_syntax_node()
        }
        FileKind::Expr => {
            Parser::parse_file_expr(db, &mut diagnostics, virtual_file, &content).as_syntax_node()
        }
        FileKind::StatementList => {
            Parser::parse_file_statement_list(db, &mut diagnostics, virtual_file, &content)
                .as_syntax_node()
        }
    };

    if diagnostics.build().check_error_free().is_err() {
        error!("formatting failed: cannot properly parse '{file_uri}' exist");
        return None;
    }

    let new_text = get_formatted_file(db, &node, config.fmt);

    let Ok(old_line_count) =
        content.chars().filter(|ch| *ch == '\n').count().saturating_add(1).try_into()
    else {
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::Arc;

    use lsp_types::{
        ClientCapabilities, DocumentFormattingParams, TextDocumentIdentifier, Url,
        WorkDoneProgressParams,
    };
    use tempfile::tempdir;

    use super::format_document;
    use crate::server::connection::ClientSender;
    use crate::state::State;

    #[test]
    fn formatting_uses_live_buffer_when_db_file_content_is_stale() {
        let workspace = tempdir().expect("failed to create tempdir");
        let file_path = workspace.path().join("stale.cairo");
        fs::write(&file_path, "fn main() { let old = 1; }\n")
            .expect("failed to write on-disk file");

        let mut state = State::new(
            ClientSender::black_hole(),
            ClientCapabilities::default(),
            workspace.path().to_path_buf(),
        );
        let uri = Url::from_file_path(&file_path).expect("failed to build file url");
        let live_text: Arc<str> = "fn main() { let live = 2; }\n".into();
        state.open_files.insert(uri.clone());
        state.open_file_texts.insert(uri.clone(), live_text.clone());

        let edits = format_document(
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier { uri },
                options: Default::default(),
                work_done_progress_params: WorkDoneProgressParams::default(),
            },
            state.snapshot(),
        )
        .expect("formatting should succeed");

        assert_eq!(edits.len(), 1);
        assert!(
            edits[0].new_text.contains("live = 2"),
            "formatter should use live buffer text, got: {:?}",
            edits[0].new_text
        );
        assert!(
            !edits[0].new_text.contains("old = 1"),
            "formatter should not read stale DB text, got: {:?}",
            edits[0].new_text
        );
    }
}
