use crate::{compile_program, get_error, get_output, EdwardsConstrainedValue, EdwardsTestCompiler};
use leo_compiler::{
    errors::{BooleanError, CompilerError, ExpressionError, FunctionError, StatementError},
    ConstrainedValue, InputValue,
};

use snarkos_models::gadgets::utilities::boolean::Boolean;

const DIRECTORY_NAME: &str = "tests/boolean/";

pub fn output_expected_boolean(program: EdwardsTestCompiler, boolean: bool) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Boolean(Boolean::Constant(
            boolean
        ))])
        .to_string(),
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
        CompilerError::FunctionError(FunctionError::StatementError(
            StatementError::ExpressionError(ExpressionError::BooleanError(
                BooleanError::CannotEvaluate(_string),
            )),
        )) => {}
        error => panic!("Expected evaluate error, got {}", error),
    }
}

fn fail_enforce(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(
            StatementError::ExpressionError(ExpressionError::BooleanError(
                BooleanError::CannotEnforce(_string),
            )),
        )) => {}
        error => panic!("Expected evaluate error, got {}", error),
    }
}

fn fail_boolean(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::BooleanError(
            BooleanError::InvalidBoolean(_string),
        )) => {}
        error => panic!("Expected invalid boolean error, got {}", error),
    }
}

fn fail_synthesis(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::BooleanError(
            BooleanError::SynthesisError(_string),
        )) => {}
        error => panic!("Expected synthesis error, got {}", error),
    }
}

#[test]
fn test_true() {
    let program = compile_program(DIRECTORY_NAME, "true.leo").unwrap();
    output_true(program);
}

#[test]
fn test_false() {
    let program = compile_program(DIRECTORY_NAME, "false.leo").unwrap();
    output_false(program);
}

#[test]
fn test_input_bool_field() {
    let mut program = compile_program(DIRECTORY_NAME, "input_bool.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Integer(1u128))]);
    fail_boolean(program);
}

#[test]
fn test_input_bool_none() {
    let mut program = compile_program(DIRECTORY_NAME, "input_bool.leo").unwrap();
    program.set_inputs(vec![None]);
    fail_synthesis(program);
}

// Boolean not !

#[test]
fn test_not_true() {
    let program = compile_program(DIRECTORY_NAME, "not_true.leo").unwrap();
    output_false(program);
}

#[test]
fn test_not_false() {
    let program = compile_program(DIRECTORY_NAME, "not_false.leo").unwrap();
    output_true(program);
}

#[test]
fn test_not_u32() {
    let program = compile_program(DIRECTORY_NAME, "not_u32.leo").unwrap();
    fail_evaluate(program);
}

// Boolean or ||

#[test]
fn test_true_or_true() {
    let program = compile_program(DIRECTORY_NAME, "true_||_true.leo").unwrap();
    output_true(program);
}

#[test]
fn test_true_or_false() {
    let program = compile_program(DIRECTORY_NAME, "true_||_false.leo").unwrap();
    output_true(program);
}

#[test]
fn test_false_or_false() {
    let program = compile_program(DIRECTORY_NAME, "false_||_false.leo").unwrap();
    output_false(program);
}

#[test]
fn test_true_or_u32() {
    let program = compile_program(DIRECTORY_NAME, "true_||_u32.leo").unwrap();
    fail_enforce(program);
}

// Boolean and &&

#[test]
fn test_true_and_true() {
    let program = compile_program(DIRECTORY_NAME, "true_&&_true.leo").unwrap();
    output_true(program);
}

#[test]
fn test_true_and_false() {
    let program = compile_program(DIRECTORY_NAME, "true_&&_false.leo").unwrap();
    output_false(program);
}

#[test]
fn test_false_and_false() {
    let program = compile_program(DIRECTORY_NAME, "false_&&_false.leo").unwrap();
    output_false(program);
}

#[test]
fn test_true_and_u32() {
    let program = compile_program(DIRECTORY_NAME, "true_&&_u32.leo").unwrap();
    fail_enforce(program);
}

// All

#[test]
fn test_all() {
    let program = compile_program(DIRECTORY_NAME, "all.leo").unwrap();
    output_false(program);
}
