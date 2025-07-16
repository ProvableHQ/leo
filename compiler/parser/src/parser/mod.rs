// Copyright (C) 2019-2025 Provable Inc.
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
use leo_errors::{Handler, ParserError, Result};
use leo_span::{Span, Symbol};

use snarkvm::prelude::Network;

use indexmap::IndexMap;
use std::{fs, path::PathBuf, unreachable};
mod context;
pub(super) use context::ParserContext;

mod expression;
mod file;
mod statement;
pub(super) mod type_;

fn get_submodule_file_paths(filename: &std::path::Path, next_paths: &[Vec<Symbol>]) -> Vec<PathBuf> {
    let src_root = filename.parent().expect("Expected filename to have a parent directory");

    let mut results = Vec::new();

    for path in next_paths {
        let path = path.iter().map(|sym| sym.to_string()).collect::<Vec<_>>();
        if path.len() >= 2 {
            let module_path = path[..path.len() - 1].join("/"); // skip final item name
            let full_path = src_root.join(format!("{module_path}.leo"));
            results.push(full_path);
        }
    }

    results
}

/// Creates a new program from a given file path and source code text.
pub fn parse<N: Network>(
    handler: Handler,
    node_builder: &NodeBuilder,
    filename: &std::path::Path,
    source: &str,
    start_pos: u32,
) -> Result<Program> {
    let mut tokens = ParserContext::<N>::new(&handler, node_builder, crate::tokenize(source, start_pos)?);

    let mut program = tokens.parse_program()?;

    for (idx, path) in get_submodule_file_paths(filename, &tokens.next_paths).iter().enumerate() {
        let next_paths = tokens.next_paths[idx][..tokens.next_paths[idx].len() - 1].to_vec();
        let source = fs::read_to_string(path).expect("meh");
        let mut module_tokens = ParserContext::<N>::new(&handler, node_builder, crate::tokenize(&source, start_pos)?);
        let module = module_tokens.parse_module(&next_paths)?; // TODO: Do not `?` here
        program.modules.insert(next_paths, module);
    }

    Ok(program)
}

pub fn parse_expression<N: Network>(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: u32,
) -> Result<Expression> {
    let mut context = ParserContext::<N>::new(&handler, node_builder, crate::tokenize(source, start_pos)?);

    let expression = context.parse_expression()?;
    if context.token.token == Token::Eof {
        Ok(expression)
    } else {
        Err(ParserError::unexpected(context.token.token, Token::Eof, context.token.span).into())
    }
}

pub fn parse_statement<N: Network>(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: u32,
) -> Result<Statement> {
    let mut context = ParserContext::<N>::new(&handler, node_builder, crate::tokenize(source, start_pos)?);

    let statement = context.parse_statement()?;
    if context.token.token == Token::Eof {
        Ok(statement)
    } else {
        Err(ParserError::unexpected(context.token.token, Token::Eof, context.token.span).into())
    }
}
