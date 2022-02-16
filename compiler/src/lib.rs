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

use leo_errors::emitter::Handler;
use leo_errors::{CompilerError, Result};
use leo_span::symbol::create_session_if_not_set_then;

use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

/// The primary entry point of the Leo compiler.
pub struct Compiler<'a> {
    handler: &'a Handler,
    main_file_path: PathBuf,
}

impl<'a> Compiler<'a> {
    ///
    /// Returns a new Leo compiler.
    ///
    pub fn new(handler: &'a Handler, main_file_path: PathBuf) -> Self {
        Self {
            handler,
            main_file_path,
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

    ///
    /// Runs the compiler stages.
    /// 
    fn compiler_stages(self) -> Result<leo_ast::Ast> {
        // Load the program file.
        let program_string = fs::read_to_string(&self.main_file_path)
            .map_err(|e| CompilerError::file_read_error(self.main_file_path.clone(), e))?;

        // Use the parser to construct the abstract syntax tree (ast).
        let ast: leo_ast::Ast = leo_parser::parse_ast(
            self.handler,
            self.main_file_path.to_str().unwrap_or_default(),
            program_string,
        )?;

        Ok(ast)
    }

    ///
    /// Returns a compiled Leo program.
    ///
    pub fn compile(self) -> Result<leo_ast::Ast> {
        create_session_if_not_set_then(|_| self.compiler_stages())
    }
}
