use cairo_lang_formatter::FormatterConfig;
use cairo_lint::CairoLintToolMetadata;
use scarb_metadata::PackageMetadata;
use serde_json::Value;

#[cfg(test)]
#[path = "package_config_test.rs"]
mod package_config_test;

#[derive(Debug, Clone, Default)]
pub struct PackageConfig {
    pub fmt: FormatterConfig,
    pub lint: CairoLintToolMetadata,
    pub compiler_config_correct_for_debugging: bool,
}

impl PackageConfig {
    pub fn from_pkg(pkg: &PackageMetadata, compiler_config: &Value) -> Self {
        Self {
            fmt: Self::fmt(pkg),
            lint: Self::lint(pkg),
            compiler_config_correct_for_debugging: check_compiler_config_for_debugging(
                compiler_config,
            ),
        }
    }

    fn fmt(pkg: &PackageMetadata) -> FormatterConfig {
        let mut fmt = serde_json::to_value(FormatterConfig::default()).unwrap();
        let fmt_scarb_config = pkg.tool_metadata("fmt").unwrap_or(&Value::Null);

        merge_serde_json_value(&mut fmt, fmt_scarb_config);

        serde_json::from_value(fmt).unwrap_or_default()
    }

    fn lint(pkg: &PackageMetadata) -> CairoLintToolMetadata {
        let mut lint = serde_json::to_value(CairoLintToolMetadata::default()).unwrap();
        let lint_scarb_config = pkg.tool_metadata("cairo-lint").unwrap_or(&Value::Null);

        merge_serde_json_value(&mut lint, lint_scarb_config);

        serde_json::from_value(lint).unwrap_or_default()
    }
}

fn merge_serde_json_value(a: &mut Value, b: &Value) {
    if let (Value::Object(a_map), Value::Object(b_map)) = (a, b) {
        for (b_key, b_val) in b_map {
            if let Some(a_val) = a_map.get_mut(b_key) {
                if a_val.is_object() && b_val.is_object() {
                    merge_serde_json_value(a_val, b_val);
                } else {
                    *a_val = b_val.clone();
                }
            } else {
                // Needed for `a` and `b` created from hashmaps.
                a_map.insert(b_key.clone(), b_val.clone());
            }
        }
    }
}

fn check_compiler_config_for_debugging(config: &Value) -> bool {
    let bool_field = |key| config.get(key).and_then(|v| v.as_bool()).unwrap_or(false);
    let str_field = |key| config.get(key).and_then(|v| v.as_str()).unwrap_or("");

    bool_field("add_functions_debug_info")
        && bool_field("add_statements_code_locations_debug_info")
        && bool_field("add_statements_functions_debug_info")
        && bool_field("add_types_debug_info")
        && str_field("compiler_optimizations") == "Disabled"
}
