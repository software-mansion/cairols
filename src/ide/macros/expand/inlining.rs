use crate::lang::db::AnalysisDatabase;
use cairo_lang_filesystem::{
    db::{FilesGroup, get_parent_and_mapping},
    ids::FileId,
    span::{TextOffset, TextSpan},
};
use cairo_lang_syntax::node::SyntaxNode;
use if_chain::if_chain;

pub fn new_span(
    db: &AnalysisDatabase,
    old_file: FileId,
    new_file: FileId,
    span: TextSpan,
) -> Option<TextSpan> {
    let old_len = TextOffset::from_str(&db.file_content(old_file)?);
    let new_len = TextOffset::from_str(&db.file_content(new_file)?);
    let suffix_len = old_len - span.end;
    let new_end = new_len.sub_width(suffix_len);

    Some(TextSpan { start: span.start, end: new_end })
}

#[derive(Clone, Copy, PartialEq)]
pub enum ExpandMacroInliningStrategy {
    Replace,
    Append,
}

impl ExpandMacroInliningStrategy {
    pub fn from_remove_original_item(remove_original_item: bool) -> Self {
        if remove_original_item {
            ExpandMacroInliningStrategy::Replace
        } else {
            ExpandMacroInliningStrategy::Append
        }
    }
}

#[derive(Clone)]
pub struct FileWithOrigin {
    pub file: FileId,
    pub generated_from: SyntaxNode,
    pub strategy: ExpandMacroInliningStrategy,
}

// Inline files with strategy `Replace`, starting from `file`, keep other files in `extra_files`.
pub fn apply_changes(
    db: &AnalysisDatabase,
    file: FileId,
    files: &[FileWithOrigin],
    extra_files: &mut Vec<String>,
) -> Option<String> {
    let mut result_content = db.file_content(file)?.to_string();

    let mut replacements = Vec::new();

    for file_with_origin in files {
        if_chain! {
            if let Some((parent, _)) = get_parent_and_mapping(db, file_with_origin.file);
            if parent == file;
            if let Some(child_content) = apply_changes(db, file_with_origin.file, files, extra_files);

            then {
                match file_with_origin.strategy {
                    ExpandMacroInliningStrategy::Replace => {
                        replacements.push((file_with_origin.generated_from.span(db).to_str_range(), child_content));
                    }
                    ExpandMacroInliningStrategy::Append => {
                        extra_files.push(child_content);
                    }
                }
            }
        };
    }

    replacements.sort_by(|a, b| b.0.start.cmp(&a.0.start));

    for (range, replacement_content) in replacements {
        result_content.replace_range(range.clone(), &replacement_content);
    }

    Some(result_content)
}
