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
//! it into the Leo AST. Uses `leo-parser-rowan` for IDE-grade error recovery
//! and lossless syntax trees.

#[cfg(target_arch = "wasm32")]
extern crate self as snarkvm;

#[cfg(target_arch = "wasm32")]
mod snarkvm_wasm;
#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub use snarkvm_wasm::{console, prelude, synthesizer};

mod errors;
mod rowan;
pub use rowan::{parse_expression, parse_library, parse_module, parse_program, parse_statement};

#[cfg(test)]
mod test;
