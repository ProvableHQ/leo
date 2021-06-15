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

use crate::parse_program;
use leo_ast::Ast;
use leo_parser::parser;

pub fn parse_program_ast(file_string: &str) -> Ast {
    const TEST_PROGRAM_PATH: &str = "";
    let test_program_file_path = std::path::PathBuf::from(TEST_PROGRAM_PATH);

    let mut ast = Ast::new(
        parser::parse(test_program_file_path.to_str().expect("unwrap fail"), &file_string)
            .expect("Failed to parse file."),
    );
    ast.canonicalize().expect("Failed to canonicalize program.");

    ast
}

#[test]
fn test_big_self_in_circuit_replacement() {
    // Check program is valid.
    let program_string = include_str!("big_self_in_circuit_replacement.leo");
    // Check we get expected ast.
    let ast = parse_program_ast(program_string);
    let expected_json = include_str!("big_self_in_circuit_replacement.json");
    let expected_ast: Ast = Ast::from_json_string(expected_json).expect("Unable to parse json.");

    assert_eq!(expected_ast, ast);
}

#[test]
fn test_big_self_outside_circuit_fail() {
    // Check program is invalid.
    let program_string = include_str!("big_self_outside_circuit_fail.leo");
    let program = parse_program(program_string);
    assert!(program.is_err());
}

#[test]
fn test_array_expansion() {
    let program_string = include_str!("array_expansion.leo");
    let ast = parse_program_ast(program_string);
    let expected_json = include_str!("array_expansion.json");
    let expected_ast: Ast = Ast::from_json_string(expected_json).expect("Unable to parse json.");

    assert_eq!(expected_ast, ast);
}

#[test]
fn test_array_size_zero_fail() {
    let program_string = include_str!("array_size_zero_fail.leo");
    let program = parse_program(program_string);
    assert!(program.is_err());
}

#[test]
fn test_compound_assignment() {
    let program_string = include_str!("compound_assignment.leo");
    let ast = parse_program_ast(program_string);
    let expected_json = include_str!("compound_assignment.json");
    let expected_ast: Ast = Ast::from_json_string(expected_json).expect("Unable to parse json.");

    assert_eq!(expected_ast, ast);
}

#[test]
fn test_illegal_array_range_fail() {
    // Check program is invalid.
    let program_string = include_str!("illegal_array_range_fail.leo");
    let program = parse_program(program_string);
    assert!(program.is_err());
}

#[test]
fn test_string_transformation() {
    let program_string = include_str!("string_transformation.leo");
    let ast = parse_program_ast(program_string);
    let expected_json = include_str!("string_transformation.json");
    let expected_ast: Ast = Ast::from_json_string(expected_json).expect("Unable to parse json.");

    assert_eq!(expected_ast, ast);
}
