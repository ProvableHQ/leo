use crate::{
    get_error,
    get_output,
    integers::u32::output_one,
    parse_program,
    EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_compiler::{
    errors::{CompilerError, ExpressionError, FunctionError, StatementError},
    ConstrainedValue,
};

use snarkos_models::gadgets::utilities::boolean::Boolean;

pub(crate) fn output_empty(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(EdwardsConstrainedValue::Return(vec![]).to_string(), output.to_string());
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
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::Error(_),
        ))) => {}
        error => panic!("Expected function undefined, got {}", error),
    }
}

// Inline function call

#[test]
fn test_empty() {
    let bytes = include_bytes!("empty.leo");
    let program = parse_program(bytes).unwrap();

    output_empty(program);
}

#[test]
fn test_return() {
    let bytes = include_bytes!("return.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}

#[test]
fn test_undefined() {
    let bytes = include_bytes!("undefined.leo");
    let program = parse_program(bytes).unwrap();

    fail_undefined_identifier(program);
}

// Function scope

#[test]
fn test_global_scope_fail() {
    let bytes = include_bytes!("scope_fail.leo");
    let program = parse_program(bytes).unwrap();

    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::FunctionError(value),
        ))) => match *value {
            FunctionError::StatementError(StatementError::ExpressionError(ExpressionError::Error(_))) => {}
            error => panic!("Expected function undefined, got {}", error),
        },
        error => panic!("Expected function undefined, got {}", error),
    }
}

#[test]
fn test_value_unchanged() {
    let bytes = include_bytes!("value_unchanged.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}

// Multiple returns

#[test]
fn test_multiple_returns() {
    let bytes = include_bytes!("multiple.leo");
    let program = parse_program(bytes).unwrap();

    output_multiple(program);
}

#[test]
fn test_multiple_returns_main() {
    let bytes = include_bytes!("multiple_main.leo");
    let program = parse_program(bytes).unwrap();

    output_multiple(program);
}
