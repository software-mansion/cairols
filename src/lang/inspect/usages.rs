use std::collections::HashMap;
use std::ops::ControlFlow;
use std::sync::Arc;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::{TextSpan, TextWidth};
use cairo_lang_syntax::node::ast::TerminalIdentifier;
use cairo_lang_syntax::node::{Terminal, TypedStablePtr, TypedSyntaxNode};
use cairo_lang_utils::Upcast;
use memchr::memmem::Finder;

use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::inspect::defs::SymbolDef;

macro_rules! flow {
    ($expr:expr) => {
        let ControlFlow::Continue(()) = $expr else {
            return;
        };
    };
}

// TODO(mkaput): Reject text matches that are not word boundaries.
// TODO(mkaput): Implement search scopes: for example, variables will never be used in other files.
// TODO(mkaput): Deal with `crate` keyword.
/// An implementation of the find-usages functionality.
///
/// This algorithm is based on the standard IDE trick:
/// first, a fast text search to get a superset of matches is performed,
/// and then each match is checked using a precise goto-definition algorithm.
pub struct FindUsages<'a> {
    symbol: &'a SymbolDef,
    db: &'a AnalysisDatabase,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FoundUsage {
    pub file: FileId,
    pub span: TextSpan,
}

impl<'a> FindUsages<'a> {
    pub(super) fn new(symbol: &'a SymbolDef, db: &'a AnalysisDatabase) -> Self {
        Self { symbol, db }
    }

    /// Collects all found usages.
    pub fn collect(self) -> Vec<FoundUsage> {
        let mut result = vec![];
        self.search(&mut |usage| {
            result.push(usage);
            ControlFlow::Continue(())
        });
        result
    }

    /// Executes this search and calls the given sink for each found usage.
    #[tracing::instrument(skip_all)]
    pub fn search(self, sink: &mut dyn FnMut(FoundUsage) -> ControlFlow<(), ()>) {
        let db = self.db;

        let needle = self.symbol.name(db);

        let finder = Finder::new(needle.as_bytes());

        for (file, text) in Self::scope_files(db) {
            // Search occurrences of the symbol's name.
            for offset in finder.find_iter(text.as_bytes()) {
                let offset = TextWidth::at(&text, offset).as_offset();
                if let Some(node) = db.find_syntax_node_at_offset(file, offset) {
                    if let Some(identifier) = TerminalIdentifier::cast_token(db.upcast(), node) {
                        flow!(self.found_identifier(identifier, sink));
                    }
                }
            }
        }
    }

    fn scope_files(db: &AnalysisDatabase) -> impl Iterator<Item = (FileId, Arc<str>)> + '_ {
        let mut files = HashMap::new();
        for crate_id in db.crates() {
            for &module_id in db.crate_modules(crate_id).iter() {
                if let Ok(file_id) = db.module_main_file(module_id) {
                    if let Some(text) = db.file_content(file_id) {
                        files.insert(file_id, text);
                    }
                }
            }
        }
        files.into_iter()
    }

    fn found_identifier(
        &self,
        identifier: TerminalIdentifier,
        sink: &mut dyn FnMut(FoundUsage) -> ControlFlow<(), ()>,
    ) -> ControlFlow<(), ()> {
        // Declaration is not a usage, so filter it out.
        if Some(identifier.stable_ptr().untyped()) == self.symbol.definition_stable_ptr() {
            return ControlFlow::Continue(());
        }
        let Some(found_symbol) = SymbolDef::find(self.db, &identifier) else {
            return ControlFlow::Continue(());
        };
        if found_symbol == *self.symbol {
            let syntax_db = self.db.upcast();
            let syntax_node = identifier.as_syntax_node();
            let usage = FoundUsage {
                file: syntax_node.stable_ptr().file_id(syntax_db),
                span: syntax_node.span_without_trivia(syntax_db),
            };
            sink(usage)
        } else {
            ControlFlow::Continue(())
        }
    }
}
