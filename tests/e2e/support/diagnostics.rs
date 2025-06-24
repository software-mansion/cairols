use crate::support::MockClient;
use crate::support::cursor::text_chunk_at_range;
use lsp_types::{Diagnostic, Url};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DiagnosticsWithUrl {
    pub url: String,
    pub diagnostics: Vec<DiagnosticAndRelatedInfo>,
}

#[derive(Debug, Serialize)]
pub struct DiagnosticAndRelatedInfo {
    pub related_code: String,
    pub diagnostic: Diagnostic,
}

pub fn get_related_diagnostic_code(
    client: &MockClient,
    diagnostic: &Diagnostic,
    file_url: &Url,
) -> String {
    let fixture = client.as_ref();
    let file_content = fixture.read_file(file_url.path());
    text_chunk_at_range(file_content, diagnostic.range)
}
