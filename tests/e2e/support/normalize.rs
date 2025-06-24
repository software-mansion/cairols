use std::path::Path;
use std::str::FromStr;

use itertools::Itertools;
use lsp_types::{Diagnostic, DiagnosticRelatedInformation, Location, Url};
use regex::Regex;

use crate::support::fixture::Fixture;
use crate::support::scarb::scarb_registry_std_path;

/// Performs various normalization steps of the input data, to remove any runtime-specific artifacts
/// and make comparisons in test assertions deterministic.
pub fn normalize(fixture: impl AsRef<Fixture>, data: impl ToString) -> String {
    let fixture = fixture.as_ref();
    normalize_well_known_paths(fixture, normalize_paths(data.to_string()))
}

/// Replace all well-known paths/urls for a fixture with placeholders.
fn normalize_well_known_paths(fixture: &Fixture, data: String) -> String {
    let mut data = data
        .replace(&fixture.root_url().to_string(), "[ROOT_URL]")
        .replace(&normalize_path(&fixture.root_path()), "[ROOT]");

    if let Ok(pwd) = std::env::current_dir() {
        data = data.replace(&normalize_path(&pwd), "[PWD]");
    }

    let cairols_source = Path::new(env!("CARGO_MANIFEST_DIR"));
    data = data.replace(&normalize_path(cairols_source), "[CAIROLS_SOURCE]");

    data = data.replace(&normalize_path(scarb_registry_std_path()), "[SCARB_REGISTRY_STD]");

    let re = Regex::new(r"vfs://(\d+)/").unwrap();
    data = re.replace_all(&data, "vfs://").to_string();

    data
}

/// Normalizes path separators.
fn normalize_paths(data: String) -> String {
    data.replace('\\', "/")
}

/// Normalize a path to a consistent format.
fn normalize_path(path: &Path) -> String {
    normalize_paths(path.to_string_lossy().to_string())
}

/// Normalizes paths, sorts the diagnostics by the URL, filters out corelib diagnostics
/// Returns a list of tuples containing: (Original URL, Normalized URL, Normalized Diagnostics for
/// given URL)
pub fn normalize_diagnostics(
    fixture: &impl AsRef<Fixture>,
    diagnostics: impl IntoIterator<Item = (Url, Vec<Diagnostic>)>,
) -> Vec<(Url, String, Vec<Diagnostic>)> {
    diagnostics
        .into_iter()
        .filter(|(url, _)| !url.path().contains("core/src"))
        .sorted_by(|(url_a, _), (url_b, _)| url_a.path().cmp(url_b.path()))
        .map(|(url, diagnostics)| {
            (
                url.clone(),
                normalize(fixture, &url),
                diagnostics
                    .iter()
                    .map(|diag| Diagnostic {
                        range: diag.range,
                        severity: diag.severity,
                        code: diag.code.clone(),
                        code_description: diag.code_description.clone(),
                        source: diag.source.clone(),
                        message: diag.message.clone(),
                        related_information: diag.related_information.clone().map(|infos| {
                            infos
                                .iter()
                                .map(|x| DiagnosticRelatedInformation {
                                    location: Location {
                                        uri: Url::from_str(&normalize(
                                            fixture,
                                            x.location.uri.as_str(),
                                        ))
                                        .unwrap(),
                                        range: x.location.range,
                                    },
                                    message: x.message.clone(),
                                })
                                .collect()
                        }),
                        tags: diag.tags.clone(),
                        data: diag.data.clone(),
                    })
                    .collect(),
            )
        })
        .collect()
}
