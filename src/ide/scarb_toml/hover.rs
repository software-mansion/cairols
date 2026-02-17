use std::ops::Range;

use cairo_lang_filesystem::db::FilesGroup;
use lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind};
use scarb_manifest_schema::get_shared_traverser;
use toml_edit::{Document, Item};

use super::utils::{byte_offset_to_lsp_position, lsp_position_to_byte_offset};
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;

pub(crate) fn hover(params: HoverParams, db: &AnalysisDatabase) -> Option<Hover> {
    let uri = &params.text_document_position_params.text_document.uri;
    let file_id = db.file_for_url(uri)?;
    let raw_toml = db.file_content(file_id)?;
    let position = params.text_document_position_params.position;
    let offset = lsp_position_to_byte_offset(raw_toml, position);

    if let Some((toml_path, location)) = find_at_offset(raw_toml, offset) {
        let traverser = get_shared_traverser();

        let description = traverser
            .traverse(toml_path)
            .unwrap_or_default()
            .get("description")
            .map(|v: &serde_json::Value| v.to_string());

        let range = lsp_types::Range {
            start: byte_offset_to_lsp_position(raw_toml, location.start),
            end: byte_offset_to_lsp_position(raw_toml, location.end),
        };

        return Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: description?,
            }),
            range: Some(range),
        });
    }
    None
}

fn find_at_offset(raw_toml: &str, offset: usize) -> Option<(Vec<String>, Range<usize>)> {
    let doc = Document::parse(raw_toml).expect("cannot parse TOML");
    let mut path: Vec<String> = Vec::new();
    search_item_with_path(doc.as_item(), offset, &mut path)
}

fn search_item_with_path(
    item: &Item,
    offset: usize,
    path: &mut Vec<String>,
) -> Option<(Vec<String>, Range<usize>)> {
    match item {
        Item::Table(table) => {
            for (k, child) in table.iter() {
                if let Some((key, value_item)) = table.get_key_value(k) {
                    if let Some(k_span) = key.span()
                        && k_span.contains(&offset)
                    {
                        let mut full = path.clone();
                        full.push(key.get().to_string());
                        return Some((full, k_span));
                    }
                    if let Some(v_span) = value_item.span()
                        && v_span.contains(&offset)
                    {
                        let mut full = path.clone();
                        full.push(key.get().to_string());
                        return Some((full, v_span));
                    }
                }
                match child {
                    Item::Table(inner) => {
                        path.push(k.to_string());
                        if let Some(res) =
                            search_item_with_path(&Item::Table(inner.clone()), offset, path)
                        {
                            path.pop();
                            return Some(res);
                        } else {
                            path.pop();
                        }
                    }
                    Item::ArrayOfTables(array) => {
                        path.push(k.to_string());
                        for t in array.iter() {
                            if let Some(res) =
                                search_item_with_path(&Item::Table(t.clone()), offset, path)
                            {
                                path.pop();
                                return Some(res);
                            }
                        }
                        path.pop();
                    }
                    Item::Value(v) => {
                        if let Some(v_span) = v.span()
                            && v_span.contains(&offset)
                        {
                            // Return the key name for this value
                            if let Some((key, _)) = table.get_key_value(k) {
                                let mut full = path.clone();
                                full.push(key.get().to_string());
                                return Some((full, v_span));
                            }
                        }
                    }
                    Item::None => {}
                }
            }
            None
        }
        Item::ArrayOfTables(array) => {
            for table in array.iter() {
                if let Some(res) = search_item_with_path(&Item::Table(table.clone()), offset, path)
                {
                    return Some(res);
                }
            }
            None
        }
        Item::Value(v) => v.span().map(|s| (path.clone(), s)),
        _ => None,
    }
}
