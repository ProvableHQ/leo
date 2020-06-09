use crate::{get_error, get_output, parse_program, EdwardsConstrainedValue, EdwardsTestCompiler};
use leo_compiler::{
    errors::{BooleanError, CompilerError, ExpressionError, FunctionError, StatementError},
    ConstrainedValue,
};
use leo_types::InputValue;

use snarkos_models::gadgets::utilities::boolean::Boolean;

pub fn output_expected_boolean(program: EdwardsTestCompiler, boolean: bool) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Boolean(Boolean::Constant(boolean))]).to_string(),
        output.to_string()
    );
}

pub fn output_true(program: EdwardsTestCompiler) {
    output_expected_boolean(program, true)
}

pub fn output_false(program: EdwardsTestCompiler) {
    output_expected_boolean(program, false)
}

fn fail_evaluate(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::BooleanError(BooleanError::CannotEvaluate(_string)),
        ))) => {}
        error => panic!("Expected evaluate error, got {}", error),
    }
}

fn fail_enforce(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::BooleanError(BooleanError::CannotEnforce(_string)),
        ))) => {}
        error => panic!("Expected evaluate error, got {}", error),
    }
}

fn fail_boolean(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::BooleanError(BooleanError::InvalidBoolean(_string))) => {}
        error => panic!("Expected invalid boolean error, got {}", error),
    }
}

fn fail_synthesis(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::BooleanError(BooleanError::SynthesisError(_string))) => {}
        error => panic!("Expected synthesis error, got {}", error),
    }
}

#[test]
fn test_true() {
    let bytes = include_bytes!("true.leo");
    let program = parse_program(bytes).unwrap();

    output_true(program);
}

#[test]
fn test_false() {
    let bytes = include_bytes!("false.leo");
    let program = parse_program(bytes).unwrap();

    output_false(program);
}

#[test]
fn test_input_bool_field() {
    let bytes = include_bytes!("input_bool.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![Some(InputValue::Integer(1u128))]);

    fail_boolean(program);
}

#[test]
fn test_input_bool_none() {
    let bytes = include_bytes!("input_bool.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![None]);

    fail_synthesis(program);
}

// Boolean not !

#[test]
fn test_not_true() {
    let bytes = include_bytes!("not_true.leo");
    let program = parse_program(bytes).unwrap();

    output_false(program);
}

#[test]
fn test_not_false() {
    let bytes = include_bytes!("not_false.leo");
    let program = parse_program(bytes).unwrap();

    output_true(program);
}

#[test]
fn test_not_u32() {
    let bytes = include_bytes!("not_u32.leo");
    let program = parse_program(bytes).unwrap();

    fail_evaluate(program);
}

// Boolean or ||

#[test]
fn test_true_or_true() {
    let bytes = include_bytes!("true_||_true.leo");
    let program = parse_program(bytes).unwrap();

    output_true(program);
}

#[test]
fn test_true_or_false() {
    let bytes = include_bytes!("true_||_false.leo");
    let program = parse_program(bytes).unwrap();

    output_true(program);
}

#[test]
fn test_false_or_false() {
    let bytes = include_bytes!("false_||_false.leo");
    let program = parse_program(bytes).unwrap();

    output_false(program);
}

#[test]
fn test_true_or_u32() {
    let bytes = include_bytes!("true_||_u32.leo");
    let program = parse_program(bytes).unwrap();

    fail_enforce(program);
}

// Boolean and &&

#[test]
fn test_true_and_true() {
    let bytes = include_bytes!("true_&&_true.leo");
    let program = parse_program(bytes).unwrap();

    output_true(program);
}

#[test]
fn test_true_and_false() {
    let bytes = include_bytes!("true_&&_false.leo");
    let program = parse_program(bytes).unwrap();

    output_false(program);
}

#[test]
fn test_false_and_false() {
    let bytes = include_bytes!("false_&&_false.leo");
    let program = parse_program(bytes).unwrap();

    output_false(program);
}

#[test]
fn test_true_and_u32() {
    let bytes = include_bytes!("true_&&_u32.leo");
    let program = parse_program(bytes).unwrap();

    fail_enforce(program);
}

// All

#[test]
fn test_all() {
    let bytes = include_bytes!("all.leo");
    let program = parse_program(bytes).unwrap();

    output_false(program);
}
