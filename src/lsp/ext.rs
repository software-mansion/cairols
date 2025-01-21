//! CairoLS extensions to the Language Server Protocol.

use std::path::PathBuf;

use lsp_types::notification::Notification;
use lsp_types::request::Request;
use lsp_types::{TextDocumentPositionParams, Url};
use serde::{Deserialize, Serialize};

/// Provides content of virtual file from the database.
pub(crate) struct ProvideVirtualFile;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub(crate) struct ProvideVirtualFileRequest {
    pub uri: Url,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub(crate) struct ProvideVirtualFileResponse {
    pub content: Option<String>,
}

impl Request for ProvideVirtualFile {
    type Params = ProvideVirtualFileRequest;
    type Result = ProvideVirtualFileResponse;
    const METHOD: &'static str = "vfs/provide";
}

/// Collects information about all Cairo crates that are currently being analyzed.
pub struct ViewAnalyzedCrates;

impl Request for ViewAnalyzedCrates {
    type Params = ();
    type Result = String;
    const METHOD: &'static str = "cairo/viewAnalyzedCrates";
}

/// Provides string with code after macros expansion.
pub struct ExpandMacro;

impl Request for ExpandMacro {
    type Params = TextDocumentPositionParams;
    type Result = Option<String>;
    const METHOD: &'static str = "cairo/expandMacro";
}

/// Notifies about corelib version mismatch.
#[derive(Debug)]
pub struct CorelibVersionMismatch;

impl Notification for CorelibVersionMismatch {
    type Params = String;
    const METHOD: &'static str = "cairo/corelib-version-mismatch";
}

/// Collects versions of LS and it's dependencies.
#[derive(Debug)]
pub struct ToolchainInfo;

#[derive(Serialize, Deserialize)]
pub struct PathAndVersion {
    pub path: PathBuf,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ToolchainInfoResponse {
    pub ls: PathAndVersion,
    pub scarb: Option<PathAndVersion>,
}

impl Request for ToolchainInfo {
    type Params = ();
    type Result = ToolchainInfoResponse;
    const METHOD: &'static str = "cairo/toolchainInfo";
}

pub struct ViewSyntaxTree;

#[derive(Serialize, Deserialize)]
pub struct ViewSyntaxTreeParams {
    pub uri: Url,
}

impl Request for ViewSyntaxTree {
    type Params = ViewSyntaxTreeParams;
    type Result = Option<String>;
    const METHOD: &'static str = "cairo/viewSyntaxTree";
}

#[cfg(feature = "testing")]
pub mod testing {
    use lsp_types::notification::Notification;

    /// Notifies about the end of project updating.
    #[derive(Debug)]
    pub struct ProjectUpdatingFinished;

    impl Notification for ProjectUpdatingFinished {
        type Params = ();
        const METHOD: &'static str = "cairo/projectUpdatingFinished";
    }
}
