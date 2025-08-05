use std::collections::HashMap;

use cairo_lang_diagnostics::{
    DiagnosticEntry, DiagnosticLocation, Diagnostics, PluginFileDiagnosticNotes, Severity,
};
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_utils::Upcast;
use lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString, Range,
    Url,
};
use tracing::{error, trace};

use crate::lang::lsp::{LsProtoGroup, ToLsp};

/// Converts internal diagnostics to LSP format.
pub fn map_cairo_diagnostics_to_lsp<'db, T>(
    db: &'db T::DbType,
    diags: &mut HashMap<(Url, FileId<'db>), Vec<Diagnostic>>,
    diagnostics: &Diagnostics<'db, T>,
    trace_macro_diagnostics: bool,
    plugin_file_notes: &PluginFileDiagnosticNotes<'db>,
) where
    T::DbType: salsa::Database + for<'a> Upcast<'a, dyn FilesGroup>,
    T: DiagnosticEntry<'db> + salsa::Update,
{
    for diagnostic in if trace_macro_diagnostics {
        diagnostics.get_all()
    } else {
        diagnostics.get_diagnostics_without_duplicates(db)
    } {
        let mut message = diagnostic.format(db);
        let diagnostic_location = diagnostic.location(db);
        let (_, parent_file_notes) =
            diagnostic_location.user_location_with_plugin_notes(db.upcast(), plugin_file_notes);
        let mut related_information = vec![];
        for note in diagnostic.notes(db).iter().chain(&parent_file_notes) {
            if let Some(location) = &note.location {
                let Some((range, file_id)) = get_mapped_range_and_add_mapping_note(
                    db,
                    location,
                    trace_macro_diagnostics.then_some(&mut related_information),
                    "Next note mapped from here.",
                ) else {
                    continue;
                };
                let Some(uri) = db.url_for_file(file_id) else {
                    trace!("url for file not found: {:?}", file_id.long(db));
                    continue;
                };
                related_information.push(DiagnosticRelatedInformation {
                    location: Location { uri, range },
                    message: note.text.clone(),
                });
            } else {
                message += &format!("\nnote: {}", note.text);
            }
        }

        let Some((range, mapped_file_id)) = get_mapped_range_and_add_mapping_note(
            db,
            &diagnostic.location(db),
            trace_macro_diagnostics.then_some(&mut related_information),
            "Diagnostic mapped from here.",
        ) else {
            continue;
        };

        let diagnostic = Diagnostic {
            range,
            message,
            related_information: (!related_information.is_empty()).then_some(related_information),
            severity: Some(match diagnostic.severity() {
                Severity::Error => DiagnosticSeverity::ERROR,
                Severity::Warning => DiagnosticSeverity::WARNING,
            }),
            code: diagnostic.error_code().map(|code| NumberOrString::String(code.to_string())),
            ..Diagnostic::default()
        };
        let Some(mapped_file_url) = db.url_for_file(mapped_file_id) else {
            continue;
        };

        diags.entry((mapped_file_url, mapped_file_id)).or_default().push(diagnostic);
    }
}

/// Returns the mapped range of a location, optionally adds a note about the mapping of the
/// location.
fn get_mapped_range_and_add_mapping_note<'db>(
    db: &'db (impl for<'a> Upcast<'a, dyn FilesGroup> + ?Sized),
    orig: &DiagnosticLocation<'db>,
    related_info: Option<&mut Vec<DiagnosticRelatedInformation>>,
    message: &str,
) -> Option<(Range, FileId<'db>)> {
    let mapped = orig.user_location(db.upcast());
    let mapped_range = get_lsp_range(db.upcast(), &mapped)?;
    if let Some(related_info) = related_info {
        if *orig != mapped {
            if let Some(range) = get_lsp_range(db.upcast(), orig) {
                related_info.push(DiagnosticRelatedInformation {
                    location: Location { uri: db.url_for_file(orig.file_id)?, range },
                    message: message.to_string(),
                });
            }
        }
    }
    Some((mapped_range, mapped.file_id))
}

/// Converts an internal diagnostic location to an LSP range.
fn get_lsp_range<'db>(
    db: &'db dyn FilesGroup,
    location: &DiagnosticLocation<'db>,
) -> Option<Range> {
    let Some(span) = location.span.position_in_file(db, location.file_id) else {
        error!("failed to get range for diagnostic");
        return None;
    };
    Some(span.to_lsp())
}
