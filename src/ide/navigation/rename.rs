use std::collections::HashMap;

use anyhow::anyhow;
use itertools::Itertools;
use lsp_server::ErrorCode;
use lsp_types::{Location, RenameParams, TextEdit, WorkspaceEdit};

use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::defs::SymbolDef;
use crate::lang::lsp::{LsProtoGroup, ToCairo};
use crate::lsp::result::{LSPError, LSPResult};

pub fn rename(params: RenameParams, db: &AnalysisDatabase) -> LSPResult<Option<WorkspaceEdit>> {
    let new_name = params.new_name;

    // Copied from https://github.com/starkware-libs/cairo/blob/cefadd5ea60d9f0790d71ae14c97d48aa93bd5bf/crates/cairo-lang-parser/src/lexer.rs#L197.
    let is_valid_ident = new_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');

    if !is_valid_ident {
        return Err(LSPError::new(
            anyhow!("`{new_name}` is not a valid identifier"),
            ErrorCode::RequestFailed,
        ));
    }

    let symbol = || {
        let file = db.file_for_url(&params.text_document_position.text_document.uri)?;
        let position = params.text_document_position.position.to_cairo();
        let identifier = db.find_identifier_at_position(file, position)?;

        SymbolDef::find(db, &identifier)
    };
    let Some(symbol) = symbol() else {
        return Ok(None);
    };

    // TODO: handle non-inline modules and crates separately (files need to be renamed too).
    if matches!(symbol, SymbolDef::Module(_)) {
        return Err(LSPError::new(
            anyhow!("Rename for crate/modules is not yet supported"),
            ErrorCode::RequestFailed,
        ));
    }

    let locations: Vec<_> = symbol
        .usages(db)
        .include_declaration(true)
        .locations()
        .unique()
        .filter_map(|loc| db.lsp_location(loc))
        .collect();

    let changes: HashMap<_, Vec<TextEdit>> =
        locations.into_iter().fold(HashMap::new(), |mut acc, Location { uri, range }| {
            acc.entry(uri).or_default().push(TextEdit { range, new_text: new_name.clone() });
            acc
        });

    Ok(Some(WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    }))
}
