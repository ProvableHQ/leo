use crate::{compile_program, get_error, get_output};

use leo_compiler::errors::IntegerError;
use leo_compiler::{
    compiler::Compiler,
    errors::{CompilerError, FunctionError},
    ConstrainedValue, InputValue, Integer,
};
use snarkos_curves::{bls12_377::Fr, edwards_bls12::EdwardsProjective};
use snarkos_models::gadgets::utilities::uint32::UInt32;

const DIRECTORY_NAME: &str = "tests/array/";

// [1, 1, 1]
fn output_ones(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Array(
            vec![ConstrainedValue::Integer(Integer::U32(UInt32::constant(1u32))); 3]
        )]),
        output
    );
}

// [[0, 0, 0],
//  [0, 0, 0]]
fn output_multi(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Array(
            vec![
                ConstrainedValue::Array(vec![
                    ConstrainedValue::Integer(Integer::U32(
                        UInt32::constant(0u32)
                    ));
                    3
                ]);
                2
            ]
        )]),
        output
    )
}

fn fail_array(program: Compiler<Fr, EdwardsProjective>) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::InvalidArray(_string)) => {}
        error => panic!("Expected invalid array error, got {}", error),
    }
}

fn fail_synthesis(program: Compiler<Fr, EdwardsProjective>) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::IntegerError(
            IntegerError::SynthesisError(_string),
        )) => {}
        error => panic!("Expected synthesis error, got {}", error),
    }
}

// Expressions

#[test]
fn test_inline() {
    let program = compile_program(DIRECTORY_NAME, "inline.leo").unwrap();
    output_ones(program);
}

#[test]
fn test_initializer() {
    let program = compile_program(DIRECTORY_NAME, "initializer.leo").unwrap();
    output_ones(program);
}

#[test]
fn test_spread() {
    let program = compile_program(DIRECTORY_NAME, "spread.leo").unwrap();
    output_ones(program);
}

#[test]
fn test_slice() {
    let program = compile_program(DIRECTORY_NAME, "slice.leo").unwrap();
    output_ones(program);
}

#[test]
fn test_multi() {
    let program = compile_program(DIRECTORY_NAME, "multi.leo").unwrap();
    output_multi(program);
}

// Inputs

#[test]
fn test_input_array() {
    let mut program = compile_program(DIRECTORY_NAME, "input_array.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Array(vec![
        InputValue::Integer(
            1usize
        );
        3
    ]))]);
    output_ones(program)
}

#[test]
fn test_input_array_fail() {
    let mut program = compile_program(DIRECTORY_NAME, "input_array.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Integer(1usize))]);
    fail_array(program);
}

#[test]
fn test_input_field_none() {
    let mut program = compile_program(DIRECTORY_NAME, "input_array.leo").unwrap();
    program.set_inputs(vec![None]);
    fail_synthesis(program)
}
