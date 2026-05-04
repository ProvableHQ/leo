// Copyright (C) 2019-2026 Provable Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! Go-to-definition resolution for Leo LSP.
//!
//! This module is intentionally compiler-free. The worker lowers compiler and
//! syntax state into [`CachedPackageAnalysis`], and this feature module only
//! answers cursor queries against that compact, snapshot-safe representation.
//! Keeping this boundary small makes PR 4 references and PR 5 rename able to
//! reuse the same lookup primitives without learning server or compiler internals.

use crate::{
    document_store::DocumentViewKey,
    project_model::path_to_file_uri,
    semantics::{CachedPackageAnalysis, CompactRange, SourceFingerprint},
};
use line_index::{LineIndex, TextSize, WideEncoding, WideLineCol};
use lsp_types::{GotoDefinitionResponse, Location, LocationLink, Position, Range, Uri};
use serde_json::Value;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
    time::UNIX_EPOCH,
};

/// Cursor query captured before any async package-analysis wait.
///
/// The LSP position is converted to a UTF-8 byte offset immediately against the
/// current open document. Pending requests keep that exact offset so multiple
/// requests waiting on the same package analysis can still resolve to different
/// targets.
#[derive(Debug, Clone)]
pub struct DefinitionQuery {
    /// Requesting document URI, retained so pending requests can be cleared on close.
    pub uri: Uri,
    /// Native path for the document where the cursor started.
    pub file_path: Arc<PathBuf>,
    /// Original LSP position used as a fallback origin range for links.
    pub position: Position,
    /// UTF-8 byte offset resolved from `position` before any async wait.
    pub offset: u32,
    /// Line index for the exact open-buffer text that produced `offset`.
    pub line_index: Arc<LineIndex>,
    /// Freshness key for the document view active when the request arrived.
    pub view_key: DocumentViewKey,
    /// Whether the client accepts `LocationLink` responses with origin ranges.
    pub link_support: bool,
}

/// Feature-level result before JSON serialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefinitionResult {
    /// No safe navigation target was found.
    None,
    /// Plain LSP locations for clients without `LocationLink` support.
    Locations(Vec<Location>),
    /// Rich links including the source selection range.
    Links(Vec<LocationLink>),
}

/// Convert an LSP UTF-16 position into a byte offset for the current document.
pub fn position_to_offset(line_index: &LineIndex, position: Position) -> Option<u32> {
    let wide = WideLineCol { line: position.line, col: position.character };
    let utf8 = line_index.to_utf8(WideEncoding::Utf16, wide)?;
    Some(u32::from(line_index.offset(utf8)?))
}

/// Resolve a go-to-definition query against a fresh package analysis.
pub fn resolve(query: &DefinitionQuery, package: &CachedPackageAnalysis) -> DefinitionResult {
    let Some(occurrence) = package.index.occurrence_at(query.file_path.as_ref(), query.offset) else {
        return DefinitionResult::None;
    };
    let Some(key_id) = occurrence.occurrence.key_id() else {
        return DefinitionResult::None;
    };

    let origin_range = compact_range_to_open_lsp_range(occurrence.occurrence.range, package, &query.line_index)
        .unwrap_or_else(|| Range::new(query.position, query.position));
    let mut locations = Vec::new();
    let mut links = Vec::new();

    for target in package.index.definitions_for(key_id) {
        let Some((target_uri, target_range)) = compact_range_to_location_parts(*target, package) else {
            continue;
        };

        if query.link_support {
            links.push(LocationLink {
                origin_selection_range: Some(origin_range),
                target_uri,
                target_range,
                target_selection_range: target_range,
            });
        } else {
            locations.push(Location { uri: target_uri, range: target_range });
        }
    }

    if query.link_support {
        if links.is_empty() { DefinitionResult::None } else { DefinitionResult::Links(links) }
    } else if locations.is_empty() {
        DefinitionResult::None
    } else {
        DefinitionResult::Locations(locations)
    }
}

/// Serialize a feature result into the standard LSP response payload.
pub fn response_value(result: DefinitionResult) -> Value {
    match result {
        DefinitionResult::None => Value::Null,
        DefinitionResult::Locations(locations) => serde_json::to_value(GotoDefinitionResponse::Array(locations))
            .expect("GotoDefinitionResponse::Array should serialize"),
        DefinitionResult::Links(links) => serde_json::to_value(GotoDefinitionResponse::Link(links))
            .expect("GotoDefinitionResponse::Link should serialize"),
    }
}

/// Convert a source occurrence range in the requesting open buffer to LSP coordinates.
fn compact_range_to_open_lsp_range(
    range: CompactRange,
    package: &CachedPackageAnalysis,
    line_index: &LineIndex,
) -> Option<Range> {
    let file = package.analyzed_files.get(range.file)?;
    if !matches!(file.fingerprint, SourceFingerprint::OpenBuffer) {
        return None;
    }
    byte_range_to_lsp_range(line_index, range.start, range.end)
}

/// Convert a compact definition target into a safe external URI/range pair.
fn compact_range_to_location_parts(range: CompactRange, package: &CachedPackageAnalysis) -> Option<(Uri, Range)> {
    let file = package.analyzed_files.get(range.file)?;
    let uri = path_to_file_uri(file.path.as_ref())?;
    let lsp_range = match &file.fingerprint {
        SourceFingerprint::OpenBuffer => {
            byte_range_to_lsp_range(file.open_line_index.as_ref()?, range.start, range.end)?
        }
        SourceFingerprint::Disk { .. } => {
            // Disk-backed targets are only emitted if the file still matches the
            // exact bytes analyzed by the worker. This avoids returning ranges
            // into a dependency file that changed after indexing.
            let text = read_verified_disk_text(file.path.as_ref(), &file.fingerprint)?;
            let line_index = LineIndex::new(text.as_str());
            byte_range_to_lsp_range(&line_index, range.start, range.end)?
        }
        SourceFingerprint::Volatile => return None,
    };
    Some((uri, lsp_range))
}

/// Convert UTF-8 byte offsets into the UTF-16 positions required by LSP.
fn byte_range_to_lsp_range(line_index: &LineIndex, start: u32, end: u32) -> Option<Range> {
    let start = line_index.try_line_col(TextSize::from(start))?;
    let end = line_index.try_line_col(TextSize::from(end))?;
    let start = line_index.to_wide(WideEncoding::Utf16, start)?;
    let end = line_index.to_wide(WideEncoding::Utf16, end)?;
    Some(Range::new(Position::new(start.line, start.col), Position::new(end.line, end.col)))
}

/// Re-read disk text only when it still matches the analysis fingerprint.
fn read_verified_disk_text(path: &Path, expected: &SourceFingerprint) -> Option<String> {
    let SourceFingerprint::Disk { modified_nanos, len, content_hash } = expected else {
        return None;
    };
    let metadata = std::fs::metadata(path).ok()?;
    let current_modified = metadata.modified().ok()?.duration_since(UNIX_EPOCH).ok()?.as_nanos();
    // Size and mtime are cheap rejection tests; the content hash is the final
    // guard for same-size rewrites or coarse filesystem timestamp behavior.
    if *len != metadata.len() || *modified_nanos != Some(current_modified) {
        return None;
    }
    let text = std::fs::read_to_string(path).ok()?;
    if *content_hash != hash_text(text.as_str()) {
        return None;
    }
    Some(text)
}

/// Hash text using the same lightweight process-local hasher as analysis.
fn hash_text(text: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}
