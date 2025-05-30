use std::path;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, OnceLock};

use anyhow::{Context, Result, bail, ensure};
use lsp_types::notification::ShowMessage;
use lsp_types::{MessageType, ShowMessageParams};
use scarb_metadata::{Metadata, MetadataCommand};
use tracing::{debug, error, warn};
use which::which;

use crate::env_config::{self, CAIRO_LS_LOG, scarb_cache_path};
use crate::lsp::ext::ScarbPathMissing;
use crate::server::client::Notifier;

pub const SCARB_TOML: &str = "Scarb.toml";

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
        let Some(scarb_path) = self.discover() else {
            bail!("could not find scarb executable");
        };

        let result = MetadataCommand::new()
            .scarb_path(scarb_path)
            .manifest_path(manifest)
            .inherit_stderr()
            .exec()
            .context("failed to execute: scarb metadata");

        if !self.is_silent && result.is_err() {
            self.notifier.notify::<ShowMessage>(ShowMessageParams {
                typ: MessageType::ERROR,
                message: "`scarb metadata` failed. Check if your project builds correctly via \
                              `scarb build`."
                    .to_string(),
            });
        }

        result
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
