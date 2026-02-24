use std::ops::Range;

use anyhow::{Result, anyhow};
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::span::{TextOffset, TextSpan, TextWidth};
use lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind};
use scarb_manifest_schema::get_shared_traverser;
use toml_edit::{Document, Item};

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::{LsProtoGroup, ToCairo, ToLsp};

pub(crate) fn hover(params: HoverParams, db: &AnalysisDatabase) -> Option<Hover> {
    let uri = &params.text_document_position_params.text_document.uri;
    let file_id = db.file_for_url(uri)?;
    let raw_toml = db.file_content(file_id)?;

    let position = params.text_document_position_params.position.to_cairo();
    let offset = position.offset_in_file(db, file_id)?.as_u32() as usize;

    if let Ok((toml_path, location)) = find_at_offset(raw_toml, offset) {
        let traverser = get_shared_traverser();

        let description = traverser
            .traverse(toml_path)
            .unwrap_or_default()
            .get("description")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned);

        let start = TextOffset::START.add_width(TextWidth::at(raw_toml, location.start));
        let end = TextOffset::START.add_width(TextWidth::at(raw_toml, location.end));
        let range = TextSpan::new(start, end).position_in_file(db, file_id)?.to_lsp();

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

fn find_at_offset(raw_toml: &str, offset: usize) -> Result<(Vec<String>, Range<usize>)> {
    let doc = Document::parse(raw_toml)?;
    let mut path: Vec<String> = Vec::new();
    find_item_at_offset_inner(doc.as_item(), offset, &mut path)
}

fn find_item_at_offset_inner(
    item: &Item,
    offset: usize,
    path: &mut Vec<String>,
) -> Result<(Vec<String>, Range<usize>)> {
    match item {
        Item::Table(table) => {
            for (k, child) in table.iter() {
                if let Some((key, value_item)) = table.get_key_value(k) {
                    if let Some(k_span) = key.span()
                        && k_span.contains(&offset)
                    {
                        let mut full = path.clone();
                        full.push(key.get().to_string());
                        return Ok((full, k_span));
                    }
                    if let Some(v_span) = value_item.span()
                        && v_span.contains(&offset)
                    {
                        let mut full = path.clone();
                        full.push(key.get().to_string());
                        return Ok((full, v_span));
                    }
                }
                match child {
                    Item::Table(inner) => {
                        path.push(k.to_string());
                        if let Ok(res) =
                            find_item_at_offset_inner(&Item::Table(inner.clone()), offset, path)
                        {
                            path.pop();
                            return Ok(res);
                        } else {
                            path.pop();
                        }
                    }
                    Item::ArrayOfTables(array) => {
                        path.push(k.to_string());
                        for t in array.iter() {
                            if let Ok(res) =
                                find_item_at_offset_inner(&Item::Table(t.clone()), offset, path)
                            {
                                path.pop();
                                return Ok(res);
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
                                return Ok((full, v_span));
                            }
                        }
                    }
                    Item::None => {
                        return Err(anyhow!(
                            "No key found for offset: {offset}, for path: {path:?}"
                        ));
                    }
                }
            }
        }
        _ => return Err(anyhow!("No key found for offset: {offset}, for path: {path:?}")),
    }
    Err(anyhow!("No key found for offset: {offset}, for path: {path:?}"))
}
