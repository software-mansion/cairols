use toml_edit::{Document, DocumentMut, Item, Value};

// Remove `[patch]` from a member manifest and merge it into the workspace root manifest.
pub fn move_patch_to_workspace_root(
    member_raw_toml: &str,
    workspace_root_raw_toml: &str,
) -> Option<(String, String)> {
    let mut member_doc = Document::parse(member_raw_toml.to_owned()).ok()?.into_mut();
    let patch = member_doc.as_table_mut().remove("patch")?;

    let mut workspace_root_doc =
        Document::parse(workspace_root_raw_toml.to_owned()).ok()?.into_mut();
    if let Some(existing_patch) = workspace_root_doc.as_table_mut().get_mut("patch") {
        merge_patch_items(existing_patch, &patch)?;
    } else {
        workspace_root_doc.as_table_mut().insert("patch", patch);
    }

    Some((member_doc.to_string(), workspace_root_doc.to_string()))
}

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

fn merge_patch_items(target: &mut Item, patch: &Item) -> Option<()> {
    let target = target.as_table_mut()?;
    let patch = patch.as_table()?;

    for (source, source_patches) in patch.iter() {
        if let Some(target_patches) = target.get_mut(source) {
            merge_patch_source(target_patches, source_patches)?;
        } else {
            target.insert(source, source_patches.clone());
        }
    }

    Some(())
}

// Merge entries for the same patch source, rejecting conflicting dependencies. Dependency
// specifications can be written as inline tables or regular TOML tables; compare their parsed
// values instead of descending into them and combining mutually exclusive fields such as `git`
// and `path`.
fn merge_patch_source(target: &mut Item, patch: &Item) -> Option<()> {
    let target = target.as_table_mut()?;
    let patch = patch.as_table()?;

    for (dependency, specification) in patch.iter() {
        if let Some(existing) = target.get(dependency) {
            if !patch_specifications_equal(existing, specification) {
                return None;
            }
        } else {
            target.insert(dependency, specification.clone());
        }
    }

    Some(())
}

fn patch_specifications_equal(left: &Item, right: &Item) -> bool {
    fn as_toml_value(item: &Item) -> Option<toml::Value> {
        let mut document = DocumentMut::new();
        document.as_table_mut().insert("dependency", item.clone());
        toml::from_str::<toml::Table>(&document.to_string()).ok()?.remove("dependency")
    }

    as_toml_value(left).zip(as_toml_value(right)).is_some_and(|(left, right)| left == right)
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
