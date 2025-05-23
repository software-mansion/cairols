use anyhow::anyhow;
use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::ids::FileLongId;
use cairo_lang_utils::LookupIntern;
use itertools::Itertools;
use lsp_server::ErrorCode;
use lsp_types::{
    ClientCapabilities, DocumentChangeOperation, DocumentChanges, Location, OneOf,
    OptionalVersionedTextDocumentIdentifier, RenameFile, RenameParams, ResourceOp,
    TextDocumentEdit, TextEdit, Url, WorkspaceEdit,
};
use std::collections::HashMap;

use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::defs::{SymbolDef, SymbolSearch};
use crate::lang::lsp::{LsProtoGroup, ToCairo};
use crate::lsp::capabilities::client::ClientCapabilitiesExt;
use crate::lsp::result::{LSPError, LSPResult};

// TODO(#381): handle crates separately (manifest needs to be changed too).
pub fn rename(
    params: RenameParams,
    db: &AnalysisDatabase,
    client_capabilities: &ClientCapabilities,
) -> LSPResult<Option<WorkspaceEdit>> {
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

        // Declaration is used here because rename without changing the declaration would break the compilation
        // e.g. when renaming trait usage - we also rename the trait
        SymbolSearch::find_declaration(db, &identifier).map(|search| search.def)
    };
    let Some(symbol) = symbol() else {
        return Ok(None);
    };
    if let SymbolDef::ExprInlineMacro(_) = symbol {
        return Err(LSPError::new(
            anyhow!("Renaming inline macros is not supported"),
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

    let resource_op = if let SymbolDef::Module(module_def) = symbol {
        match module_def.module_id() {
            ModuleId::CrateRoot(_) => {
                return Err(LSPError::new(
                    anyhow!("Rename for crates is not yet supported"),
                    ErrorCode::RequestFailed,
                ));
            }
            ModuleId::Submodule(submodule_id) => {
                if db.is_submodule_inline(submodule_id) {
                    None
                } else {
                    resource_op_for_non_inline_submodule(
                        db,
                        module_def.module_id(),
                        client_capabilities,
                        &new_name,
                    )?
                }
            }
        }
    } else {
        None
    };

    let workspace_edit = if client_capabilities.workspace_edit_rename_resource_support() {
        WorkspaceEdit {
            changes: None,
            document_changes: Some(merge_into_document_changes(changes, resource_op)),
            change_annotations: None,
        }
    } else {
        WorkspaceEdit { changes: Some(changes), document_changes: None, change_annotations: None }
    };

    Ok(Some(workspace_edit))
}

fn resource_op_for_non_inline_submodule(
    db: &AnalysisDatabase,
    module_id: ModuleId,
    client_capabilities: &ClientCapabilities,
    new_name: &str,
) -> LSPResult<Option<ResourceOp>> {
    if !client_capabilities.workspace_edit_rename_resource_support() {
        return Err(LSPError::new(
            anyhow!(
                "Renaming this module requires renaming files \
                 which is not supported by this editor."
            ),
            ErrorCode::RequestFailed,
        ));
    }

    let file_id = db.module_main_file(module_id).map_err(|_| {
        LSPError::new(anyhow!("Module main file not found"), ErrorCode::RequestFailed)
    })?;

    let path = match file_id.lookup_intern(db) {
        FileLongId::OnDisk(path) => path,
        FileLongId::Virtual(_) | FileLongId::External(_) => {
            return Err(LSPError::new(
                anyhow!("Cannot rename virtual files"),
                ErrorCode::RequestFailed,
            ));
        }
    };

    let ops = if path.exists() {
        let old_uri = Url::from_file_path(&path).unwrap();
        let new_uri =
            Url::from_file_path(path.with_file_name(format!("{new_name}.cairo"))).unwrap();

        Some(ResourceOp::Rename(RenameFile {
            old_uri,
            new_uri,
            options: None,
            annotation_id: None,
        }))
    } else {
        None
    };

    Ok(ops)
}

fn merge_into_document_changes(
    changes: HashMap<Url, Vec<TextEdit>>,
    resource_op: Option<ResourceOp>,
) -> DocumentChanges {
    let text_document_edits: Vec<_> = changes
        .into_iter()
        .map(|(uri, text_edits)| TextDocumentEdit {
            text_document: OptionalVersionedTextDocumentIdentifier { uri, version: None },
            edits: text_edits.into_iter().map(OneOf::Left).collect(),
        })
        .map(DocumentChangeOperation::Edit)
        .collect();

    let document_change_operations = resource_op
        .into_iter()
        .map(DocumentChangeOperation::Op)
        .chain(text_document_edits)
        .collect();

    DocumentChanges::Operations(document_change_operations)
}
