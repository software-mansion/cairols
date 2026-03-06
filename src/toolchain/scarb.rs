use std::path;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, OnceLock};

use anyhow::{Context, Result, anyhow, bail, ensure};
use lsp_types::notification::ShowMessage;
use lsp_types::{MessageType, ShowMessageParams};
use scarb_metadata::Metadata;
use serde::Deserialize;
use serde_json::Value;
use tracing::{debug, error, warn};
use which::which;

use crate::env_config::{self, CAIRO_LS_LOG, scarb_cache_path};
use crate::lsp::ext::ScarbPathMissing;
use crate::server::client::Notifier;

pub const SCARB_TOML: &str = "Scarb.toml";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScarbMetadataDiagnostic {
    pub severity: ScarbMetadataDiagnosticSeverity,
    pub message: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ScarbMetadataDiagnosticSeverity {
    Error,
    Warning,
}

/// The ultimate object for invoking Scarb.
///
/// This object tries to maintain good UX when doing any Scarb operations, for example, by sending
/// progress notifications to the language client.
///
/// This object is small and cheap to clone, so it can be passed around freely.
#[derive(Clone)]
pub struct ScarbToolchain {
    /// Cached path to the `scarb` executable.
    scarb_path_cell: Arc<OnceLock<Option<PathBuf>>>,

    /// Cached scarb version.
    version: Arc<OnceLock<Option<String>>>,

    /// Cached scarb cache path version.
    cache_path: Arc<OnceLock<Option<PathBuf>>>,

    /// The notifier object used to send notifications to the language client.
    notifier: Notifier,

    /// States whether this instance is in _silent mode_.
    ///
    /// See [`ScarbToolchain::silent`] for more info.
    is_silent: bool,
}

impl ScarbToolchain {
    /// Constructs a new [`ScarbToolchain`].
    pub fn new(notifier: Notifier) -> Self {
        ScarbToolchain {
            scarb_path_cell: Default::default(),
            version: Default::default(),
            cache_path: Default::default(),
            notifier,
            is_silent: false,
        }
    }

    /// Finds the path to the `scarb` executable to use.
    ///
    /// This method may send notifications to the language client if there are any actionable issues
    /// with the found `scarb` installation or if it could not be found.
    pub fn discover(&self) -> Option<&Path> {
        self.scarb_path_cell
            .get_or_init(|| {
                // While running tests, we do not have SCARB env set,
                // but we expect `scarb` binary to be in the PATH.
                if cfg!(feature = "testing") {
                    return Some(
                        which("scarb")
                            .expect("running tests requires a `scarb` binary available in `PATH`"),
                    );
                }

                let path = env_config::scarb_path();
                // TODO(mkaput): Perhaps we should display this notification again after reloading?
                if path.is_none() {
                    if self.is_silent {
                        // If we are in silent mode, then missing Scarb is probably dealt with
                        // at the caller site.
                        warn!("attempt to use scarb without SCARB env being set");
                    } else {
                        error!("attempt to use scarb without SCARB env being set");
                        self.notifier.notify::<ScarbPathMissing>(());
                    }
                }
                path
            })
            .as_ref()
            .map(PathBuf::as_path)
    }

    /// Creates a clone instance of this object that will be in _silent mode_.
    ///
    /// Silent mode means that any operations invoked through this instance should avoid performing
    /// any user-visible actions.
    pub fn silent(&self) -> Self {
        if self.is_silent {
            // Going silent from silent is noop, so skip any shenanigans we do here.
            self.clone()
        } else {
            Self {
                // Disassociate this instance from the shared path cell if it has not been
                // initialized yet.
                //
                // This maintains a good UX for the following scenario (timeline):
                // 1. CairoLS is started without a path to Scarb provided.
                // 2. Some internal operation is silently attempting to query Scarb, which will
                //    initialize the cell but only log a warning.
                // 3. User-invoked operation makes an attempt to query Scarb.
                //
                // At this point we want to show missing Scarb notification,
                // but without this trick we would never do
                // as the path cell would be already initialized.
                scarb_path_cell: match self.scarb_path_cell.get() {
                    Some(_) => self.scarb_path_cell.clone(),
                    None => Default::default(),
                },
                version: self.version.clone(),
                cache_path: self.cache_path.clone(),
                notifier: self.notifier.clone(),
                is_silent: true,
            }
        }
    }

    /// Calls `scarb metadata` for the given `Scarb.toml` and parse its output.
    ///
    /// This is a blocking operation that may be long-running. It should only be called from within
    /// a background task. The `scarb metadata` command performs workspace resolution, which does a
    /// lot of IO, including network requests (for fetching registry index and downloading
    /// packages).
    ///
    /// This method may send notifications to the language client, informing the user about
    /// the progress of the operation or any actionable issues.
    #[tracing::instrument(skip(self))]
    pub fn metadata(&self, manifest: &Path) -> Result<Metadata> {
        self.metadata_with_diagnostics(manifest).0
    }

    /// Calls `scarb metadata` for the given `Scarb.toml`, parses its output and
    /// collects diagnostics emitted by Scarb itself.
    #[tracing::instrument(skip(self))]
    pub fn metadata_with_diagnostics(
        &self,
        manifest: &Path,
    ) -> (Result<Metadata>, Vec<ScarbMetadataDiagnostic>) {
        let Some(scarb_path) = self.discover() else {
            return (Err(anyhow!("could not find scarb executable")), Vec::new());
        };

        let output = Command::new(scarb_path)
            .arg("--json")
            .arg("--manifest-path")
            .arg(manifest)
            .arg("metadata")
            .arg("--format-version")
            .arg("1")
            .output()
            .context("failed to execute: scarb metadata");

        let (result, diagnostics) = match output {
            Ok(output) => parse_metadata_command_output(manifest, output),
            Err(err) => (Err(err), Vec::new()),
        };

        if !self.is_silent && result.is_err() {
            self.notifier.notify::<ShowMessage>(ShowMessageParams {
                typ: MessageType::ERROR,
                message: "`scarb metadata` failed. Check if your project builds correctly via \
                              `scarb build`."
                    .to_string(),
            });
        }

        (result, diagnostics)
    }

    pub fn proc_macro_server(&self, cwd: &Path) -> Result<Child> {
        let Some(scarb_path) = self.discover() else { bail!("failed to get scarb path") };

        let proc_macro_server = Command::new(scarb_path)
            .current_dir(cwd)
            .arg("--quiet") // If not set scarb will print all "Compiling ..." messages we don't need (and these can crash input parsing).
            .arg("proc-macro-server")
            .envs(std::env::var("RUST_BACKTRACE").map(|value| ("RUST_BACKTRACE", value)))
            // This is tracing directive so we can just forward it to scarb.
            .envs(std::env::var(CAIRO_LS_LOG).map(|value| ("SCARB_LOG", value)))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            // We use this channel for debugging.
            .stderr(Stdio::inherit())
            .spawn()?;

        Ok(proc_macro_server)
    }

    pub fn version(&self) -> Option<String> {
        self.version
            .get_or_init(|| self.fetch_version().inspect_err(|err| error!("{err:#?}")).ok())
            .clone()
    }

    pub fn cache_path(&self) -> Option<PathBuf> {
        self.cache_path.get_or_init(|| self.fetch_cache_path().ok()).clone()
    }

    pub fn is_from_scarb_cache(&self, file_path: &Path) -> bool {
        self.cache_path().is_some_and(|cache_path| file_path.starts_with(cache_path))
    }

    fn fetch_version(&self) -> Result<String> {
        let Some(scarb_path) = self.discover() else { bail!("failed to get scarb path") };

        let output = Command::new(scarb_path).arg("--version").output()?;

        ensure!(output.status.success(), "failed to get scarb version");

        let version = String::from_utf8_lossy(&output.stdout).to_string();

        Ok(version)
    }

    fn fetch_cache_path(&self) -> Result<PathBuf> {
        if let Some(scarb_cache_path) = scarb_cache_path() {
            return Ok(scarb_cache_path);
        }

        let Some(scarb_path) = self.discover() else { bail!("failed to get scarb path") };

        let output = Command::new(scarb_path).arg("cache").arg("path").output()?;

        ensure!(output.status.success(), "failed to get scarb cache path");

        let cache_path = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim().to_string());

        path::absolute(&cache_path)
            .with_context(|| {
                format!("failed to make scarb cache path absolute: {}", cache_path.display())
            })
            .inspect(|p| debug!("Scarb cache path: {}", p.display()))
            .inspect_err(|err| error!("{err:#?}"))
    }
}

