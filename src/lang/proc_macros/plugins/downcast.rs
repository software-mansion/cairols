use salsa::Database;

use crate::lang::db::AnalysisDatabase;

/// This function is necessary due to trait bounds that cannot be bypassed in any other way.
/// `generate_code()` takes db: [`&dyn Database`](`Database`),
/// but we need to use
/// [`ProcMacroGroup`](`crate::lang::proc_macros::db::ProcMacroGroup`). To do
/// this, we first convert the `db` reference to its original concrete type that implements both
/// traits [`AnalysisDatabase`]. After this,
/// [`ProcMacroGroup`](`crate::lang::proc_macros::db::ProcMacroGroup`) can be
/// accessed.
///
/// Safety: This function MUST only be invoked with an object that is of type
/// [AnalysisDatabase]. Using it with any other type leads to undefined behavior.
pub(super) unsafe fn unsafe_downcast_ref(db: &dyn Database) -> &AnalysisDatabase {
    unsafe {
        // Replicated logic from `impl dyn Any downcast_ref_unchecked()`.
        // This approach works as long as `impl dyn Any downcast_ref_unchecked()` implementation is
        // unchanged and the caller can ensure that `db` is truly an instance of AnalysisDatabase.
        &*(db as *const dyn Database as *const AnalysisDatabase)
    }
}

#[cfg(test)]
mod unsafe_downcast_ref_tests {
    use std::collections::HashMap;

    use cairo_lang_macro_v1::TokenStream;
    use salsa::{Database, Setter};
    use scarb_proc_macro_server_types::methods::ProcMacroResult;
    use scarb_proc_macro_server_types::scope::{ProcMacroScope, Workspace};

    use super::unsafe_downcast_ref;
    use crate::lang::db::AnalysisDatabase;
    use crate::lang::proc_macros::client::plain_request_response::PlainExpandAttributeParams;
    use crate::lang::proc_macros::db::ProcMacroGroup;

    #[test]
    fn cast_succeed() {
        let mut db = AnalysisDatabase::new();

        let context = ProcMacroScope {
            component: Default::default(),
            workspace: Workspace { manifest_path: "".into() },
        };

        let input = PlainExpandAttributeParams {
            context,
            attr: "asd".to_string(),
            args: "asd".to_string(),
            item: "asd".to_string(),
        };
        let output = ProcMacroResult {
            token_stream: TokenStream::new("asd".to_string()),
            diagnostics: Default::default(),
            code_mappings: None,
        };
        let macro_resolution: HashMap<_, _> = [(input, output)].into_iter().collect();

        db.proc_macro_input().set_attribute_macro_resolution(&mut db).to(macro_resolution.clone());

        let syntax: &dyn Database = &db;
        let analysis_db = unsafe { unsafe_downcast_ref(syntax) };

        assert_eq!(
            analysis_db.proc_macro_input().attribute_macro_resolution(analysis_db),
            &macro_resolution
        );
    }
}
