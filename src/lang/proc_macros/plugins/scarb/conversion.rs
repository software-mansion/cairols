use cairo_lang_macro::TextSpan;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};

pub trait SpanSource<'db> {
    fn text_span(&self, db: &'db dyn Database) -> TextSpan;
}

impl<'db, T: TypedSyntaxNode<'db>> SpanSource<'db> for T {
    fn text_span(&self, db: &'db dyn Database) -> TextSpan {
        let node = self.as_syntax_node();
        let span = node.span_without_trivia(db);
        TextSpan::new(span.start.as_u32(), span.end.as_u32())
    }
}

pub struct CallSiteLocation<'db> {
    pub stable_ptr: SyntaxStablePtrId<'db>,
    pub span: TextSpan,
}

impl<'db> CallSiteLocation<'db> {
    pub fn new<T: TypedSyntaxNode<'db>>(node: &T, db: &'db dyn Database) -> Self {
        Self { stable_ptr: node.stable_ptr(db).untyped(), span: node.text_span(db) }
    }
}

use cairo_lang_filesystem::ids::{CodeMapping, CodeOrigin};
use cairo_lang_filesystem::span::{
    TextOffset as CairoTextOffset, TextSpan as CairoTextSpan, TextWidth as CairoTextWidth,
};
use salsa::Database;
use scarb_proc_macro_server_types::methods::{
    CodeMapping as InterfaceCodeMapping, CodeOrigin as InterfaceCodeOrigin,
    TextOffset as InterfaceTextOffset, TextSpan as InterfaceTextSpan,
};

fn text_offset_from_proc_macro_server(
    proc_macro_server_text_offset: InterfaceTextOffset,
) -> CairoTextOffset {
    CairoTextOffset::START.add_width(CairoTextWidth::new_for_testing(proc_macro_server_text_offset))
}

fn text_span_from_proc_macro_server(
    proc_macro_server_text_span: InterfaceTextSpan,
) -> CairoTextSpan {
    CairoTextSpan {
        start: text_offset_from_proc_macro_server(proc_macro_server_text_span.start),
        end: text_offset_from_proc_macro_server(proc_macro_server_text_span.end),
    }
}

pub fn code_mapping_from_proc_macro_server(
    proc_macro_server_mapping: InterfaceCodeMapping,
) -> CodeMapping {
    CodeMapping {
        span: text_span_from_proc_macro_server(proc_macro_server_mapping.span),
        origin: match proc_macro_server_mapping.origin {
            InterfaceCodeOrigin::Start(v) => {
                CodeOrigin::Start(text_offset_from_proc_macro_server(v))
            }
            InterfaceCodeOrigin::Span(v) => CodeOrigin::Span(text_span_from_proc_macro_server(v)),
            InterfaceCodeOrigin::CallSite(v) => {
                CodeOrigin::CallSite(text_span_from_proc_macro_server(v))
            }
        },
    }
}
