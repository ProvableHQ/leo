use crate::{compile_program, get_error, get_output};

use leo_compiler::errors::{BooleanError, ExpressionError};
use leo_compiler::{
    compiler::Compiler,
    errors::{CompilerError, FunctionError, StatementError},
    ConstrainedValue,
};
use snarkos_curves::{bls12_377::Fr, edwards_bls12::EdwardsProjective};
use snarkos_models::gadgets::utilities::boolean::Boolean;

const DIRECTORY_NAME: &str = "tests/boolean/";

fn output_true(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Boolean(
            Boolean::Constant(true)
        )]),
        output
    );
}

fn output_false(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Boolean(
            Boolean::Constant(false)
        )]),
        output
    );
}

fn fail_evaluate(program: Compiler<Fr, EdwardsProjective>) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(
            StatementError::ExpressionError(ExpressionError::BooleanError(
                BooleanError::CannotEvaluate(_string),
            )),
        )) => {}
        error => panic!("Expected evaluate error, got {}", error),
    }
}

fn fail_enforce(program: Compiler<Fr, EdwardsProjective>) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(
            StatementError::ExpressionError(ExpressionError::BooleanError(
                BooleanError::CannotEnforce(_string),
            )),
        )) => {}
        error => panic!("Expected evaluate error, got {}", error),
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
