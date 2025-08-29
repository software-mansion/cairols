mod printer;

use lsp_types::TextDocumentPositionParams;

use crate::ide::introspection::syntax_tree::printer::{
    file_syntax_tree, syntax_tree_branch_above_leaf,
};
use crate::lang::db::upstream::file_syntax;
use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::lsp::{LsProtoGroup, ToCairo};

pub fn get_syntax_tree_for_file(
    db: &AnalysisDatabase,
    params: TextDocumentPositionParams,
) -> Option<String> {
    let file_id = db.file_for_url(&params.text_document.uri)?;
    let file_syntax = file_syntax(db, file_id).ok()?;

    let full_tree = file_syntax_tree(db, &file_syntax);

    let position = params.position.to_cairo();
    let syntax_node_at_cursor = db.find_syntax_node_at_position(file_id, position);

    let branch_tree = syntax_node_at_cursor.map(|s| syntax_tree_branch_above_leaf(db, &s));

    Some(format!("{full_tree}\n\n{}", branch_tree.unwrap_or_default()))
}
