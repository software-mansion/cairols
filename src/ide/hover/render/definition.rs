use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::plugin::InlineMacroExprPlugin;
use cairo_lang_doc::db::DocGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_syntax::node::ast::{
    GenericParam, OptionWrappedGenericParamList, TerminalIdentifier,
};
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use lsp_types::Hover;

use crate::ide::hover::markdown_contents;
use crate::ide::markdown::{RULE, fenced_code_block};
use crate::ide::ty::InferredValue;
use crate::lang::db::AnalysisDatabase;
use crate::lang::defs::{ResolvedItem, SymbolDef};
use crate::lang::lsp::ToLsp;
use cairo_lang_defs::ids::ImportableId;
use cairo_lang_semantic::items::functions::GenericFunctionId;
use cairo_lang_semantic::resolve::{ResolvedConcreteItem, ResolverData};
use cairo_lang_semantic::substitution::SemanticRewriter;
use cairo_lang_semantic::{ConcreteTypeId, TypeLongId};
use cairo_lang_utils::LookupIntern;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;

/// Get declaration and documentation "definition" of an item referred by the given identifier.
pub fn definition(
    db: &AnalysisDatabase,
    identifier: &TerminalIdentifier,
    file_id: FileId,
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<Hover> {
    let (resolved_item, resolver_data, symbol) =
        SymbolDef::find_with_resolved_item(db, identifier)?;

    let md = match &symbol {
        SymbolDef::Item(item) => {
            let mut md = String::new();
            md += &fenced_code_block(&item.definition_path(db));
            md += &fenced_code_block(
                &concrete_signature(db, resolved_item, resolver_data, importables)
                    .map(|signature| item.signature_with_text(db, &signature))
                    .unwrap_or_else(|| item.signature(db)),
            );
            if let Some(doc) = item.documentation(db) {
                md += RULE;
                md += &doc;
            }
            md
        }

        SymbolDef::Module(module) => {
            let mut md = String::new();
            md += &fenced_code_block(&module.definition_path());
            md += &fenced_code_block(&module.signature(db));
            if let Some(doc) = module.documentation(db) {
                md += RULE;
                md += &doc;
            }
            md
        }

        SymbolDef::Variable(var) => fenced_code_block(&var.signature(db, importables)?),
        SymbolDef::ExprInlineMacro(macro_name) => {
            let crate_id = db.file_modules(file_id).ok()?.first()?.owning_crate(db);

            let mut md = fenced_code_block(macro_name);

            if let Some(doc) = db
                .crate_inline_macro_plugins(crate_id)
                .get(macro_name.as_str())
                .map(|&id| db.lookup_intern_inline_macro_plugin(id))?
                .documentation()
            {
                md += RULE;
                md += &doc;
            }
            md
        }
        SymbolDef::Member(member) => {
            let mut md = String::new();

            // Signature is the signature of the struct, so it makes sense that the definition
            // path is too.
            md += &fenced_code_block(&member.struct_item().definition_path(db));
            md += &fenced_code_block(&member.struct_item().signature(db));

            if let Some(doc) = db.get_item_documentation(member.member_id().into()) {
                md += RULE;
                md += &doc;
            }
            md
        }
        SymbolDef::Variant(variant) => {
            let mut md = String::new();

            // Signature is the signature of the enum, so it makes sense that the definition
            // path is too.
            md += &fenced_code_block(&variant.enum_item().definition_path(db));
            md += &fenced_code_block(
                &concrete_signature(db, resolved_item, resolver_data, importables)
                    .map(|signature| variant.enum_item().signature_with_text(db, &signature))
                    .unwrap_or_else(|| variant.enum_item().signature(db)),
            );

            if let Some(doc) = db.get_item_documentation(variant.variant_id().into()) {
                md += RULE;
                md += &doc;
            }
            md
        }
    };

    Some(Hover {
        contents: markdown_contents(md),
        range: identifier
            .as_syntax_node()
            .span_without_trivia(db)
            .position_in_file(db, file_id)
            .map(|p| p.to_lsp()),
    })
}

fn concrete_signature(
    db: &AnalysisDatabase,
    resolved_item: ResolvedItem,
    resolver_data: Option<ResolverData>,
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<String> {
    let mut resolver_data = resolver_data?;

    let (generics, concrete_generic_args) = match resolved_item {
        ResolvedItem::Concrete(ResolvedConcreteItem::Function(concrete_func)) => {
            let concrete_func = concrete_func.get_concrete(db);

            let declaration = match concrete_func.generic_function {
                GenericFunctionId::Extern(func) => func.stable_ptr(db).lookup(db).declaration(db),
                GenericFunctionId::Free(func) => func.stable_ptr(db).lookup(db).declaration(db),
                GenericFunctionId::Impl(impl_id) => {
                    impl_id.function.stable_ptr(db).lookup(db).declaration(db)
                }
            };

            (
                generic_param_list_to_vec(declaration.generic_params(db), db),
                concrete_func.generic_args,
            )
        }
        ResolvedItem::Concrete(ResolvedConcreteItem::Variant(concrete_variant)) => {
            let concrete_enum = concrete_variant.concrete_enum_id.lookup_intern(db);

            (
                generic_param_list_to_vec(
                    concrete_variant
                        .concrete_enum_id
                        .enum_id(db)
                        .stable_ptr(db)
                        .lookup(db)
                        .generic_params(db),
                    db,
                ),
                concrete_enum.generic_args,
            )
        }
        ResolvedItem::Concrete(ResolvedConcreteItem::Trait(concrete_trait))
        | ResolvedItem::Concrete(ResolvedConcreteItem::SelfTrait(concrete_trait)) => (
            generic_param_list_to_vec(
                concrete_trait.trait_id(db).stable_ptr(db).lookup(db).generic_params(db),
                db,
            ),
            concrete_trait.generic_args(db),
        ),

        ResolvedItem::Concrete(ResolvedConcreteItem::Type(ty)) => {
            if let TypeLongId::Concrete(ConcreteTypeId::Struct(struct_id)) = ty.lookup_intern(db) {
                let struct_id = struct_id.lookup_intern(db);

                (
                    generic_param_list_to_vec(
                        struct_id.struct_id.stable_ptr(db).lookup(db).generic_params(db),
                        db,
                    ),
                    struct_id.generic_args,
                )
            } else {
                return None;
            }
        }
        _ => return None,
    };

    if generics.is_empty() {
        return None;
    }

    let mut inference = resolver_data.inference_data.inference(db);
    let _ = inference.solve();

    let generic_args_concrete = concrete_generic_args
        .into_iter()
        .map(|arg| inference.rewrite(arg))
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    if generic_args_concrete.iter().any(|arg| !arg.is_fully_concrete(db)) {
        return None;
    }

    let mut result = generics.into_iter().zip(generic_args_concrete).fold(
        "\n\n".to_string(),
        |mut acc, (generic, concrete)| {
            let left = generic.as_syntax_node().get_text_without_trivia(db);

            let right = InferredValue::try_from_generic_arg_id(concrete)
                .map(|value| value.format(db, importables))
                .unwrap_or_else(|| concrete.format(db));

            acc.push_str(&left);
            acc.push_str(" = ");
            acc.push_str(&right);
            acc.push('\n');
            acc
        },
    );
    result.push('\n');

    Some(result)
}

fn generic_param_list_to_vec(
    option_wrapped_generic_param_list: OptionWrappedGenericParamList,
    db: &AnalysisDatabase,
) -> Vec<GenericParam> {
    match option_wrapped_generic_param_list {
        OptionWrappedGenericParamList::Empty(_) => vec![],
        OptionWrappedGenericParamList::WrappedGenericParamList(list) => {
            list.generic_params(db).elements(db)
        }
    }
}
