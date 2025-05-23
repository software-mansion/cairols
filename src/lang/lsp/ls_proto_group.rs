use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_utils::{LookupIntern, Upcast};
use lsp_types::{Location, Url};
use salsa::InternKey;
use tracing::error;

use crate::lang::lsp::ToLsp;

#[cfg(test)]
#[path = "ls_proto_group_test.rs"]
mod test;

pub trait LsProtoGroup: Upcast<dyn FilesGroup> {
    /// Get a [`FileId`] from an [`Url`].
    ///
    /// Returns `None` on failure, and errors are logged.
    fn file_for_url(&self, uri: &Url) -> Option<FileId> {
        match uri.scheme() {
            "file" => uri
                .to_file_path()
                .inspect_err(|()| error!("invalid file url: {uri}"))
                .ok()
                .map(|path| FileId::new(self.upcast(), path)),
            "vfs" => uri
                .host_str()
                .or_else(|| {
                    error!("invalid vfs url, missing host string: {uri:?}");
                    None
                })?
                .parse::<usize>()
                .inspect_err(|e| {
                    error!("invalid vfs url, host string is not a valid integer, {e}: {uri:?}")
                })
                .ok()
                .map(Into::into)
                .map(FileId::from_intern_id),
            _ => {
                error!("invalid url, scheme is not supported by this language server: {uri:?}");
                None
            }
        }
    }

    /// Get the canonical [`Url`] for a [`FileId`].
    fn url_for_file(&self, file_id: FileId) -> Option<Url> {
        let vf = match file_id.lookup_intern(self.upcast()) {
            FileLongId::OnDisk(path) => return Some(Url::from_file_path(path).unwrap()),
            FileLongId::Virtual(vf) => vf,
            FileLongId::External(id) => self.upcast().try_ext_as_virtual(id)?,
        };
        // NOTE: The URL is constructed using setters and path segments in order to
        //   url-encode any funky characters in parts that LS is not controlling.
        let mut url = Url::parse("vfs://").unwrap();
        url.set_host(Some(&file_id.as_intern_id().to_string())).unwrap();
        url.path_segments_mut().unwrap().push(&format!("{}.cairo", vf.name));
        Some(url)
    }

    /// Converts a [`FileId`]-[`TextSpan`] pair into a [`Location`].
    fn lsp_location(&self, (file, span): (FileId, TextSpan)) -> Option<Location> {
        let db = self.upcast();
        let found_uri = db.url_for_file(file)?;
        let range = span.position_in_file(db, file)?.to_lsp();
        let location = Location { uri: found_uri, range };
        Some(location)
    }
}

impl<T> LsProtoGroup for T where T: Upcast<dyn FilesGroup> + ?Sized {}
