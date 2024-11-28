use std::path::Path;

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
        .replace(&normalize_path(fixture.root_path()), "[ROOT]");

    if let Ok(pwd) = std::env::current_dir() {
        data = data.replace(&normalize_path(&pwd), "[PWD]");
    }

    let cairols_source = Path::new(env!("CARGO_MANIFEST_DIR"));
    data = data.replace(&normalize_path(cairols_source), "[CAIROLS_SOURCE]");

    data = data.replace(&normalize_path(scarb_registry_std_path()), "[SCARB_REGISTRY_STD]");

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
