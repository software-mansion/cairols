use cairo_language_server::lsp::ext::ShowMemoryUsage;
use cairo_language_server::lsp::ext::testing_requests::{DatabaseSwapped, ForceDatabaseSwap};

use crate::support::sandbox;

#[test]
fn show_memory_usage_returns_structured_summary() {
    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => "[crate_roots]\napp = \"src\"",
            "src/lib.cairo" => "fn main() {}",
        }
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo");

    let report = ls.send_request::<ShowMemoryUsage>(());

    assert!(!report.summary.by_layer.is_empty());
    assert!(report.summary.totals.total_size >= report.summary.totals.size_of_metadata);
    assert!(!report.top_queries.is_empty() || !report.top_structs.is_empty());
}

#[test]
fn benchmark_request_can_force_database_swap() {
    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => "[crate_roots]\napp = \"src\"",
            "src/lib.cairo" => "fn main() {}",
        }
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo");

    let response = ls.send_request::<ForceDatabaseSwap>(());
    let notification = ls.wait_for_notification::<DatabaseSwapped>(|_| true);

    assert_eq!(response.reason, notification.reason);
}
