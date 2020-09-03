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

use crate::parse_program;

#[test]
fn test_address_name_fail() {
    let bytes = include_bytes!("address_fail.leo");
    let syntax_error = parse_program(bytes).is_err();

    assert!(syntax_error);
}

#[test]
fn test_console_name_fail() {
    let bytes = include_bytes!("console_fail.leo");
    let syntax_error = parse_program(bytes).is_err();

    assert!(syntax_error);
}

#[test]
fn test_field_name_fail() {
    let bytes = include_bytes!("field_fail.leo");
    let syntax_error = parse_program(bytes).is_err();

    assert!(syntax_error);
}

#[test]
fn test_group_name_fail() {
    let bytes = include_bytes!("group_fail.leo");
    let syntax_error = parse_program(bytes).is_err();

    assert!(syntax_error);
}

#[test]
fn test_i8_name_fail() {
    let bytes = include_bytes!("i8_fail.leo");
    let syntax_error = parse_program(bytes).is_err();

    assert!(syntax_error);
}

#[test]
fn test_input_name_fail() {
    let bytes = include_bytes!("input_fail.leo");
    let syntax_error = parse_program(bytes).is_err();

    assert!(syntax_error);
}

#[test]
fn test_true_name_fail() {
    let bytes = include_bytes!("true_fail.leo");
    let syntax_error = parse_program(bytes).is_err();

    assert!(syntax_error);
}

#[test]
fn test_u8_name_fail() {
    let bytes = include_bytes!("u8_fail.leo");
    let syntax_error = parse_program(bytes).is_err();

    assert!(syntax_error);
}
