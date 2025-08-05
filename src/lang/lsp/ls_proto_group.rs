use std::num::NonZeroU32;

use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_utils::Intern;
use cairo_lang_utils::Upcast;
use lsp_types::{Location, Url};
use salsa::Id;
use tracing::error;

use crate::lang::lsp::ToLsp;

#[cfg(test)]
#[path = "ls_proto_group_test.rs"]
mod test;

pub trait LsProtoGroup: for<'db> Upcast<'db, dyn FilesGroup> {
    /// Get a [`FileId`] from an [`Url`].
    ///
    /// Returns `None` on failure, and errors are logged.
    fn file_for_url<'db>(&'db self, uri: &Url) -> Option<FileId<'db>> {
        match uri.scheme() {
            "file" => uri
                .to_file_path()
                .inspect_err(|()| error!("invalid file url: {uri}"))
                .ok()
                .map(|path| FileLongId::OnDisk(path).intern(self.upcast())),
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
                .map(|id| unsafe { Id::from_u32(id.into()) })
                .map(FileId::from_intern_id),
            _ => {
                error!("invalid url, scheme is not supported by this language server: {uri:?}");
                None
            }
        }
    }

    /// Get the canonical [`Url`] for a [`FileId`].
    fn url_for_file<'db>(&'db self, file_id: FileId<'db>) -> Option<Url> {
        let vf = match file_id.long(self.upcast()) {
            FileLongId::OnDisk(path) => return Some(Url::from_file_path(path).unwrap()),
            FileLongId::Virtual(vf) => vf,
            FileLongId::External(id) => &self.upcast().try_ext_as_virtual(*id)?,
        };
        // NOTE: The URL is constructed using setters and path segments in order to
        //   url-encode any funky characters in parts that LS is not controlling.
        let mut url = Url::parse("vfs://").unwrap();
        url.set_host(Some(&file_id.as_intern_id().as_u32().to_string())).unwrap();
        url.path_segments_mut().unwrap().push(&format!("{}.cairo", vf.name));
        Some(url)
    }

    /// Converts a [`FileId`]-[`TextSpan`] pair into a [`Location`].
    fn lsp_location<'db>(&'db self, (file, span): (FileId<'db>, TextSpan)) -> Option<Location> {
        let db = self.upcast();
        let found_uri = db.url_for_file(file)?;
        let range = span.position_in_file(db, file)?.to_lsp();
        let location = Location { uri: found_uri, range };
        Some(location)
    }
}

impl<T> LsProtoGroup for T where T: for<'db> Upcast<'db, dyn FilesGroup> + ?Sized {}
