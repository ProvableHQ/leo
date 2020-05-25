use crate::{compile_program, get_error};
use leo_compiler::errors::CompilerError;

const DIRECTORY_NAME: &str = "tests/syntax/";

#[test]
fn test_semicolon() {
    let error = compile_program(DIRECTORY_NAME, "semicolon.leo").err().unwrap();

    match error {
        CompilerError::SyntaxError(_) => {},
        _ => panic!("wrong error")
    }
}