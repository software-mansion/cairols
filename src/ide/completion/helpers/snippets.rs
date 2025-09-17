use cairo_lang_semantic::Signature;
use itertools::Itertools;

/// Creates an LSP snippet for the function with the given name the signature.
///
/// Example: for a function with a signature
/// ```cairo
/// fn xyz(a: u8, b: ByteArray) -> felt252 {}
/// ```
/// returns a string "xyz({$1:a}, {$2:b})".
pub fn snippet_for_function_call(function_name: &str, signature: &Signature) -> String {
    let params_snippet = signature
        .params
        .iter()
        .filter(|param| param.name != "self")
        .enumerate()
        .map(|(index, param)| format!("${{{}:{}}}", index + 1, param.name))
        .join(", ");
    format!("{function_name}({params_snippet})")
}
