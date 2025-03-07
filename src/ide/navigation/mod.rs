use cairo_lang_defs::ids::SubmoduleId;
use cairo_lang_syntax::node::TypedStablePtr;
use cairo_lang_syntax::node::ast::MaybeModuleBody;

use crate::lang::db::AnalysisDatabase;

pub mod goto_definition;
pub mod highlight;
pub mod references;
pub mod rename;

/// Infallible version of `db.is_submodule_inline`.
fn is_submodule_inline(db: &AnalysisDatabase, submodule_id: SubmoduleId) -> bool {
    match submodule_id.stable_ptr(db).lookup(db).body(db) {
        MaybeModuleBody::Some(_) => true,
        MaybeModuleBody::None(_) => false,
    }
}
