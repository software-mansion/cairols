use cairo_lang_parser::db::ParserGroup;
use cairo_lang_parser::printer::print_tree;
use lsp_types::Url;

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;

pub fn get_syntax_tree_for_file(db: &AnalysisDatabase, uri: Url) -> Option<String> {
    let file_id = db.file_for_url(&uri)?;
    let file_syntax = db.file_syntax(file_id).ok()?;
    Some(print_tree(db, &file_syntax, true, false))
}
