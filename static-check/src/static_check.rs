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
    /// Returns a new `SymbolTable` from a given program and import parser.
    ///
    pub fn run(program: &Program, import_parser: &ImportParser) -> Result<SymbolTable, StaticCheckError> {
        let mut check = Self::new();

        // Run checks on program and imports.
        check.check(program, import_parser)?;

        Ok(check.table)
    }

    ///
    /// Returns a new `SymbolTable` from a given program, input, and import parser.
    ///
    pub fn run_with_input(
        program: &Program,
        import_parser: &ImportParser,
        input: &Input,
    ) -> Result<SymbolTable, StaticCheckError> {
        let mut check = Self::new();

        // Load program input types.
        check.insert_input(input)?;

        // Run checks on program and imports.
        check.check(program, import_parser)?;

        Ok(check.table)
    }

    ///
    /// Computes pass one and pass two checks on self.
    ///
    pub fn check(&mut self, program: &Program, import_parser: &ImportParser) -> Result<(), StaticCheckError> {
        // Run pass one checks.
        self.pass_one(program, import_parser)?;

        // Run pass two checks.
        self.pass_two(program)
    }

    ///
    /// Inserts the program input types into the symbol table.
    ///
    pub fn insert_input(&mut self, input: &Input) -> Result<(), StaticCheckError> {
        self.table
            .insert_input(input)
            .map_err(|err| StaticCheckError::SymbolTableError(err))
    }

    ///
    /// Checks for duplicate circuit and function names given an unresolved program.
    ///
    /// If a circuit or function name has no duplicates, then it is inserted into the symbol table.
    /// Variables defined later in the unresolved program cannot have the same name.
    ///
    pub fn pass_one(&mut self, program: &Program, import_parser: &ImportParser) -> Result<(), StaticCheckError> {
        self.table
            .check_duplicate_program(program, import_parser)
            .map_err(|err| StaticCheckError::SymbolTableError(err))
    }

    ///
    /// Checks for unknown types in circuit and function definitions given an unresolved program.
    ///
    /// If a circuit or function definition only contains known types, then it is inserted into the
    /// symbol table. Variables defined later in the unresolved program can lookup the definition and
    /// refer to its expected types.
    ///
    pub fn pass_two(&mut self, program: &Program) -> Result<(), StaticCheckError> {
        self.table
            .check_unknown_types_program(program)
            .map_err(|err| StaticCheckError::SymbolTableError(err))
    }
}
