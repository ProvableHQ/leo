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

pub mod arrays;
pub mod circuits;
pub mod functions;
pub mod tuples;
pub mod variables;

use leo_grammar::Grammar;
use leo_type_inference::TypeInference;

use leo_ast::{Input, LeoAst, Program};
use leo_imports::ImportParser;
use leo_symbol_table::SymbolTable;
use std::path::PathBuf;

const TEST_PROGRAM_PATH: &str = "";
const TEST_PROGRAM_NAME: &str = "test";

/// A helper struct to test a `TypeInference` check.
pub struct TestTypeInference {
    program: Program,
    symbol_table: SymbolTable,
}

impl TestTypeInference {
    pub fn new(bytes: &[u8]) -> Self {
        // Get file string from bytes.
        let file_string = String::from_utf8_lossy(bytes);

        // Get test file path.
        let file_path = PathBuf::from(TEST_PROGRAM_PATH);

        // Get parser syntax tree.
        let ast = Grammar::new(&file_path, &*file_string).unwrap();

        // Get typed syntax tree.
        let typed = LeoAst::new(TEST_PROGRAM_NAME, &ast);
        let program = typed.into_repr();

        // Create empty import parser.
        let import_parser = ImportParser::default();

        // Create empty input.
        let input = Input::new();

        // Create symbol table.
        let symbol_table = SymbolTable::new(&program, &import_parser, &input).unwrap();

        // Store fields for new type inference check.
        Self { program, symbol_table }
    }

    pub fn check(self) {
        TypeInference::run(&self.program, self.symbol_table).unwrap();
    }

    pub fn expect_error(self) {
        assert!(TypeInference::run(&self.program, self.symbol_table).is_err());
    }
}

#[test]
fn test_new() {
    let bytes = include_bytes!("empty.leo");

    let type_inference = TestTypeInference::new(bytes);

    type_inference.check()
}
