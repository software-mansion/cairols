use std::ops::Not;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_doc::db::DocGroup;
use cairo_lang_filesystem::ids::{
    FileId, FileKind, FileLongId, SmolStrId, SpanInFile, VirtualFile,
};
use cairo_lang_filesystem::span::{TextOffset, TextPosition, TextSpan, TextWidth};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::diagnostic::{NotFoundItemType, SemanticDiagnostics};
use cairo_lang_semantic::expr::inference::InferenceId;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_semantic::resolve::{ResolutionContext, Resolver};
use cairo_lang_syntax::node::ast::{PathSegment, TerminalIdentifier};
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode, ast};
use cairo_lang_utils::Intern;
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use cairo_language_common::CommonGroup;
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};

use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::defs::{NonMacroModuleId, ResolvedItem, SymbolDef, SymbolSearch};
use crate::lang::lsp::{LsProtoGroup, ToCairo};

/// Get the definition location of a symbol at a given text document position.
pub fn goto_definition(
    params: GotoDefinitionParams,
    db: &AnalysisDatabase,
) -> Option<GotoDefinitionResponse> {
    let file = db.file_for_url(&params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position.to_cairo();

    // Try to apply identifier correction before resultants.
    let Some(node) = db.find_identifier_at_position(file, position) else {
        return try_goto_doc_link_def(db, file, position).map(GotoDefinitionResponse::Scalar);
    };

    let resultants = db.get_node_resultants(node.as_syntax_node())?;
    let locations: OrderedHashSet<_> =
        resultants.iter().filter_map(|node| goto(db, *node)).collect();
    let mut locations: Vec<_> = locations.into_iter().collect();

    match locations.len() {
        0 => None,
        1 => Some(GotoDefinitionResponse::Scalar(locations.pop().unwrap())),
        _ => Some(GotoDefinitionResponse::Array(locations)),
    }
}

fn try_goto_doc_link_def(
    db: &AnalysisDatabase,
    file: FileId<'_>,
    position: TextPosition,
) -> Option<Location> {
    let cursor_offset = position.offset_in_file(db, file)?;
    let doc_token = db.find_syntax_node_at_position(file, position)?;
    let doc_token_span = doc_token.span_without_trivia(db);
    if !doc_token_span.contains(TextSpan::cursor(cursor_offset)) {
        return None;
    }

    // Get the doc-comment links using compiler-provided spans.
    let links = db.get_embedded_markdown_links(doc_token);
    let cursor_span = TextSpan::cursor(cursor_offset);
    let link = links.into_iter().find(|link| link.link_span.contains(cursor_span))?;

    let dest_text = link.dest_text?.to_string();

    let expr_path = parse_doc_link_path(db, dest_text.as_str())?;

    // We want to resolve the path up to the segment containing the cursor
    // Default to the whole path.
    let segments =
        doc_link_segments_for_offset(db, &expr_path, &dest_text, link.dest_span?, cursor_offset)
            .unwrap_or_else(|| expr_path.segments(db).elements_vec(db));

    // Run resolver to retrieve the definition node.
    let module_id = db.find_module_containing_node(doc_token)?;
    let mut resolver = Resolver::new(db, module_id, InferenceId::NoContext);
    let resolved_item = resolver
        .resolve_generic_path_with_args(
            &mut SemanticDiagnostics::new(module_id),
            segments,
            NotFoundItemType::Identifier,
            ResolutionContext::Default,
        )
        .ok()?;

    let resolved_node = ResolvedItem::Generic(resolved_item).definition_node(db)?;
    goto(db, resolved_node)
}

// Create a virtual file and run the parser on it to get a path to the target symbol.
// This is needed because we can't parse doc comments directly.
fn parse_doc_link_path<'db>(
    db: &'db AnalysisDatabase,
    dest_text: &str,
) -> Option<ast::ExprPath<'db>> {
    let virtual_file = FileLongId::Virtual(VirtualFile {
        parent: None,
        name: SmolStrId::from(db, "doc-link"),
        content: SmolStrId::from(db, dest_text),
        code_mappings: Default::default(),
        kind: FileKind::Expr,
        original_item_removed: false,
    })
    .intern(db);

    let expr = match db.file_expr_syntax(virtual_file) {
        Ok(expr) => expr,
        Err(_) => return None,
    };
    let ast::Expr::Path(expr_path) = expr else {
        return None;
    };
    Some(expr_path)
}

