use crate::parse_program;
use leo_ast::ParserError;
use leo_compiler::errors::CompilerError;

#[test]
fn test_semicolon() {
    let bytes = include_bytes!("semicolon.leo");
    let error = parse_program(bytes).err().unwrap();

    match error {
        CompilerError::ParserError(ParserError::SyntaxError(_)) => {}
        _ => panic!("test_semicolon failed the wrong expected error, should be a ParserError"),
    }
}
