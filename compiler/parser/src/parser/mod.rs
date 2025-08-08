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
use leo_span::{
    Span,
    Symbol,
    source_map::{FileName, SourceFile},
};

use indexmap::IndexMap;

mod context;
pub(super) use context::ParserContext;

mod expression;
mod file;
mod statement;
pub(super) mod type_;

/// Creates a new program from a given file path and source code text.
pub fn parse(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &SourceFile,
    modules: &Vec<std::rc::Rc<SourceFile>>,
    network: NetworkName,
) -> Result<Program> {
    // Parse the main file
    let mut tokens =
        ParserContext::new(&handler, node_builder, crate::tokenize(&source.src, source.absolute_start)?, None, network);
    let mut program = tokens.parse_program()?;
    let program_name = tokens.program_name;

    let root_dir = match &source.name {
        FileName::Real(path) => path.parent().map(|p| p.to_path_buf()),
        _ => None,
    };

    for module in modules {
        let mut module_tokens = ParserContext::new(
            &handler,
            node_builder,
            crate::tokenize(&module.src, module.absolute_start)?,
            program_name,
            network,
        );

        if let Some(key) = compute_module_key(&module.name, root_dir.as_deref()) {
            let module = module_tokens.parse_module(&key)?;
            program.modules.insert(key, module);
        }
    }

    Ok(program)
}

fn compute_module_key(name: &FileName, root_dir: Option<&std::path::Path>) -> Option<Vec<Symbol>> {
    let path = match name {
        FileName::Custom(name) => std::path::Path::new(name).to_path_buf(),
        FileName::Real(path) => {
            let root = root_dir?;
            path.strip_prefix(root).ok()?.to_path_buf()
        }
    };

    let mut key: Vec<Symbol> =
        path.components().map(|comp| Symbol::intern(&comp.as_os_str().to_string_lossy())).collect();

    // Strip file extension from the last component (if any)
    if let Some(last) = path.file_name() {
        if let Some(stem) = std::path::Path::new(last).file_stem() {
            key.pop();
            key.push(Symbol::intern(&stem.to_string_lossy()));
        }
    }

    Some(key)
}

pub fn parse_expression(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: u32,
    network: NetworkName,
) -> Result<Expression> {
    let mut context = ParserContext::new(&handler, node_builder, crate::tokenize(source, start_pos)?, None, network);

    let expression = context.parse_expression()?;
    if context.token.token == Token::Eof {
        Ok(expression)
    } else {
        Err(ParserError::unexpected(context.token.token, Token::Eof, context.token.span).into())
    }
}

pub fn parse_statement(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: u32,
    network: NetworkName,
) -> Result<Statement> {
    let mut context = ParserContext::new(&handler, node_builder, crate::tokenize(source, start_pos)?, None, network);

    let statement = context.parse_statement()?;
    if context.token.token == Token::Eof {
        Ok(statement)
    } else {
        Err(ParserError::unexpected(context.token.token, Token::Eof, context.token.span).into())
    }
}
