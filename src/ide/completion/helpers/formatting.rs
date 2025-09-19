use cairo_lang_defs::ids::{TopLevelLanguageElementId, TraitId};
use cairo_lang_semantic::Signature;
use cairo_lang_semantic::items::modifiers::get_relevant_modifier;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};

use crate::ide::format::types::format_type;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};

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
    let mut s = String::new();

    // Start with `fn` and a space, but omit the function name (label already contains it).
    s.push_str("fn(");

    let self_type = trait_id.map(|id| id.full_path(db));

    let mut first = true;
    for param in &signature.params {
        if !first {
            s.push_str(", ");
        }
        first = false;

        let modifier = get_relevant_modifier(&param.mutability);
        if !modifier.is_empty() {
            s.push_str(modifier);
            s.push(' ');
        }

        s.push_str(&param.name.to_string(db));
        s.push_str(": ");
        let mut ty_text = format_type(db, param.ty, &importables);

        if let Some(self_type) = &self_type {
            ty_text = replace_self_type(&ty_text, self_type)
        }

        s.push_str(&ty_text);
    }

    s.push_str(") -> ");

    let mut ret_text = format_type(db, signature.return_type, &importables);
    if let Some(self_type) = &self_type {
        ret_text = replace_self_type(&ret_text, self_type)
    }
    s.push_str(&ret_text);

    if !signature.panicable {
        s.push_str(" nopanic");
    }

    s
}

pub fn replace_self_type(input: &str, self_type: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let self_type_len = self_type.len();
    let bytes = input.as_bytes();

    while i < input.len() {
        // Detect the start of the expected type followed by turbofish
        if i + self_type_len + 2 < bytes.len()
            && &input[i..i + self_type_len] == self_type
            && &input[i + self_type_len..i + self_type_len + 2] == "::"
        {
            // Ensure it's followed by < indicating turbofish
            let generics_start = i + self_type_len + 2;
            if input.as_bytes().get(generics_start) == Some(&b'<') {
                // Append 'Self' instead of the original type and turbofish
                result.push_str("Self");
                i = skip_generics(input, generics_start);
                continue;
            }
        }

        // Append the current character to the result and continue
        result.push(input.as_bytes()[i] as char);
        i += 1;
    }

    result
}

// Skips the generics part and returns new position just after closing '>'
fn skip_generics(input: &str, start: usize) -> usize {
    let mut nesting = 0;
    let mut j = start;
    let bytes = input.as_bytes();

    while j < input.len() {
        match bytes[j] {
            b'<' => nesting += 1,
            b'>' => {
                nesting -= 1;
                if nesting == 0 {
                    return j + 1;
                }
            }
            _ => (),
        }
        j += 1;
    }

    j
}
