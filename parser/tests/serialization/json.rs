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

use leo_ast::Ast;
#[cfg(not(feature = "ci_skip"))]
use leo_ast::Program;
use leo_errors::LeoError;

use std::path::{Path, PathBuf};

fn to_ast(program_filepath: &Path) -> Result<Ast, LeoError> {
    let program_string = std::fs::read_to_string(program_filepath).expect("failed to open test");

    // Parses the Leo file and constructs a leo ast.
    leo_parser::parse_ast("test", &program_string)
}

#[test]
#[cfg(not(feature = "ci_skip"))]
fn test_serialize() {
    // Construct an ast from the given test file.
    let ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/main.leo");

        to_ast(&program_filepath).unwrap()
    };

    // Serializes the ast into JSON format.
    let serialized_ast: Program = serde_json::from_value(serde_json::to_value(ast.as_repr()).unwrap()).unwrap();

    // Load the expected ast.
    let expected: Program = serde_json::from_str(include_str!("expected_leo_ast.json")).unwrap();

    assert_eq!(expected, serialized_ast);
}

#[test]
#[cfg(not(feature = "ci_skip"))]
fn test_deserialize() {
    // Load the expected ast.
    let expected_ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/main.leo");

        to_ast(&program_filepath).unwrap()
    };

    // Construct an ast by deserializing a ast JSON file.
    let serialized_ast = include_str!("expected_leo_ast.json");
    let ast = Ast::from_json_string(serialized_ast).unwrap();

    assert_eq!(expected_ast, ast);
}

#[test]
fn test_serialize_deserialize_serialize() {
    // Construct an ast from the given test file.
    let ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/main.leo");

        to_ast(&program_filepath).unwrap()
    };

    // Serializes the ast into JSON format.
    let serialized_ast = ast.to_json_string().unwrap();

    // Deserializes the serialized ast into an ast.
    let ast = Ast::from_json_string(&serialized_ast).unwrap();

    // Reserializes the ast into JSON format.
    let reserialized_ast = ast.to_json_string().unwrap();

    assert_eq!(serialized_ast, reserialized_ast);
}

#[test]
fn test_generic_parser_error() {
    let error_result = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/parser_error.leo");

        to_ast(&program_filepath)
    }
    .map_err(|err| matches!(err, LeoError::ParserError(_)));

    assert!(error_result.err().unwrap());
}
