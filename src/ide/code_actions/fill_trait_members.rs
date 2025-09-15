use std::collections::HashMap;

use cairo_lang_defs::ids::{
    ImportableId, NamedLanguageElementId, TraitConstantId, TraitFunctionId,
};
use cairo_lang_semantic::diagnostic::{NotFoundItemType, SemanticDiagnostics};
use cairo_lang_semantic::items::trt::TraitSemantic;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_semantic::resolve::ResolvedConcreteItem;
use cairo_lang_semantic::substitution::GenericSubstitution;
use cairo_lang_semantic::{ConcreteTraitId, GenericArgumentId, GenericParam, Parameter};
use cairo_lang_syntax::node::ast::{ImplItem, ItemImpl, MaybeImplBody};
use cairo_lang_syntax::node::{Token, TypedSyntaxNode};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use itertools::{Itertools, chain};
use lsp_types::{CodeAction, CodeActionKind, CodeActionParams, Range, TextEdit, WorkspaceEdit};

use crate::ide::format::types::{InferredValue, format_type};
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use crate::lang::lsp::ToLsp;

/// Generates a completion adding all trait members that have not yet been specified.
/// Functions are added with empty bodies, consts with placeholder values.
pub fn fill_trait_members<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    params: &CodeActionParams,
) -> Option<CodeAction> {
    let file = db.find_module_file_containing_node(ctx.node)?.file_id(db).ok()?;
    let importables = db.visible_importables_from_module(ctx.module_file_id)?;

    let item_impl = ctx.node.ancestor_of_type::<ItemImpl>(db)?;

    // Do not complete `impl`s without braces.
    let MaybeImplBody::Some(impl_body) = item_impl.body(db) else {
        return None;
    };

    let specified_impl_items = impl_body.items(db);

    let already_implemented_item_names = specified_impl_items
        .elements(db)
        .filter_map(|item| match item {
            ImplItem::Function(item) => Some(item.declaration(db).name(db).token(db).text(db)),
            ImplItem::Type(item) => Some(item.name(db).token(db).text(db)),
            ImplItem::Constant(item) => Some(item.name(db).token(db).text(db)),
            _ => None, // No other items can appear in trait impl.
        })
        .collect_vec();

    let concrete_trait_id = find_concrete_trait_id(db, ctx, &item_impl)?;
    let trait_id = concrete_trait_id.trait_id(db);

    let mut trait_constants = db.trait_constants(trait_id).ok()?;
    let mut trait_types = db.trait_types(trait_id).ok()?;
    let mut trait_functions = db.trait_functions(trait_id).ok()?;

    trait_constants.retain(|key, _| !already_implemented_item_names.contains(&&**key));
    trait_types.retain(|key, _| !already_implemented_item_names.contains(&&**key));
    trait_functions.retain(|key, _| !already_implemented_item_names.contains(&&**key));

    if trait_constants.is_empty() && trait_types.is_empty() && trait_functions.is_empty() {
        return None;
    }

    let trait_generics = db.trait_generic_params(trait_id).ok()?;
    let specified_generics = concrete_trait_id.generic_args(db);
    let substitution = GenericSubstitution::new(&trait_generics, &specified_generics);

    let code = chain!(
        trait_types.values().map(|id| format!("type {} = ();", id.name(db))),
        trait_constants.values().filter_map(|&id| constant_code(
            db,
            id,
            &substitution,
            &importables
        )),
        trait_functions.values().filter_map(|&id| function_code(
            db,
            id,
            &substitution,
            &importables
        ))
    )
    .join("\n\n");

    let impl_body_end_before_right_brace =
        specified_impl_items.as_syntax_node().span_end_without_trivia(db);

    let code_insert_position =
        impl_body_end_before_right_brace.position_in_file(db, file)?.to_lsp();

    let edit_start = code_insert_position;
    let edit_end = code_insert_position;

    let mut changes = HashMap::new();
    let url = params.text_document.uri.clone();
    let change = TextEdit { range: Range::new(edit_start, edit_end), new_text: code };

    changes.insert(url, vec![change]);

    let edit = WorkspaceEdit::new(changes);

    Some(CodeAction {
        title: String::from("Implement missing members"),
        kind: Some(CodeActionKind::QUICKFIX),
        edit: Some(edit),
        ..Default::default()
    })
}

/// Obtains semantic model of [`ItemImpl`] and returns a [`ConcreteTraitId`] it refers to.
fn find_concrete_trait_id<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    item_impl: &ItemImpl<'db>,
) -> Option<ConcreteTraitId<'db>> {
    let mut resolver = ctx.resolver(db);

    let mut diagnostics = SemanticDiagnostics::default();

    match resolver.resolve_concrete_path(
        &mut diagnostics,
        item_impl.trait_path(db).segments(db).elements(db).collect_vec(),
        NotFoundItemType::Trait,
    ) {
        Ok(ResolvedConcreteItem::Trait(id)) => Some(id),
        _ => None,
    }
}

/// Generates a declaration of a [`TraitConstantId`] containing its name, type,
/// and a placeholder value.
/// Generics are substituted with concrete types according to a given [`SubstitutionRewriter`]
fn constant_code<'db>(
    db: &'db AnalysisDatabase,
    id: TraitConstantId<'db>,
    substitution: &GenericSubstitution<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
) -> Option<String> {
    let name = id.name(db);
    let ty = format_type(
        db,
        substitution.substitute(db, db.trait_constant_type(id).ok()?).ok()?,
        importables,
    );

    Some(format!("const {name}: {ty} = ();"))
}

