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

//! The parser to convert Leo code text into a [`Program`] AST type.
//!
//! This module contains the [`parse()`] function which calls the underlying [`tokenize()`]
//! method to create a new program AST.

use crate::{tokenizer::*, Token};

use leo_ast::*;
use leo_errors::{emitter::Handler, Result};
use leo_span::{span::BytePos, Span};

use snarkvm::prelude::Network;

use indexmap::IndexMap;
use std::unreachable;

mod context;
pub(super) use context::ParserContext;

mod expression;
mod file;
mod statement;
pub(super) mod type_;

/// Creates a new program from a given file path and source code text.
pub fn parse<N: Network>(
    handler: &Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: BytePos,
) -> Result<Program> {
    let mut tokens = ParserContext::<N>::new(handler, node_builder, crate::tokenize(source, start_pos)?);

    tokens.parse_program()
}
