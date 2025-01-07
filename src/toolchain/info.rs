use std::env::current_exe;

use lsp_server::ErrorCode;

use crate::lsp::ext::{PathAndVersion, ToolchainInfoResponse};
use crate::lsp::result::{LSPResult, LSPResultEx};
use crate::state::StateSnapshot;

pub fn toolchain_info(state: StateSnapshot) -> LSPResult<ToolchainInfoResponse> {
    let ls_path = current_exe().with_failure_code(ErrorCode::RequestFailed)?;
    let ls_version = env!("CARGO_PKG_VERSION").to_string();

    let ls = PathAndVersion { path: ls_path, version: ls_version };

    let scarb_path = state.scarb_toolchain.discover();
    let scarb_version = state.scarb_toolchain.version();

    let scarb = match (scarb_version, scarb_path) {
        (Some(scarb_version), Some(scarb_path)) => {
            Some(PathAndVersion { path: scarb_path.to_path_buf(), version: scarb_version })
        }
        _ => None,
    };

    Ok(ToolchainInfoResponse { ls, scarb })
}
