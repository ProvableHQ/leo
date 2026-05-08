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

//! UTF-8/UTF-16 and file-range conversion shared by navigation features.
//!
//! The worker cache stores compact UTF-8 byte ranges and source fingerprints.
//! This module is the only LSP feature layer that turns those ranges back into
//! client-facing UTF-16 coordinates, after verifying disk-backed files still
//! match the bytes that were analyzed.

use crate::{
    project_model::path_to_file_uri,
    semantics::{AnalyzedFileSet, CompactRange, SourceFingerprint},
};
use line_index::{LineIndex, TextSize, WideEncoding, WideLineCol};
use lsp_types::{Position, Range, Uri};
use std::{path::Path, time::UNIX_EPOCH};
use xxhash_rust::xxh3::xxh3_64;

/// Convert an LSP UTF-16 position into a UTF-8 byte offset.
pub fn position_to_offset(line_index: &LineIndex, position: Position) -> Option<u32> {
    let wide = WideLineCol { line: position.line, col: position.character };
    let utf8 = line_index.to_utf8(WideEncoding::Utf16, wide)?;
    Some(u32::from(line_index.offset(utf8)?))
}

/// Convert a source occurrence range in the requesting open buffer to LSP coordinates.
pub fn compact_range_to_origin_lsp_range(
    range: CompactRange,
    analyzed_files: &AnalyzedFileSet,
    line_index: &LineIndex,
) -> Option<Range> {
    let file = analyzed_files.get(range.file)?;
    if file.is_sentinel || !matches!(file.fingerprint, SourceFingerprint::OpenBuffer) {
        return None;
    }
    byte_range_to_lsp_range(line_index, range.start, range.end)
}

/// Convert a compact target into a safe external URI/range pair.
pub fn compact_range_to_location_parts(range: CompactRange, analyzed_files: &AnalyzedFileSet) -> Option<(Uri, Range)> {
    let file = analyzed_files.get(range.file)?;
    if file.is_sentinel {
        return None;
    }
    let uri = path_to_file_uri(file.path.as_ref())?;
    let lsp_range = match &file.fingerprint {
        SourceFingerprint::OpenBuffer => {
            byte_range_to_lsp_range(file.open_line_index.as_ref()?, range.start, range.end)?
        }
        SourceFingerprint::Disk { .. } => {
            let text = read_verified_disk_text(file.path.as_ref(), &file.fingerprint)?;
            let line_index = LineIndex::new(text.as_str());
            byte_range_to_lsp_range(&line_index, range.start, range.end)?
        }
        SourceFingerprint::Volatile => return None,
    };
    Some((uri, lsp_range))
}

/// Convert a compact range with a caller-provided line index into a location.
pub fn compact_range_to_location_with_line_index(
    range: CompactRange,
    analyzed_files: &AnalyzedFileSet,
    line_index: &LineIndex,
) -> Option<(Uri, Range)> {
    let file = analyzed_files.get(range.file)?;
    if file.is_sentinel {
        return None;
    }
    let uri = path_to_file_uri(file.path.as_ref())?;
    Some((uri, byte_range_to_lsp_range(line_index, range.start, range.end)?))
}

/// Convert UTF-8 byte offsets into the UTF-16 positions required by LSP.
pub fn byte_range_to_lsp_range(line_index: &LineIndex, start: u32, end: u32) -> Option<Range> {
    let start = line_index.try_line_col(TextSize::from(start))?;
    let end = line_index.try_line_col(TextSize::from(end))?;
    let start = line_index.to_wide(WideEncoding::Utf16, start)?;
    let end = line_index.to_wide(WideEncoding::Utf16, end)?;
    Some(Range::new(Position::new(start.line, start.col), Position::new(end.line, end.col)))
}

/// Re-read disk text only when it still matches the analysis fingerprint.
pub fn read_verified_disk_text(path: &Path, expected: &SourceFingerprint) -> Option<String> {
    let SourceFingerprint::Disk { modified_nanos, len, content_hash } = expected else {
        return None;
    };
    let metadata = std::fs::metadata(path).ok()?;
    let current_modified = metadata.modified().ok()?.duration_since(UNIX_EPOCH).ok()?.as_nanos();
    if *len != metadata.len() || *modified_nanos != Some(current_modified) {
        return None;
    }
    let text = std::fs::read_to_string(path).ok()?;
    if *content_hash != hash_text(text.as_str()) {
        return None;
    }
    Some(text)
}

/// Hash source text for stale-target detection without retaining full text.
pub fn hash_text(text: &str) -> u64 {
    xxh3_64(text.as_bytes())
}
