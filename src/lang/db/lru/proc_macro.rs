use super::set;
use crate::lang::db::AnalysisDatabase;

pub fn set_lru_capacity(db: &mut AnalysisDatabase) {
    set!(
        in db: 4096 for crate::lang::proc_macros::db {
            GetStoredDeriveExpansionQuery,
            GetStoredAttributeExpansionQuery,
            GetStoredInlineMacrosExpansionQuery,
        }
    );
}
