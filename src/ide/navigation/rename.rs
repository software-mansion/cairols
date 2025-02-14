use std::collections::HashMap;

use lsp_types::{Location, RenameParams, TextEdit, WorkspaceEdit};

use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::defs::SymbolDef;
use crate::lang::lsp::{LsProtoGroup, ToCairo};

// TODO: handle non-inline modules and crates separately (files need to be renamed too).
pub fn rename(params: RenameParams, db: &AnalysisDatabase) -> Option<WorkspaceEdit> {
    let new_name = params.new_name;

    let is_new_name_valid = new_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    if !is_new_name_valid {
        return None;
    }

    let file = db.file_for_url(&params.text_document_position.text_document.uri)?;
    let position = params.text_document_position.position.to_cairo();
    let identifier = db.find_identifier_at_position(file, position)?;

    let symbol = SymbolDef::find(db, &identifier)?;

    let locations = symbol.locations(db, true);

    let changes: HashMap<_, Vec<TextEdit>> =
        locations.into_iter().fold(HashMap::new(), |mut acc, Location { uri, range }| {
            acc.entry(uri).or_default().push(TextEdit { range, new_text: new_name.clone() });
            acc
        });

    Some(WorkspaceEdit { changes: Some(changes), document_changes: None, change_annotations: None })
}
