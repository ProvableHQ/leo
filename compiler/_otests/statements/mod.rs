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

use crate::{assert_satisfied, expect_asg_error, generate_main_input, parse_program, parse_program_with_input};
use leo_ast::InputValue;

pub mod conditional;

// Ternary if {bool}? {expression} : {expression};

#[test]
fn test_ternary_basic() {
    let program_string = include_str!("ternary_basic.leo");
    let mut program = parse_program(program_string).unwrap();

    let main_input = generate_main_input(vec![
        ("a", Some(InputValue::Boolean(true))),
        ("b", Some(InputValue::Boolean(true))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(program_string).unwrap();

    let main_input = generate_main_input(vec![
        ("a", Some(InputValue::Boolean(false))),
        ("b", Some(InputValue::Boolean(false))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}

#[test]
fn test_ternary_non_const_conditional_fail() {
    let program_string = include_str!("ternary_non_const_conditional_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

// Iteration for i {start}..{stop} { statements }

#[test]
fn test_iteration_basic() {
    let program_string = include_str!("iteration_basic.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_num_returns_fail() {
    let program_string = include_str!("num_returns_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_block() {
    let bytes = include_str!("block.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_iteration_input() {
    let input_string = include_str!("iteration_input.in");
    let program_string = include_str!("iteration_input.leo");
    let error = parse_program_with_input(program_string, input_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_iteration_wrong_type() {
    let program_string = include_str!("iteration_type_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_iteration_variable() {
    let program_string = include_str!("iteration_variable.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}
