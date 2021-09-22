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
use leo_errors::{LeoError, Result};

use std::fs::File;
use std::io::BufReader;
use std::iter::Iterator;
use std::path::{Path, PathBuf};

fn to_ast(program_filepath: &Path) -> Result<Ast> {
    let program_string = std::fs::read_to_string(program_filepath).expect("failed to open test");

    // Parses the Leo file and constructs a leo ast.
    leo_parser::parse_ast("", &program_string)
}

fn setup() {
    std::env::set_var("LEO_TESTFRAMEWORK", "true");
}

fn clean() {
    std::env::remove_var("LEO_TESTFRAMEWORK");
}

#[test]
#[cfg(not(feature = "ci_skip"))]
fn test_serialize() {
    setup();

    // Construct an ast from the given test file.
    let ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/leo/one_plus_one.leo");

        to_ast(&program_filepath).unwrap()
    };

    // Serializes the ast into JSON format.
    let serialized_ast: Program = serde_json::from_value(serde_json::to_value(ast.as_repr()).unwrap()).unwrap();

    // Load the expected ast.
    let expected: Program = serde_json::from_str(include_str!("./expected_leo_ast/one_plus_one.json")).unwrap();

    clean();
    assert_eq!(expected, serialized_ast);
}

#[test]
#[cfg(not(feature = "ci_skip"))]
fn test_serialize_no_span() {
    setup();

    let program_paths = vec![
        "tests/serialization/leo/linear_regression.leo",
        "tests/serialization/leo/palindrome.leo",
        "tests/serialization/leo/pedersen_hash.leo",
        "tests/serialization/leo/silly_sudoku.leo",
    ];

    let json_paths = vec![
        "tests/serialization/expected_leo_ast/linear_regression.json",
        "tests/serialization/expected_leo_ast/palindrome.json",
        "tests/serialization/expected_leo_ast/pedersen_hash.json",
        "tests/serialization/expected_leo_ast/silly_sudoku.json",
    ];

    for (program_path, json_path) in program_paths.into_iter().zip(json_paths) {
        // Construct an ast from the given test file.
        let ast = {
            let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            program_filepath.push(program_path);
            to_ast(&program_filepath).unwrap()
        };

        let json_reader = {
            let mut json_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            json_filepath.push(json_path);
            let file = File::open(json_filepath).expect("Failed to read expected ast file");
            BufReader::new(file)
        };

        // Serializes the ast into JSON format.
        let mut serialized_ast: serde_json::Value = serde_json::to_value(ast.as_repr()).unwrap();
        remove_key_from_json(&mut serialized_ast, "span");

        // Load the expected ast.
        let expected: serde_json::Value = serde_json::from_reader(json_reader).unwrap();

        assert_eq!(expected, serialized_ast);
    }
    clean();
}

// Helper function to recursively filter keys from AST JSON.
// Redeclaring here since we don't want to make this public.
fn remove_key_from_json(value: &mut serde_json::Value, key: &str) {
    match value {
        serde_json::value::Value::Object(map) => {
            map.remove(key);
            for val in map.values_mut() {
                remove_key_from_json(val, key);
            }
        }
        serde_json::value::Value::Array(values) => {
            for val in values.iter_mut() {
                remove_key_from_json(val, key);
            }
        }
        _ => (),
    }
}

// TODO Renable when we don't write spans to snapshots.
/* #[test]
#[cfg(not(feature = "ci_skip"))]
fn test_deserialize() {
    setup();

    // Load the expected ast.
    let expected_ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/main.leo");

        to_ast(&program_filepath).unwrap()
    };

    // Construct an ast by deserializing a ast JSON file.
    let serialized_ast = include_str!("expected_leo_ast.json");
    let ast = Ast::from_json_string(serialized_ast).unwrap();

    clean();
    assert_eq!(expected_ast, ast);
}

#[test]
fn test_serialize_deserialize_serialize() {
    setup();

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

    clean();
    assert_eq!(serialized_ast, reserialized_ast);
} */

#[test]
fn test_generic_parser_error() {
    setup();

    let error_result = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/leo/parser_error.leo");

        to_ast(&program_filepath)
    }
    .map_err(|err| matches!(err, LeoError::ParserError(_)));

    clean();
    assert!(error_result.err().unwrap());
}