// Returns path segments up to the one containing `cursor_offset`, or the last segment before it.
fn doc_link_segments_for_offset<'db>(
    db: &'db AnalysisDatabase,
    dest_expr_path: &ast::ExprPath<'db>,
    dest_text: &str,
    dest_span: TextSpan,
    cursor_offset: TextOffset,
) -> Option<Vec<PathSegment<'db>>> {
    let absolute_cursor = TextSpan::cursor(cursor_offset);
    if !dest_span.contains(absolute_cursor) {
        return None;
    }
    let relative_cursor_offset = (cursor_offset - dest_span.start).as_u32();
    if dest_text.as_bytes().get(relative_cursor_offset as usize) == Some(&b':') {
        return None;
    }

    let relative_cursor =
        TextOffset::START.add_width(TextWidth::new_for_testing(relative_cursor_offset));
    let relative_cursor_span = TextSpan::cursor(relative_cursor);
    let segments = dest_expr_path.segments(db).elements_vec(db);

    let mut left_hand_segments = Vec::new();
    for segment in segments.into_iter() {
        let span = segment.as_syntax_node().span(db);
        left_hand_segments.push(segment);
        if span.contains(relative_cursor_span) {
            break;
        }
    }
    (!left_hand_segments.is_empty()).then_some(left_hand_segments)
}

fn goto<'db>(db: &'db AnalysisDatabase, syntax_node: SyntaxNode<'db>) -> Option<Location> {
    let identifier =
        syntax_node.ancestors_with_self(db).find_map(|node| TerminalIdentifier::cast(db, node))?;
    let symbol = SymbolSearch::find_definition(db, &identifier)?.def;

    let og_location = SpanInFile {
        file_id: identifier.as_syntax_node().stable_ptr(db).file_id(db),
        span: identifier.as_syntax_node().span_without_trivia(db),
    };

    #[allow(unused_doc_comments)]
    /// Try looking for declaration if we were on the definition.
    /// It is done to ensure better UX when finding references of impl items.
    /// For details, refer to [`SymbolSearch::find_declaration`].
    let symbol = if Some(og_location) == symbol.definition_location(db) {
        SymbolSearch::find_declaration(db, &identifier)?.def
    } else {
        symbol
    };

    let span_in_file = try_special_case_non_inline_module(db, &symbol)
        .map_or_else(|| symbol.definition_originating_location(db), Some)?;

    db.lsp_location(span_in_file)
}

// In the case of a non-inline module redirect to a module file instead of a definition node.
fn try_special_case_non_inline_module<'db>(
    db: &'db AnalysisDatabase,
    symbol: &SymbolDef<'db>,
) -> Option<SpanInFile<'db>> {
    if let SymbolDef::Module(module_def) = symbol {
        let module_id = module_def.non_macro_module_id();
        match module_id {
            NonMacroModuleId::CrateRoot(_) => None,
            NonMacroModuleId::Submodule(submodule_id) => db
                .is_submodule_inline(submodule_id)
                .not()
                .then(|| {
                    let file = db.module_main_file(module_def.module_id()).ok()?;

                    match file.long(db) {
                        FileLongId::OnDisk(_) => Some(SpanInFile {
                            file_id: file,
                            span: db.file_syntax(file).ok()?.span(db),
                        }),
                        FileLongId::Virtual(_) | FileLongId::External(_) => None,
                    }
                })
                .flatten(),
        }
    } else {
        None
    }
}
