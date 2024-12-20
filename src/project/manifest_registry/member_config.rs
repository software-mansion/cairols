use cairo_lang_formatter::FormatterConfig;
use scarb_metadata::PackageMetadata;
use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub struct MemberConfig {
    pub fmt: FormatterConfig,
}

impl MemberConfig {
    pub fn from_pkg(pkg: &PackageMetadata) -> Self {
        Self { fmt: Self::fmt(pkg) }
    }

    fn fmt(pkg: &PackageMetadata) -> FormatterConfig {
        let mut fmt = serde_json::to_value(FormatterConfig::default()).unwrap();
        let fmt_scarb_config = pkg.tool_metadata("fmt").unwrap_or(&Value::Null);

        merge_serde_json_value(&mut fmt, fmt_scarb_config);

        serde_json::from_value(fmt).unwrap_or_default()
    }
}

fn merge_serde_json_value(a: &mut Value, b: &Value) {
    if let (Value::Object(ref mut a_map), Value::Object(ref b_map)) = (a, b) {
        for (b_key, b_val) in b_map {
            if let Some(a_val) = a_map.get_mut(b_key) {
                if a_val.is_object() && b_val.is_object() {
                    merge_serde_json_value(a_val, b_val);
                } else {
                    *a_val = b_val.clone();
                }
            }
        }
    }
}
