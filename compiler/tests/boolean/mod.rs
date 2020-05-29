use crate::{compile_program, get_error, get_output};

use leo_compiler::errors::{BooleanError, ExpressionError};
use leo_compiler::group::edwards_bls12::EdwardsGroupType;
use leo_compiler::{
    compiler::Compiler,
    errors::{CompilerError, FunctionError, StatementError},
    ConstrainedValue, InputValue,
};
use snarkos_curves::edwards_bls12::{EdwardsParameters, Fq};
use snarkos_models::curves::ModelParameters;
use snarkos_models::gadgets::utilities::boolean::Boolean;

const DIRECTORY_NAME: &str = "tests/boolean/";

fn output_true(
    program: Compiler<<EdwardsParameters as ModelParameters>::BaseField, Fq, EdwardsGroupType>,
) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fq>::Return(vec![ConstrainedValue::Boolean(Boolean::Constant(true))]),
        output
    );
}

fn output_false(
    program: Compiler<<EdwardsParameters as ModelParameters>::BaseField, Fq, EdwardsGroupType>,
) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fq>::Return(vec![ConstrainedValue::Boolean(Boolean::Constant(false))]),
        output
    );
}

fn fail_evaluate(
    program: Compiler<<EdwardsParameters as ModelParameters>::BaseField, Fq, EdwardsGroupType>,
) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(
            StatementError::ExpressionError(ExpressionError::BooleanError(
                BooleanError::CannotEvaluate(_string),
            )),
        )) => {}
        error => panic!("Expected evaluate error, got {}", error),
    }
}

fn fail_enforce(
    program: Compiler<<EdwardsParameters as ModelParameters>::BaseField, Fq, EdwardsGroupType>,
) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(
            StatementError::ExpressionError(ExpressionError::BooleanError(
                BooleanError::CannotEnforce(_string),
            )),
        )) => {}
        error => panic!("Expected evaluate error, got {}", error),
    }
}

fn fail_boolean(
    program: Compiler<<EdwardsParameters as ModelParameters>::BaseField, Fq, EdwardsGroupType>,
) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::BooleanError(
            BooleanError::InvalidBoolean(_string),
        )) => {}
        error => panic!("Expected invalid boolean error, got {}", error),
    }
}

fn fail_synthesis(
    program: Compiler<<EdwardsParameters as ModelParameters>::BaseField, Fq, EdwardsGroupType>,
) {
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
    program.set_inputs(vec![Some(InputValue::Integer(1usize))]);
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
