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

//! The compiler for Leo programs.
//!
//! The [`Compiler`] type compiles Leo programs into R1CS circuits.

#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![doc = include_str!("../README.md")]

#[cfg(test)]
mod test;

use leo_ast::Program;
pub use leo_ast::{Ast, InputAst};
use leo_errors::emitter::Handler;
use leo_errors::{CompilerError, Result};
pub use leo_passes::SymbolTable;
use leo_passes::*;

use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
/// The primary entry point of the Leo compiler.
pub struct Compiler<'a> {
    handler: &'a Handler,
    main_file_path: PathBuf,
    output_directory: PathBuf,
    pub ast: Ast,
    pub input_ast: Option<InputAst>,
}

impl<'a> Compiler<'a> {
    ///
    /// Returns a new Leo compiler.
    ///
    pub fn new(handler: &'a Handler, main_file_path: PathBuf, output_directory: PathBuf) -> Self {
        Self {
            handler,
            main_file_path,
            output_directory,
            ast: Ast::new(Program::new("Initial".to_string())),
            input_ast: None,
        }
    }

    ///
    /// Returns a SHA256 checksum of the program file.
    ///
    pub fn checksum(&self) -> Result<String> {
        // Read in the main file as string
        let unparsed_file = fs::read_to_string(&self.main_file_path)
            .map_err(|e| CompilerError::file_read_error(self.main_file_path.clone(), e))?;

        // Hash the file contents
        let mut hasher = Sha256::new();
        hasher.update(unparsed_file.as_bytes());
        let hash = hasher.finalize();

        Ok(format!("{:x}", hash))
    }

    // Parses and stores a program file content from a string, constructs a syntax tree, and generates a program.
    pub fn parse_program_from_string(&mut self, program_string: &str) -> Result<()> {
        // Use the parser to construct the abstract syntax tree (ast).
        let ast: leo_ast::Ast = leo_parser::parse_ast(
            self.handler,
            self.main_file_path.to_str().unwrap_or_default(),
            program_string,
        )?;
        // Write the AST snapshot post parsing.
        ast.to_json_file_without_keys(self.output_directory.clone(), "initial_ast.json", &["span"])?;

        self.ast = ast;

        Ok(())
    }

    /// Parses and stores the main program file, constructs a syntax tree, and generates a program.
    pub fn parse_program(&mut self) -> Result<()> {
        // Load the program file.
        let program_string = fs::read_to_string(&self.main_file_path)
            .map_err(|e| CompilerError::file_read_error(self.main_file_path.clone(), e))?;

        self.parse_program_from_string(&program_string)
    }

    /// Parses and stores the input file, constructs a syntax tree, and generates a program input.
    pub fn parse_input_from_string(&mut self, input_file_path: PathBuf, input_string: &str) -> Result<()> {
        let input_ast =
            leo_parser::parse_input(self.handler, input_file_path.to_str().unwrap_or_default(), input_string)?;
        input_ast.to_json_file_without_keys(self.output_directory.clone(), "inital_input_ast.json", &["span"])?;

        self.input_ast = Some(input_ast);
        Ok(())
    }

    /// Parses and stores the input file, constructs a syntax tree, and generates a program input.
    pub fn parse_input(&mut self, input_file_path: PathBuf) -> Result<()> {
        // Load the input file if it exists.
        if input_file_path.exists() {
            let input_string = fs::read_to_string(&input_file_path)
                .map_err(|e| CompilerError::file_read_error(input_file_path.clone(), e))?;

            self.parse_input_from_string(input_file_path, &input_string)?;
        }

        Ok(())
    }

    ///
    /// Runs the compiler stages.
    ///
    fn compiler_stages(&self) -> Result<SymbolTable<'_>> {
        let symbol_table = CreateSymbolTable::do_pass((&self.ast, self.handler))?;

        Ok(symbol_table)
    }

    ///
    /// Returns a compiled Leo program.
    ///
    pub fn compile(&self) -> Result<SymbolTable<'_>> {
        self.compiler_stages()
    }
}
