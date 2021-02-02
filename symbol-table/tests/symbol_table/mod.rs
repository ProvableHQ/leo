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

use crate::TestSymbolTable;

use leo_ast::Input;
use leo_imports::ImportParser;
use leo_symbol_table::{SymbolTable, SymbolTableError};

///
/// Defines a circuit `Foo {}`.
/// Attempts to define a second circuit `Foo {}`.
///
/// Expected output: SymbolTableError
/// Message: "Duplicate circuit definition found for `Foo`."
///
#[test]
fn test_duplicate_circuit() {
    let program_string = include_str!("duplicate_circuit.leo");
    let resolver = TestSymbolTable::new(program_string);

    resolver.expect_pass_one_error();
}

///
/// Defines a function `main() {}`.
/// Attempts to define a second function `main() {}`.
///
/// Expected output: SymbolTableError
/// Message: "Duplicate function definition found for `main`."
///
#[test]
fn test_duplicate_function() {
    let program_string = include_str!("duplicate_function.leo");
    let resolver = TestSymbolTable::new(program_string);

    resolver.expect_pass_one_error();
}

///
/// Defines a function that returns `Self`.
///
/// Expected output: TypeError
/// Message: "Type `Self` is only available in circuit definitions and circuit functions."
///
#[test]
fn test_self_not_available() {
    let program_string = include_str!("self_not_available.leo");
    let resolver = TestSymbolTable::new(program_string);

    resolver.expect_pass_two_error();
}

///
/// Defines a circuit with variable whose type is `Bar`, an undefined circuit.
///
/// Expected output: TypeError
/// Message: "Type circuit `Bar` must be defined before it is used in an expression."
///
#[test]
fn test_undefined_circuit() {
    let program_string = include_str!("undefined_circuit.leo");
    let resolver = TestSymbolTable::new(program_string);

    resolver.expect_pass_two_error();
}

///
/// Imports an undefined function `boo` from file foo.leo.
///
/// Expected output: SymbolTableError
/// Message: Cannot find imported symbol `boo` in imported file ``
///
#[test]
fn test_import_undefined() {
    let program_string = include_str!("import_undefined.leo");
    let import_string = include_str!("imports/foo.leo");

    let program_table = TestSymbolTable::new(program_string);
    let import_table = TestSymbolTable::new(import_string);

    let import_program = import_table.ast.into_repr();

    let mut imports = ImportParser::default();
    imports.insert_import("foo".to_owned(), import_program);

    // Create new symbol table.
    let static_check = &mut SymbolTable::default();

    // Run pass one and expect an error.
    let error = static_check
        .check_names(&program_table.ast.into_repr(), &imports, &Input::new())
        .unwrap_err();

    match error {
        SymbolTableError::Error(_) => {} // Ok
        error => panic!("Expected a symbol table error found `{}`", error),
    }
}

///
/// Imports all functions from file foo.leo.
/// Calls function `foo` defined in foo.leo.
///
/// Expected output: Test Pass
///
#[test]
fn test_import_star() {
    let program_string = include_str!("import_star.leo");
    let import_string = include_str!("imports/foo.leo");

    let program_table = TestSymbolTable::new(program_string);
    let import_table = TestSymbolTable::new(import_string);

    let import_program = import_table.ast.into_repr();

    let mut imports = ImportParser::default();
    imports.insert_import("foo".to_owned(), import_program);

    program_table.expect_success(imports);
}

///
/// Imports a circuit named `Bar` from file bar.leo.
/// Renames `Bar` => `Baz`.
/// Defines a circuit named `Bar` in main.leo.
/// Instantiates circuits `Bar` and `Baz`.
///
/// Expected output: Test Pass
///
#[test]
fn test_import_circuit_alias() {
    let program_string = include_str!("import_circuit_alias.leo");
    let import_string = include_str!("imports/bar.leo");

    let program_table = TestSymbolTable::new(program_string);
    let import_table = TestSymbolTable::new(import_string);

    let import_program = import_table.ast.into_repr();

    let mut imports = ImportParser::default();
    imports.insert_import("bar".to_owned(), import_program);

    program_table.expect_success(imports);
}

///
/// Imports a function named `foo` from file foo.leo.
/// Renames `foo` => `boo`.
/// Defines a function named `foo` in main.leo.
/// Calls functions `foo` and `boo`.
///
/// Expected output: Test Pass
///
#[test]
fn test_import_function_alias() {
    let program_string = include_str!("import_function_alias.leo");
    let import_string = include_str!("imports/foo.leo");

    let program_table = TestSymbolTable::new(program_string);
    let import_table = TestSymbolTable::new(import_string);

    let import_program = import_table.ast.into_repr();

    let mut imports = ImportParser::default();
    imports.insert_import("foo".to_owned(), import_program);

    program_table.expect_success(imports);
}
