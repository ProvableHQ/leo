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
use itertools::Itertools;

mod context;
pub(super) use context::ParserContext;

mod expression;
mod file;
mod statement;
pub(super) mod type_;

/// Parses the main source file and any associated modules into a `Program` AST.
///
/// # Arguments
/// * `handler` - Used for diagnostics and error reporting.
/// * `node_builder` - Factory for building AST nodes.
/// * `source` - The main source file to parse.
/// * `modules` - A list of module source files (each wrapped in `Rc<SourceFile>`).
/// * `network` - The Aleo network context (e.g., TestnetV0).
///
/// # Returns
/// * `Ok(Program)` - The parsed program including all modules.
/// * `Err(CompilerError)` - If any part of parsing fails.
pub fn parse(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &SourceFile,
    modules: &[std::rc::Rc<SourceFile>],
    network: NetworkName,
) -> Result<Program> {
    // === Parse the main source file ===
    let mut tokens = ParserContext::new(
        &handler,
        node_builder,
        crate::tokenize(&source.src, source.absolute_start)?,
        None, // no program name yet
        network,
    );

    // Build the main program AST
    let mut program = tokens.parse_program()?;
    let program_name = tokens.program_name;

    // Determine the root directory of the main file (for module resolution)
    let root_dir = match &source.name {
        FileName::Real(path) => path.parent().map(|p| p.to_path_buf()),
        _ => None,
    };

    // === Parse each module file ===
    for module in modules {
        let mut module_tokens = ParserContext::new(
            &handler,
            node_builder,
            crate::tokenize(&module.src, module.absolute_start)?,
            program_name,
            network,
        );

        // Compute the module key from its filename (e.g., `foo/bar.leo` => ["foo", "bar"])
        if let Some(key) = compute_module_key(&module.name, root_dir.as_deref()) {
            // Ensure no module uses a keyword in its name
            for segment in &key {
                if let Some(keyword) = Token::symbol_to_keyword(*segment) {
                    return Err(
                        ParserError::keyword_used_as_module_name(key.iter().format("::").to_string(), keyword).into()
                    );
                }
            }

            let module_ast = module_tokens.parse_module(&key)?;
            program.modules.insert(key, module_ast);
        }
    }

    Ok(program)
}

/// Computes a module key from a `FileName`, optionally relative to a root directory.
///
/// This function converts a file path like `src/foo/bar.leo` into a `Vec<Symbol>` key
/// like `["foo", "bar"]`, suitable for inserting into the program's module map.
///
/// # Arguments
/// * `name` - The filename of the module, either real (from disk) or synthetic (custom).
/// * `root_dir` - The root directory to strip from the path, if any.
///
/// # Returns
/// * `Some(Vec<Symbol>)` - The computed module key.
/// * `None` - If the path can't be stripped or processed.
fn compute_module_key(name: &FileName, root_dir: Option<&std::path::Path>) -> Option<Vec<Symbol>> {
    // Normalize the path depending on whether it's a custom or real file
    let path = match name {
        FileName::Custom(name) => std::path::Path::new(name).to_path_buf(),
        FileName::Real(path) => {
            let root = root_dir?;
            path.strip_prefix(root).ok()?.to_path_buf()
        }
    };

    // Convert path components (e.g., "foo/bar") into symbols: ["foo", "bar"]
    let mut key: Vec<Symbol> =
        path.components().map(|comp| Symbol::intern(&comp.as_os_str().to_string_lossy())).collect();

    // Strip the file extension from the last component (e.g., "bar.leo" → "bar")
    if let Some(last) = path.file_name() {
        if let Some(stem) = std::path::Path::new(last).file_stem() {
            key.pop(); // Remove "bar.leo"
            key.push(Symbol::intern(&stem.to_string_lossy())); // Add "bar"
        }
    }

    Some(key)
}

pub fn parse_module(
    handler: Handler,
    node_builder: &NodeBuilder,
    mod_path: &[Symbol],
    source: &str,
    start_pos: u32,
    network: NetworkName,
) -> Result<Module> {
    let mut context = ParserContext::new(&handler, node_builder, crate::tokenize(source, start_pos)?, None, network);

    let module = context.parse_module(mod_path)?;
    if context.token.token == Token::Eof {
        Ok(module)
    } else {
        Err(ParserError::unexpected(context.token.token, Token::Eof, context.token.span).into())
    }
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
