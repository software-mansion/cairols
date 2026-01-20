//! Utilities for building Markdown texts.
//!
//! Markdown is used because this is the format used by the LSP protocol for rich text.

use std::ops::Range;

use pulldown_cmark::{BrokenLink, Event, LinkType, Options, Parser as MarkdownParser, Tag, TagEnd};

/// Horizontal rule.
pub const RULE: &str = "---\n";
/// or //! - both have 3 characters we often want to skip.
pub const COMMENT_TOKEN_PREFIX_LEN: usize = 3;

/// Surround the given code with `cairo` fenced code block.
pub fn fenced_code_block(code: &str) -> String {
    if code.trim().is_empty() {
        return String::new();
    }
    format!("```cairo\n{code}\n```\n")
}

pub struct DocLink {
    /// Byte offsets into the original markdown content (not the link label text).
    /// Example: in "See [`crate::Foo`]" the range covers the full "[`crate::Foo`]" span.
    pub range: Range<usize>,
    /// Byte offsets into the original markdown content for the link label itself.
    /// Example: in "See [`crate::Foo`]" the label range covers "crate::Foo".
    pub label_range: Range<usize>,
    /// Only the first label chunk is captured; mixed code/text labels are ignored.
    pub label_text: String,
}

pub fn parse_doc_links(content: &str) -> Vec<DocLink> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    // Force unresolved reference links to emit `Link` events so we can still parse their ranges.
    let mut replacer = |broken_link: BrokenLink<'_>| {
        if matches!(broken_link.link_type, LinkType::ShortcutUnknown | LinkType::Shortcut) {
            return Some((broken_link.reference.to_string().into(), "".into()));
        }
        None
    };
    let parser =
        MarkdownParser::new_with_broken_link_callback(content, options, Some(&mut replacer));

    let mut results = Vec::new();
    let mut link_start: Option<usize> = None;
    let mut label_range: Option<Range<usize>> = None;
    let mut label_text: Option<String> = None;
    let mut in_link = false;

    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(Tag::Link { .. }) => {
                link_start = Some(range.start);
                label_range = None;
                label_text = None;
                in_link = true;
            }
            Event::End(TagEnd::Link) => {
                if let Some(start) = link_start.take() {
                    let label_range = label_range.take().unwrap_or(start..start);
                    let label_text = label_text.take().unwrap_or_default();
                    results.push(DocLink { range: start..range.end, label_range, label_text });
                }
                in_link = false;
            }
            Event::Text(text) if in_link && label_range.is_none() => {
                label_range = Some(range.start..range.end);
                label_text = Some(text.to_string());
            }
            Event::Code(text) if in_link && label_range.is_none() => {
                label_range = Some(range.start..range.end);
                label_text = Some(text.to_string());
            }
            _ => {}
        }
    }

    results
}
