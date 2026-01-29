//! CairoLS extensions to the Language Server Protocol.

use std::path::PathBuf;

use lsp_types::notification::Notification;
use lsp_types::request::Request;
use lsp_types::{TextDocumentPositionParams, Url};
use serde::{Deserialize, Serialize};

/// Provides content of virtual file from the database.
pub struct ProvideVirtualFile;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct ProvideVirtualFileRequest {
    pub uri: Url,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct ProvideVirtualFileResponse {
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

impl Request for ViewSyntaxTree {
    type Params = TextDocumentPositionParams;
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

// Server status notifications.
#[derive(Debug)]
pub struct ServerStatus;

impl Notification for ServerStatus {
    type Params = ServerStatusParams;
    const METHOD: &'static str = "cairo/serverStatus";
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ServerStatusEvent {
    AnalysisStarted,
    AnalysisFinished,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatusParams {
    pub event: ServerStatusEvent,
    pub idle: bool,
}

#[derive(Debug)]
pub struct ProcMacroControllerStatus;

impl Notification for ProcMacroControllerStatus {
    type Params = ProcMacroControllerStatusParams;
    const METHOD: &'static str = "cairo/procMacroControllerStatus";
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum ProcMacroControllerStatusEvent {
    MacrosBuildingStarted,
    MacrosBuildingFinished,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcMacroControllerStatusParams {
    pub event: ProcMacroControllerStatusEvent,
    pub idle: bool,
}

#[derive(Debug)]
pub struct ScarbPathMissing {}

impl Notification for ScarbPathMissing {
    type Params = ();
    const METHOD: &'static str = "scarb/could-not-find-scarb-executable";
}

#[derive(Debug)]
pub struct ExecuteInTerminal {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ExecuteInTerminalParams {
    pub command: String,
    pub cwd: PathBuf,
}

impl Notification for ExecuteInTerminal {
    type Params = ExecuteInTerminalParams;
    const METHOD: &'static str = "cairo/executeInTerminal";
}

pub struct ShowMemoryUsage;

impl Request for ShowMemoryUsage {
    type Params = ();
    type Result = serde_json::Value;
    const METHOD: &'static str = "cairo/showMemoryUsage";
}
