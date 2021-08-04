// Copyright (C) 2019-2021 Aleo Systems Inc.
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

//! The parser to convert Leo code text into an [`AST`] type.
//!
//! This module contains the [`parse_ast()`] method which calls the underlying [`parse()`]
//! method to create a new program ast.

#![allow(clippy::vec_init_then_push)]

pub(crate) mod tokenizer;
pub use tokenizer::KEYWORD_TOKENS;
pub(crate) use tokenizer::*;

pub mod parser;
pub use parser::*;

use leo_ast::Ast;
use leo_errors::Result;

#[cfg(test)]
mod test;

/// Creates a new AST from a given file path and source code text.
pub fn parse_ast<T: AsRef<str>, Y: AsRef<str>>(path: T, source: Y) -> Result<Ast> {
    Ok(Ast::new(parser::parse(path.as_ref(), source.as_ref())?))
}
