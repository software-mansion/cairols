use std::num::NonZeroU32;

use cairo_lang_filesystem::db::get_external_files;
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_utils::Intern;
use cairo_lang_utils::Upcast;
use lsp_types::{Location, Url};
use salsa::{Database, Id};
use tracing::error;

use crate::lang::lsp::ToLsp;

#[cfg(test)]
#[path = "ls_proto_group_test.rs"]
mod test;

/// Get a [`FileId`] from an [`Url`].
///
/// Returns `None` on failure, and errors are logged.
pub fn file_for_url<'db>(db: &'db dyn Database, uri: &Url) -> Option<FileId<'db>> {
    match uri.scheme() {
        "file" => uri
            .to_file_path()
            .inspect_err(|()| error!("invalid file url: {uri}"))
            .ok()
            .map(|path| FileLongId::OnDisk(path).intern(db)),
        "vfs" => uri
            .host_str()
            .or_else(|| {
                error!("invalid vfs url, missing host string: {uri:?}");
                None
            })?
            .parse::<NonZeroU32>()
            .inspect_err(|e| {
                error!("invalid vfs url, host string is not a valid integer, {e}: {uri:?}")
            })
            .ok()
            .map(|id| unsafe { Id::from_index(id.into()) })
            .map(FileId::from_intern_id),
        _ => {
            error!("invalid url, scheme is not supported by this language server: {uri:?}");
            None
        }
    }
}

/// Get the canonical [`Url`] for a [`FileId`].
pub fn url_for_file<'db>(db: &'db dyn Database, file_id: FileId<'db>) -> Option<Url> {
    let vf = match file_id.long(db) {
        FileLongId::OnDisk(path) => return Some(Url::from_file_path(path).unwrap()),
        FileLongId::Virtual(vf) => vf,
        FileLongId::External(id) => {
            &get_external_files(db.upcast()).try_ext_as_virtual(db.upcast(), *id)?
        }
    };
    // NOTE: The URL is constructed using setters and path segments in order to
    //   url-encode any funky characters in parts that LS is not controlling.
    let mut url = Url::parse("vfs://").unwrap();
    url.set_host(Some(&file_id.as_intern_id().index().to_string())).unwrap();
    url.path_segments_mut().unwrap().push(&format!("{}.cairo", vf.name));
    Some(url)
}

/// Converts a [`FileId`]-[`TextSpan`] pair into a [`Location`].
pub fn lsp_location<'db>(
    db: &'db dyn Database,
    (file, span): (FileId<'db>, TextSpan),
) -> Option<Location> {
    let found_uri = url_for_file(db, file)?;
    let range = span.position_in_file(db.upcast(), file)?.to_lsp();
    let location = Location { uri: found_uri, range };
    Some(location)
}
