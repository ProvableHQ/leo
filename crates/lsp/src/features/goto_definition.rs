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
    features::lsp_range::{compact_range_to_location_parts, compact_range_to_origin_lsp_range},
    semantics::CachedPackageAnalysis,
};
use line_index::LineIndex;
use lsp_types::{GotoDefinitionResponse, Location, LocationLink, Position, Range, Uri};
use serde_json::Value;
use std::{path::PathBuf, sync::Arc};

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

/// Resolve a go-to-definition query against a fresh package analysis.
pub fn resolve(query: &DefinitionQuery, package: &CachedPackageAnalysis) -> DefinitionResult {
    let Some(occurrence) = package.index.occurrence_at(query.file_path.as_ref(), query.offset) else {
        return DefinitionResult::None;
    };
    let Some(key_id) = occurrence.occurrence.key_id() else {
        return DefinitionResult::None;
    };

    let origin_range = compact_range_to_origin_lsp_range(
        occurrence.occurrence.range,
        package.analyzed_files.as_ref(),
        &query.line_index,
    )
    .unwrap_or_else(|| Range::new(query.position, query.position));
    let mut locations = Vec::new();
    let mut links = Vec::new();

    for target in package.index.definitions_for(key_id) {
        let Some((target_uri, target_range)) =
            compact_range_to_location_parts(*target, package.analyzed_files.as_ref())
        else {
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
