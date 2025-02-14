use lsp_types::{RenameParams, WorkspaceEdit};

use crate::lang::db::AnalysisDatabase;

pub fn rename(_params: RenameParams, _db: &AnalysisDatabase) -> Option<WorkspaceEdit> {
    None
}
