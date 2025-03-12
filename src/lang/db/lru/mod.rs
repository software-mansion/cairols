use super::AnalysisDatabase;

mod defs;
mod docs;
mod files;
mod lowering;
mod parser;
mod proc_macro;
mod semantic;

const DEFAULT_CAPACITY: usize = 1024;

macro_rules! set {
    (in $db:ident: $capacity:ident for $location:path {$($query:ident),+ $(,)?}) => {
        use $location::{$($query),*};
        $($query.in_db_mut($db).set_lru_capacity($capacity);)*
    };
    (in $db:ident: $capacity:literal for $location:path {$($query:ident),+ $(,)?}) => {
        use $location::{$($query),*};
        $($query.in_db_mut($db).set_lru_capacity($capacity);)*
    };
}

use set;

/// Sets the sizes of LRU caches for all queries relevant for the language server.
pub fn set_lru_capacities(db: &mut AnalysisDatabase) {
    files::set_lru_capacity(db);
    parser::set_lru_capacity(db);
    defs::set_lru_capacity(db);
    semantic::set_lru_capacity(db);
    lowering::set_lru_capacity(db);
    docs::set_lru_capacity(db);
    proc_macro::set_lru_capacity(db);
}
