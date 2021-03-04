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

use crate::expect_compiler_error;
use crate::get_output;
use crate::parse_program_with_input;

#[test]
fn test_registers_pass() {
    let program_string = include_str!("registers_pass.leo");
    let input_string = include_str!("input/main.in");
    let expected = include_bytes!("output/registers_pass.out");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    let actual = get_output(program);

    assert!(expected.eq(actual.bytes().as_slice()));
}

#[test]
fn test_registers_fail() {
    let program_string = include_str!("registers_fail.leo");
    let input_string = include_str!("input/main.in");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_registers_array() {
    let program_string = include_str!("registers_array.leo");
    let input_string = include_str!("input/array.in");
    let expected = include_bytes!("output/registers_array.out");

    let program = parse_program_with_input(program_string, input_string).unwrap();

    let actual = get_output(program);

    assert!(expected.eq(actual.bytes().as_slice()));
}
