use crate::{assert_satisfied, expect_compiler_error, parse_program_with_input, EdwardsTestCompiler};
use leo_compiler::errors::CompilerError;

fn expect_fail(program: EdwardsTestCompiler) {
    match expect_compiler_error(program) {
        CompilerError::FunctionError(_) => {}
        err => panic!("expected input parser error, got {:?}", err),
    }
}

#[test]
fn test_inputs_pass() {
    let program_bytes = include_bytes!("main.leo");
    let input_bytes = include_bytes!("inputs/main.in");

    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_inputs_fail_name() {
    let program_bytes = include_bytes!("main.leo");
    let input_bytes = include_bytes!("inputs/main_fail_name.in");

    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    expect_fail(program);
}

#[test]
fn test_inputs_fail_type() {
    let program_bytes = include_bytes!("main.leo");
    let input_bytes = include_bytes!("inputs/main_fail_type.in");

    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    expect_fail(program);
}

#[test]
fn test_inputs_multiple() {
    let program_bytes = include_bytes!("main_multiple.leo");
    let input_bytes = include_bytes!("inputs/main_multiple.in");

    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}
