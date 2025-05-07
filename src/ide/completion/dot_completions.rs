use cairo_lang_defs::ids::{
    FileIndex, LanguageElementId, ModuleFileId, NamedLanguageElementId, TopLevelLanguageElementId,
    TraitFunctionId,
};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::types::peel_snapshots;
use cairo_lang_semantic::{ConcreteTypeId, TypeLongId};
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode, ast};
use lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat};
use tracing::debug;

use crate::ide::ty::format_type;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::importer::new_import_edit;
use crate::lang::methods::find_methods_for_type;
use itertools::Itertools;

pub fn dot_completions(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
    expr: ast::ExprBinary,
) -> Option<Vec<CompletionItem>> {
    // Get a resolver in the current context.
    let function_with_body = ctx.lookup_item_id?.function_with_body()?;
    let module_file_id = function_with_body.module_file_id(db);
    let resolver = ctx.resolver(db);
    let importables =
        db.visible_importables_from_module(ModuleFileId(ctx.module_id, FileIndex(0)))?;

    // Extract lhs node.
    let node = expr.lhs(db);
    let stable_ptr = node.stable_ptr(db).untyped();
    // Get its semantic model.
    let expr_id = db.lookup_expr_by_ptr(function_with_body, node.stable_ptr(db)).ok()?;
    let semantic_expr = db.expr_semantic(function_with_body, expr_id);
    // Get the type.
    let ty = semantic_expr.ty();
    if ty.is_missing(db) {
        debug!("type is missing");
        return None;
    }

    // Find relevant methods for type.
    let relevant_methods = find_methods_for_type(db, resolver, ty, stable_ptr);

    let mut completions = Vec::new();
    for trait_function in relevant_methods {
        let Some(completion) = completion_for_method(db, ctx, module_file_id, trait_function)
        else {
            continue;
        };
        completions.push(completion);
    }

    // Find members of the type.
    let (_, long_ty) = peel_snapshots(db, ty);
    if let TypeLongId::Concrete(ConcreteTypeId::Struct(concrete_struct_id)) = long_ty {
        db.concrete_struct_members(concrete_struct_id).ok()?.iter().for_each(|(name, member)| {
            let completion = CompletionItem {
                label: name.to_string(),
                detail: Some(format_type(db, member.ty, &importables)),
                kind: Some(CompletionItemKind::FIELD),
                ..CompletionItem::default()
            };
            completions.push(completion);
        });
    }
    Some(completions)
}

/// Returns a completion item for a method.
fn completion_for_method(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext,
    module_file_id: ModuleFileId,
    trait_function: TraitFunctionId,
) -> Option<CompletionItem> {
    let trait_id = trait_function.trait_id(db);
    let name = trait_function.name(db);
    db.trait_function_signature(trait_function).ok()?;

    // TODO(spapini): Add signature.
    let detail = trait_id.full_path(db);
    let mut additional_text_edits = vec![];

    // If the trait is not in scope, add a use statement.

    if let Some(trait_path) = db.visible_traits_from_module(module_file_id)?.get(&trait_id) {
        // Path is single element if item is already in scope.
        let is_not_in_scope = trait_path.split("::").collect_vec().len() != 1;

        let import = is_not_in_scope.then(|| new_import_edit(db, ctx, trait_path)).flatten();

        additional_text_edits.extend(import);
    }

    let completion = CompletionItem {
        label: format!("{}()", name),
        insert_text: Some(format!("{}($1)", name)),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        detail: Some(detail),
        kind: Some(CompletionItemKind::METHOD),
        additional_text_edits: Some(additional_text_edits),
        ..CompletionItem::default()
    };
    Some(completion)
}
