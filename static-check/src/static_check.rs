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

use crate::{StaticCheckError, SymbolTable};
use leo_imports::ImportParser;
use leo_typed::{Input, Program};

/// Performs a static type check over a program.
pub struct StaticCheck {
    table: SymbolTable,
}

impl StaticCheck {
    ///
    /// Returns a new `StaticCheck` with an empty symbol table.
    ///
    pub fn new() -> Self {
        Self {
            table: SymbolTable::new(None),
        }
    }

    ///
    /// Returns a new `SymbolTable` from a given program.
    ///
    pub fn run(
        program: &Program,
        input: &Input,
        import_parser: &ImportParser,
    ) -> Result<SymbolTable, StaticCheckError> {
        let mut check = Self::new();

        // Load program input types.
        check.insert_input(input)?;

        // // Load the program imports into the symbol table.
        // check.insert_imports()?;

        // Run pass one checks
        check.pass_one(program, import_parser)?;

        // Run pass two checks
        check.pass_two(program)?;

        Ok(check.table)
    }

    ///
    /// Inserts the program input types into the symbol table.
    ///
    pub fn insert_input(&mut self, input: &Input) -> Result<(), StaticCheckError> {
        self.table
            .insert_input(input)
            .map_err(|err| StaticCheckError::SymbolTableError(err))
    }

    // ///
    // /// Inserts the program imports into the symbol table.
    // ///
    // pub fn insert_imports(&mut self, imports: &ImportParser) -> Result<(), StaticCheckError> {}

    ///
    /// Checks for duplicate circuit and function names given an unresolved program.
    ///
    /// If a circuit or function name has no duplicates, then it is inserted into the symbol table.
    /// Variables defined later in the unresolved program cannot have the same name.
    ///
    pub fn pass_one(&mut self, program: &Program, import_parser: &ImportParser) -> Result<(), StaticCheckError> {
        // Check unresolved program import names.
        self.table.check_imports(&program.imports, import_parser)?;

        // Check unresolved program circuit names.
        self.table.check_duplicate_circuits(&program.circuits)?;

        // Check unresolved program function names.
        self.table.check_duplicate_functions(&program.functions)?;

        Ok(())
    }

    ///
    /// Checks for unknown types in circuit and function definitions given an unresolved program.
    ///
    /// If a circuit or function definition only contains known types, then it is inserted into the
    /// symbol table. Variables defined later in the unresolved program can lookup the definition and
    /// refer to its expected types.
    ///
    pub fn pass_two(&mut self, program: &Program) -> Result<(), StaticCheckError> {
        // Check unresolved program circuit definitions.
        self.table.check_unknown_types_circuits(&program.circuits)?;

        // Check unresolved program function definitions.
        self.table.check_unknown_types_functions(&program.functions)?;

        Ok(())
    }
}
