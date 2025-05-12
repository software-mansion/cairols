use crate::support::MockClient;
use crate::support::cursor::text_chunk_at_range;
use cairo_language_server::lsp::ext::{ProvideVirtualFile, ProvideVirtualFileRequest};
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
    client: &mut MockClient,
    diagnostic: &Diagnostic,
    file_url: &Url,
) -> String {
    let fixture = client.as_ref();
    let file_content = fixture.maybe_read_file(file_url.path()).unwrap_or_else(|| {
        client
            .send_request::<ProvideVirtualFile>(ProvideVirtualFileRequest { uri: file_url.clone() })
            .content
            .unwrap()
    });
    text_chunk_at_range(file_content, diagnostic.range)
}
