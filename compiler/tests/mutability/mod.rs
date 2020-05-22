use crate::compile_program;

use leo_compiler::{
    compiler::Compiler,
    errors::{CompilerError, FunctionError, StatementError},
    types::{InputValue, Integer},
    ConstrainedValue,
};
use leo_gadgets::integers::uint32::UInt32;

use snarkos_curves::edwards_bls12::{EdwardsParameters, Fq};
use snarkos_gadgets::curves::edwards_bls12::FqGadget;
use snarkos_models::gadgets::r1cs::TestConstraintSystem;

const DIRECTORY_NAME: &str = "tests/mutability/";

fn mut_success(program: Compiler<EdwardsParameters, Fq, FqGadget>) {
    let mut cs = TestConstraintSystem::<Fq>::new();
    let output = program.compile_constraints(&mut cs).unwrap();

    assert!(cs.is_satisfied());
    assert_eq!(
        ConstrainedValue::<EdwardsParameters, Fq, FqGadget>::Return(vec![
            ConstrainedValue::Integer(Integer::U32(UInt32::constant(0)))
        ]),
        output
    );
}

fn mut_fail(program: Compiler<EdwardsParameters, Fq, FqGadget>) {
    let mut cs = TestConstraintSystem::<Fq>::new();
    let err = program.compile_constraints(&mut cs).unwrap_err();

    // It would be ideal if assert_eq!(Error1, Error2) were possible but unfortunately it is not due to
    // https://github.com/rust-lang/rust/issues/34158#issuecomment-224910299
    match err {
        CompilerError::FunctionError(FunctionError::StatementError(
            StatementError::ImmutableAssign(_string),
        )) => {}
        err => panic!("Expected immutable assign error, got {}", err),
    }
}

#[test]
fn test_let() {
    let program = compile_program(DIRECTORY_NAME, "let.leo").unwrap();
    mut_fail(program);
}

#[test]
fn test_let_mut() {
    let program = compile_program(DIRECTORY_NAME, "let_mut.leo").unwrap();
    mut_success(program);
}

#[test]
fn test_array() {
    let program = compile_program(DIRECTORY_NAME, "array.leo").unwrap();
    mut_fail(program);
}

#[test]
fn test_array_mut() {
    let program = compile_program(DIRECTORY_NAME, "array_mut.leo").unwrap();
    mut_success(program);
}

#[test]
fn test_circuit() {
    let program = compile_program(DIRECTORY_NAME, "circuit.leo").unwrap();
    mut_fail(program);
}

#[test]
fn test_circuit_mut() {
    let program = compile_program(DIRECTORY_NAME, "circuit_mut.leo").unwrap();
    mut_success(program);
}

#[test]
fn test_function_input() {
    let mut program = compile_program(DIRECTORY_NAME, "function_input.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Integer(1))]);
    mut_fail(program);
}

#[test]
fn test_function_input_mut() {
    let mut program = compile_program(DIRECTORY_NAME, "function_input_mut.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Integer(1))]);
    mut_success(program);
}
