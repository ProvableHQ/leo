// Copyright (C) 2019-2024 Aleo Systems Inc.
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

use crate::{Token, tokenizer::*};

use leo_ast::*;
use leo_errors::{ParserError, Result, emitter::Handler};
use leo_span::{Span, span::BytePos};

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

pub fn parse_expression<N: Network>(
    handler: &Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: BytePos,
) -> Result<Expression> {
    let mut context = ParserContext::<N>::new(handler, node_builder, crate::tokenize(source, start_pos)?);

    let expression = context.parse_expression()?;
    if context.token.token == Token::Eof {
        Ok(expression)
    } else {
        Err(ParserError::unexpected(context.token.token, Token::Eof, context.token.span).into())
    }
}

pub fn parse_statement<N: Network>(
    handler: &Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: BytePos,
) -> Result<Statement> {
    let mut context = ParserContext::<N>::new(handler, node_builder, crate::tokenize(source, start_pos)?);

    let statement = context.parse_statement()?;
    if context.token.token == Token::Eof {
        Ok(statement)
    } else {
        Err(ParserError::unexpected(context.token.token, Token::Eof, context.token.span).into())
    }
}
