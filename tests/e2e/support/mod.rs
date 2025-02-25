pub mod cairo_project_toml;
pub mod client_capabilities;
pub mod cursor;
pub mod data;
pub mod fixture;
pub mod insta;
pub mod jsonrpc;
mod mock_client;
pub mod normalize;
pub mod scarb;

use serde_json::Value;

pub use self::cursor::cursors;
pub use self::mock_client::MockClient;

pub const TOOL_VERSIONS: &str = include_str!("../../../.tool-versions");

/// Create a sandboxed environment for testing language server features.
///
/// This macro creates a [`fixture::Fixture`] first and sets it up according to the provided
/// properties, and then runs a [`MockClient`] on it.
///
/// See actual tests for usage examples.
macro_rules! sandbox {
    (
        $(files { $($file:expr => $content:expr),* $(,)? })?
        $(cwd = $cwd:expr;)?
        $(client_capabilities = $client_capabilities:expr;)?
        $(workspace_configuration = $overriding_workspace_configuration:expr;)?
    ) => {{
        use $crate::support::{
            client_capabilities,
            fixture::Fixture,
            MockClient
        };

        let mut fixture = Fixture::new();

        fixture.add_file(".tool-versions", crate::support::TOOL_VERSIONS);

        $($(fixture.add_file($file, $content);)*)?


        #[allow(unused_mut)]
        let mut client_capabilities = client_capabilities::base();

        #[allow(unused_assignments, unused_mut)]
        let mut workspace_configuration = serde_json::json!({
            "cairo1": {
                "enableProcMacros": false,
                "enableLinter": false
            }
        });

        $(
            use $crate::support::merge_json;

            merge_json(&mut workspace_configuration, &$overriding_workspace_configuration);
        )?
        client_capabilities =
                client_capabilities::with_workspace_configuration(client_capabilities, true);

        $(
            client_capabilities = $client_capabilities(client_capabilities);
        )?

        let client = MockClient::start(fixture, client_capabilities, workspace_configuration);
        $(
            client.set_cwd($cwd);
        )?
        client
    }};
}

#[doc(hidden)]
/// Merges `b`'s kv pairs into `a`, potentially overriding the previous values
/// It takes nested maps into account, descending recursively to achieve merging nested objects
pub(crate) fn merge_json(a: &mut Value, b: &Value) {
    if let (Value::Object(a_map), Value::Object(b_map)) = (a, b) {
        for (k, v) in b_map {
            let v_in_a = a_map.get_mut(k);
            if let Some(value) = v_in_a {
                if value.is_object() {
                    merge_json(value, b_map.get(k).unwrap_or(&Value::Object(Default::default())));
                    continue;
                }
            }
            a_map.insert(k.clone(), v.clone());
        }
    } else {
        panic!("Non-object Value merging is not supported.");
    }
}

pub(crate) use sandbox;
