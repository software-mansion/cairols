//! Handling of LS configuration via environment variables.
//!
//! The [`report_to_logs`] function logs each variable value as a debug message.
//!
//! **Note**: Each variable should be described by a name constant, accessor function and entry
//! in the [`report_to_logs`] function.

use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;

use tracing::debug;

pub const CAIRO_LS_DB_REPLACE_INTERVAL: &'_ str = "CAIRO_LS_DB_REPLACE_INTERVAL";
pub const CAIRO_LS_DB_REPLACE_MUTATIONS: &'_ str = "CAIRO_LS_DB_REPLACE_MUTATIONS";
pub const CAIRO_LS_LOG: &'_ str = "CAIRO_LS_LOG";
pub const CAIRO_LS_PROFILE: &'_ str = "CAIRO_LS_PROFILE";
pub const SCARB: &'_ str = "SCARB";
pub const SCARB_CACHE: &'_ str = "SCARB_CACHE";
pub const SCARB_TARGET_DIR: &'_ str = "SCARB_TARGET_DIR";

/// Time interval between compiler database regenerations (to free unused memory).
pub fn db_replace_interval() -> Duration {
    const DEFAULT: u64 = 15 * 60;

    env::var(CAIRO_LS_DB_REPLACE_INTERVAL)
        .ok()
        .and_then(|v| v.parse().ok())
        .map(Duration::from_secs)
        .unwrap_or_else(|| Duration::from_secs(DEFAULT))
}

/// Number of mutations required to refresh the compiler database (to free unused memory).
pub fn db_replace_mutations() -> u64 {
    const DEFAULT: u64 = 5_000;

    env::var(CAIRO_LS_DB_REPLACE_MUTATIONS).ok().and_then(|v| v.parse().ok()).unwrap_or(DEFAULT)
}

/// LS tracing filter, see [`tracing_subscriber::EnvFilter`] for more.
pub fn log_env_filter() -> String {
    env::var(CAIRO_LS_LOG).unwrap_or_default()
}

/// Whether to generate LS tracing profile in the opened project directory.
pub fn tracing_profile() -> bool {
    env::var_os(CAIRO_LS_PROFILE).map(env_to_bool).unwrap_or_default()
}

/// Path to the Scarb binary to call during analysis.
pub fn scarb_path() -> Option<PathBuf> {
    env::var_os(SCARB).map(PathBuf::from)
}

/// Path to the Scarb cache directory. Available if LS is run via `scarb cairo-language-server`.
pub fn scarb_cache_path() -> Option<PathBuf> {
    env::var_os(SCARB_CACHE).map(PathBuf::from)
}

/// Path to the Scarb target directory. Available if LS is run via `scarb cairo-language-server`.
pub fn scarb_target_path() -> Option<PathBuf> {
    env::var_os(SCARB_TARGET_DIR).map(PathBuf::from)
}

/// Print all environment variables values (or defaults) as debug messages in logs.
pub fn report_to_logs() {
    debug!("{CAIRO_LS_DB_REPLACE_INTERVAL}={:?}", db_replace_interval());
    debug!("{CAIRO_LS_LOG}={}", log_env_filter());
    debug!("{CAIRO_LS_PROFILE}={}", tracing_profile());
    debug!("{SCARB}={}", scarb_path().map(|p| p.display().to_string()).unwrap_or_default());
}

fn env_to_bool(os: OsString) -> bool {
    matches!(os.to_str(), Some("1") | Some("true"))
}
