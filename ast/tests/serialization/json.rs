use leo_ast::LeoAst;

use std::path::PathBuf;

#[test]
#[cfg(not(feature = "ci_skip"))]
fn test_serialize() {
    let mut program_filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    program_filepath.push("tests/serialization/main.leo");

    let expected = include_str!("./expected_ast.json");

    // Loads the Leo code as a string from the given file path.
    let program_string = LeoAst::load_file(&program_filepath).unwrap();

    // Parses the Leo file and constructs an abstract syntax tree.
    let ast = LeoAst::new(&program_filepath, &program_string).unwrap();

    // Serializes the abstract syntax tree into JSON format.
    let serialized_ast = LeoAst::to_json_string(&ast).unwrap();

    // println!("{:#?}", serialized_ast);

    assert_eq!(expected, serialized_ast);
}
