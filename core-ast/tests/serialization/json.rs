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

use leo_core_ast::LeoCoreAst;
#[cfg(not(feature = "ci_skip"))]
use leo_core_ast::Program;
use leo_grammar::Grammar;

use std::path::{Path, PathBuf};

fn to_core_ast(program_filepath: &Path) -> LeoCoreAst {
    // Loads the Leo code as a string from the given file path.
    let program_string = Grammar::load_file(program_filepath).unwrap();

    // Parses the Leo file and constructs a pest ast.
    let ast = Grammar::new(&program_filepath, &program_string).unwrap();

    // Parses the pest ast and constructs a core ast.
    LeoCoreAst::new("leo_core_tree", &ast)
}

#[test]
#[cfg(not(feature = "ci_skip"))]
fn test_serialize() {
    // Construct a core ast from the given test file.
    let core_ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/main.leo");

        to_core_ast(&program_filepath)
    };

    // Serializes the core ast into JSON format.
    let serialized_core_ast: Program =
        serde_json::from_value(serde_json::to_value(core_ast.into_repr()).unwrap()).unwrap();

    // Load the expected core ast.
    let expected: Program = serde_json::from_str(include_str!("expected_core_ast.json")).unwrap();

    assert_eq!(expected, serialized_core_ast);
}

#[test]
#[cfg(not(feature = "ci_skip"))]
fn test_deserialize() {
    // Load the expected core ast.
    let expected_core_ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/main.leo");

        to_core_ast(&program_filepath)
    };

    // Construct a core ast by deserializing a core ast JSON file.
    let serialized_typed_ast = include_str!("expected_core_ast.json");
    let core_ast = LeoCoreAst::from_json_string(serialized_typed_ast).unwrap();

    assert_eq!(expected_core_ast, core_ast);
}

#[test]
fn test_serialize_deserialize_serialize() {
    // Construct a core ast from the given test file.
    let core_ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/main.leo");

        to_core_ast(&program_filepath)
    };

    // Serializes the core ast into JSON format.
    let serialized_core_ast = core_ast.to_json_string().unwrap();

    // Deserializes the serialized core ast into a LeoCoreAst.
    let core_ast = LeoCoreAst::from_json_string(&serialized_core_ast).unwrap();

    // Reserializes the core ast into JSON format.
    let reserialized_core_ast = core_ast.to_json_string().unwrap();

    assert_eq!(serialized_core_ast, reserialized_core_ast);
}
