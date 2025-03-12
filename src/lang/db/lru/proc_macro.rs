use super::{DEFAULT_CAPACITY, set};
use crate::lang::db::AnalysisDatabase;

pub fn set_lru_capacity(db: &mut AnalysisDatabase) {
    set!(
        in db: DEFAULT_CAPACITY for crate::lang::proc_macros::db {
            GetStoredDeriveExpansionQuery,
            GetStoredAttributeExpansionQuery,
            GetStoredInlineMacrosExpansionQuery,
        }
    );
}
