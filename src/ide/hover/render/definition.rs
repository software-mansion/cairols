use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::ImportableId;
use cairo_lang_defs::plugin::InlineMacroExprPlugin;
use cairo_lang_doc::db::DocGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_semantic::expr::inference::InferenceId;
use cairo_lang_semantic::items::functions::GenericFunctionId;
use cairo_lang_semantic::resolve::{ResolvedConcreteItem, ResolverData};
use cairo_lang_semantic::substitution::SemanticRewriter;
use cairo_lang_syntax::node::ast::{
    FunctionDeclaration, GenericParam, OptionWrappedGenericParamList, TerminalIdentifier,
};
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use itertools::Itertools;

use crate::ide::markdown::{RULE, fenced_code_block};
use crate::ide::ty::InferredValue;
use crate::lang::db::AnalysisDatabase;
use crate::lang::defs::{ResolvedItem, SymbolDef, SymbolSearch};

/// Get declaration and documentation "definition" of an item referred by the given identifier.
pub fn definition(
    db: &AnalysisDatabase,
    identifier: &TerminalIdentifier,
    file_id: FileId,
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<String> {
    let search = SymbolSearch::find_definition(db, identifier)?;

    let md = match &search.def {
        SymbolDef::Item(item) => {
            let mut md = String::new();
            md += &fenced_code_block(&item.definition_path(db));
            md += &fenced_code_block(
                &concrete_signature(db, search.resolved_item, search.resolver_data, importables)
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
            md += &fenced_code_block(&variant.enum_item().signature(db));

            if let Some(doc) = db.get_item_documentation(variant.variant_id().into()) {
                md += RULE;
                md += &doc;
            }
            md
        }
    };

    Some(md)
}

fn concrete_signature(
    db: &AnalysisDatabase,
    resolved_item: ResolvedItem,
    resolver_data: Option<ResolverData>,
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<String> {
    let resolver_data = resolver_data?;

    match resolved_item {
        ResolvedItem::Concrete(ResolvedConcreteItem::Function(concrete_func)) => {
            let mut inference_data =
                resolver_data.inference_data.clone_with_inference_id(db, InferenceId::NoContext);
            let mut inference = inference_data.inference(db);
            let _ = inference.solve();

            let concrete_func = concrete_func.get_concrete(db);

            let generics = match concrete_func.generic_function {
                GenericFunctionId::Extern(func) => {
                    get_generics(func.stable_ptr(db).lookup(db).declaration(db), db)
                }
                GenericFunctionId::Free(func) => {
                    get_generics(func.stable_ptr(db).lookup(db).declaration(db), db)
                }
                GenericFunctionId::Impl(impl_id) => {
                    get_generics(impl_id.function.stable_ptr(db).lookup(db).declaration(db), db)
                }
            };

            if generics.is_empty() {
                return None;
            }

            let generic_args_concrete = concrete_func
                .generic_args
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
        _ => None,
    }
}

fn get_generics(declaration: FunctionDeclaration, db: &AnalysisDatabase) -> Vec<GenericParam> {
    match declaration.generic_params(db) {
        OptionWrappedGenericParamList::Empty(_) => vec![],
        OptionWrappedGenericParamList::WrappedGenericParamList(list) => {
            list.generic_params(db).elements(db).collect_vec()
        }
    }
}
