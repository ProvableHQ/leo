use crate::{boolean::output_true, parse_program};
use leo_compiler::errors::CompilerError;
use leo_inputs::InputParserError;

use std::path::PathBuf;

fn fail_input_parser(error: CompilerError) {
    match error {
        CompilerError::InputParserError(InputParserError::InputNotFound(_)) => {}
        err => panic!("expected input parser error, got {}", err),
    }
}

#[test]
fn test_inputs_pass() {
    let program_bytes = include_bytes!("main.leo");
    let input_bytes = include_bytes!("inputs.leo");
    let input_string = String::from_utf8_lossy(input_bytes);

    let mut program = parse_program(program_bytes).unwrap();
    let path = PathBuf::new();
    program.parse_inputs(&path, &input_string).unwrap();

    output_true(program);
}

#[test]
fn test_inputs_fail_name() {
    let program_bytes = include_bytes!("main.leo");
    let input_bytes = include_bytes!("inputs_fail_name.leo");
    let input_string = String::from_utf8_lossy(input_bytes);

    let mut program = parse_program(program_bytes).unwrap();
    let path = PathBuf::new();
    let error = program.parse_inputs(&path, &input_string).unwrap_err();

    fail_input_parser(error);
}

#[test]
fn test_inputs_fail_type() {
    let program_bytes = include_bytes!("main.leo");
    let input_bytes = include_bytes!("inputs_fail_type.leo");
    let input_string = String::from_utf8_lossy(input_bytes);

    let mut program = parse_program(program_bytes).unwrap();
    let path = PathBuf::new();
    let error = program.parse_inputs(&path, &input_string).unwrap_err();

    fail_input_parser(error);
}

#[test]
fn test_inputs_multiple() {
    let program_bytes = include_bytes!("main_multiple.leo");
    let input_bytes = include_bytes!("inputs_multiple.leo");
    let input_string = String::from_utf8_lossy(input_bytes);

    let mut program = parse_program(program_bytes).unwrap();
    let path = PathBuf::new();
    program.parse_inputs(&path, &input_string).unwrap();

    output_true(program);
}
