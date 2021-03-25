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

use crate::{assert_satisfied, expect_asg_error, parse_program};

// Expressions

#[test]
fn test_inline() {
    let program_string = include_str!("inline.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_inline_fail() {
    let program_string = include_str!("inline_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_inline_undefined() {
    let program_string = include_str!("inline_undefined.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

// Members

#[test]
fn test_member_variable() {
    let program_string = include_str!("member_variable.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_variable_fail() {
    let program_string = include_str!("member_variable_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_member_variable_and_function() {
    let program_string = include_str!("member_variable_and_function.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_function() {
    let program_string = include_str!("member_function.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_function_fail() {
    let program_string = include_str!("member_function_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_member_function_invalid() {
    let program_string = include_str!("member_function_invalid.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_member_function_nested() {
    let program_string = include_str!("member_function_nested.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_static_function() {
    let program_string = include_str!("member_static_function.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_static_function_nested() {
    let program_string = include_str!("member_static_function_nested.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_static_function_invalid() {
    let program_string = include_str!("member_static_function_invalid.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error)
}

#[test]
fn test_member_static_function_undefined() {
    let program_string = include_str!("member_static_function_undefined.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error)
}

// Constant
#[test]
fn test_const_self_variable_fail() {
    let program_string = include_str!("const_self_variable_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

// Mutability

#[test]
fn test_mutate_function_fail() {
    let program_string = include_str!("mut_function_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_mutate_self_variable() {
    let program_string = include_str!("mut_self_variable.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_mutate_self_variable_branch() {
    let program_string = include_str!("mut_self_variable_branch.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_mutate_self_variable_conditional() {
    let program_string = include_str!("mut_self_variable_conditional.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_mutate_self_variable_fail() {
    let program_string = include_str!("mut_self_variable_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_mutate_self_function_fail() {
    let program_string = include_str!("mut_self_function_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_mutate_self_static_function_fail() {
    let program_string = include_str!("mut_self_static_function_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_mutate_static_function_fail() {
    let program_string = include_str!("mut_static_function_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_mutate_variable() {
    let program_string = include_str!("mut_variable.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_mutate_variable_fail() {
    let program_string = include_str!("mut_variable_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

// Self

#[test]
fn test_self_fail() {
    let program_string = include_str!("self_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_self_member_pass() {
    let program_string = include_str!("self_member.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_self_member_invalid() {
    let program_string = include_str!("self_member_invalid.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_self_member_undefined() {
    let program_string = include_str!("self_member_undefined.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

// Inline circuit member

#[test]
fn test_inline_member_pass() {
    let program_string = include_str!("inline_member_pass.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_inline_member_fail() {
    let program_string = include_str!("inline_member_fail.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

// All

#[test]
fn test_pedersen_mock() {
    let program_string = include_str!("pedersen_mock.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_define_circuit_inside_circuit_function() {
    let program_string = include_str!("define_circuit_inside_circuit_function.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_duplicate_name_context() {
    let program_string = include_str!("duplicate_name_context.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}
