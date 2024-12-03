use std::path::{Path, PathBuf};
use std::{fmt, fs};

use cairo_lang_filesystem::db::CORELIB_CRATE_NAME;
use cairo_lang_project::PROJECT_FILE_NAME;
use lsp_types::notification::ShowMessage;
use lsp_types::{MessageType, ShowMessageParams};
use serde::Deserialize;
use tracing::error;

use crate::server::client::Notifier;
use crate::toolchain::scarb::SCARB_TOML;

#[cfg(test)]
#[path = "project_manifest_path_test.rs"]
mod project_manifest_path_test;

const MAX_CRATE_DETECTION_DEPTH: usize = 20;

/// An absolute path to a manifest file of a single Cairo project.
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ProjectManifestPath {
    /// `cairo_project.toml` file.
    CairoProject(PathBuf),

    /// `Scarb.toml` file.
    ///
    /// This could either be a single package or a workspace manifest.
    Scarb(PathBuf),
}

impl ProjectManifestPath {
    /// Looks for a project manifest that **can** include source files at the given path.
    ///
    /// Returns `None` if the file at `path` is detached from any Cairo project.
    ///
    /// ## Precedence
    ///
    /// The following files are searched for in order:
    /// 1. `cairo_project.toml`
    /// 2. `Scarb.toml`
    ///
    /// This precedence rule also applies to manifest files themselves.
    /// If there are all `cairo_project.toml`, `Scarb.toml` and `Scarb.lock`
    /// files in the same directory, the `cairo_project.toml` file will be chosen for each
    /// and a notification with a warning will be sent to the client.
    /// The last step will be skipped for `core` crate as it is expected to have both manifests.
    pub fn discover(path: &Path, notifier: &Notifier) -> Option<ProjectManifestPath> {
        let project_config_path = find_in_parent_dirs(path.to_path_buf(), PROJECT_FILE_NAME);
        let scarb_manifest_path = find_in_parent_dirs(path.to_path_buf(), SCARB_TOML);

        if project_config_path.is_some() && scarb_manifest_path.is_some() {
            let is_core = match fs::read_to_string(scarb_manifest_path.as_ref().unwrap()) {
                Ok(content) => {
                    toml::from_str::<ScarbManifest>(&content).unwrap_or_default().package.name
                        == CORELIB_CRATE_NAME
                }
                Err(err) => {
                    error!("{err:?}");
                    false
                }
            };

            if !is_core {
                notifier.notify::<ShowMessage>(ShowMessageParams {
                    typ: MessageType::WARNING,
                    message: format!(
                        "Found conflicting manifest files: {} and {}. `cairo_project.toml` \
                         manifest will be used.",
                        project_config_path.as_ref().unwrap().to_string_lossy(),
                        scarb_manifest_path.as_ref().unwrap().to_string_lossy(),
                    ),
                });
            }
        }

        return project_config_path
            .map(ProjectManifestPath::CairoProject)
            .or(scarb_manifest_path.map(ProjectManifestPath::Scarb));

        #[derive(Default, Deserialize)]
        struct ScarbManifest {
            package: Package,
        }
        #[derive(Default, Deserialize)]
        struct Package {
            name: String,
        }

        fn find_in_parent_dirs(mut path: PathBuf, target_file_name: &str) -> Option<PathBuf> {
            for _ in 0..MAX_CRATE_DETECTION_DEPTH {
                if !path.pop() {
                    return None;
                }

                let manifest_path = path.join(target_file_name);
                // Check if the file exists and we can actually access it.
                if fs::metadata(&manifest_path).is_ok() {
                    return Some(manifest_path);
                };
            }
            None
        }
    }
}

impl fmt::Display for ProjectManifestPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectManifestPath::CairoProject(path) | ProjectManifestPath::Scarb(path) => {
                fmt::Display::fmt(&path.display(), f)
            }
        }
    }
}
