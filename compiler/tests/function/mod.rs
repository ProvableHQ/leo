use crate::{compile_program, get_error, get_output, integer::u32::output_one};

use leo_compiler::{
    compiler::Compiler,
    errors::{CompilerError, ExpressionError, FunctionError, StatementError},
    ConstrainedValue,
};
use snarkos_curves::{bls12_377::Fr, edwards_bls12::EdwardsProjective};
use snarkos_models::gadgets::utilities::boolean::Boolean;

const DIRECTORY_NAME: &str = "tests/function/";

pub(crate) fn output_empty(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![]),
        output
    );
}

// (true, false)
pub(crate) fn output_multiple(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![
            ConstrainedValue::Boolean(Boolean::Constant(true)),
            ConstrainedValue::Boolean(Boolean::Constant(false))
        ]),
        output
    )
}

fn fail_undefined_identifier(program: Compiler<Fr, EdwardsProjective>) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(
            StatementError::ExpressionError(ExpressionError::UndefinedIdentifier(_)),
        )) => {}
        error => panic!("Expected function undefined, got {}", error),
    }
}

// Inline function call

#[test]
fn test_empty() {
    let program = compile_program(DIRECTORY_NAME, "empty.leo").unwrap();
    output_empty(program);
}

#[test]
fn test_return() {
    let program = compile_program(DIRECTORY_NAME, "return.leo").unwrap();
    output_one(program);
}

#[test]
fn test_undefined() {
    let program = compile_program(DIRECTORY_NAME, "undefined.leo").unwrap();
    fail_undefined_identifier(program);
}

// Function scope

#[test]
fn test_global_scope_fail() {
    let program = compile_program(DIRECTORY_NAME, "scope_fail.leo").unwrap();
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(
            StatementError::ExpressionError(ExpressionError::FunctionError(value)),
        )) => match *value {
            FunctionError::StatementError(StatementError::ExpressionError(
                ExpressionError::UndefinedIdentifier(_),
            )) => {}
            error => panic!("Expected function undefined, got {}", error),
        },
        error => panic!("Expected function undefined, got {}", error),
    }
}

#[test]
fn test_value_unchanged() {
    let program = compile_program(DIRECTORY_NAME, "value_unchanged.leo").unwrap();
    output_one(program);
}

// Multiple returns

#[test]
fn test_multiple_returns() {
    let program = compile_program(DIRECTORY_NAME, "multiple.leo").unwrap();
    output_multiple(program);
}
#[test]
fn test_multiple_returns_main() {
    let program = compile_program(DIRECTORY_NAME, "multiple_main.leo").unwrap();
    output_multiple(program);
}
