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

use leo_ast::LeoAst;
#[cfg(not(feature = "ci_skip"))]
use leo_ast::Program;
use leo_grammar::Grammar;

use std::path::{Path, PathBuf};

fn to_ast(program_filepath: &Path) -> LeoAst {
    // Loads the Leo code as a string from the given file path.
    let program_string = Grammar::load_file(program_filepath).unwrap();

    // Parses the Leo file and constructs a grammar ast.
    let ast = Grammar::new(&program_filepath, &program_string).unwrap();

    // Parses the pest ast and constructs a Leo ast.
    LeoAst::new("leo_tree", &ast)
}

#[test]
#[cfg(not(feature = "ci_skip"))]
fn test_serialize() {
    // Construct a ast from the given test file.
    let leo_ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/main.leo");

        to_ast(&program_filepath)
    };

    // Serializes the ast into JSON format.
    let serialized_leo_ast: Program =
        serde_json::from_value(serde_json::to_value(leo_ast.into_repr()).unwrap()).unwrap();

    // Load the expected ast.
    let expected: Program = serde_json::from_str(include_str!("expected_leo_ast.json")).unwrap();

    assert_eq!(expected, serialized_leo_ast);
}

#[test]
#[cfg(not(feature = "ci_skip"))]
fn test_deserialize() {
    // Load the expected ast.
    let expected_leo_ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/main.leo");

        to_ast(&program_filepath)
    };

    // Construct an ast by deserializing a ast JSON file.
    let serialized_ast = include_str!("expected_leo_ast.json");
    let leo_ast = LeoAst::from_json_string(serialized_ast).unwrap();

    assert_eq!(expected_leo_ast, leo_ast);
}

#[test]
fn test_serialize_deserialize_serialize() {
    // Construct a ast from the given test file.
    let leo_ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/main.leo");

        to_ast(&program_filepath)
    };

    // Serializes the ast into JSON format.
    let serialized_leo_ast = leo_ast.to_json_string().unwrap();

    // Deserializes the serialized ast into a LeoAst.
    let leo_ast = LeoAst::from_json_string(&serialized_leo_ast).unwrap();

    // Reserializes the ast into JSON format.
    let reserialized_leo_ast = leo_ast.to_json_string().unwrap();

    assert_eq!(serialized_leo_ast, reserialized_leo_ast);
}
