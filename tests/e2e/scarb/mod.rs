use serde::Serialize;

mod ignore_warnings_from_non_path_deps;
mod opening_multiple_workspaces;
mod scarb_toml_change;
mod simple_deps;
mod unmanaged_core;

#[derive(Serialize)]
struct AnalyzedCratesResult {
    analyzed_crates: String,
    analyzed_crates_diff: String,
}
