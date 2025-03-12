use crate::lang::db::AnalysisDatabase;

use super::{NORMAL_CAPACITY, set};

pub fn set_lru_capacity(db: &mut AnalysisDatabase) {
    let capacity = 1025;
    set!(
        in db: capacity for cairo_lang_doc::db {
            GetItemDocumentationQuery, GetItemDocumentationAsTokensQuery,
            GetItemSignatureQuery, GetItemSignatureWithLinksQuery
        }
    );
}
