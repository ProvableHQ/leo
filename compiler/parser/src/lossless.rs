// Copyright (C) 2019-2026 Provable Inc.
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

//! LALRPOP-based lossless parser implementation.
//!
//! This module uses `leo-parser-lossless` (LALRPOP) and converts its output
//! to the Leo AST via the `conversions` module.

use itertools::Itertools as _;

use leo_ast::{NetworkName, NodeBuilder};
use leo_errors::{Handler, ParserError, Result};
use leo_span::{
    Symbol,
    source_map::{FileName, SourceFile},
    sym,
};

#[path = "conversions.rs"]
mod conversions;

/// Parses a single expression from source code.
pub fn parse_expression(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: u32,
    _network: NetworkName,
) -> Result<leo_ast::Expression> {
    let node = leo_parser_lossless::parse_expression(handler.clone(), source, start_pos)?;
    let conversion_context = conversions::ConversionContext::new(&handler, node_builder);
    conversion_context.to_expression(&node)
}

/// Parses a single statement from source code.
pub fn parse_statement(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: u32,
    _network: NetworkName,
) -> Result<leo_ast::Statement> {
    let node = leo_parser_lossless::parse_statement(handler.clone(), source, start_pos)?;
    let conversion_context = conversions::ConversionContext::new(&handler, node_builder);
    conversion_context.to_statement(&node)
}

/// Parses a module (non-main source file) into a Module AST.
pub fn parse_module(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &str,
    start_pos: u32,
    program_name: Symbol,
    path: Vec<Symbol>,
    _network: NetworkName,
) -> Result<leo_ast::Module> {
    let node_module = leo_parser_lossless::parse_module(handler.clone(), source, start_pos)?;
    let conversion_context = conversions::ConversionContext::new(&handler, node_builder);
    conversion_context.to_module(&node_module, program_name, path)
}

/// Parses a complete program with its modules into a Program AST.
pub fn parse(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &SourceFile,
    modules: &[std::rc::Rc<SourceFile>],
    _network: NetworkName,
) -> Result<leo_ast::Program> {
    let conversion_context = conversions::ConversionContext::new(&handler, node_builder);

    let program_node = leo_parser_lossless::parse_main(handler.clone(), &source.src, source.absolute_start)?;
    let mut program = conversion_context.to_main(&program_node)?;
    let program_name = *program.program_scopes.first().unwrap().0;

    // Determine the root directory of the main file (for module resolution)
    let root_dir = match &source.name {
        FileName::Real(path) => path.parent().map(|p| p.to_path_buf()),
        _ => None,
    };

    for module in modules {
        let node_module = leo_parser_lossless::parse_module(handler.clone(), &module.src, module.absolute_start)?;
        if let Some(key) = compute_module_key(&module.name, root_dir.as_deref()) {
            // Ensure no module uses a keyword in its name
            for segment in &key {
                if symbol_is_keyword(*segment) {
                    return Err(ParserError::keyword_used_as_module_name(key.iter().format("::"), segment).into());
                }
            }

            let module_ast = conversion_context.to_module(&node_module, program_name, key.clone())?;
            program.modules.insert(key, module_ast);
        }
    }

    Ok(program)
}

/// Creates a new AST from a given file path and source code text.
pub fn parse_ast(
    handler: Handler,
    node_builder: &NodeBuilder,
    source: &SourceFile,
    modules: &[std::rc::Rc<SourceFile>],
    network: NetworkName,
) -> Result<leo_ast::Ast> {
    Ok(leo_ast::Ast::new(parse(handler, node_builder, source, modules, network)?))
}

fn symbol_is_keyword(symbol: Symbol) -> bool {
    matches!(
        symbol,
        sym::address
            | sym::aleo
            | sym::As
            | sym::assert
            | sym::assert_eq
            | sym::assert_neq
            | sym::Async
            | sym::block
            | sym::bool
            | sym::Const
            | sym::constant
            | sym::constructor
            | sym::Else
            | sym::False
            | sym::field
            | sym::FnUpper
            | sym::Fn
            | sym::For
            | sym::Final
            | sym::group
            | sym::i8
            | sym::i16
            | sym::i32
            | sym::i64
            | sym::i128
            | sym::If
            | sym::import
            | sym::In
            | sym::inline
            | sym::Let
            | sym::leo
            | sym::mapping
            | sym::storage
            | sym::network
            | sym::private
            | sym::program
            | sym::public
            | sym::record
            | sym::Return
            | sym::scalar
            | sym::script
            | sym::SelfLower
            | sym::signature
            | sym::string
            | sym::Struct
            | sym::True
            | sym::u8
            | sym::u16
            | sym::u32
            | sym::u64
            | sym::u128
    )
}

/// Computes a module key from a `FileName`, optionally relative to a root directory.
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

    if let Some(last) = path.file_name()
        && let Some(stem) = std::path::Path::new(last).file_stem()
    {
        key.pop();
        key.push(Symbol::intern(&stem.to_string_lossy()));
    }

    Some(key)
}
