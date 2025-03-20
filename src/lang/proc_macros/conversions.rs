use cairo_lang_macro::TokenStream as TokenStreamV2;
use cairo_lang_macro_v1::{
    TokenStream as TokenStreamV1, TokenStreamMetadata as TokenStreamMetadataV1,
};

pub fn token_stream_v2_to_v1(token_stream_v2: &TokenStreamV2) -> TokenStreamV1 {
    let metadata_v2 = token_stream_v2.metadata.clone();
    let token_stream = TokenStreamV1::new(token_stream_v2.to_string());
    token_stream.with_metadata(TokenStreamMetadataV1 {
        original_file_path: metadata_v2.original_file_path,
        file_id: metadata_v2.file_id,
    })
}
