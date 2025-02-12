use std::fs;
use std::path::{Path, PathBuf};

use assert_fs::TempDir;
use assert_fs::prelude::*;
use lsp_types::Url;

/// A temporary directory that is a context for testing the language server.
pub struct Fixture {
    t: TempDir,
    files: Vec<PathBuf>,
}

impl Fixture {
    /// Creates a new [`Fixture`] with an empty temporary directory.
    pub fn new() -> Self {
        let t = TempDir::new().unwrap();
        Self { t, files: Vec::new() }
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

    pub fn file_url(&self, path: impl AsRef<Path>) -> Url {
        Url::from_file_path(self.file_absolute_path(path)).unwrap()
    }

    pub fn read_file(&self, path: impl AsRef<Path>) -> String {
        fs::read_to_string(self.file_absolute_path(path)).unwrap()
    }

    /// Returns all files paths in the fixture.
    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }
}
