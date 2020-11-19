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

#[test]
#[cfg(not(feature = "ci_skip"))]
fn test_serialize() {
    use leo_grammar::Grammar;
    use std::path::PathBuf;

    let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    program_filepath.push("tests/serialization/main.leo");

    let expected = include_str!("./expected_ast.json");

    // Loads the Leo code as a string from the given file path.
    let program_string = Grammar::load_file(&program_filepath).unwrap();

    // Parses the Leo file and constructs an abstract syntax tree.
    let ast = Grammar::new(&program_filepath, &program_string).unwrap();

    // Serializes the abstract syntax tree into JSON format.
    let serialized_ast = Grammar::to_json_string(&ast).unwrap();

    assert_eq!(expected, serialized_ast);
}
