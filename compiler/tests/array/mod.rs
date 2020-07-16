use crate::{
    get_error,
    get_output,
    integers::fail_integer,
    parse_program,
    EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_compiler::{
    errors::{CompilerError, FunctionError},
    ConstrainedValue,
    Integer,
};
use leo_inputs::types::{IntegerType, U32Type};
use leo_types::InputValue;

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
        CompilerError::FunctionError(FunctionError::Error(_string)) => {}
        error => panic!("Expected function error, found {}", error),
    }
}

pub(crate) fn input_value_u32_one() -> InputValue {
    InputValue::Integer(IntegerType::U32Type(U32Type {}), 1.to_string())
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

    program.set_inputs(vec![Some(InputValue::Array(vec![input_value_u32_one(); 3]))]);

    output_ones(program)
}

#[test]
fn test_input_array_fail() {
    let bytes = include_bytes!("input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![Some(input_value_u32_one())]);

    fail_array(program);
}

#[test]
fn test_input_field_none() {
    let bytes = include_bytes!("input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![None]);

    fail_integer(program)
}
