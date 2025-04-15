use std::{fmt::Display, iter::zip, ops::Not, path::PathBuf, sync::LazyLock};

use crate::support::{
    MockClient,
    cursor::peek_caret,
    cursors,
    diagnostics::{DiagnosticAndRelatedInfo, DiagnosticsWithUrl, get_related_diagnostic_code},
    fixture::Fixture,
    itertools::IteratorExtension,
    normalize::normalize_diagnostics,
    sandbox,
};
use cairo_lang_utils::iterators::zip_eq3;
use cairo_language_server::lsp::ext::ExpandMacro;
use itertools::Itertools;
use lsp_types::{Position, TextDocumentIdentifier, TextDocumentPositionParams};
use serde::Serialize;
use serde_json::json;

mod builtin;
mod fixtures;
mod procedural;

pub const SCARB_TEST_MACROS_PACKAGE_NAME: &str = "scarb_procedural_macros";

pub static SCARB_TEST_MACROS_PACKAGE: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(SCARB_TEST_MACROS_PACKAGE_NAME)
        .canonicalize()
        .expect("should be able to obtain an absolute path to `scarb_procedural_macros`")
});

pub const SCARB_TEST_MACROS_V2_PACKAGE_NAME: &str = "scarb_procedural_macros_v2";

pub static SCARB_TEST_MACROS_V2_PACKAGE: LazyLock<PathBuf> = LazyLock::new(|| {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(SCARB_TEST_MACROS_V2_PACKAGE_NAME)
        .canonicalize()
        .expect("should be able to obtain an absolute path to `scarb_procedural_macros_v2`")
});

// NOTE: This procedure is implemented as a macro to delegate
// the choice of the snapshot destination in the file system
// and avoid a possible mess with specifying those locations manually.
//
/// Tests a Cairo code snippet in the context of a [`MacroTest`].
/// Serializes the components of the obtained report to appropriate formats
/// (TOML for expansions and YAML for diagnostics) and tests them against the saved snapshots.
macro_rules! test_macro_expansion_and_diagnostics {
    (
        project = $project:ident,
        cwd = $cwd:expr,
        files { $($path:expr => $content:expr),* $(,)? }
    ) => {
        let files = vec![$(($path.to_string(), $content.to_string()),)*];

        let (report, ls) =
            <$project as $crate::macros::MacroTest>::test_many($cwd, files);

        ::insta::assert_snapshot!(report);

        drop(ls);
    };

    ($test_setup:ident, $code_with_cursors:expr) => {
        let (report, ls) =
            <$test_setup as $crate::macros::MacroTest>::test(::indoc::indoc!($code_with_cursors));
        ::insta::assert_snapshot!(report);
        drop(ls);  // Delaying the drop allows `insta` to generate the snapshot descriptions automatically.
    };
}

pub(crate) use test_macro_expansion_and_diagnostics;

/// Test report. Contains all diagnostics sent by the language server in the test project
/// and all macro expansions collected during the test.
#[derive(Debug, Serialize)]
pub struct Report {
    pub expansions: Option<ExpansionsReport>,
    pub mapped_diagnostics: Option<DiagnosticsReport>,
}

/// Macro expansion obtained by calling the LSP method
/// on a set of lines in the source code,
/// collectively represented as `analyzed_lines`.
#[derive(Debug, Serialize)]
pub struct ExpansionGroup {
    analyzed_lines: String,
    generated_code: String,
}

/// A helper structure for nice TOML representation of the expansions from [`Report`].
#[derive(Debug, Serialize)]
pub struct ExpansionsReport {
    pub expansions: Vec<ExpansionGroup>,
}

/// A helper structure for nice YAML representation of the diagnostics from [`Report`].
#[derive(Debug, Serialize)]
pub struct DiagnosticsReport {
    pub mapped_diagnostics: Vec<DiagnosticsWithUrl>,
}

impl Report {
    fn new(expansions: Vec<ExpansionGroup>, mapped_diagnostics: Vec<DiagnosticsWithUrl>) -> Self {
        let expansions = expansions.is_empty().not().then_some(ExpansionsReport { expansions });
        let mapped_diagnostics =
            mapped_diagnostics.is_empty().not().then_some(DiagnosticsReport { mapped_diagnostics });
        Self { mapped_diagnostics, expansions }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref expansions) = self.expansions {
            writeln!(
                f,
                "{}",
                toml::to_string_pretty(expansions)
                    .expect("ExpansionsReport should be serializable to TOML.")
            )?;
        }

