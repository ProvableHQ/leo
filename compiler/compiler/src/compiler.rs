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
use leo_ast::Program;
pub use leo_ast::{Ast, InputAst};
use leo_errors::emitter::Handler;
use leo_errors::{CompilerError, Result};
pub use leo_passes::SymbolTable;
use leo_passes::*;
use leo_span::source_map::FileName;
use leo_span::symbol::with_session_globals;

use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;

use crate::OutputOptions;

/// The primary entry point of the Leo compiler.
#[derive(Clone)]
pub struct Compiler<'a> {
    /// The handler is used for error and warning emissions.
    handler: &'a Handler,
    /// The path to the main leo file.
    main_file_path: PathBuf,
    /// The path to where the compiler outputs all generated files.
    output_directory: PathBuf,
    /// The AST for the program.
    pub ast: Ast,
    /// The input ast for the program if it exists.
    pub input_ast: Option<InputAst>,
    /// Compiler options on some optional output files.
    output_options: OutputOptions,
}

impl<'a> Compiler<'a> {
    ///
    /// Returns a new Leo compiler.
    ///
    pub fn new(
        handler: &'a Handler,
        main_file_path: PathBuf,
        output_directory: PathBuf,
        output_options: Option<OutputOptions>,
    ) -> Self {
        Self {
            handler,
            main_file_path,
            output_directory,
            ast: Ast::new(Program::new("Initial".to_string())),
            input_ast: None,
            output_options: output_options.unwrap_or_default(),
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
    pub fn parse_program_from_string(&mut self, program_string: &str, name: FileName) -> Result<()> {
        // Register the source (`program_string`) in the source map.
        let prg_sf = with_session_globals(|s| s.source_map.new_source(program_string, name));

        // Use the parser to construct the abstract syntax tree (ast).
        let ast: leo_ast::Ast = leo_parser::parse_ast(self.handler, &prg_sf.src, prg_sf.start_pos)?;

        if self.output_options.ast_initial {
            // Write the AST snapshot post parsing.
            if self.output_options.spans_enabled {
                ast.to_json_file(self.output_directory.clone(), "initial_ast.json")?;
            } else {
                ast.to_json_file_without_keys(self.output_directory.clone(), "initial_ast.json", &["span"])?;
            }
        }

        self.ast = ast;

        Ok(())
    }

    /// Parses and stores the main program file, constructs a syntax tree, and generates a program.
    pub fn parse_program(&mut self) -> Result<()> {
        // Load the program file.
        let program_string = fs::read_to_string(&self.main_file_path)
            .map_err(|e| CompilerError::file_read_error(&self.main_file_path, e))?;

        self.parse_program_from_string(&program_string, FileName::Real(self.main_file_path.clone()))
    }

    /// Parses and stores the input file, constructs a syntax tree, and generates a program input.
    pub fn parse_input(&mut self, input_file_path: PathBuf) -> Result<()> {
        if input_file_path.exists() {
            // Load the input file into the source map.
            let input_sf = with_session_globals(|s| s.source_map.load_file(&input_file_path))
                .map_err(|e| CompilerError::file_read_error(&input_file_path, e))?;

            // Parse and serialize it.
            let input_ast = leo_parser::parse_input(self.handler, &input_sf.src, input_sf.start_pos)?;
            if self.output_options.ast_initial {
                // Write the input AST snapshot post parsing.
                if self.output_options.spans_enabled {
                    input_ast.to_json_file(self.output_directory.clone(), "initial_input_ast.json")?;
                } else {
                    input_ast.to_json_file_without_keys(
                        self.output_directory.clone(),
                        "initial_input_ast.json",
                        &["span"],
                    )?;
                }
            }

            self.input_ast = Some(input_ast);
        }
        Ok(())
    }

    ///
    /// Runs the symbol table pass.
    ///
    pub fn symbol_table_pass(&'a self) -> Result<SymbolTable> {
        CreateSymbolTable::do_pass((&self.ast, self.handler))
    }

    ///
    /// Runs the type checker pass.
    ///
    pub fn type_checker_pass(&'a self, symbol_table: RefCell<SymbolTable>) -> Result<RefCell<SymbolTable>> {
        TypeChecker::do_pass((&self.ast, self.handler, symbol_table))
    }

    ///
    /// Runs the flattening pass.
    ///
    pub fn flattening_pass(&mut self) -> Result<()> {
        self.ast = Flattener::do_pass((std::mem::take(&mut self.ast), self.handler))?;

        if self.output_options.ast_initial {
            // Write the input AST snapshot post parsing.
            if self.output_options.spans_enabled {
                self.ast
                    .to_json_file(self.output_directory.clone(), "flattened_ast.json")?;
            } else {
                self.ast
                    .to_json_file_without_keys(self.output_directory.clone(), "flattened_ast.json", &["span"])?;
            }
        }

        Ok(())
    }

    ///
    /// Runs the compiler stages.
    ///
    pub fn compiler_stages(&mut self) -> Result<()> {
        let mut st = RefCell::new(self.symbol_table_pass()?);
        st = self.type_checker_pass(st)?;
        self.flattening_pass()
    }

    ///
    /// Returns a compiled Leo program.
    ///
    pub fn compile(&mut self) -> Result<()> {
        self.parse_program()?;
        self.compiler_stages()
    }
}
