use cairo_lang_filesystem::db::{ext_as_virtual, get_parent_and_mapping};
use cairo_lang_filesystem::ids::{FileId, FileLongId, SmolStrId, SpanInFile};
use cairo_lang_filesystem::span::{TextPosition, TextPositionSpan};
use cairo_lang_utils::Intern;
use cairo_language_common::CommonGroup;
use lsp_types::{Location, Url};
use percent_encoding::percent_decode_str;
use salsa::Database;
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
        let db = self.as_dyn_database();
        match uri.scheme() {
            "file" => uri
                .to_file_path()
                .inspect_err(|()| error!("invalid file url: {uri}"))
                .ok()
                .map(|path| FileLongId::OnDisk(path).intern(db)),
            "vfs" => {
                // The path is `<location>/<name>.cairo`, see `url_for_file`. It is split while
                // still percent-encoded, so that a `/` inside the name does not count.
                let file = (|| {
                    let (location, name) = uri.path().rsplit_once('/')?;
                    let location = percent_decode_str(location).decode_utf8().ok()?;
                    let name =
                        percent_decode_str(name.strip_suffix(".cairo")?).decode_utf8().ok()?;
                    file_generated_at(db, &location, &name)
                })();
                if file.is_none() {
                    error!("vfs url does not correspond to any generated file: {uri:?}");
                }
                file
            }
            _ => {
                error!("invalid url, scheme is not supported by this language server: {uri:?}");
                None
            }
        }
    }

    /// Get the canonical [`Url`] for a [`FileId`].
    ///
    /// Virtual files are addressed the way the compiler names them in [`FileLongId::full_path`]:
    /// by the location in the parent they were generated from, which recursively names the
    /// parent the same way down to a file on disk. The file name is kept as the last path
    /// segment so that the editor uses it as the tab title:
    ///
    /// vfs:///<on-disk path>:<l>:<c>: <l>:<c>[<name>]:<l>:<c>: <l>:<c>/<name>.cairo
    ///
    /// Unlike a Salsa id, this description identifies the file in any analysis database, so the
    /// URL stays valid across the diagnostics database, database swaps and server restarts.
    fn url_for_file<'db>(&self, file_id: FileId<'db>) -> Option<Url> {
        let vf = match file_id.long(self) {
            FileLongId::OnDisk(path) => return Some(Url::from_file_path(path).unwrap()),
            FileLongId::Virtual(vf) => vf,
            FileLongId::External(id) => ext_as_virtual(self.as_dyn_database(), *id),
        };
        let Some(parent) = vf.parent else {
            error!("virtual file has no parent and cannot be exposed to the client");
            return None;
        };

        let mut location = String::new();
        parent.fmt_location(&mut location, self.as_dyn_database()).ok()?;

        // NOTE: The file name is pushed as a path segment in order to url-encode any funky
        //   characters in it. The `.cairo` suffix makes the editor treat the document as Cairo.
        let mut url = Url::parse("vfs:///").unwrap();
        url.set_path(&location);
        url.path_segments_mut()
            .unwrap()
            .push(&format!("{}.cairo", vf.name.to_string(self.as_dyn_database())));
        Some(url)
    }

    /// Converts a [`FileId`]-[`TextSpan`] pair into a [`Location`].
    fn lsp_location<'db>(&self, SpanInFile { file_id, span }: SpanInFile<'db>) -> Option<Location> {
        let found_uri = self.url_for_file(file_id)?;
        let range = span.position_in_file(self.as_dyn_database(), file_id)?.to_lsp();
        let location = Location { uri: found_uri, range };
        Some(location)
    }
}

impl<T: Database + ?Sized> LsProtoGroup for T {}

/// Resolves the file named `name` that was generated at `location`, as printed by
/// [`SpanInFile::fmt_location`]: `<parent full_path>:<line>:<col>: <line>:<col>`.
///
/// Resolving the parent first is the recursion: one level of macro expansion per call.
fn file_generated_at<'db>(
    db: &'db dyn Database,
    location: &str,
    name: &str,
) -> Option<FileId<'db>> {
    let (rest, end) = location.rsplit_once(": ")?;
    let (rest, start_col) = rest.rsplit_once(':')?;
    let (parent_path, start_line) = rest.rsplit_once(':')?;
    let (end_line, end_col) = end.split_once(':')?;

    let parent = file_by_full_path(db, parent_path)?;
    let span = TextPositionSpan {
        start: position(start_line, start_col)?,
        end: position(end_line, end_col)?,
    };
    let span = TextPositionSpan::offset_in_file(span, db, parent)?;
    find_generated_file(db, SpanInFile { file_id: parent, span }, name)
}

/// Resolves a file from its [`FileLongId::full_path`]:
/// a path on disk, or `<location>[<name>]` for a generated file.
fn file_by_full_path<'db>(db: &'db dyn Database, full_path: &str) -> Option<FileId<'db>> {
    match full_path.strip_suffix(']').and_then(|s| s.rsplit_once('[')) {
        None => Some(FileLongId::OnDisk(full_path.into()).intern(db)),
        Some((location, name)) => file_generated_at(db, location, name),
    }
}

/// Parses a 1-based `line` and `col` as printed by the compiler into a 0-based [`TextPosition`].
fn position(line: &str, col: &str) -> Option<TextPosition> {
    Some(TextPosition {
        line: line.parse::<usize>().ok()?.checked_sub(1)?,
        col: col.parse::<usize>().ok()?.checked_sub(1)?,
    })
}

/// Finds the virtual file named `name` that was generated from `origin`.
///
/// Every generated file is a descendant of the on-disk file its origin ultimately comes from,
/// and [`CommonGroup::file_and_subfiles_with_corresponding_modules`] collects all of them:
/// plugin expansions (attributes, derives) and inline macro expansions (item-level or inside
/// function bodies). It is a tracked query, so the files are generated on demand and only files
/// that exist are ever returned.
fn find_generated_file<'db>(
    db: &'db dyn Database,
    origin: SpanInFile<'db>,
    name: &str,
) -> Option<FileId<'db>> {
    let name = SmolStrId::from(db, name);
    let root = origin.user_location(db).file_id;
    let (files, _) = db.file_and_subfiles_with_corresponding_modules(root)?;
    files.iter().copied().find(|&file| {
        get_parent_and_mapping(db, file).is_some_and(|(parent, _)| parent == origin)
            && file.file_name(db) == name
    })
}
