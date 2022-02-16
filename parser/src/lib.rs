// Copyright (C) 2019-2022 Aleo Systems Inc.
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
#![doc = include_str!("../README.md")]

pub(crate) mod tokenizer;
// use leo_input::LeoInputParser;
pub use tokenizer::KEYWORD_TOKENS;
pub(crate) use tokenizer::*;

pub mod parser;
pub use parser::*;

use leo_ast::{Ast, Input, ProgramInput, ProgramState};
use leo_errors::emitter::Handler;
use leo_errors::Result;

#[cfg(test)]
mod test;

/// Creates a new AST from a given file path and source code text.
pub fn parse_ast<T: AsRef<str>, Y: AsRef<str>>(handler: &Handler, path: T, source: Y) -> Result<Ast> {
    Ok(Ast::new(parser::parse(handler, path.as_ref(), source.as_ref())?))
}

/// Parses program input from from the input file path and state file path
pub fn parse_program_input<T: AsRef<str>, Y: AsRef<str>, T2: AsRef<str>, Y2: AsRef<str>>(
    handler: &Handler,
    input_string: T,
    input_path: Y,
    state_string: T2,
    state_path: Y2,
) -> Result<Input> {
    let program_input: ProgramInput =
        parser::parse_input(handler, input_path.as_ref(), input_string.as_ref())?.try_into()?;
    let program_state: ProgramState =
        parser::parse_input(handler, state_path.as_ref(), state_string.as_ref())?.try_into()?;

    Ok(Input {
        program_input,
        program_state,
    })
}
