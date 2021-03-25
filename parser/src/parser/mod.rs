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
use context::*;

mod expression;
mod file;
mod statement;
mod type_;

use std::unimplemented;

use crate::{tokenizer::*, DeprecatedError, SyntaxError, Token};
use indexmap::IndexMap;
use leo_ast::*;

pub type SyntaxResult<T> = Result<T, SyntaxError>;

/// Creates a new program from a given file path and source code text.
pub fn parse(path: &str, source: &str) -> SyntaxResult<Program> {
    let mut tokens = ParserContext::new(crate::tokenize(path, source)?);

    match tokens.parse_program() {
        Ok(x) => Ok(x),
        Err(mut e) => {
            e.set_path(
                path,
                &source.lines().map(|x| x.to_string()).collect::<Vec<String>>()[..],
            );
            Err(e)
        }
    }
}
