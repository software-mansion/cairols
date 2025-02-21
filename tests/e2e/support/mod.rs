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
            use $crate::support::merge_json_flat;

            merge_json_flat(&mut workspace_configuration, $overriding_workspace_configuration);
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
/// Merges `b`'s kv pairs into `a` (non recursively), potentially overriding the previous values
pub(crate) fn merge_json_flat(a: &mut Value, b: Value) {
    if let (Value::Object(a_map), Value::Object(b_map)) = (a, b) {
        for (k, v) in b_map {
            a_map.insert(k, v);
        }
    } else {
        panic!("Non-object Value merging is not supported.");
    }
}

pub(crate) use sandbox;
