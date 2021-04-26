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

//! The parser to convert Leo code text into an [`Program`] AST type.
//!
//! This module contains the [`parse()`] method which calls the underlying [`tokenize()`]
//! method to create a new program ast.

mod context;
pub use context::*;

pub mod expression;
pub mod file;
pub mod statement;
pub mod type_;

use std::unimplemented;

use crate::{errors::assert_no_whitespace, tokenizer::*, DeprecatedError, SyntaxError, Token};
use indexmap::IndexMap;
use leo_ast::*;

pub type SyntaxResult<T> = Result<T, SyntaxError>;

/// Creates a new program from a given file path and source code text.
pub fn parse(path: &str, source: &str) -> SyntaxResult<Program> {
    let mut tokens = ParserContext::new(crate::tokenize(path, source.into())?);

    tokens.parse_program()
}
