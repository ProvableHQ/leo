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

use crate::semantics::{OccurrenceRole, SemanticKind, SemanticTokenOccurrence};
use line_index::{LineIndex, TextSize, WideEncoding};
use lsp_types::{
    SemanticTokenModifier,
    SemanticTokenType,
    SemanticTokensFullOptions,
    SemanticTokensLegend,
    SemanticTokensOptions,
    SemanticTokensServerCapabilities,
    WorkDoneProgressOptions,
};
use serde_json::{Value, json};
use std::{path::PathBuf, sync::Arc};

const DECLARATION_MODIFIER_BIT: u32 = 1 << 0;
const READONLY_MODIFIER_BIT: u32 = 1 << 1;

/// Advertise the semantic token capability exposed by `leo-lsp`.
pub fn capability() -> SemanticTokensServerCapabilities {
    SemanticTokensOptions {
        work_done_progress_options: WorkDoneProgressOptions { work_done_progress: None },
        legend: legend(),
        range: None,
        full: Some(SemanticTokensFullOptions::Bool(true)),
    }
    .into()
}

/// Return the fixed semantic-token legend used by both the server capability
/// and the token encoder.
pub fn legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: vec![
            SemanticTokenType::NAMESPACE,
            SemanticTokenType::TYPE,
            SemanticTokenType::INTERFACE,
            SemanticTokenType::FUNCTION,
            SemanticTokenType::PARAMETER,
            SemanticTokenType::VARIABLE,
            SemanticTokenType::PROPERTY,
            SemanticTokenType::KEYWORD,
            SemanticTokenType::COMMENT,
            SemanticTokenType::STRING,
            SemanticTokenType::NUMBER,
            SemanticTokenType::OPERATOR,
        ],
        token_modifiers: vec![SemanticTokenModifier::DECLARATION, SemanticTokenModifier::READONLY],
    }
}

/// Encode semantic occurrences into the LSP relative-token wire format.
///
/// The encoder streams the requested document in stable source order, converts
/// UTF-8 byte offsets into UTF-16 columns, and skips tokens the LSP client
/// cannot represent as a single-line semantic token.
pub fn encode_tokens(
    occurrences: &[SemanticTokenOccurrence],
    requested_file_path: Option<&PathBuf>,
    line_index: &LineIndex,
) -> Arc<[u32]> {
    let mut data = Vec::with_capacity(occurrences.len().saturating_mul(5));
    let mut previous_line = 0_u32;
    let mut previous_start = 0_u32;
    let mut emitted_any = false;

    for occurrence in occurrences {
        if requested_file_path.is_some_and(|path| occurrence.range.path.as_ref() != path) {
            continue;
        }

        let start = TextSize::from(occurrence.range.start);
        let end = TextSize::from(occurrence.range.end);
        let Some(start_utf8) = line_index.try_line_col(start) else {
            continue;
        };
        let Some(end_utf8) = line_index.try_line_col(end) else {
            continue;
        };
        if start_utf8.line != end_utf8.line {
            // Multi-line tokens are not representable in the wire format that
            // VS Code and other clients expect for semantic token ranges.
            continue;
        }

        let Some(start_utf16) = line_index.to_wide(WideEncoding::Utf16, start_utf8) else {
            continue;
        };
        let Some(end_utf16) = line_index.to_wide(WideEncoding::Utf16, end_utf8) else {
            continue;
        };
        if end_utf16.col <= start_utf16.col {
            continue;
        }

        let delta_line = if emitted_any { start_utf16.line - previous_line } else { start_utf16.line };
        let delta_start =
            if emitted_any && delta_line == 0 { start_utf16.col - previous_start } else { start_utf16.col };

        // The wire format stores `[delta_line, delta_start, length, type, modifiers]`.
        data.extend_from_slice(&[
            delta_line,
            delta_start,
            end_utf16.col - start_utf16.col,
            semantic_kind_index(occurrence.token_kind),
            semantic_modifier_bits(occurrence.role, occurrence.readonly),
        ]);

        emitted_any = true;
        previous_line = start_utf16.line;
        previous_start = start_utf16.col;
    }

    Arc::<[u32]>::from(data)
}

/// Wrap an encoded token slice in the standard LSP response payload.
pub fn response_value(encoded_tokens: &[u32]) -> Value {
    json!({ "data": encoded_tokens })
}

/// Return the empty semantic-token payload used for unmanaged or closed files.
pub fn empty_response_value() -> Value {
    response_value(&[])
}

fn semantic_kind_index(kind: SemanticKind) -> u32 {
    match kind {
        SemanticKind::Namespace => 0,
        SemanticKind::Type => 1,
        SemanticKind::Interface => 2,
        SemanticKind::Function => 3,
        SemanticKind::Parameter => 4,
        SemanticKind::Variable => 5,
        SemanticKind::Property => 6,
        SemanticKind::Keyword => 7,
        SemanticKind::Comment => 8,
        SemanticKind::String => 9,
        SemanticKind::Number => 10,
        SemanticKind::Operator => 11,
    }
}

fn semantic_modifier_bits(role: OccurrenceRole, readonly: bool) -> u32 {
    let declaration_bits = if matches!(role, OccurrenceRole::Declaration) { DECLARATION_MODIFIER_BIT } else { 0 };
    let readonly_bits = if readonly { READONLY_MODIFIER_BIT } else { 0 };
    declaration_bits | readonly_bits
}
