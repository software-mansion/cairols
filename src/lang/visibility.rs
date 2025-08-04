use cairo_lang_defs::ids::{ModuleFileId, ModuleId};
use cairo_lang_semantic::{
    expr::inference::InferenceId,
    items::visibility::{Visibility, peek_visible_in},
    resolve::Resolver,
};

use super::db::AnalysisDatabase;

pub fn peek_visible_in_with_edition<'db>(
    db: &'db AnalysisDatabase,
    visibility_in_module: Visibility,
    containing_module_id: ModuleId<'db>,
    user_module_id: ModuleFileId<'db>,
) -> bool {
    Resolver::new(db, user_module_id, InferenceId::NoContext)
        .ignore_visibility_checks(containing_module_id)
        || peek_visible_in(db, visibility_in_module, containing_module_id, user_module_id.0)
}