fn parse_metadata_command_output(
    manifest: &Path,
    output: std::process::Output,
) -> (Result<Metadata>, Vec<ScarbMetadataDiagnostic>) {
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    let mut metadata_json: Option<String> = None;
    let mut diagnostics: Vec<ScarbMetadataDiagnostic> = Vec::new();

    for line in stdout.lines().filter(|line| !line.trim().is_empty()) {
        if let Some(diagnostic) = parse_scarb_diagnostic_event(line) {
            diagnostics.push(diagnostic);
            continue;
        }

        metadata_json = Some(line.to_string());
    }

    if output.status.success() {
        let metadata = metadata_json
            .ok_or_else(|| anyhow!("`scarb metadata` produced no metadata output"))
            .and_then(|json| {
                serde_json::from_str::<Metadata>(&json)
                    .context("failed to parse `scarb metadata` output")
            });
        return (metadata, diagnostics);
    }

    if diagnostics.is_empty() {
        diagnostics.push(ScarbMetadataDiagnostic {
            severity: ScarbMetadataDiagnosticSeverity::Error,
            message: non_empty_command_output(&stdout, &stderr).unwrap_or_else(|| {
                "`scarb metadata` failed without additional details".to_string()
            }),
            line: None,
            column: None,
        });
    }

    let error_output = non_empty_command_output(&stdout, &stderr)
        .unwrap_or_else(|| "`scarb metadata` failed without additional details".to_string());
    let result = Err(anyhow!(
        "failed to execute `scarb metadata` for {}: {}",
        manifest.display(),
        error_output
    ));

    (result, diagnostics)
}

