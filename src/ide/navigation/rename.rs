use std::collections::HashMap;

use anyhow::anyhow;
use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_semantic::keyword::SELF_TYPE_KW;
use cairo_lang_syntax::node::ast::TerminalIdentifier;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedSyntaxNode};
use itertools::Itertools;
use lsp_server::ErrorCode;
use lsp_types::{
    ClientCapabilities, DocumentChangeOperation, DocumentChanges, Location, OneOf,
    OptionalVersionedTextDocumentIdentifier, RenameFile, RenameParams, ResourceOp,
    TextDocumentEdit, TextEdit, Url, WorkspaceEdit,
};

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::defs::{NonMacroModuleId, SymbolDef, SymbolSearch};
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

    let Some(file) = db.file_for_url(&params.text_document_position.text_document.uri) else {
        return Ok(None);
    };
    let position = params.text_document_position.position.to_cairo();
    let Some(identifier) = db.find_identifier_at_position(file, position) else {
        return Ok(None);
    };

    if identifier.text(db) == SELF_TYPE_KW {
        return Err(LSPError::new(
            anyhow!(
                "Renaming via `{SELF_TYPE_KW}` reference is not supported. Rename the item directly instead."
            ),
            ErrorCode::RequestFailed,
        ));
    }

    let position = params.text_document_position.position.to_cairo();
    let Some(identifier) = db.find_identifier_at_position(file, position) else { return Ok(None) };

    let Some(resultants) = db.get_node_resultants(identifier.as_syntax_node()) else {
        return Ok(None);
    };

    let symbols: Vec<_> =
        resultants.iter().filter_map(|node| declaration_from_resultant(db, *node)).collect();

    let mut resource_ops = vec![];
    // Handle special cases.
    for symbol in &symbols {
        if let SymbolDef::PluginInlineMacro(_) = &symbol.def {
            return Err(LSPError::new(
                anyhow!("Renaming builtin inline macros is not supported"),
                ErrorCode::RequestFailed,
            ));
        }

        if let SymbolDef::Module(module_def) = &symbol.def {
            match module_def.non_macro_module_id() {
                NonMacroModuleId::CrateRoot(_) => {
                    return Err(LSPError::new(
                        anyhow!("Renaming crates is not yet supported"),
                        ErrorCode::RequestFailed,
                    ));
                }
                NonMacroModuleId::Submodule(submodule_id) => {
                    if !db.is_submodule_inline(submodule_id) {
                        let res_op = resource_op_for_non_inline_submodule(
                            db,
                            module_def.module_id(),
                            client_capabilities,
                            &new_name,
                        )?;
                        resource_ops.extend(res_op);
                    }
                }
            }
        }
    }

    let locations = symbols
        .into_iter()
        .flat_map(|symbol| find_usages(db, symbol))
        .filter_map(|loc| db.lsp_location(loc))
        .collect::<Vec<_>>();

    let changes: HashMap<_, Vec<TextEdit>> =
        locations.into_iter().fold(HashMap::new(), |mut acc, Location { uri, range }| {
            acc.entry(uri).or_default().push(TextEdit { range, new_text: new_name.clone() });
            acc
        });

    let workspace_edit = if client_capabilities.workspace_edit_rename_resource_support() {
        WorkspaceEdit {
            changes: None,
            document_changes: Some(merge_into_document_changes(changes, resource_ops)),
            change_annotations: None,
        }
    } else {
        WorkspaceEdit { changes: Some(changes), document_changes: None, change_annotations: None }
    };

    Ok(Some(workspace_edit))
}

fn declaration_from_resultant<'db>(
    db: &'db AnalysisDatabase,
    resultant: SyntaxNode<'db>,
) -> Option<SymbolSearch<'db>> {
    let identifier =
        resultant.ancestors_with_self(db).find_map(|node| TerminalIdentifier::cast(db, node))?;
    // Declaration is used here because rename without changing the declaration would break the compilation,
    // e.g. when renaming trait usage - we also rename the trait
    SymbolSearch::find_declaration(db, &identifier)
}

fn find_usages<'db>(
    db: &'db AnalysisDatabase,
    symbol: SymbolSearch<'db>,
) -> Vec<(FileId<'db>, TextSpan)> {
    let symbol_name = Some(symbol.def.name(db));

    symbol
        .usages(db)
        .include_declaration(true)
        .originating_locations(db)
        .filter(|(file, span)| {
            db.find_syntax_node_at_offset(*file, span.start)
                // 1. Sanity check: `span` is a span of a terminal identifier without trivia.
                //    It should be the same as the span of a token at offset `span.start`.
                // 2. Filter out any symbol with a different name than the definition.
                //    Rationale: These symbols most likely are nodes mapped back to call site.
                //    If this is not the case, it means that the macro does something unusual and
                //    the user should rename the remaining cases themselves.
                .is_some_and(|node| node.span(db) == *span && node.text(db) == symbol_name)
        })
        .unique()
        .collect()
}

fn resource_op_for_non_inline_submodule<'db>(
    db: &'db AnalysisDatabase,
    module_id: ModuleId<'db>,
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

    let path = match file_id.long(db) {
        FileLongId::OnDisk(path) => path,
        FileLongId::Virtual(_) | FileLongId::External(_) => {
            return Err(LSPError::new(
                anyhow!("Cannot rename virtual files"),
                ErrorCode::RequestFailed,
            ));
        }
    };

    let ops = if path.exists() {
        let old_uri = Url::from_file_path(path).unwrap();
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
    resource_ops: Vec<ResourceOp>,
) -> DocumentChanges {
    let text_document_edits: Vec<_> = changes
        .into_iter()
        .map(|(uri, text_edits)| TextDocumentEdit {
            text_document: OptionalVersionedTextDocumentIdentifier { uri, version: None },
            edits: text_edits.into_iter().map(OneOf::Left).collect(),
        })
        .map(DocumentChangeOperation::Edit)
        .collect();

    let document_change_operations = resource_ops
        .into_iter()
        .map(DocumentChangeOperation::Op)
        .chain(text_document_edits)
        .collect();

    DocumentChanges::Operations(document_change_operations)
}
