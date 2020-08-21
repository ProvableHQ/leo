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

static TEST_ADDRESS_1: &'static str = "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8";
static TEST_ADDRESS_2: &'static str = "aleo18qgam03qe483tdrcc3fkqwpp38ehff4a2xma6lu7hams6lfpgcpq3dq05r";

#[test]
fn test_valid() {
    let bytes = include_bytes!("valid.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program)
}

#[test]
fn test_invalid_prefix() {
    let bytes = include_bytes!("invalid_prefix.leo");
    let syntax_error = parse_program(bytes).is_err();

    assert!(syntax_error);
}

#[test]
fn test_invalid_length() {
    let bytes = include_bytes!("invalid_length.leo");
    let syntax_error = parse_program(bytes).is_err();

    assert!(syntax_error);
}

#[test]
fn test_empty() {
    let bytes = include_bytes!("empty.leo");
    let syntax_error = parse_program(bytes).is_err();

    assert!(syntax_error);
}

#[test]
fn test_implicit_valid() {
    let bytes = include_bytes!("implicit_valid.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_implicit_invalid() {
    let bytes = include_bytes!("implicit_invalid.leo");
    let program = parse_program(bytes).unwrap();

    let _output = expect_compiler_error(program);
}

#[test]
fn test_console_assert_pass() {
    let bytes = include_bytes!("console_assert_pass.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_console_assert_fail() {
    let bytes = include_bytes!("console_assert_fail.leo");
    let program = parse_program(bytes).unwrap();

    let _output = expect_compiler_error(program);
}

#[test]
fn test_ternary() {
    let bytes = include_bytes!("ternary.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![
        ("s", Some(InputValue::Boolean(true))),
        ("c", Some(InputValue::Address(TEST_ADDRESS_1.to_string()))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![
        ("s", Some(InputValue::Boolean(false))),
        ("c", Some(InputValue::Address(TEST_ADDRESS_2.to_string()))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}

#[test]
fn test_equal() {
    let bytes = include_bytes!("equal.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![
        ("a", Some(InputValue::Address(TEST_ADDRESS_1.to_string()))),
        ("b", Some(InputValue::Address(TEST_ADDRESS_1.to_string()))),
        ("c", Some(InputValue::Boolean(true))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![
        ("a", Some(InputValue::Address(TEST_ADDRESS_1.to_string()))),
        ("b", Some(InputValue::Address(TEST_ADDRESS_2.to_string()))),
        ("c", Some(InputValue::Boolean(false))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}
