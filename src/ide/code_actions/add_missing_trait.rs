use std::collections::HashMap;

use cairo_lang_defs::ids::{ModuleId, NamedLanguageElementId};
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::TextOffset;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode, ast};
use cairo_lang_utils::Upcast;
use lsp_types::{CodeAction, CodeActionKind, Range, TextEdit, Url, WorkspaceEdit};
use tracing::debug;

use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use crate::lang::lsp::{LsProtoGroup, ToLsp};
use crate::lang::methods::find_methods_for_type;
use crate::lang::syntax::SyntaxNodeExt;

/// Create a Quick Fix code action to add a missing trait given a `CannotCallMethod` diagnostic.
pub fn add_missing_trait(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
    uri: Url,
) -> Option<Vec<CodeAction>> {
    let file_id = db.file_for_url(&uri)?;

    missing_traits_actions(db, file_id, ctx, uri)
}

/// Returns a list of code actions to add missing traits to the current module, or `None` if the
/// type is missing.
fn missing_traits_actions(
    db: &AnalysisDatabase,
    file_id: FileId,
    ctx: &AnalysisContext<'_>,
    uri: Url,
) -> Option<Vec<CodeAction>> {
    let syntax_db = db.upcast();
    let function_with_body = ctx.lookup_item_id?.function_with_body()?;
    let resolver = ctx.resolver(db);

    let expr_node = ctx.node.ancestor_of_type::<ast::ExprBinary>(db)?.lhs(db);
    let stable_ptr = expr_node.stable_ptr().untyped();
    // Get its semantic model.
    let expr_id = db.lookup_expr_by_ptr(function_with_body, expr_node.stable_ptr()).ok()?;
    let semantic_expr = db.expr_semantic(function_with_body, expr_id);
    // Get the type.
    let ty = semantic_expr.ty();
    if ty.is_missing(db) {
        debug!("type is missing");
        return None;
    }

    let module_start_offset = if let Some(ModuleId::Submodule(submodule_id)) =
        db.find_module_containing_node(&ctx.node)
    {
        let module_def_ast = submodule_id.stable_ptr(db.upcast()).lookup(syntax_db);
        if let ast::MaybeModuleBody::Some(body) = module_def_ast.body(syntax_db) {
            body.items(syntax_db).as_syntax_node().span_start_without_trivia(syntax_db)
        } else {
            TextOffset::default()
        }
    } else {
        TextOffset::default()
    };
    let module_start_position =
        module_start_offset.position_in_file(db.upcast(), file_id).unwrap().to_lsp();
    let relevant_methods = find_methods_for_type(db, resolver, ty, stable_ptr);
    let current_module = db.find_module_file_containing_node(&ctx.node)?;
    let module_visible_traits = db.visible_traits_from_module(current_module)?;
    let unknown_method_name = ctx.node.get_text(db.upcast());
    let mut code_actions = vec![];

    for method in relevant_methods {
        let method_name = method.name(db.upcast());
        if method_name == unknown_method_name {
            if let Some(trait_path) = module_visible_traits.get(&method.trait_id(db.upcast())) {
                code_actions.push(CodeAction {
                    title: format!("Import {}", trait_path),
                    kind: Some(CodeActionKind::QUICKFIX),
                    edit: Some(WorkspaceEdit {
                        changes: Some(HashMap::from_iter([(
                            uri.clone(),
                            vec![TextEdit {
                                range: Range::new(module_start_position, module_start_position),
                                new_text: format!("use {};\n", trait_path),
                            }],
                        )])),
                        document_changes: None,
                        change_annotations: None,
                    }),
                    diagnostics: None,
                    ..Default::default()
                });
            }
        }
    }
    Some(code_actions)
}
