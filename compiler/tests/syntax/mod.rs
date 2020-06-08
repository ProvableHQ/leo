use crate::compile_program;
use leo_compiler::errors::CompilerError;

const DIRECTORY_NAME: &str = "tests/syntax/";

#[test]
fn test_semicolon() {
    let error = compile_program(DIRECTORY_NAME, "semicolon.leo").err().unwrap();

    match error {
        CompilerError::ParserError(_) => {}
        _ => panic!("test_semicolon failed the wrong expected error, should be a ParserError"),
    }
}
