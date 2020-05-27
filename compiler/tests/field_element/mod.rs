use crate::{compile_program, get_error, get_output, get_output_allocated};
use leo_compiler::errors::FieldElementError;
use leo_compiler::{
    compiler::Compiler,
    errors::{CompilerError, FunctionError},
    ConstrainedValue, FieldElement, InputValue,
};

use snarkos_curves::edwards_bls12::{EdwardsParameters, Fq};
use snarkos_gadgets::curves::edwards_bls12::FqGadget;
use snarkos_models::curves::Field;
use snarkos_models::gadgets::curves::FieldGadget;
use snarkos_models::gadgets::r1cs::{ConstraintSystem, TestConstraintSystem};

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

fn output_zero_allocated(program: Compiler<EdwardsParameters, Fq, FqGadget>) {
    let cs = &mut TestConstraintSystem::<Fq>::new();
    let output = get_output_allocated(cs, program);
    assert_eq!(
        ConstrainedValue::<EdwardsParameters, Fq, FqGadget>::Return(vec![
            ConstrainedValue::FieldElement(FieldElement::Allocated(FqGadget::zero(cs).unwrap()))
        ]),
        output
    )
}

fn output_one_allocated(program: Compiler<EdwardsParameters, Fq, FqGadget>) {
    let cs = &mut TestConstraintSystem::<Fq>::new();
    let output = get_output_allocated(cs, program);
    assert_eq!(
        ConstrainedValue::<EdwardsParameters, Fq, FqGadget>::Return(vec![
            ConstrainedValue::FieldElement(FieldElement::Allocated(FqGadget::one(cs).unwrap()))
        ]),
        output
    )
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
fn test_input_field() {
    let mut program = compile_program(DIRECTORY_NAME, "input_field.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Field(Fq::one()))]);

    let mut cs = TestConstraintSystem::<Fq>::new();
    let one_allocated = FqGadget::one(cs.ns(|| "one")).unwrap();

    let output = program.compile_constraints(&mut cs).unwrap();
    assert!(cs.is_satisfied());

    assert_eq!(
        ConstrainedValue::<EdwardsParameters, Fq, FqGadget>::Return(vec![
            ConstrainedValue::FieldElement(FieldElement::Allocated(one_allocated))
        ]),
        output
    );
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

#[test]
fn test_ternary_first() {
    let mut program = compile_program(DIRECTORY_NAME, "ternary.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Boolean(true))]);

    output_one_allocated(program)
}

#[test]
fn test_ternary_second() {
    let mut program = compile_program(DIRECTORY_NAME, "ternary.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Boolean(false))]);

    output_zero_allocated(program)
}

#[test]
fn test_assertion_pass() {
    let mut program = compile_program(DIRECTORY_NAME, "assertion.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Field(Fq::one()))]);

    let cs = &mut TestConstraintSystem::<Fq>::new();
    let _output = program.compile_constraints(cs).unwrap();

    assert!(cs.is_satisfied());
}

#[test]
fn test_assertion_fail() {
    let mut program = compile_program(DIRECTORY_NAME, "assertion.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Field(Fq::zero()))]);

    let cs = &mut TestConstraintSystem::<Fq>::new();
    let _output = program.compile_constraints(cs).unwrap();

    assert!(!cs.is_satisfied());
}
