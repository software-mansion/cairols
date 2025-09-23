use std::num::NonZeroU32;

use cairo_lang_filesystem::db::ext_as_virtual;
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_utils::Intern;
use lsp_types::{Location, Url};
use salsa::{Database, Id};
use tracing::error;

use crate::lang::lsp::ToLsp;

#[cfg(test)]
#[path = "ls_proto_group_test.rs"]
mod test;

pub trait LsProtoGroup: Database {
    /// Get a [`FileId`] from an [`Url`].
    ///
    /// Returns `None` on failure, and errors are logged.
    fn file_for_url<'db>(&'db self, uri: &Url) -> Option<FileId<'db>> {
        match uri.scheme() {
            "file" => uri
                .to_file_path()
                .inspect_err(|()| error!("invalid file url: {uri}"))
                .ok()
                .map(|path| FileLongId::OnDisk(path).intern(self.as_dyn_database())),
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
    fn url_for_file<'db>(&self, file_id: FileId<'db>) -> Option<Url> {
        let vf = match file_id.long(self) {
            FileLongId::OnDisk(path) => return Some(Url::from_file_path(path).unwrap()),
            FileLongId::Virtual(vf) => vf,
            FileLongId::External(id) => ext_as_virtual(self.as_dyn_database(), *id),
        };
        // NOTE: The URL is constructed using setters and path segments in order to
        //   url-encode any funky characters in parts that LS is not controlling.
        let mut url = Url::parse("vfs://").unwrap();
        url.set_host(Some(&file_id.as_intern_id().index().to_string())).unwrap();
        url.path_segments_mut()
            .unwrap()
            .push(&format!("{}.cairo", vf.name.to_string(self.as_dyn_database())));
        Some(url)
    }

    /// Converts a [`FileId`]-[`TextSpan`] pair into a [`Location`].
    fn lsp_location<'db>(&self, (file, span): (FileId<'db>, TextSpan)) -> Option<Location> {
        let found_uri = self.url_for_file(file)?;
        let range = span.position_in_file(self.as_dyn_database(), file)?.to_lsp();
        let location = Location { uri: found_uri, range };
        Some(location)
    }
}

impl<T: Database + ?Sized> LsProtoGroup for T {}
