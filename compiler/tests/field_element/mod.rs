use crate::{compile_program, get_error, get_output};
use leo_compiler::errors::FieldElementError;
use leo_compiler::{
    compiler::Compiler,
    errors::{CompilerError, FunctionError},
    ConstrainedValue, FieldElement, InputValue,
};

use snarkos_curves::edwards_bls12::{EdwardsParameters, Fq};
use snarkos_gadgets::curves::edwards_bls12::FqGadget;
use snarkos_models::curves::Field;

const DIRECTORY_NAME: &str = "tests/field_element/";

fn output_zero(program: Compiler<EdwardsParameters, Fq, FqGadget>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<EdwardsParameters, Fq, FqGadget>::Return(vec![
            ConstrainedValue::FieldElement(FieldElement::Constant(Fq::zero()))
        ]),
        output
    );
}

fn output_one(program: Compiler<EdwardsParameters, Fq, FqGadget>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<EdwardsParameters, Fq, FqGadget>::Return(vec![
            ConstrainedValue::FieldElement(FieldElement::Constant(Fq::one()))
        ]),
        output
    );
}

fn fail_field(program: Compiler<EdwardsParameters, Fq, FqGadget>) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::FieldElementError(
            FieldElementError::InvalidField(_string),
        )) => {}
        error => panic!("Expected invalid field error, got {}", error),
    }
}

fn fail_synthesis(program: Compiler<EdwardsParameters, Fq, FqGadget>) {
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
