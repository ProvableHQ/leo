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
    with_session_globals,
};
use walkdir::WalkDir;

use indexmap::IndexMap;
use std::{fs, unreachable};
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
    network: NetworkName,
) -> Result<Program> {
    // Parse the main file
    let mut tokens =
        ParserContext::new(&handler, node_builder, crate::tokenize(&source.src, source.absolute_start)?, None, network);
    let mut program = tokens.parse_program()?;
    let program_name = tokens.program_name;

    if let FileName::Real(entry_file_path) = &source.name {
        let root_dir = entry_file_path.parent().expect("Expected file to have a parent directory").to_path_buf();

        let mut start_pos = source.absolute_end + 1;

        // Insert the main file under the empty Vec (vec![])
        // It's assumed that tokens.parse_program already fills the main body

        // Walk all files under root_dir (including subdirectories)
        for entry in WalkDir::new(&root_dir).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file()) {
            let path = entry.path();

            // Skip the original file
            if path == entry_file_path {
                continue;
            }

            // Read file content
            let source = fs::read_to_string(path).expect("Unable to read module file");

            // Track the source file and get its end position
            let source_file =
                with_session_globals(|s| s.source_map.new_source(&source, FileName::Real(path.to_path_buf())));

            let mut module_tokens = ParserContext::new(
                &handler,
                node_builder,
                crate::tokenize(&source, start_pos)?,
                program_name, // we know the program name by now
                network,
            );

            // Compute the module key: Vec<Symbol> representing path relative to root_dir
            let rel_path = path.strip_prefix(&root_dir).expect("Path should be under root dir");
            let mut key: Vec<Symbol> =
                rel_path.components().map(|comp| Symbol::intern(&comp.as_os_str().to_string_lossy())).collect();

            // Strip file extension from the last component (if any)
            if let Some(last) = rel_path.file_name() {
                if let Some(stem) = std::path::Path::new(last).file_stem() {
                    key.pop(); // Remove the current (with-extension) last Symbol
                    key.push(Symbol::intern(&stem.to_string_lossy())); // Add stem-only Symbol
                }
            }

            let module = module_tokens.parse_module(&key)?;
            program.modules.insert(key, module);

            start_pos = source_file.absolute_end + 1;
        }
    }

    Ok(program)
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
