use std::ops::ControlFlow;

use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::{TextOffset, TextSpan, TextWidth};
use cairo_lang_syntax::node::ast::TerminalIdentifier;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedStablePtr, TypedSyntaxNode};
use cairo_lang_utils::smol_str::format_smolstr;
use memchr::memmem::Finder;

use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::defs::{ResolvedItem, SymbolDef, SymbolSearch};
use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_semantic::keyword::SELF_TYPE_KW;
use cairo_lang_semantic::resolve::{ResolvedConcreteItem, ResolvedGenericItem};
use search_scope::SearchScope;

pub mod search_scope;

macro_rules! flow {
    ($expr:expr) => {
        let ControlFlow::Continue(()) = $expr else {
            return;
        };
    };
}

// TODO(mkaput): Deal with `crate` keyword.
// TODO(mkaput): Think about how to deal with `mod foo;` vs `mod foo { ... }`.
/// An implementation of the find-usages functionality.
///
/// This algorithm is based on the standard IDE trick:
/// first, a fast text search to get a superset of matches is performed,
/// and then each match is checked using a precise goto-definition algorithm.
pub struct FindUsages<'a> {
    symbol: &'a SymbolDef,
    symbol_item: &'a ResolvedItem,
    db: &'a AnalysisDatabase,
    include_declaration: bool,
    in_scope: Option<SearchScope>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FoundUsage {
    pub file: FileId,
    pub span: TextSpan,
}

impl FoundUsage {
    fn originating_location(&self, db: &AnalysisDatabase) -> Self {
        let (file, span) = get_originating_location(db, self.file, self.span, None);

        Self { file, span }
    }
}

impl<'a> FindUsages<'a> {
    pub(super) fn new(
        symbol: &'a SymbolDef,
        symbol_item: &'a ResolvedItem,
        db: &'a AnalysisDatabase,
    ) -> Self {
        Self { symbol, symbol_item, db, include_declaration: false, in_scope: None }
    }

    /// If set to `true`, treat the symbol's declaration as a usage and include it in search
    /// results.
    ///
    /// Not all symbols have a declaration, macros are the prime example here.
    pub fn include_declaration(mut self, include: bool) -> Self {
        self.include_declaration = include;
        self
    }

