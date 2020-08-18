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

use crate::{assert_satisfied, expect_compiler_error, generate_main_input, parse_program};
use leo_typed::InputValue;

#[test]
fn test_let() {
    let bytes = include_bytes!("let.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_let_mut() {
    let bytes = include_bytes!("let_mut.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_const_fail() {
    let bytes = include_bytes!("const.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_const_mut_fail() {
    let bytes = include_bytes!("const_mut.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_array() {
    let bytes = include_bytes!("array.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_array_mut() {
    let bytes = include_bytes!("array_mut.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_circuit() {
    let bytes = include_bytes!("circuit.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_circuit_mut() {
    let bytes = include_bytes!("circuit_mut.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_function_input() {
    let bytes = include_bytes!("function_input.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(true)))]);

    program.set_main_input(main_input);

    expect_compiler_error(program);
}

#[test]
fn test_function_input_mut() {
    let bytes = include_bytes!("function_input_mut.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(true)))]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}
