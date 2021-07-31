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

use crate::{tokenizer::*, Token};
use indexmap::IndexMap;
use leo_ast::*;
use leo_errors::{LeoError, ParserError, Span};

pub type SyntaxResult<T> = Result<T, LeoError>;

pub(crate) fn assert_no_whitespace(left_span: &Span, right_span: &Span, left: &str, right: &str) -> SyntaxResult<()> {
    if left_span.col_stop != right_span.col_start {
        let mut error_span = left_span + right_span;
        error_span.col_start = left_span.col_stop - 1;
        error_span.col_stop = right_span.col_start - 1;
        return Err(LeoError::from(ParserError::unexpected_whitespace(
            left,
            right,
            &error_span,
        )));
    }

    Ok(())
}

/// Creates a new program from a given file path and source code text.
pub fn parse(path: &str, source: &str) -> SyntaxResult<Program> {
    let mut tokens = ParserContext::new(crate::tokenize(path, source.into())?);

    tokens.parse_program()
}
