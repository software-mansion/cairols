use toml_edit::{Document, Item, Value};

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
