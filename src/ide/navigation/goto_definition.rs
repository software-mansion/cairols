use std::ops::Not;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::ast::TerminalIdentifier;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::defs::{SymbolDef, SymbolSearch};
use crate::lang::lsp::{LsProtoGroup, ToCairo};

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
    let locations: OrderedHashSet<_> =
        resultants.into_iter().filter_map(|node| goto(db, node)).collect();
    let mut locations: Vec<_> = locations.into_iter().collect();

    match locations.len() {
        0 => None,
        1 => Some(GotoDefinitionResponse::Scalar(locations.pop().unwrap())),
        _ => Some(GotoDefinitionResponse::Array(locations)),
    }
}

fn goto<'db>(db: &'db AnalysisDatabase, syntax_node: SyntaxNode<'db>) -> Option<Location> {
    let identifier =
        syntax_node.ancestors_with_self(db).find_map(|node| TerminalIdentifier::cast(db, node))?;
    let symbol = SymbolSearch::find_definition(db, &identifier)?.def;

    let og_location = (
        identifier.as_syntax_node().stable_ptr(db).file_id(db),
        identifier.as_syntax_node().span_without_trivia(db),
    );

    #[allow(unused_doc_comments)]
    /// Try looking for declaration if we were on the definition.
    /// It is done to ensure better UX when finding references of impl items.
    /// For details, refer to [`SymbolSearch::find_declaration`].
    let symbol = if Some(og_location) == symbol.definition_location(db) {
        SymbolSearch::find_declaration(db, &identifier)?.def
    } else {
        symbol
    };

    let (found_file, span) = try_special_case_non_inline_module(db, &symbol)
        .map_or_else(|| symbol.definition_originating_location(db), Some)?;

    db.lsp_location((found_file, span))
}

// In the case of a non-inline module redirect to a module file instead of a definition node.
fn try_special_case_non_inline_module<'db>(
    db: &'db AnalysisDatabase,
    symbol: &SymbolDef<'db>,
) -> Option<(FileId<'db>, TextSpan)> {
    if let SymbolDef::Module(module_def) = symbol {
        let module_id = module_def.module_id();
        match module_id {
            ModuleId::CrateRoot(_) | ModuleId::MacroCall { id: _, generated_file_id: _ } => None,
            ModuleId::Submodule(submodule_id) => db
                .is_submodule_inline(submodule_id)
                .not()
                .then(|| {
                    let file = db.module_main_file(module_def.module_id()).ok()?;

                    match file.long(db) {
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
