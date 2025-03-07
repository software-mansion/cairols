use cairo_lang_defs::ids::{
    LanguageElementId, ModuleFileId, ModuleId, NamedLanguageElementId, TopLevelLanguageElementId,
    TraitFunctionId,
};
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::TextOffset;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::types::peel_snapshots;
use cairo_lang_semantic::{ConcreteTypeId, TypeLongId};
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode, ast};
use cairo_lang_utils::Upcast;
use lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat, Position, Range, TextEdit};
use tracing::debug;

use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use crate::lang::lsp::ToLsp;
use crate::lang::methods::find_methods_for_type;
use itertools::Itertools;

pub fn dot_completions(
    db: &AnalysisDatabase,
    file_id: FileId,
    ctx: &AnalysisContext<'_>,
    expr: ast::ExprBinary,
) -> Option<Vec<CompletionItem>> {
    let syntax_db = db.upcast();
    // Get a resolver in the current context.
    let function_with_body = ctx.lookup_item_id?.function_with_body()?;
    let module_file_id = function_with_body.module_file_id(db.upcast());
    let resolver = ctx.resolver(db);

    // Extract lhs node.
    let node = expr.lhs(syntax_db);
    let stable_ptr = node.stable_ptr().untyped();
    // Get its semantic model.
    let expr_id = db.lookup_expr_by_ptr(function_with_body, node.stable_ptr()).ok()?;
    let semantic_expr = db.expr_semantic(function_with_body, expr_id);
    // Get the type.
    let ty = semantic_expr.ty();
    if ty.is_missing(db) {
        debug!("type is missing");
        return None;
    }

    // Find relevant methods for type.
    let offset = if let Some(ModuleId::Submodule(submodule_id)) =
        db.find_module_containing_node(&expr.as_syntax_node())
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
    let position = offset.position_in_file(db.upcast(), file_id).unwrap().to_lsp();
    let relevant_methods = find_methods_for_type(db, resolver, ty, stable_ptr);

    let mut completions = Vec::new();
    for trait_function in relevant_methods {
        let Some(completion) = completion_for_method(db, module_file_id, trait_function, position)
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
                detail: Some(member.ty.format(db.upcast())),
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
    module_file_id: ModuleFileId,
    trait_function: TraitFunctionId,
    position: Position,
) -> Option<CompletionItem> {
    let trait_id = trait_function.trait_id(db.upcast());
    let name = trait_function.name(db.upcast());
    db.trait_function_signature(trait_function).ok()?;

    // TODO(spapini): Add signature.
    let detail = trait_id.full_path(db.upcast());
    let mut additional_text_edits = vec![];

    // If the trait is not in scope, add a use statement.

    if let Some(trait_path) = db.visible_traits_from_module(module_file_id)?.get(&trait_id) {
        // Path is single element if item is already in scope.
        if trait_path.split("::").collect_vec().len() != 1 {
            additional_text_edits.push(TextEdit {
                range: Range::new(position, position),
                new_text: format!("use {};\n", trait_path),
            });
        }
    }

    let completion = CompletionItem {
        label: format!("{}()", name),
        insert_text: Some(format!("{}($0)", name)),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        detail: Some(detail),
        kind: Some(CompletionItemKind::METHOD),
        additional_text_edits: Some(additional_text_edits),
        ..CompletionItem::default()
    };
    Some(completion)
}
