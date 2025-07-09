use crate::lang::db::AnalysisDatabase;
use cairo_lang_filesystem::{
    db::{FilesGroup, get_parent_and_mapping},
    ids::FileId,
    span::{TextOffset, TextSpan},
};
use cairo_lang_syntax::node::SyntaxNode;

/// Computes the corresponding span in a new file after inlining modifications from an old file.
///
/// # Example
/// Suppose the old file contains the text "foo1 foo2 foo3 foo4" when the span is on "foo2 foo3" and modifications changed the text
/// to "foo1 foo2 new foo foo3 foo4". Then the new span would now cover "foo2 new foo foo3".
///
/// # Note
/// This function assumes that the modification does not alter the text before or after the original span.
/// It is always true if we perform inlining of lower vfs into this one.
pub fn span_after_inlining(
    db: &AnalysisDatabase,
    old_file: FileId,
    new_file: FileId,
    old_file_span: TextSpan,
) -> Option<TextSpan> {
    let old_len = TextOffset::from_str(&db.file_content(old_file)?);
    let new_len = TextOffset::from_str(&db.file_content(new_file)?);

    Some(TextSpan {
        start: old_file_span.start,
        end: new_len.sub_width(old_len - old_file_span.end),
    })
}

#[derive(Clone)]
pub struct FileWithOrigin {
    pub file: FileId,
    pub generated_from: SyntaxNode,
}

// Inline `files`, starting from `file`.
pub fn inline_files(
    db: &AnalysisDatabase,
    file: FileId,
    files: &[FileWithOrigin],
) -> Option<String> {
    let mut result_content = db.file_content(file)?.to_string();

    let mut replacements = Vec::new();

    for file_with_origin in files {
        if let Some((parent, _)) = get_parent_and_mapping(db, file_with_origin.file)
            && parent == file
            && let Some(child_content) = inline_files(db, file_with_origin.file, files)
        {
            replacements
                .push((file_with_origin.generated_from.span(db).to_str_range(), child_content));
        };
    }

    replacements.sort_by(|a, b| b.0.start.cmp(&a.0.start));

    for (range, replacement_content) in replacements {
        result_content.replace_range(range.clone(), &replacement_content);
    }

    Some(result_content)
}