    /// Will search only in provided scope.
    ///
    /// Useful mostly in `textDocument/documentHighlight` to narrow search scope.
    pub fn in_scope(mut self, scope: SearchScope) -> Self {
        self.in_scope = Some(scope);
        self
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

    /// Collects just the originating locations of all found usages.
    pub fn originating_locations(
        self,
        db: &AnalysisDatabase,
    ) -> impl Iterator<Item = (FileId, TextSpan)> {
        self.collect().into_iter().map(|usage| usage.originating_location(db).location())
    }

    /// Executes this search and calls the given sink for each found usage.
    #[tracing::instrument(skip_all)]
    pub fn search(self, sink: &mut dyn FnMut(FoundUsage) -> ControlFlow<(), ()>) {
        let db = self.db;

        #[allow(unused_doc_comments)]
        /// We include definition instead of declaration here.
        /// It is done to ensure better UX when finding references of impl items.
        /// For details, refer to [`SymbolSearch::find_definition`] and [`SymbolSearch::find_declaration`].
        if self.include_declaration {
            if let Some(stable_ptr) = self.symbol.definition_stable_ptr(db) {
                // Definition can be in vfs, common for `#[generate_trait]`, map it back to user code.
                let usage = FoundUsage::from_stable_ptr(db, stable_ptr);
                flow!(sink(usage));
            }
        }

        let search_for_self_usages = matches!(
            self.symbol_item,
            ResolvedItem::Concrete(ResolvedConcreteItem::Impl(_))
                | ResolvedItem::Concrete(ResolvedConcreteItem::SelfTrait(_))
                | ResolvedItem::Generic(ResolvedGenericItem::Trait(_))
        );

        let search_scope = self.in_scope.clone().unwrap_or_else(|| self.symbol.search_scope(db));

        let needle = match self.symbol {
            // Small optimisation for inline macros: we can be sure that any usages will have a `!`
            // at the end, so we do not need to search for occurrences without it.
            SymbolDef::ExprInlineMacro(macro_name) => format_smolstr!("{macro_name}!"),
            symbol => symbol.name(db),
        };

        let finder = Finder::new(needle.as_bytes());
        let self_finder = Finder::new(SELF_TYPE_KW.as_bytes());

        for (file, text, search_span) in search_scope.files_contents_and_spans(db) {
            let mut found_offsets: Vec<TextOffset> =
                Self::match_offsets(&finder, &text, search_span).collect();
            if search_for_self_usages {
                let mut self_usages_offsets =
                    Self::match_offsets(&self_finder, &text, search_span).collect();
                found_offsets.append(&mut self_usages_offsets);
            }
            // Search occurrences of the symbol's name.
            for offset in found_offsets {
                if let Some(node) = db.find_syntax_node_at_offset(file, offset) {
                    if let Some(identifier) = TerminalIdentifier::cast_token(db, node) {
                        flow!(self.found_identifier(db, identifier, sink));
                    }
                }
            }
        }
    }

    fn match_offsets<'b>(
        finder: &'b Finder<'b>,
        text: &'b str,
        search_span: Option<TextSpan>,
    ) -> impl Iterator<Item = TextOffset> + use<'b> {
        finder
            .find_iter(text.as_bytes())
            .map(|offset| TextWidth::at(text, offset).as_offset())
            .filter(move |&offset| {
                search_span.is_none_or(|span| span.start <= offset && offset <= span.end)
            })
            .filter(|offset| {
                // Reject matches that are not at word boundaries - for example, an identifier
                // `core` will never be a direct usage of a needle `or`.
                // This speeds up short identifiers significantly.
                let idx = offset.as_u32() as usize;
                !{
                    let char_before = text[..idx].chars().next_back();
                    char_before.is_some_and(|ch| ch.is_alphabetic() || ch == '_')
                } && !{
                    let char_after = text[idx + finder.needle().len()..].chars().next();
                    char_after.is_some_and(|ch| ch.is_alphanumeric() || ch == '_')
                }
            })
    }

    fn found_identifier(
        &self,
        db: &dyn SyntaxGroup,
        identifier: TerminalIdentifier,
        sink: &mut dyn FnMut(FoundUsage) -> ControlFlow<(), ()>,
    ) -> ControlFlow<(), ()> {
        // Declaration is not a usage, so filter it out.
        if Some(identifier.stable_ptr(self.db).untyped()) == self.symbol.definition_stable_ptr(db) {
            return ControlFlow::Continue(());
        }

        // Declaration search is used here to ensure that all appropriate impl items are included
        // when looking for usages of trait items.
        let found_symbol_definition =
            SymbolSearch::find_definition(self.db, &identifier).map(|ss| ss.def);
        let found_symbol_declaration =
            SymbolSearch::find_declaration(self.db, &identifier).map(|ss| ss.def);

        // Check if declaration or definition matches
        if found_symbol_definition.as_ref() == Some(self.symbol)
            || found_symbol_declaration.as_ref() == Some(self.symbol)
        {
            let usage = FoundUsage::from_syntax_node(self.db, identifier.as_syntax_node());
            sink(usage)
        } else {
            ControlFlow::Continue(())
        }
    }
}

impl FoundUsage {
    fn from_syntax_node(db: &AnalysisDatabase, syntax_node: SyntaxNode) -> Self {
        Self {
            file: syntax_node.stable_ptr(db).file_id(db),
            span: syntax_node.span_without_trivia(db),
        }
    }

    fn from_stable_ptr(db: &AnalysisDatabase, stable_ptr: SyntaxStablePtrId) -> Self {
        Self::from_syntax_node(db, stable_ptr.lookup(db))
    }

    /// Converts this object to a file-span tuple, losing any extra carried information.
    pub fn location(self) -> (FileId, TextSpan) {
        (self.file, self.span)
    }
}
