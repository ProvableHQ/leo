use crate::{
    assert_satisfied,
    expect_compiler_error,
    expect_synthesis_error,
    get_outputs,
    parse_program,
    parse_program_with_input,
    EdwardsTestCompiler,
};
use leo_compiler::errors::{BooleanError, CompilerError, ExpressionError, FunctionError, StatementError};

pub fn output_true(program: EdwardsTestCompiler) {
    let expected = include_bytes!("outputs_/registers_true.out");
    let actual = get_outputs(program);

    assert_eq!(expected, actual.bytes().as_slice());
}

pub fn output_false(program: EdwardsTestCompiler) {
    let expected = include_bytes!("outputs_/registers_false.out");
    let actual = get_outputs(program);

    assert_eq!(expected, actual.bytes().as_slice());
}

fn fail_boolean_statement(program: EdwardsTestCompiler) {
    match expect_compiler_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::BooleanError(BooleanError::Error(_)),
        ))) => {}
        _ => panic!("Expected boolean error, got {}"),
    }
}

#[test]
fn test_input_pass() {
    let program_bytes = include_bytes!("assert_eq_input.leo");
    let input_bytes = include_bytes!("input/true_true.in");

    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input_fail() {
    let program_bytes = include_bytes!("assert_eq_input.leo");
    let input_bytes = include_bytes!("input/true_false.in");

    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    expect_synthesis_error(program);
}

#[test]
fn test_registers() {
    let program_bytes = include_bytes!("output_register.leo");
    let true_input_bytes = include_bytes!("input/registers_true.in");
    let false_input_bytes = include_bytes!("input/registers_false.in");

    // test true input register => true output register
    let program = parse_program_with_input(program_bytes, true_input_bytes).unwrap();

    output_true(program);

    // test false input register => false output register
    let program = parse_program_with_input(program_bytes, false_input_bytes).unwrap();

    output_false(program);
}

// Boolean not !

#[test]
fn test_not_true() {
    let bytes = include_bytes!("not_true.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_not_false() {
    let bytes = include_bytes!("not_false.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_not_u32() {
    let bytes = include_bytes!("not_u32.leo");
    let program = parse_program(bytes).unwrap();

    fail_boolean_statement(program);
}

// Boolean or ||

#[test]
fn test_true_or_true() {
    let bytes = include_bytes!("true_or_true.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_true_or_false() {
    let bytes = include_bytes!("true_or_false.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_false_or_false() {
    let bytes = include_bytes!("false_or_false.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_true_or_u32() {
    let bytes = include_bytes!("true_or_u32.leo");
    let program = parse_program(bytes).unwrap();

    fail_boolean_statement(program);
}

// Boolean and &&

#[test]
fn test_true_and_true() {
    let bytes = include_bytes!("true_and_true.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_true_and_false() {
    let bytes = include_bytes!("true_and_false.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_false_and_false() {
    let bytes = include_bytes!("false_and_false.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_true_and_u32() {
    let bytes = include_bytes!("true_and_u32.leo");
    let program = parse_program(bytes).unwrap();

    fail_boolean_statement(program);
}

// All

#[test]
fn test_all() {
    let bytes = include_bytes!("all.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}
