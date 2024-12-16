use lsp_types::{Location, ReferenceParams};

use crate::lang::db::AnalysisDatabase;

pub fn references(_params: ReferenceParams, _db: &AnalysisDatabase) -> Option<Vec<Location>> {
    // TODO(mkaput): Implement this.
    None
}
