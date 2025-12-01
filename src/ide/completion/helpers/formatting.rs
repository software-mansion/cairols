use cairo_lang_defs::ids::{EnumId, NamedLanguageElementLongId, TraitId, VariantId};
use cairo_lang_doc::db::DocGroup;
use cairo_lang_doc::documentable_item::DocumentableItemId;
use cairo_lang_semantic::items::modifiers::get_relevant_modifier;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_semantic::{Mutability, Signature, TypeId};
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode};
use itertools::Itertools;

use crate::ide::format::types::format_type;
use crate::lang::db::AnalysisDatabase;

/// Generates a concise one-line signature without resolver generics, and a name prefixed by `fn`.
/// Example output: `fn(param_a: felt252, param_b: Array<B>) -> Something<B, C> nopanic`
pub fn generate_abbreviated_signature(
    db: &AnalysisDatabase,
    signature: &Signature,
    trait_id: Option<TraitId>,
) -> String {
    let importables = db
        .visible_importables_from_module(
            db.find_module_containing_node(signature.stable_ptr.lookup(db).as_syntax_node())
                .unwrap(),
        )
        .unwrap();
    // Build a concise one-line signature without resolver generics, prefixed by `fn`.
    let mut s = String::from("fn(");

    let formatted_params = signature
        .params
        .iter()
        .map(|param| {
            let mut s = String::new();
            let modifier = get_relevant_modifier(&param.mutability);

            if !modifier.is_empty() && param.mutability != Mutability::Mutable {
                s.push_str(modifier);
                s.push(' ');
            }

            s.push_str(&param.name.to_string(db));
            s.push_str(": ");
            let ty_text = format_type(db, param.ty, &importables, trait_id);

            s.push_str(&ty_text);
            s
        })
        .join(", ");

    s.push_str(&formatted_params);
    s.push_str(") -> ");

    let ret_text = format_type(db, signature.return_type, &importables, trait_id);

    s.push_str(&ret_text);

    if !signature.panicable {
        s.push_str(" nopanic");
    }

    s
}

/// Formats a type with a given context node, fetching the importables if possible and shortening paths.
/// For example, `felt252` (instead of `core::felt252` for a simpler display)
pub fn format_type_in_node_context(
    db: &AnalysisDatabase,
    context_node: SyntaxNode,
    type_id: &TypeId,
) -> String {
    let importables = if let Some(module_id) = db.find_module_containing_node(context_node)
        && let Some(importables) = db.visible_importables_from_module(module_id)
    {
        importables
    } else {
        Default::default()
    };

    format_type(db, *type_id, &importables, None)
}

/// Formats an enum variant and its type along with the enums name.
/// For example, `MyEnum::Variant1: felt252`
pub fn format_enum_variant<'db>(
    db: &'db AnalysisDatabase,
    enum_id: &'db EnumId,
    variant_id: &'db VariantId,
) -> Option<String> {
    db.get_item_signature(DocumentableItemId::Variant(*variant_id)).map(|variant_signature| {
        format!("{}::{}", enum_id.long(db).name(db).to_string(db), variant_signature)
    })
}
