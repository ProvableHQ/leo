use crate::compile_program;

use leo_compiler::{types::Integer, ConstrainedValue, InputValue};

use snarkos_curves::{bls12_377::Fr, edwards_bls12::EdwardsProjective};
use snarkos_models::gadgets::r1cs::TestConstraintSystem;
use snarkos_models::gadgets::utilities::uint32::UInt32;

const DIRECTORY_NAME: &str = "tests/mutability/";

#[test]
fn test_let() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "let.leo").unwrap();
    let output = program.compile_constraints(&mut cs).is_err();
    assert!(output);
}

#[test]
fn test_let_mut() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "let_mut.leo").unwrap();
    let output = program.compile_constraints(&mut cs).unwrap();
    println!("{}", output);

    assert!(cs.is_satisfied());
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Integer(
            Integer::U32(UInt32::constant(0))
        )]),
        output
    );
}

#[test]
fn test_function_input() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let mut program = compile_program(DIRECTORY_NAME, "function_input.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Integer(1))]);
    let output = program.compile_constraints(&mut cs).is_err();
    assert!(output);
}

#[test]
fn test_function_input_mut() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let mut program = compile_program(DIRECTORY_NAME, "function_input_mut.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Integer(1))]);
    let output = program.compile_constraints(&mut cs).unwrap();
    println!("{}", output);

    assert!(cs.is_satisfied());
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Integer(
            Integer::U32(UInt32::constant(0))
        )]),
        output
    );
}
