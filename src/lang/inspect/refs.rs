use std::collections::HashMap;

use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_syntax::node::ast::TerminalIdentifier;

use crate::lang::db::AnalysisDatabase;
use crate::lang::inspect::defs::SymbolDef;

pub struct FoundReferences {
    /// Location where searched symbol is declared.
    ///
    /// This can rarely be `None`, for example, for macros.
    pub declaration: Option<(FileId, TextSpan)>,

    /// Locations where searched symbol is used.
    pub usages: HashMap<FileId, Vec<TextSpan>>,
}

/// Finds all places in the entire analysed codebase for usages of the given identifier.
pub fn find_all_references(
    db: &AnalysisDatabase,
    identifier: TerminalIdentifier,
) -> Option<FoundReferences> {
    let symbol = SymbolDef::find(db, &identifier)?;

    // TODO(mkaput): Think about how to deal with `mod foo;` vs `mod foo { ... }`.
    // For all cases we cover here, definition == declaration.
    let declaration = symbol.definition_location(db);

    Some(FoundReferences {
        declaration,
        // TODO(mkaput): Implement this.
        usages: Default::default(),
    })
}
