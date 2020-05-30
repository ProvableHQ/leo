use crate::{compile_program, get_error, get_output, EdwardsConstrainedValue, EdwardsTestCompiler};

use leo_compiler::{types::Integer, ConstrainedValue, InputValue};

use leo_compiler::errors::{CompilerError, FunctionError, IntegerError};
use snarkos_models::gadgets::utilities::uint32::UInt32;

const DIRECTORY_NAME: &str = "tests/integer/u32/";

pub(crate) fn output_zero(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Integer(Integer::U32(
            UInt32::constant(0u32)
        ))])
        .to_string(),
        output.to_string()
    )
}

pub(crate) fn output_one(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Integer(Integer::U32(
            UInt32::constant(1u32)
        ))])
        .to_string(),
        output.to_string()
    )
}

fn output_two(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Integer(Integer::U32(
            UInt32::constant(2u32)
        ))])
        .to_string(),
        output.to_string()
    )
}

fn fail_integer(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::IntegerError(
            IntegerError::InvalidInteger(_string),
        )) => {}
        error => panic!("Expected invalid boolean error, got {}", error),
    }
}

fn fail_synthesis(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::IntegerError(
            IntegerError::SynthesisError(_string),
        )) => {}
        error => panic!("Expected synthesis error, got {}", error),
    }
}

#[test]
fn test_input_u32_bool() {
    let mut program = compile_program(DIRECTORY_NAME, "input_u32.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Boolean(true))]);
    fail_integer(program);
}

#[test]
fn test_input_u32_none() {
    let mut program = compile_program(DIRECTORY_NAME, "input_u32.leo").unwrap();
    program.set_inputs(vec![None]);
    fail_synthesis(program);
}

#[test]
fn test_zero() {
    let program = compile_program(DIRECTORY_NAME, "zero.leo").unwrap();
    output_zero(program);
}

#[test]
fn test_one() {
    let program = compile_program(DIRECTORY_NAME, "one.leo").unwrap();
    output_one(program);
}

#[test]
fn test_1_plus_1() {
    let program = compile_program(DIRECTORY_NAME, "1+1.leo").unwrap();
    output_two(program);
}

#[test]
fn test_1_minus_1() {
    let program = compile_program(DIRECTORY_NAME, "1-1.leo").unwrap();
    output_zero(program)
}

// #[test] // Todo: Catch subtraction overflow error in gadget
// fn test_1_minus_2() {
//     let program = compile_program(DIRECTORY_NAME, "1-2.leo").unwrap();
//     let error = get_error(program);
//     println!("{}", error);
// }
