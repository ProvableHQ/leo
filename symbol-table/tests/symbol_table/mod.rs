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

use crate::TestSymbolTable;

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
