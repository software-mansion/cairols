use crate::lang::db::AnalysisDatabase;

use super::{REDUCED_CAPACITY, set};

pub fn set_lru_capacity(db: &mut AnalysisDatabase) {
    set!(
        in db: REDUCED_CAPACITY for crate::lang::proc_macros::db {
            GetAttributeExpansionQuery, GetDeriveExpansionQuery, GetInlineMacrosExpansionQuery
        }
    );
}