/// Generates a declaration of a [`TraitFunctionId`] containing its signature with parameters,
/// panic indicator, implicits, and default implementation if such exists.
/// Generics are substituted with concrete types according to a given [`SubstitutionRewriter`].
/// Returns None if the function has a default implementation.
fn function_code<'db>(
    db: &'db AnalysisDatabase,
    id: TraitFunctionId<'db>,
    substitution: &GenericSubstitution<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
) -> Option<String> {
    // Do not complete functions that have default implementations.
    if db.trait_function_body(id).ok()?.is_some() {
        return None;
    }

    let signature = db.trait_function_signature(id).ok()?;

    let generic_parameters = db.trait_function_generic_params(id).ok()?;
    let generic_parameters_bracket = if generic_parameters.is_empty() {
        String::new()
    } else {
        let formatted_parameters = generic_parameters
            .into_iter()
            .map(|parameter| generic_parameter_code(db, parameter, substitution, importables))
            .collect::<Option<Vec<_>>>()?
            .join(", ");

        format!("<{formatted_parameters}>")
    };

    let parameters = signature
        .params
        .iter()
        .map(|parameter| function_parameter(db, parameter, substitution, importables))
        .collect::<Option<Vec<_>>>()?
        .join(", ");

    let name = id.name(db);
    let title = Some(format!("fn {name}{generic_parameters_bracket}({parameters})"));

    let return_type = substitution.substitute(db, signature.return_type).ok()?;
    let return_type = if return_type.is_unit(db) {
        None
    } else {
        Some(format!("-> {}", format_type(db, return_type, importables)))
    };

    let implicits = match &signature.implicits[..] {
        [] => None,
        types => Some(format!(
            "implicits({})",
            types.iter().map(|ty| format_type(db, *ty, importables)).join(", ")
        )),
    };

    let nopanic = if !signature.panicable { Some(String::from("nopanic")) } else { None };

    let body: Option<String> = Some(String::from("{}"));

    Some([title, return_type, implicits, nopanic, body].into_iter().flatten().join(" "))
}

/// Formats [`GenericParam`] to be used in function's declaration.
/// Substitutes generic parameters with proper concrete types given in impl,
/// keeps freestanding (not belonging to the trait) generics unchanged.
fn generic_parameter_code<'db>(
    db: &'db AnalysisDatabase,
    parameter: GenericParam<'db>,
    substitution: &GenericSubstitution<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
) -> Option<String> {
    match parameter {
        GenericParam::Const(param) => Some(format!(
            "const {}: {}",
            param.id.format(db),
            format_type(db, param.ty, importables)
        )),

        GenericParam::Impl(param) => {
            let concrete_trait = param.concrete_trait.ok()?;
            let trait_name = concrete_trait.name(db);
            let trait_generic_arguments = concrete_trait.generic_args(db);

            let generic_arguments_bracket = if trait_generic_arguments.is_empty() {
                String::new()
            } else {
                let formatted_arguments = trait_generic_arguments
                    .into_iter()
                    .map(|argument| generic_argument_code(db, argument, substitution, importables))
                    .collect::<Option<Vec<_>>>()?
                    .join(", ");

                format!("<{formatted_arguments}>")
            };

            Some(param.id.name(db).map_or_else(
                // concrete trait used only as a constraint
                || format!("+{trait_name}{generic_arguments_bracket}"),
                // concrete trait with explicit impl
                |name| format!("impl {name}: {trait_name}{generic_arguments_bracket}"),
            ))
        }

        GenericParam::Type(ty) => Some(ty.id.format(db)),

        GenericParam::NegImpl(_) => None,
    }
}

/// Formats [`GenericArgumentId`] as it was used as a generic argument
/// nested in a generic parameter (e.g. `T` in `+Drop<T>`).
fn generic_argument_code<'db>(
    db: &'db AnalysisDatabase,
    argument: GenericArgumentId<'db>,
    substitution: &GenericSubstitution<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
) -> Option<String> {
    match argument {
        GenericArgumentId::Type(type_id) => {
            Some(format_type(db, substitution.substitute(db, type_id).ok()?, importables))
        }
        GenericArgumentId::Constant(const_value) => {
            Some(InferredValue::Constant(const_value).format(db, importables))
        }
        // Trait constraint shouldn't appear as a generic argument
        GenericArgumentId::Impl(_) => None,
        // Negative constraints are allowed only in impl statements
        GenericArgumentId::NegImpl(_) => None,
    }
}

/// Generates [`Parameter`] declaration containing its name,
/// type and optionally a `ref` or `mut` indicator.
/// Generics are substituted with concrete types according to a given [`SubstitutionRewriter`]
fn function_parameter<'db>(
    db: &'db AnalysisDatabase,
    parameter: &Parameter<'db>,
    substitution: &GenericSubstitution<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
) -> Option<String> {
    let prefix = match parameter.mutability {
        cairo_lang_semantic::Mutability::Immutable => "",
        cairo_lang_semantic::Mutability::Mutable => "mut ",
        cairo_lang_semantic::Mutability::Reference => "ref ",
    };

    let name = parameter.id.name(db);
    let ty = format_type(db, substitution.substitute(db, parameter.ty).ok()?, importables);

    Some(format!("{prefix}{name}: {ty}"))
}
