use crate::compile_program;

use leo_compiler::{types::Integer, ConstrainedValue};

use snarkos_curves::{bls12_377::Fr, edwards_bls12::EdwardsProjective};
use snarkos_models::gadgets::r1cs::TestConstraintSystem;
use snarkos_models::gadgets::utilities::uint32::UInt32;

const DIRECTORY_NAME: &str = "tests/u32/";

#[test]
fn test_zero() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "zero.leo").unwrap();
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
fn test_one() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "one.leo").unwrap();
    let output = program.compile_constraints(&mut cs).unwrap();
    println!("{}", output);

    assert!(cs.is_satisfied());
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Integer(
            Integer::U32(UInt32::constant(1))
        )]),
        output
    );
}

#[test]
fn test_1_plus_1() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "1+1.leo").unwrap();
    let output = program.compile_constraints(&mut cs).unwrap();
    println!("{}", output);

    assert!(cs.is_satisfied());
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Integer(
            Integer::U32(UInt32::constant(2))
        )]),
        output
    );
}

#[test]
fn test_1_minus_1() {
    let mut cs = TestConstraintSystem::<Fr>::new();
    let program = compile_program(DIRECTORY_NAME, "1-1.leo").unwrap();
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

// #[test]
// fn test_1_minus_2_should_fail() {
//     // TODO (howardwu): Catch panic from subtraction overflow
//
//     let mut cs = TestConstraintSystem::<Fr>::new();
//     let program = compile_program(DIRECTORY_NAME, "1-2.leo").unwrap();
//     let output = program.compile_constraints(&mut cs);
//     assert!(output.is_err());
// }
