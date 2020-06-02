use crate::{compile_program, get_error, get_output, EdwardsConstrainedValue, EdwardsTestCompiler};
use leo_compiler::{
    errors::{CompilerError, FieldError, FunctionError},
    ConstrainedValue, FieldElement, InputValue,
};

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::curves::Field;

const DIRECTORY_NAME: &str = "tests/field_element/";

fn output_zero(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Field(FieldElement::Constant(
            Fq::zero()
        ))])
        .to_string(),
        output.to_string()
    );
}

fn output_one(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Field(FieldElement::Constant(
            Fq::one()
        ))])
        .to_string(),
        output.to_string()
    );
}

fn fail_field(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::FieldElementError(FieldError::Invalid(
            _string,
        ))) => {}
        error => panic!("Expected invalid field error, got {}", error),
    }
}

fn fail_synthesis(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::FieldElementError(
            FieldError::SynthesisError(_string),
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
