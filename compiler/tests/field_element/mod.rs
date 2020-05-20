use crate::{compile_program, get_error, get_output};

use leo_compiler::errors::FieldElementError;
use leo_compiler::{
    compiler::Compiler,
    errors::{CompilerError, FunctionError},
    ConstrainedValue, FieldElement, InputValue,
};
use snarkos_curves::{bls12_377::Fr, edwards_bls12::EdwardsProjective};
use snarkos_models::curves::Field;

const DIRECTORY_NAME: &str = "tests/field_element/";

fn output_zero(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::FieldElement(
            FieldElement::Constant(Fr::zero())
        )]),
        output
    );
}

fn output_one(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::FieldElement(
            FieldElement::Constant(Fr::one())
        )]),
        output
    );
}

fn fail_field(program: Compiler<Fr, EdwardsProjective>) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::FieldElementError(
            FieldElementError::InvalidField(_string),
        )) => {}
        error => panic!("Expected invalid field error, got {}", error),
    }
}

fn fail_synthesis(program: Compiler<Fr, EdwardsProjective>) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::FieldElementError(
            FieldElementError::SynthesisError(_string),
        )) => {}
        error => panic!("Expected synthesis error, got {}", error),
    }
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
fn test_input_field_bool() {
    let mut program = compile_program(DIRECTORY_NAME, "input_field.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Boolean(true))]);
    fail_field(program);
}

#[test]
fn test_input_field_none() {
    let mut program = compile_program(DIRECTORY_NAME, "input_field.leo").unwrap();
    program.set_inputs(vec![None]);
    fail_synthesis(program);
}
