use cairo_lang_semantic::Signature;
use itertools::Itertools;

use crate::lang::db::AnalysisDatabase;

/// Creates an LSP snippet for the function with the given name the signature.
///
/// Example: for a function with a signature
/// ```cairo
/// fn xyz(a: u8, b: ByteArray) -> felt252 {}
/// ```
/// returns a string "xyz({$1:a}, {$2:b})".
pub fn snippet_for_function_call(
    db: &AnalysisDatabase,
    function_name: &str,
    signature: &Signature,
) -> String {
    let params_snippet = signature
        .params
        .iter()
        .map(|param| param.name.to_string(db))
        .filter(|name| name != "self")
        .enumerate()
        .map(|(index, name)| format!("${{{}:{}}}", index + 1, name))
        .join(", ");
    format!("{function_name}({params_snippet})")
}
