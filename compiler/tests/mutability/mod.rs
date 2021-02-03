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

use crate::{assert_satisfied, expect_asg_error, generate_main_input, parse_program};
use leo_ast::InputValue;

#[test]
fn test_let() {
    let program_string = include_str!("let.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_let_mut() {
    let program_string = include_str!("let_mut.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_let_mut_nested() {
    let program_string = include_str!("let_mut_nested.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_const_fail() {
    let program_string = include_str!("const.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_const_mut_fail() {
    let program_string = include_str!("const_mut.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_array() {
    let program_string = include_str!("array.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_array_mut() {
    let program_string = include_str!("array_mut.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_array_tuple_mut() {
    let bytes = include_str!("array_tuple_mut.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_array_splice_mut() {
    let bytes = include_str!("array_splice_mut.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_circuit() {
    let program_string = include_str!("circuit.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_circuit_mut() {
    let program_string = include_str!("circuit_mut.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_circuit_variable_mut() {
    let program_string = include_str!("circuit_variable_mut.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_circuit_function_mut() {
    let program_string = include_str!("circuit_function_mut.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_circuit_static_function_mut() {
    let program_string = include_str!("circuit_static_function_mut.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_function_input() {
    let program_string = include_str!("function_input.leo");
    let error = parse_program(program_string).err().unwrap();
    expect_asg_error(error);
}

#[test]
fn test_function_input_mut() {
    let program_string = include_str!("function_input_mut.leo");
    let mut program = parse_program(program_string).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(true)))]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}

#[test]
#[ignore]
fn test_swap() {
    let program_string = include_str!("swap.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}
