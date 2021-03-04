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

use crate::assert_satisfied;
use crate::expect_asg_error;
use crate::expect_compiler_error;
use crate::generate_main_input;
use crate::parse_program;
use leo_ast::InputValue;

static TEST_ADDRESS_1: &str = "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8";
static TEST_ADDRESS_2: &str = "aleo18qgam03qe483tdrcc3fkqwpp38ehff4a2xma6lu7hams6lfpgcpq3dq05r";

#[test]
fn test_valid() {
    let program_string = include_str!("valid.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program)
}

#[test]
fn test_invalid_prefix() {
    let program_string = include_str!("invalid_prefix.leo");
    let syntax_error = parse_program(program_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_invalid_length() {
    let program_string = include_str!("invalid_length.leo");
    let syntax_error = parse_program(program_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_empty() {
    let program_string = include_str!("empty.leo");
    let syntax_error = parse_program(program_string).is_err();

    assert!(syntax_error);
}

#[test]
fn test_implicit_valid() {
    let program_string = include_str!("implicit_valid.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_implicit_invalid() {
    let program_string = include_str!("implicit_invalid.leo");
    let error = parse_program(program_string).err().unwrap();

    expect_asg_error(error);
}

#[test]
fn test_console_assert_pass() {
    let program_string = include_str!("console_assert_pass.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_console_assert_fail() {
    let program_string = include_str!("console_assert_fail.leo");
    let program = parse_program(program_string).unwrap();

    let _output = expect_compiler_error(program);
}

#[test]
fn test_ternary() {
    let program_string = include_str!("ternary.leo");
    let mut program = parse_program(program_string).unwrap();

    let main_input = generate_main_input(vec![
        ("s", Some(InputValue::Boolean(true))),
        ("c", Some(InputValue::Address(TEST_ADDRESS_1.to_string()))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(program_string).unwrap();

    let main_input = generate_main_input(vec![
        ("s", Some(InputValue::Boolean(false))),
        ("c", Some(InputValue::Address(TEST_ADDRESS_2.to_string()))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}

#[test]
fn test_equal() {
    let program_string = include_str!("equal.leo");
    let mut program = parse_program(program_string).unwrap();

    let main_input = generate_main_input(vec![
        ("a", Some(InputValue::Address(TEST_ADDRESS_1.to_string()))),
        ("b", Some(InputValue::Address(TEST_ADDRESS_1.to_string()))),
        ("c", Some(InputValue::Boolean(true))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(program_string).unwrap();

    let main_input = generate_main_input(vec![
        ("a", Some(InputValue::Address(TEST_ADDRESS_1.to_string()))),
        ("b", Some(InputValue::Address(TEST_ADDRESS_2.to_string()))),
        ("c", Some(InputValue::Boolean(false))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}