fn non_empty_command_output(stdout: &str, stderr: &str) -> Option<String> {
    let mut parts = Vec::new();
    let trimmed_stdout = stdout.trim();
    let trimmed_stderr = stderr.trim();

    if !trimmed_stdout.is_empty() {
        parts.push(trimmed_stdout);
    }
    if !trimmed_stderr.is_empty() {
        parts.push(trimmed_stderr);
    }

    (!parts.is_empty()).then(|| parts.join("\n"))
}

#[derive(Deserialize)]
struct ScarbDiagnosticEvent {
    #[serde(rename = "type")]
    typ: String,
    message: String,
}

fn parse_scarb_diagnostic_event(line: &str) -> Option<ScarbMetadataDiagnostic> {
    let value = serde_json::from_str::<Value>(line).ok()?;
    value.get("type")?;
    let event = serde_json::from_value::<ScarbDiagnosticEvent>(value).ok()?;

    let severity = match event.typ.as_str() {
        "error" => ScarbMetadataDiagnosticSeverity::Error,
        "warning" => ScarbMetadataDiagnosticSeverity::Warning,
        _ => return None,
    };

    let (line, column) = parse_line_and_column(&event.message);
    Some(ScarbMetadataDiagnostic { severity, message: event.message, line, column })
}

fn parse_line_and_column(message: &str) -> (Option<u32>, Option<u32>) {
    let marker = "at line ";
    let Some(start) = message.find(marker) else {
        return (None, None);
    };

    let remaining = &message[start + marker.len()..];
    let Some((line, rest)) = remaining.split_once(", column ") else {
        return (None, None);
    };

    let line = line.trim().parse::<u32>().ok();
    let column =
        rest.chars().take_while(|ch| ch.is_ascii_digit()).collect::<String>().parse::<u32>().ok();

    (line, column)
}

#[cfg(test)]
mod tests {
    use super::{
        ScarbMetadataDiagnosticSeverity, parse_line_and_column, parse_scarb_diagnostic_event,
    };

    #[test]
    fn parses_error_event_with_line_and_column() {
        let line = r#"{"type":"error","message":"failed to parse manifest at: /tmp/Scarb.toml\n\nCaused by:\n    TOML parse error at line 2, column 8\n      |\n    2 | name = 1\n      |        ^\n    invalid type: integer `1`, expected a string"}"#;

        let diagnostic = parse_scarb_diagnostic_event(line).unwrap();
        assert_eq!(diagnostic.severity, ScarbMetadataDiagnosticSeverity::Error);
        assert_eq!(diagnostic.line, Some(2));
        assert_eq!(diagnostic.column, Some(8));
    }

    #[test]
    fn parses_warning_event() {
        let line = r#"{"type":"warning","message":"some warning"}"#;

        let diagnostic = parse_scarb_diagnostic_event(line).unwrap();
        assert_eq!(diagnostic.severity, ScarbMetadataDiagnosticSeverity::Warning);
        assert_eq!(diagnostic.message, "some warning");
    }

    #[test]
    fn extracts_line_and_column_only_when_pattern_exists() {
        assert_eq!(
            parse_line_and_column("TOML parse error at line 12, column 3"),
            (Some(12), Some(3))
        );
        assert_eq!(parse_line_and_column("no position info"), (None, None));
    }
}
