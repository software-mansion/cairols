use toml_edit::{Document, InlineTable, Item, Table, Value};

// Remove a logical key path and return the rewritten manifest.
// Example: ["dependencies", "dep", "git"] removes `git` from
// `dep = { git = "...", path = "../dep" }`.
pub fn remove_key_path(raw_toml: &str, path: &[String]) -> Option<String> {
    let mut doc = Document::parse(raw_toml.to_owned()).ok()?.into_mut();
    remove_key_from_item(doc.as_item_mut(), path).then(|| doc.to_string())
}

// Replace a logical key path value and return the rewritten manifest.
pub fn replace_key_path_value(raw_toml: &str, path: &[String], new_value: Value) -> Option<String> {
    let mut doc = Document::parse(raw_toml.to_owned()).ok()?.into_mut();
    replace_key_value_in_item(doc.as_item_mut(), path, new_value).then(|| doc.to_string())
}

// Map a byte offset to the logical TOML key path at that location.
// Example: an offset inside `git` in `dep = { git = "...", path = "../dep" }`
// returns ["dependencies", "dep", "git"] when called from the `[dependencies]` table.
pub fn find_key_path_at_offset(raw_toml: &str, offset: usize) -> Option<Vec<String>> {
    let doc = Document::parse(raw_toml.to_owned()).ok()?;
    let mut path = Vec::new();
    find_key_path_in_item(doc.as_item(), offset, &mut path)
}

// Remove the last segment from either a regular table or an inline table.
fn remove_key_from_item(item: &mut Item, path: &[String]) -> bool {
    let Some((head, tail)) = path.split_first() else {
        return false;
    };

    if tail.is_empty() {
        if let Some(table) = item.as_table_mut() {
            return table.remove(head).is_some();
        }
        if let Some(inline) = item.as_value_mut().and_then(Value::as_inline_table_mut) {
            return inline.remove(head).is_some();
        }
        return false;
    }

    if let Some(table) = item.as_table_mut() {
        return table.get_mut(head).is_some_and(|child| remove_key_from_item(child, tail));
    }

    if let Some(inline) = item.as_value_mut().and_then(Value::as_inline_table_mut) {
        return inline
            .get_key_value_mut(head)
            .is_some_and(|(_, child)| remove_key_from_item(child, tail));
    }

    false
}

// Replace the last segment in a logical key path inside either a regular table or an inline table.
fn replace_key_value_in_item(item: &mut Item, path: &[String], new_value: Value) -> bool {
    let Some((head, tail)) = path.split_first() else {
        return false;
    };

    if tail.is_empty() {
        if let Some(table) = item.as_table_mut()
            && let Some(entry) = table.get_mut(head)
        {
            *entry = Item::Value(new_value);
            return true;
        }

        if let Some(inline) = item.as_value_mut().and_then(Value::as_inline_table_mut)
            && let Some((_, entry)) = inline.get_key_value_mut(head)
        {
            *entry = Item::Value(new_value);
            return true;
        }

        return false;
    }

    if let Some(table) = item.as_table_mut() {
        return table
            .get_mut(head)
            .is_some_and(|child| replace_key_value_in_item(child, tail, new_value));
    }

    if let Some(inline) = item.as_value_mut().and_then(Value::as_inline_table_mut) {
        return inline
            .get_key_value_mut(head)
            .is_some_and(|(_, child)| replace_key_value_in_item(child, tail, new_value));
    }

    false
}

// Only table-like nodes can contribute key segments.
fn find_key_path_in_item(
    item: &Item,
    offset: usize,
    path: &mut Vec<String>,
) -> Option<Vec<String>> {
    if let Some(table) = item.as_table() {
        return find_key_path_in_table(table, offset, path);
    }

    if let Some(inline) = item.as_value().and_then(Value::as_inline_table) {
        return find_key_path_in_inline_table(inline, offset, path);
    }

    None
}

// Walk a regular table and keep building the current logical path.
fn find_key_path_in_table(
    table: &Table,
    offset: usize,
    path: &mut Vec<String>,
) -> Option<Vec<String>> {
    for (k, child) in table.iter() {
        let Some((key, value_item)) = table.get_key_value(k) else {
            continue;
        };
        if let Some(found) = find_matching_key_path(key, offset, path) {
            return Some(found);
        }

        if let Some(found) = find_nested_inline_key_path(value_item, key.get(), offset, path) {
            return Some(found);
        }

        if let Some(inner) = child.as_table() {
            path.push(key.get().to_string());
            if let Some(found) = find_key_path_in_table(inner, offset, path) {
                path.pop();
                return Some(found);
            }
            path.pop();
        }
    }

    None
}

// Walk an inline table using the same path rules as regular tables.
fn find_key_path_in_inline_table(
    table: &InlineTable,
    offset: usize,
    path: &mut Vec<String>,
) -> Option<Vec<String>> {
    for (k, _) in table.iter() {
        let Some((key, value_item)) = table.get_key_value(k) else {
            continue;
        };
        if let Some(found) = find_matching_key_path(key, offset, path) {
            return Some(found);
        }

        if let Some(found) = find_nested_inline_key_path(value_item, key.get(), offset, path) {
            return Some(found);
        }
    }

    None
}

// Push the parent key, recurse into the nested inline table, then restore the path.
fn find_nested_inline_key_path(
    value_item: &Item,
    key: &str,
    offset: usize,
    path: &mut Vec<String>,
) -> Option<Vec<String>> {
    let inline = value_item.as_value().and_then(Value::as_inline_table)?;

    path.push(key.to_string());
    let found = find_key_path_in_inline_table(inline, offset, path);
    path.pop();
    found
}

// If the offset points at this key token, return the full path to it.
fn find_matching_key_path(
    key: &toml_edit::Key,
    offset: usize,
    path: &[String],
) -> Option<Vec<String>> {
    key.span().filter(|span| span.contains(&offset)).map(|_| {
        let mut full = path.to_vec();
        full.push(key.get().to_string());
        full
    })
}
