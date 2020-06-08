use crate::{
    compile_program, get_error, get_output, integers::u32::output_one, EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_compiler::{
    errors::{CompilerError, ExpressionError, FunctionError, StatementError},
    ConstrainedValue,
};

use snarkos_models::gadgets::utilities::boolean::Boolean;

const DIRECTORY_NAME: &str = "tests/function/";

pub(crate) fn output_empty(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![]).to_string(),
        output.to_string()
    );
}

// (true, false)
pub(crate) fn output_multiple(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![
            ConstrainedValue::Boolean(Boolean::Constant(true)),
            ConstrainedValue::Boolean(Boolean::Constant(false))
        ])
        .to_string(),
        output.to_string()
    )
}

fn fail_undefined_identifier(program: EdwardsTestCompiler) {
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
