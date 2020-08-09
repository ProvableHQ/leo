use leo_ast::LeoAst;
use leo_typed::LeoTypedAst;

use std::path::PathBuf;

fn to_typed_ast(program_filepath: &PathBuf) -> LeoTypedAst {
    // Loads the Leo code as a string from the given file path.
    let program_string = LeoAst::load_file(program_filepath).unwrap();

    // Parses the Leo file and constructs an abstract syntax tree.
    let ast = LeoAst::new(&program_filepath, &program_string).unwrap();

    // Parse the abstract syntax tree and constructs a typed syntax tree.
    let typed_ast = LeoTypedAst::new("leo_typed_tree", &ast);

    typed_ast
}

#[test]
#[cfg(not(feature = "ci_skip"))]
fn test_serialize() {
    // Construct a typed syntax tree from the given test file.
    let typed_ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/main.leo");

        to_typed_ast(&program_filepath)
    };

    // Serializes the typed syntax tree into JSON format.
    let serialized_typed_ast = typed_ast.to_json_string().unwrap();

    // Load the expected typed syntax tree.
    let expected = include_str!("expected_typed_ast.json");

    assert_eq!(expected, serialized_typed_ast);
}

#[test]
#[cfg(not(feature = "ci_skip"))]
fn test_deserialize() {
    // Load the expected typed syntax tree.
    let expected_typed_ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/main.leo");

        to_typed_ast(&program_filepath)
    };

    // Construct a typed syntax tree by deserializing a typed syntax tree JSON file.
    let serialized_typed_ast = include_str!("expected_typed_ast.json");
    let typed_ast = LeoTypedAst::from_json_string(serialized_typed_ast).unwrap();

    assert_eq!(expected_typed_ast, typed_ast);
}

#[test]
fn test_serialize_deserialize_serialize() {
    // Construct a typed syntax tree from the given test file.
    let typed_ast = {
        let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        program_filepath.push("tests/serialization/main.leo");

        to_typed_ast(&program_filepath)
    };

    // Serializes the typed syntax tree into JSON format.
    let serialized_typed_ast = typed_ast.to_json_string().unwrap();

    // Deserializes the typed syntax tree into a LeoTypedAst.
    let typed_ast = LeoTypedAst::from_json_string(&serialized_typed_ast).unwrap();

    // Reserializes the typed syntax tree into JSON format.
    let reserialized_typed_ast = typed_ast.to_json_string().unwrap();

    assert_eq!(serialized_typed_ast, reserialized_typed_ast);
}
