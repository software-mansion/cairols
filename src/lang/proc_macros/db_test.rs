use cairo_lang_macro::{AllocationContext, Severity, Token};
use scarb_proc_macro_server_types::methods::SpannedToken;

use super::*;

fn token_stream(tokens: &[(&str, u32, u32)]) -> TokenStream {
    let ctx = AllocationContext::default();
    TokenStream::new(
        tokens
            .iter()
            .map(|(content, start, end)| {
                TokenTree::Ident(Token::new_in(content, TextSpan::new(*start, *end), &ctx))
            })
            .collect(),
    )
}

fn spans_of(token_stream: &TokenStream) -> Vec<(u32, u32)> {
    token_stream
        .tokens
        .iter()
        .map(|token| {
            let TokenTree::Ident(token) = token;
            (token.span.start, token.span.end)
        })
        .collect()
}

#[test]
fn input_spans_are_rebased_onto_the_item() {
    // An item further down the file still hashes to the same cache key as the same item at the
    // top of the file, because the stabilizer moves its first token to offset zero.
    let mut item = token_stream(&[("fn ", 100, 103), ("foo", 103, 106), ("() {}", 106, 111)]);
    let mut call_site = TextSpan::new(100, 111);

    SpansStabilizer::new(&mut call_site, &mut item);

    assert_eq!(spans_of(&item), vec![(0, 3), (3, 6), (6, 11)]);
    assert_eq!(call_site.start, SpansStabilizer::STABLE_CALL_SITE_START);
    // The call site keeps its width, so a macro can still tell how long it is.
    assert_eq!(call_site.end, 11);
}

#[test]
fn result_spans_are_moved_back_onto_the_original_file() {
    let mut item = token_stream(&[("fn ", 100, 103), ("foo", 103, 106)]);
    let mut call_site = TextSpan::new(100, 106);
    let stabilizer = SpansStabilizer::new(&mut call_site, &mut item);

    let result = ProcMacroResult {
        token_stream: SpannedTokenStream(vec![SpannedToken {
            content: "fn bar".to_string(),
            span: TextSpan::new(0, 6),
        }]),
        ..Default::default()
    };

    let result = stabilizer.apply_original_offsets_to_result(result);
    assert_eq!(result.token_stream.0[0].span, TextSpan::new(100, 106));
}

#[test]
fn spans_pointing_at_the_call_site_are_restored_verbatim() {
    // Macros report code they invented, rather than copied, against the call site. That marker
    // must come back as the original call site, not as an offset into the item.
    let mut item = token_stream(&[("struct S {}", 40, 51)]);
    let mut call_site = TextSpan::new(20, 33);
    let stabilizer = SpansStabilizer::new(&mut call_site, &mut item);

    let result = ProcMacroResult {
        token_stream: SpannedTokenStream(vec![SpannedToken {
            content: "impl S {}".to_string(),
            span: call_site.clone(),
        }]),
        ..Default::default()
    };

    let result = stabilizer.apply_original_offsets_to_result(result);
    assert_eq!(result.token_stream.0[0].span, TextSpan::new(20, 33));
}

#[test]
fn diagnostic_spans_are_moved_back_too() {
    let mut item = token_stream(&[("fn foo() {}", 200, 211)]);
    let mut call_site = TextSpan::new(200, 211);
    let stabilizer = SpansStabilizer::new(&mut call_site, &mut item);

    let result = ProcMacroResult {
        diagnostics: vec![Diagnostic::spanned(
            TextSpan::new(3, 6),
            Severity::Error,
            "bad name".to_string(),
        )],
        ..Default::default()
    };

    let result = stabilizer.apply_original_offsets_to_result(result);
    assert_eq!(result.diagnostics[0].span(), Some(TextSpan::new(203, 206)));
}

#[test]
fn an_empty_input_stream_is_handled() {
    // Nothing to rebase against, so offsets pass through unchanged rather than panicking.
    let mut item = TokenStream::empty();
    let mut call_site = TextSpan::new(0, 0);
    let stabilizer = SpansStabilizer::new(&mut call_site, &mut item);

    let result = ProcMacroResult {
        token_stream: SpannedTokenStream::unspanned("x", TextSpan::new(1, 2)),
        ..Default::default()
    };

    let result = stabilizer.apply_original_offsets_to_result(result);
    assert_eq!(result.token_stream.0[0].span, TextSpan::new(1, 2));
}
