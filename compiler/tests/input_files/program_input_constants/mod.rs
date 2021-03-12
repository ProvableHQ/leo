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

use crate::{assert_satisfied, expect_compiler_error, parse_program_with_input, EdwardsTestCompiler};
use leo_compiler::errors::CompilerError;

fn expect_fail(program: EdwardsTestCompiler) {
    match expect_compiler_error(program) {
        CompilerError::FunctionError(_) => {}
        err => panic!("expected input parser error, got {:?}", err),
    }
}

#[test]
fn test_input_pass() {
    let program_string = include_str!("main.leo");
    let input_string = include_str!("input/main.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_array_fail() {
    let program_string = include_str!("main_array.leo");
    let input_string = include_str!("input/main_array.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_multi_dimension_array() {
    let program_string = include_str!("main_multi_dimension_array.leo");
    let input_string = include_str!("input/main_multi_dimension_array.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_fail_name() {
    let program_string = include_str!("main.leo");
    let input_string = include_str!("input/main_fail_name.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    expect_fail(program);
}

#[test]
fn test_input_fail_type() {
    let program_string = include_str!("main.leo");
    let input_string = include_str!("input/main_fail_type.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    expect_fail(program);
}

#[test]
fn test_input_multiple() {
    let program_string = include_str!("main_multiple.leo");
    let input_string = include_str!("input/main_multiple.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_array_dimensions_mismatch() {
    let program_string = include_str!("main_array_fail.leo");
    let input_string = include_str!("input/main_array_fail.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_field_input() {
    let program_string = include_str!("main_field.leo");
    let input_string = include_str!("input/main_field.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_group_input() {
    let program_string = include_str!("main_group.leo");
    let input_string = include_str!("input/main_group.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    assert_satisfied(program);
}
