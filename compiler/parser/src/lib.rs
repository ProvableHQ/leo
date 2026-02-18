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

//! The Leo parser.
//!
//! This crate provides parsing functionality for Leo source code, converting
//! it into the Leo AST. Two parser implementations are available:
//!
//! - **Default (LALRPOP-based)**: Uses `leo-parser-lossless` with LALRPOP.
//! - **Rowan-based** (feature `rowan`): Uses `leo-parser-rowan` for IDE-grade
//!   error recovery and lossless syntax trees.
//!
//! The `rowan` feature flag switches between implementations. When enabled,
//! the rowan-based parser completely replaces the LALRPOP-based one.

// Feature-gated parser implementations.

#[cfg(not(feature = "rowan"))]
mod lossless;
#[cfg(not(feature = "rowan"))]
pub use lossless::{parse, parse_ast, parse_expression, parse_module, parse_statement};

#[cfg(feature = "rowan")]
mod rowan;
#[cfg(feature = "rowan")]
pub use rowan::{parse, parse_ast, parse_expression, parse_module, parse_statement};

#[cfg(test)]
mod test;
