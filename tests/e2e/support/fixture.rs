use crate::support::cairo_project_toml::WELL_KNOWN_CAIRO_PROJECT_TOMLS;
use assert_fs::TempDir;
use assert_fs::prelude::*;
use itertools::Itertools;
use lsp_types::Url;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

const TOOL_VERSIONS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/.tool-versions"));

/// A temporary directory that is a context for testing the language server.
pub struct Fixture {
    // This is put behind a LazyLock, so that Fixture::new calls are no-cost.
    t: LazyLock<TempDir>,
    files: Vec<PathBuf>,
    insta_settings: Option<insta::internals::SettingsBindDropGuard>,
}

impl Fixture {
    /// Creates a new [`Fixture`] with an empty temporary directory.
    pub fn new() -> Self {
        Self {
            t: LazyLock::new(|| TempDir::new().unwrap()),
            files: Vec::new(),
            insta_settings: None,
        }
    }
}

/// Builder methods.
impl Fixture {
    /// Creates a new file in the fixture with provided contents.
    pub fn add_file(&mut self, path: impl AsRef<Path>, contents: impl AsRef<str>) {
        self.files.push(path.as_ref().to_owned());
        self.edit_file(path, contents);
    }

    pub fn edit_file(&mut self, path: impl AsRef<Path>, contents: impl AsRef<str>) {
        self.t.child(path).write_str(contents.as_ref().trim()).unwrap();
    }

    /// Copies the `.tool-versions` file of this repo to the fixture directory.
    pub fn add_tool_versions(&mut self) {
        self.add_file(".tool-versions", TOOL_VERSIONS);
    }
}

/// Introspection methods.
impl Fixture {
    pub fn root_path(&self) -> PathBuf {
        self.t.path().canonicalize().unwrap()
    }

    pub fn root_url(&self) -> Url {
        Url::from_directory_path(self.t.path().canonicalize().unwrap()).unwrap()
    }

    pub fn file_absolute_path(&self, path: impl AsRef<Path>) -> PathBuf {
        let path = path.as_ref();

        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.t.child(path).canonicalize().unwrap().to_owned()
        }
    }

    pub fn file_relative_path(&self, path: impl AsRef<Path>) -> PathBuf {
        let path = self.file_absolute_path(path);

        PathBuf::from("./").join(path.strip_prefix(self.root_path()).unwrap())
    }

    pub fn file_url(&self, path: impl AsRef<Path>) -> Url {
        Url::from_file_path(self.file_absolute_path(path)).unwrap()
    }

    pub fn read_file(&self, path: impl AsRef<Path>) -> String {
        fs::read_to_string(self.file_absolute_path(path)).unwrap()
    }

    pub fn maybe_read_file(&self, path: impl AsRef<Path>) -> Option<String> {
        fs::read_to_string(self.file_absolute_path(path)).ok()
    }

    /// If the url refers to a possible file in this fixture, returns the path of this file
    /// (relative to fixture root); otherwise, returns an error string.
    /// This method does not check the existence of the file.
    pub fn url_path(&self, url: &Url) -> Result<PathBuf, String> {
        let path = url.to_file_path().map_err(|()| format!("not a file url: {url}"))?;
        let path = path
            .strip_prefix(self.root_path())
            .map_err(|_| format!("url leads to a file outside test fixture: {url}"))?;
        Ok(path.to_path_buf())
    }

    /// Returns all files paths in the fixture.
    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }
}

/// Insta integration.
impl Fixture {
    /// Binds a new [`insta::Settings`] object with a description built from this fixture.
    ///
    /// This function has to be used very carefully because if used multiple times in code,
    /// **no** other setting bindings can happen between calls.
    /// This is a consequence of using settings binding guards and their reset behaviour on drops.
    #[doc(hidden)]
    pub fn update_insta_settings(&mut self) {
        let mut settings = insta::Settings::clone_current();
        settings.set_description(self.build_insta_description());

        // NOTE: We need to drop the old guard before binding new settings, as when that guard
        //   drops, it restores an "old" settings snapshot it kept inside.
        drop(self.insta_settings.take());

        self.insta_settings = Some(settings.bind_to_scope());
    }

    fn build_insta_description(&self) -> String {
        self.files
            .iter()
            .sorted()
            .map(|path| {
                let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
                (
                    path,
                    self.read_file(path).trim().to_owned().replace(test_dir.to_str().unwrap(), ""),
                )
            })
            .filter(|(path, contents)| {
                // We know paths here are always file paths that are UTF-8.
                let str_file_name = path.file_name().unwrap().to_str().unwrap();
                match str_file_name {
                    ".tool-versions" => false,
                    "cairo_project.toml" => !WELL_KNOWN_CAIRO_PROJECT_TOMLS
                        .iter()
                        .any(|it| it.trim() == contents.trim()),
                    _ => true,
                }
            })
            .map(|(path, contents)| format!("// → {path}\n{contents}", path = path.display()))
            .join("\n\n")
    }
}

/// Macro to create a [`Fixture`] with a set of predefined files and their contents.
///
/// # Usage
///
/// ```
/// let fixture = fixture! {
///     "file1.txt" => "Content of file 1",
///     "file2.txt" => "Content of file 2",
/// };
/// ```
macro_rules! fixture {
    { $($file:expr => $content:expr),* $(,)? } => {{
        let mut fixture = $crate::support::fixture::Fixture::new();
        $(fixture.add_file($file, $content);)*
        fixture
    }};
}
pub(crate) use fixture;
