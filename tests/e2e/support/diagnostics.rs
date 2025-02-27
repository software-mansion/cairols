use crate::support::MockClient;
use crate::support::cursor::text_chunk_at_range;
use lsp_types::{Diagnostic, Url};

pub fn get_related_diagnostic_code(
    client: &MockClient,
    diagnostic: &Diagnostic,
    file_url: &Url,
) -> String {
    let fixture = client.as_ref();
    let file_content = fixture.read_file(file_url.path());
    text_chunk_at_range(file_content, diagnostic.range)
}
