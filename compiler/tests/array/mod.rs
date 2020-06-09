use crate::{get_error, get_output, parse_program, EdwardsConstrainedValue, EdwardsTestCompiler};
use leo_compiler::{
    errors::{CompilerError, FunctionError},
    ConstrainedValue,
};
use leo_types::{InputValue, Integer, IntegerError};

use snarkos_models::gadgets::utilities::uint::UInt32;

// [1, 1, 1]
fn output_ones(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Array(vec![
            ConstrainedValue::Integer(
                Integer::U32(UInt32::constant(1u32))
            );
            3
        ])])
        .to_string(),
        output.to_string()
    );
}

// [[0, 0, 0],
//  [0, 0, 0]]
fn output_multi(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Array(vec![
            ConstrainedValue::Array(
                vec![ConstrainedValue::Integer(Integer::U32(UInt32::constant(0u32))); 3]
            );
            2
        ])])
        .to_string(),
        output.to_string()
    )
}

fn fail_array(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::InvalidArray(_string)) => {}
        error => panic!("Expected invalid array error, got {}", error),
    }
}

fn fail_synthesis(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::IntegerError(IntegerError::SynthesisError(_string))) => {}
        error => panic!("Expected synthesis error, got {}", error),
    }
}

// Expressions

#[test]
fn test_inline() {
    let bytes = include_bytes!("inline.leo");
    let program = parse_program(bytes).unwrap();

    output_ones(program);
}

#[test]
fn test_initializer() {
    let bytes = include_bytes!("initializer.leo");
    let program = parse_program(bytes).unwrap();

    output_ones(program);
}

#[test]
fn test_spread() {
    let bytes = include_bytes!("spread.leo");
    let program = parse_program(bytes).unwrap();

    output_ones(program);
}

#[test]
fn test_slice() {
    let bytes = include_bytes!("slice.leo");
    let program = parse_program(bytes).unwrap();

    output_ones(program);
}

#[test]
fn test_multi() {
    let bytes = include_bytes!("multi.leo");
    let program = parse_program(bytes).unwrap();

    output_multi(program);
}

// Inputs

#[test]
fn test_input_array() {
    let bytes = include_bytes!("input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![Some(InputValue::Array(vec![InputValue::Integer(1u128); 3]))]);

    output_ones(program)
}

#[test]
fn test_input_array_fail() {
    let bytes = include_bytes!("input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![Some(InputValue::Integer(1u128))]);

    fail_array(program);
}

#[test]
fn test_input_field_none() {
    let bytes = include_bytes!("input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![None]);

    fail_synthesis(program)
}
