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

use crate::{assert_satisfied, get_output, parse_program, parse_program_with_input, EdwardsTestCompiler};

pub fn output_char(program: EdwardsTestCompiler) {
    let expected = include_bytes!("output/output_char.out");
    let actual = get_output(program);

    assert_eq!(expected, actual.bytes().as_slice());
}

#[test]
fn test_registers() {
    let program_string = include_str!("output_register.leo");
    let char_input_string = include_str!("input/char_register.in");

    // test true input register => true output register
    let program = parse_program_with_input(program_string, char_input_string).unwrap();

    output_char(program);
}

#[test]
fn test_basic() {
    let program_string = include_str!("basic.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_circuit() {
    let program_string = include_str!("circuit.leo");
    let char_input_string = include_str!("input/char.in");

    let program = parse_program_with_input(program_string, char_input_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_escapes() {
    let program_string = include_str!("escapes.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_hex() {
    let program_string = include_str!("hex.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_function() {
    let program_string = include_str!("function.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_octal() {
    let program_string = include_str!("octal.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_unicode() {
    let program_string = include_str!("unicode.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}