        if let Some(ref diagnostics) = self.mapped_diagnostics {
            write!(
                f,
                "{}",
                serde_yaml::to_string(diagnostics)
                    .expect("DiagnosticsReport should be serializable to JSON.")
            )?;
        }

        Ok(())
    }
}

/// A complete setup to test features related to macros.
pub trait MacroTest {
    /// A [`Fixture`] of files making up a Scarb workspace. Contains all necessary files to
    /// perform the test (Scarb.toml, helper modules, proc macro crates etc.).
    fn fixture() -> Fixture;

    /// A configuration for the LSP client.
    fn workspace_configuration() -> serde_json::Value {
        json!({
            "cairo1": {
                "enableProcMacros": true,
                "traceMacroDiagnostics": true,
            }
        })
    }

    /// Performs a test on many Cairo files.
    ///
    /// The procedure consists of the following steps:
    /// 1. Collect and remove carets from `cairo_code_with_carets`.
    /// 2. Inject the obtained Cairo code into [`Self::fixture`].
    /// 3. Launch a language server in `cwd` directory.
    /// 4. Collect all the diagnostics.
    /// 5. Tests the given code for macro expansions according to the previously collected carets.
    /// 6. Report the results.
    fn test_many(
        cwd: &str,
        // A vector of pairs: (path to the snippet, snippet with carets).
        cairo_codes_with_carets: Vec<(String, String)>,
    ) -> (Report, MockClient) {
        let (paths, codes_with_carets): (Vec<_>, Vec<_>) =
            cairo_codes_with_carets.into_iter().unzip();

        let (codes, cursors): (Vec<_>, Vec<_>) =
            codes_with_carets.iter().map(|code| cursors(code)).unzip();

        let mut fixture = Self::fixture();
        for (path, code) in zip(&paths, &codes) {
            fixture.add_file(path, code);
        }

        let mut ls = sandbox! {
            fixture = fixture;
            cwd = cwd;
            workspace_configuration = Self::workspace_configuration();
        };

        let diagnostics = ls.open_and_wait_for_diagnostics_generation(&paths[0]);

        let mapped_diagnostics = normalize_diagnostics(&ls, diagnostics)
            .into_iter()
            .filter_map(|(original_url, normalized_url, diagnostics)| {
                if diagnostics.is_empty() {
                    return None;
                }

                Some(DiagnosticsWithUrl {
                    url: normalized_url,
                    diagnostics: diagnostics
                        .into_iter()
                        .map(|diag| DiagnosticAndRelatedInfo {
                            related_code: get_related_diagnostic_code(&ls, &diag, &original_url),
                            diagnostic: diag,
                        })
                        .collect(),
                })
            })
            .collect();

        let expansions = zip_eq3(paths, codes, cursors)
            .flat_map(|(path, code, cursors)| {
                cursors
                    .carets()
                    .into_iter()
                    .map(|position| {
                        let expansion = get_expansion_at(&mut ls, &path, position) + "\n";
                        (expansion, position)
                    })
                    .into_ordered_group_map()
                    .into_iter()
                    .map(|(generated_code, positions)| {
                        let analyzed_lines = positions
                            .into_iter()
                            .map(|position| peek_caret(&code, position))
                            .join("");
                        ExpansionGroup { generated_code, analyzed_lines }
                    })
                    .collect::<Vec<_>>() // Needs an intermediate collection because of the flat_map.
            })
            .collect();

        let report = Report::new(expansions, mapped_diagnostics);

        (report, ls)
    }
}

/// Requests the language server to expand a macro located at `position` in a file with `source_path`
/// and returns the received response.
fn get_expansion_at(ls: &mut MockClient, source_path: &str, position: Position) -> String {
    let macro_expansion = ls.send_request::<ExpandMacro>(TextDocumentPositionParams {
        position,
        text_document: TextDocumentIdentifier { uri: ls.doc_id(source_path).uri },
    });

    macro_expansion.unwrap_or_else(|| String::from("No expansion information."))
}
