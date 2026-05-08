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

//! LSP feature modules owned by `leo-lsp`.
//!
//! Each child module owns one protocol-facing capability or one shared helper
//! layer used by those capabilities. Server routing stays outside this module;
//! feature modules consume snapshot-safe semantic data and return LSP-ready
//! values.

/// Go-to-definition query resolution.
pub mod goto_definition;
/// Shared LSP range and URI conversion helpers.
pub mod lsp_range;
/// Find-all-references query resolution.
pub mod references;
/// Semantic token capability wiring and wire-format encoding helpers.
pub mod semantic_tokens;
