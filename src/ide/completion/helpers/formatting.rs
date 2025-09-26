use cairo_lang_defs::ids::TraitId;
use cairo_lang_semantic::items::modifiers::get_relevant_modifier;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_semantic::{Mutability, Signature};
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use itertools::Itertools;

use crate::ide::format::types::format_type;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};

/// Generates a concise one-line signature without resolver generics, and a name prefixed by `fn`.
/// Example output: `fn(param_a: felt252, param_b: Array<B>) -> Something<B, C> nopanic`
pub fn generate_abbreviated_signature(
    db: &AnalysisDatabase,
    signature: &Signature,
    trait_id: Option<TraitId>,
) -> String {
    let importables = db
        .visible_importables_from_module(
            db.find_module_file_containing_node(signature.stable_ptr.lookup(db).as_syntax_node())
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
