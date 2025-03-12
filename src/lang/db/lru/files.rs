use super::set;
use crate::lang::db::AnalysisDatabase;

pub fn set_lru_capacity(db: &mut AnalysisDatabase) {
    set!(
        in db: 8192 for cairo_lang_filesystem::db {
            PrivRawFileContentQuery
        }
    );
}
