use cairo_lang_defs::ids::{NamedLanguageElementId, TraitFunctionId};
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::{
    FunctionWithBodySemantic, SemanticExprLookup,
};
use cairo_lang_semantic::items::imp::ImplSemantic;
use cairo_lang_semantic::items::structure::StructSemantic;
use cairo_lang_semantic::items::trt::TraitSemantic;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_semantic::types::peel_snapshots;
use cairo_lang_semantic::{ConcreteTypeId, TypeId, TypeLongId};
use cairo_lang_syntax::node::ast::Expr;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use itertools::chain;
use lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat};
use tracing::debug;

use crate::ide::completion::helpers::binary_expr::dot_rhs::dot_expr_rhs;
use crate::ide::completion::helpers::formatting::generate_abbreviated_signature;
use crate::ide::completion::helpers::snippets::snippet_for_function_call;
use crate::ide::completion::{CompletionItemOrderable, CompletionRelevance};
use crate::ide::format::types::format_type;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::importer::import_edit_for_trait_if_needed;
use crate::lang::methods::find_methods_for_type;
use crate::lang::text_matching::text_matches;

pub fn dot_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    was_node_corrected: bool,
) -> Vec<CompletionItemOrderable> {
    dot_completions_ex(db, ctx, was_node_corrected).unwrap_or_default()
}

fn dot_completions_ex<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    was_node_corrected: bool,
) -> Option<Vec<CompletionItemOrderable>> {
    let expr = dot_expr_rhs(db, &ctx.node, was_node_corrected)?;
    let rhs = match expr.rhs(db) {
        Expr::FunctionCall(function_call) => Some(function_call.path(db)),
        Expr::Path(path) => Some(path),
        _ => None,
    }
    .and_then(|rhs| Some(rhs.segments(db).elements(db).next()?.as_syntax_node()));

    // This way we ignore `my_struct.method(<caret>)` cases, but make sure to allow `my_struct.method(arg1, arg2, my_struct2.method<caret>())` cases.
    if let Some(rhs) = rhs
        && !rhs.is_descendant_or_self(db, &ctx.node.parent(db)?)
    {
        return None;
    }

    let typed = expr.rhs(db).as_syntax_node().get_text_without_trivia(db).to_string(db);
    // Get a resolver in the current context.
    let function_with_body = ctx.lookup_item_id?.function_with_body()?;
    let mut resolver = ctx.resolver(db);
    let importables = db.visible_importables_from_module(ctx.module_id)?;

    // Extract lhs node.
    let node = expr.lhs(db);
    let stable_ptr = node.stable_ptr(db).untyped();
    // Get its semantic model.
    let expr_id = db.lookup_expr_by_ptr(function_with_body, node.stable_ptr(db)).ok()?;
    let semantic_db: &dyn SemanticGroup = db;
    let semantic_expr = semantic_db.expr_semantic(function_with_body, expr_id);
    // Get the type.
    let ty = semantic_expr.ty();
    if ty.is_missing(db) {
        debug!("type is missing");
        return None;
    }

    let mut completions = Vec::new();

    let crate_id = db.find_module_containing_node(ctx.node)?.owning_crate(db);
    let types = deref_targets(db, crate_id, ty)?;

    for ty in types {
        // Find relevant methods for type.
        let relevant_methods = find_methods_for_type(db, &mut resolver, ty, stable_ptr)
            .into_iter()
            .filter(|method| text_matches(method.name(db).to_string(db), strip_parens(&typed)));

        for trait_function in relevant_methods {
            let Some(completion) = completion_for_method(db, ctx, trait_function) else {
                continue;
            };
            completions.push(completion);
        }

        // Find members of the type.
        let (_, long_ty) = peel_snapshots(db, ty);
        if let TypeLongId::Concrete(ConcreteTypeId::Struct(concrete_struct_id)) = long_ty {
            db.concrete_struct_members(concrete_struct_id).ok()?.iter().for_each(
                |(name, member)| {
                    let completion = CompletionItemOrderable {
                        item: CompletionItem {
                            label: name.to_string(db),
                            detail: Some(format_type(db, member.ty, &importables, None)),
                            kind: Some(CompletionItemKind::FIELD),
                            ..CompletionItem::default()
                        },
                        // We set the relevance to High as we want the members to be shown before the methods.
                        // The [`find_methods_for_type`] function takes current crate methods first, so they will be shown
                        // before the methods from other crates.
                        relevance: CompletionRelevance::High,
                    };
                    completions.push(completion);
                },
            );
        }
    }
    Some(completions)
}

fn deref_targets<'db>(
    db: &'db AnalysisDatabase,
    crate_id: CrateId<'db>,
    ty: TypeId<'db>,
) -> Option<Vec<TypeId<'db>>> {
    let deref_chain = db.deref_chain(ty, crate_id, true).ok()?;

    Some(chain!(Some(ty), deref_chain.derefs.iter().map(|info| info.target_ty)).collect())
}

/// Returns a completion item for a method.
fn completion_for_method<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    trait_function: TraitFunctionId<'db>,
) -> Option<CompletionItemOrderable> {
    let trait_id = trait_function.trait_id(db);
    let name = trait_function.name(db).to_string(db);
    let signature = db.trait_function_signature(trait_function).ok()?;

    let abbreviated_signature =
        generate_abbreviated_signature(db, signature, Some(trait_function.trait_id(db)));

    let mut additional_text_edits = vec![];

    // If the trait is not in scope, add a use statement.
    if let Some(edit) = import_edit_for_trait_if_needed(db, ctx, trait_id) {
        additional_text_edits.push(edit);
    }

    let completion = CompletionItemOrderable {
        item: CompletionItem {
            label: format!("{name}()"),
            insert_text: Some(snippet_for_function_call(db, &name, signature)),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some(abbreviated_signature),
            kind: Some(CompletionItemKind::METHOD),
            additional_text_edits: Some(additional_text_edits),
            ..CompletionItem::default()
        },
        // We set the relevance to medium as we want methods to be shown after the members of the struct.
        relevance: CompletionRelevance::Medium,
    };
    Some(completion)
}

// Strips the starting (left) parentheses and anything after from the input string.
fn strip_parens(input: &str) -> String {
    if let Some(index) = input.find('(') {
        String::from(&input[..index])
    } else {
        input.to_string()
    }
}
