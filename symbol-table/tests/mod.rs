// Copyright (C) 2019-2020 Aleo Systems Inc.
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

pub mod symbol_table;

use leo_ast::{Ast, Input};
use leo_grammar::Grammar;
use leo_symbol_table::{SymbolTable, SymbolTableError};

use leo_imports::ImportParser;
use std::path::PathBuf;

const TEST_PROGRAM_PATH: &str = "";

/// A helper struct to test a `SymbolTable`.
pub struct TestSymbolTable {
    ast: Ast,
}

impl TestSymbolTable {
    ///
    /// Returns a Leo syntax tree given a Leo program.
    ///
    pub fn new(program_string: &str) -> Self {
        // Get test file path.
        let file_path = PathBuf::from(TEST_PROGRAM_PATH);

        // Get parser syntax tree.
        let grammar = Grammar::new(&file_path, program_string).unwrap();

        // Get Leo syntax tree.
        let ast_result = Ast::new(TEST_PROGRAM_PATH, &grammar);

        // We always expect a valid ast for testing.
        Self {
            ast: ast_result.unwrap(),
        }
    }

    ///
    /// Parse the Leo syntax tree into a symbol table.
    ///
    /// Expect no errors during parsing.
    ///
    pub fn expect_success(self, import_parser: ImportParser) {
        // Get program.
        let program = self.ast.into_repr();

        // Create empty input.
        let input = Input::new();

        // Create new symbol table.
        let _symbol_table = SymbolTable::new(&program, &import_parser, &input).unwrap();
    }

    ///
    /// Parse the Leo syntax tree into a symbol table.
    ///
    /// Expect an error involving entries in the symbol table.
    ///
    pub fn expect_pass_one_error(self) {
        // Get program.
        let program = self.ast.into_repr();

        // Create new symbol table.
        let static_check = &mut SymbolTable::default();

        // Create empty import parser.
        let import_parser = ImportParser::default();

        // Run pass one and expect an error.
        let error = static_check
            .check_names(&program, &import_parser, &Input::new())
            .unwrap_err();

        match error {
            SymbolTableError::Error(_) => {} // Ok
            error => panic!("Expected a symbol table error found `{}`", error),
        }
    }

    ///
    /// Parse the Leo syntax tree into a symbol table.
    ///
    /// Expect an error involving types in the symbol table.
    ///
    pub fn expect_pass_two_error(self) {
        // Get program.
        let program = self.ast.into_repr();

        // Create a new symbol table.
        let static_check = &mut SymbolTable::default();

        // Create empty import parser.
        let import_parser = ImportParser::default();

        // Run the pass one and expect no errors.
        static_check
            .check_names(&program, &import_parser, &Input::new())
            .unwrap();

        // Run the pass two and expect and error.
        let error = static_check.check_types(&program).unwrap_err();

        match error {
            SymbolTableError::TypeError(_) => {} //Ok
            error => panic!("Expected a type error found `{}`", error),
        }
    }
}
