// Copyright (C) 2019-2023 Aleo Systems Inc.
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

#![forbid(unsafe_code)]
#![allow(clippy::vec_init_then_push)]
#![doc = include_str!("../README.md")]

pub(crate) mod tokenizer;
use leo_span::span::BytePos;
pub use tokenizer::KEYWORD_TOKENS;
pub(crate) use tokenizer::*;

pub mod parser;
pub use parser::*;

use leo_ast::{input::InputData, Ast, NodeBuilder, ProgramInput};
use leo_errors::{emitter::Handler, Result};

#[cfg(test)]
mod test;

/// Creates a new AST from a given file path and source code text.
pub fn parse_ast(handler: &Handler, node_builder: &NodeBuilder, source: &str, start_pos: BytePos) -> Result<Ast> {
    Ok(Ast::new(parser::parse(handler, node_builder, source, start_pos)?))
}

/// Parses program inputs from the input file path
pub fn parse_program_inputs(
    handler: &Handler,
    node_builder: &NodeBuilder,
    input_string: &str,
    start_pos: BytePos,
) -> Result<InputData> {
    let program_input: ProgramInput =
        parser::parse_input(handler, node_builder, input_string, start_pos)?.try_into()?;

    Ok(InputData { program_input })
}
