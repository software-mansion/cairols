use std::{fmt::Display, ops::Not};

use crate::support::{
    MockClient,
    cursor::peek_caret,
    cursors,
    fixture::Fixture,
    normalize::{DiagnosticsWithUrl, normalize_diagnostics},
    sandbox,
};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_language_server::lsp::ext::ExpandMacro;
use itertools::Itertools;
#[cfg(doc)]
use itertools::Itertools;
use lsp_types::{Position, TextDocumentIdentifier, TextDocumentPositionParams};
use serde::Serialize;
use serde_json::json;

mod fixtures;
mod procedural;

/// Test report. Contains all diagnostics sent by the language server in the test project
/// and all macro expansions collected during the test.
#[derive(Debug, Serialize)]
pub struct Report {
    expansions: Option<Vec<ExpansionGroup>>,
    mapped_diagnostics: Option<Vec<MappedDiagnostic>>,
}

impl Report {
    fn new(expansions: Vec<ExpansionGroup>, mapped_diagnostics: Vec<MappedDiagnostic>) -> Self {
        let expansions = expansions.is_empty().not().then_some(expansions);
        let mapped_diagnostics = mapped_diagnostics.is_empty().not().then_some(mapped_diagnostics);
        Self { mapped_diagnostics, expansions }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            toml::to_string_pretty(&self).expect("Report should be serializable to TOML.")
        )
    }
}

/// Macro expansion obtained by calling the LSP method
/// on a set of lines in the source code,
/// collectively represented as `analyzed_lines`.
#[derive(Debug, Serialize)]
pub struct ExpansionGroup {
    analyzed_lines: String,
    generated_code: String,
}

/// Representation of the [`lsp_types::Diagnostic`] mapped to the source code with additional URL,
/// suitable for TOML serialization.
#[derive(Debug, Serialize)]
pub struct MappedDiagnostic {
    url: String,
    range: String,
    message: String,
    severity: Option<String>,
}

impl MappedDiagnostic {
    pub fn new(diagnostic: lsp_types::Diagnostic, url: String) -> Self {
        let severity = diagnostic.severity.map(|severity| format!("{severity:?}"));

        let lsp_types::Range { start, end } = diagnostic.range;
        let range = format!("{}:{}-{}:{}", start.line, start.character, end.line, end.character);

        Self { url, range, message: diagnostic.message, severity }
    }
}

/// An environment for testing macros.
pub trait MacroTestFixture {
    /// Name of the main package where `lib.cairo` with the test snippet will be created during the test.
    const TEST_PACKAGE: &str;

    /// A [`Fixture`] of files making up a Scarb workspace. Contains all necessary files to
    /// perform the test (Scarb.toml, helper modules, proc macro crates etc.).
    fn fixture() -> Fixture;

    /// Performs a test.
    ///
    /// The procedure consists of the following steps:
    /// 1. Collect and remove carets from `cairo_code_with_carets`.
    /// 2. Inject the obtained Cairo code into [`Self::fixture`] at [`Self::TEST_PACKAGE`]/src/lib.cairo.
    /// 3. Launch a language server in [`Self::TEST_PACKAGE`] directory.
    /// 4. Collect all the diagnostics.
    /// 5. Tests the given code for macro expansions according to the previously collected carets.
    /// 6. Report the results.
    fn test_macro_expansion_and_diagnostics(cairo_code_with_carets: &str) -> Report {
        let (cairo_code, cursors) = cursors(cairo_code_with_carets);

        let test_package = Self::TEST_PACKAGE;
        let lib_cairo = &format!("{test_package}/src/lib.cairo");

        let mut ls = sandbox! {
            fixture = Self::fixture();
            files {
                lib_cairo => &cairo_code,
            }
            cwd = test_package;
            workspace_configuration = json!({
                "cairo1": {
                    "enableProcMacros": true,
                    "traceMacroDiagnostics": true,
                }
            });
        };

        let diagnostics = ls.open_and_wait_for_diagnostics_generation(lib_cairo);
        let diagnostics = normalize_diagnostics(&ls, diagnostics)
            .into_iter()
            .flat_map(|DiagnosticsWithUrl { url, diagnostics }| {
                diagnostics
                    .into_iter()
                    .map(|diagnostic| MappedDiagnostic::new(diagnostic, url.clone()))
                    .collect_vec()
            })
            .collect_vec();

        let expansions = cursors.carets().into_iter().map(|position| {
            let expansion = get_expansion_at(&mut ls, lib_cairo, position) + "\n";
            (expansion, position)
        });
        let expansions = into_ordered_group_map(expansions)
            .into_iter()
            .map(|(generated_code, positions)| {
                let analyzed_lines = positions
                    .into_iter()
                    .map(|position| peek_caret(&cairo_code, position))
                    .join("");
                ExpansionGroup { generated_code, analyzed_lines }
            })
            .collect_vec();

        Report::new(expansions, diagnostics)
    }
}

/// Requests the language server to expand a macro located at `position` in a file with `source_path`
/// and returns the received response.
fn get_expansion_at(ls: &mut MockClient, source_path: &str, position: Position) -> String {
    let macro_expansion = ls.send_request::<ExpandMacro>(TextDocumentPositionParams {
        position,
        text_document: TextDocumentIdentifier { uri: ls.doc_id(source_path).uri },
    });

    macro_expansion.unwrap_or_else(|| String::from("No expansion information.\n"))
}

/// Groups the iterator identically to [`Itertools::into_group_map`]
/// but produces an [`OrderedHashMap`] instead.
fn into_ordered_group_map<U: std::hash::Hash + Eq, V>(
    iter: impl IntoIterator<Item = (U, V)>,
) -> OrderedHashMap<U, Vec<V>> {
    let mut groups = OrderedHashMap::<U, Vec<V>>::default();

    for (key, value) in iter {
        groups.entry(key).or_default().push(value);
    }

    groups
}
