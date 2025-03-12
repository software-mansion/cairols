use super::AnalysisDatabase;

mod defs;
mod docs;
mod files;
mod lowering;
mod parser;
mod proc_macro;
mod semantic;
mod syntax;

const MINIMAL_CAPACITY: usize = 0;
const REDUCED_CAPACITY: usize = 64;
const NORMAL_CAPACITY: usize = 512;
const INCREASED_CAPACITY: usize = 1024;

macro_rules! set {
    (in $db:ident: $capacity:ident for $location:path {$($query:ident),+ $(,)?}) => {
        use $location::{$($query),*};
        $($query.in_db_mut($db).set_lru_capacity($capacity);)*
    };
}

pub(self) use set;

/// Sets the sizes of LRU caches for all queries relevant for the language server.
pub(super) fn set_lru_capacities(db: &mut AnalysisDatabase) {
    // let extreme_capacity = 16_384;
    // set!(
    //     in db: extreme_capacity for cairo_lang_syntax::node::db {
    //         GetChildrenQuery
    //     }
    // );

    let high_capacity = 8192;
    set!(
        in db: high_capacity for cairo_lang_semantic::db {
            PrivTypeIsVarFreeQuery, PrivImplIsVarFreeQuery, ConcreteFunctionSignatureQuery,
            ImplConcreteTraitQuery, GetClosureParamsQuery,
        }
    );
    set!(
        in db: high_capacity for cairo_lang_filesystem::db {
            PrivRawFileContentQuery
        }
    );

    // files::set_lru_capacity(db);
    // parser::set_lru_capacity(db);
    // syntax::set_lru_capacity(db);
    // defs::set_lru_capacity(db);
    // semantic::set_lru_capacity(db);
    // lowering::set_lru_capacity(db);
    // docs::set_lru_capacity(db);
    // proc_macro::set_lru_capacity(db);
}
