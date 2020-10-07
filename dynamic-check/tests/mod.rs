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

use leo_ast::LeoAst;
use leo_dynamic_check::DynamicCheck;

use leo_symbol_table::SymbolTable;
use leo_typed::LeoTypedAst;
use std::path::PathBuf;

const TEST_PROGRAM_PATH: &str = "";
const TEST_PROGRAM_NAME: &str = "test";

/// A helper struct to test a `DynamicCheck`.
pub struct TestDynamicCheck {
    dynamic_check: DynamicCheck,
}

impl TestDynamicCheck {
    pub fn new(bytes: &[u8]) -> Self {
        // Get file string from bytes.
        let file_string = String::from_utf8_lossy(bytes);

        // Get test file path.
        let file_path = PathBuf::from(TEST_PROGRAM_PATH);

        // Get parser syntax tree.
        let ast = LeoAst::new(&file_path, &*file_string).unwrap();

        // Get typed syntax tree.
        let typed = LeoTypedAst::new(TEST_PROGRAM_NAME, &ast);
        let program = typed.into_repr();

        // Create symbol table.
        let mut symbol_table = SymbolTable::new(None);

        // Load symbols into symbol table.
        symbol_table.pass_one(&program).unwrap();

        symbol_table.pass_two(&program).unwrap();

        // Create dynamic check
        let dynamic_check = DynamicCheck::new(&program, symbol_table);

        Self { dynamic_check }
    }

    pub fn solve(self) {
        self.dynamic_check.solve();
    }
}

#[test]
fn test_new() {
    let bytes = include_bytes!("empty.leo");

    let dynamic_check = TestDynamicCheck::new(bytes);

    dynamic_check.solve()
}
