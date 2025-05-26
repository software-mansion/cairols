use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::defs::{SymbolDef, SymbolSearch};
use crate::lang::lsp::{LsProtoGroup, ToCairo};
use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::ast::TerminalIdentifier;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::LookupIntern;
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};
use std::ops::Not;

/// Get the definition location of a symbol at a given text document position.
pub fn goto_definition(
    params: GotoDefinitionParams,
    db: &AnalysisDatabase,
) -> Option<GotoDefinitionResponse> {
    let file = db.file_for_url(&params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position.to_cairo();

    // Try to apply identifier correction before resultants.
    let node = db.find_identifier_at_position(file, position)?;

    let resultants = db.get_node_resultants(node.as_syntax_node())?;
    let mut locations: Vec<_> = resultants.into_iter().filter_map(|node| goto(db, node)).collect();

    match locations.len() {
        0 => None,
        1 => Some(GotoDefinitionResponse::Scalar(locations.pop().unwrap())),
        _ => Some(GotoDefinitionResponse::Array(locations)),
    }
}

fn goto(db: &AnalysisDatabase, syntax_node: SyntaxNode) -> Option<Location> {
    let identifier =
        syntax_node.ancestors_with_self(db).find_map(|node| TerminalIdentifier::cast(db, node))?;
    let symbol = SymbolSearch::find_definition(db, &identifier)?.def;

    let (found_file, span) = try_special_case_non_inline_module(db, &symbol)
        .map_or_else(|| symbol.definition_location(db), Some)?;

    db.lsp_location((found_file, span))
}

// In the case of a non-inline module redirect to a module file instead of a definition node.
fn try_special_case_non_inline_module(
    db: &AnalysisDatabase,
    symbol: &SymbolDef,
) -> Option<(FileId, TextSpan)> {
    if let SymbolDef::Module(module_def) = symbol {
        let module_id = module_def.module_id();
        match module_id {
            ModuleId::CrateRoot(_) => None,
            ModuleId::Submodule(submodule_id) => db
                .is_submodule_inline(submodule_id)
                .not()
                .then(|| {
                    let file = db.module_main_file(module_def.module_id()).ok()?;

                    match file.lookup_intern(db) {
                        FileLongId::OnDisk(_) => Some((file, db.file_syntax(file).ok()?.span(db))),
                        FileLongId::Virtual(_) | FileLongId::External(_) => None,
                    }
                })
                .flatten(),
        }
    } else {
        None
    }
}
