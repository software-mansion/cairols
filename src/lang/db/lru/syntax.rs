use crate::lang::db::AnalysisDatabase;

use super::{REDUCED_CAPACITY, set};

pub fn set_lru_capacity(db: &mut AnalysisDatabase) {
    set!(
        in db: REDUCED_CAPACITY for cairo_lang_syntax::node::db {
            GetChildrenQuery
        }
    );
}
