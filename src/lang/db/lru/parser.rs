use crate::lang::db::AnalysisDatabase;

use super::{NORMAL_CAPACITY, set};

pub fn set_lru_capacity(db: &mut AnalysisDatabase) {
    let medium_capacity = 1024;

    set!(
        in db: medium_capacity for cairo_lang_parser::db {
            FileExprSyntaxQuery
        }
    );
}
