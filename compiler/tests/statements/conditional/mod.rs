use crate::{
    get_output,
    integers::u32::{output_number, output_one, output_zero},
    parse_program,
    EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_inputs::types::{IntegerType, U32Type};
use leo_typed::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::r1cs::TestConstraintSystem;

fn empty_output_satisfied(program: EdwardsTestCompiler) {
    let output = get_output(program);

    assert_eq!(EdwardsConstrainedValue::Return(vec![]).to_string(), output.to_string());
}

// Tests a statements.conditional enforceBit() program
//
// function main(bit: u8) {
//     if bit == 1u8 {
//       assert_eq!(bit, 1u8);
//     } else {
//       assert_eq!(bit, 0u8);
//     }
// }
#[test]
fn test_assert() {
    let bytes = include_bytes!("assert.leo");
    let mut program_1_pass = parse_program(bytes).unwrap();
    let mut program_0_pass = program_1_pass.clone();
    let mut program_2_fail = program_1_pass.clone();

    // Check that an input value of 1 satisfies the constraint system

    program_1_pass.set_inputs(vec![Some(InputValue::Integer(
        IntegerType::U32Type(U32Type {}),
        1.to_string(),
    ))]);
    empty_output_satisfied(program_1_pass);

    // Check that an input value of 0 satisfies the constraint system

    program_0_pass.set_inputs(vec![Some(InputValue::Integer(
        IntegerType::U32Type(U32Type {}),
        0.to_string(),
    ))]);
    empty_output_satisfied(program_0_pass);

    // Check that an input value of 2 does not satisfy the constraint system

    program_2_fail.set_inputs(vec![Some(InputValue::Integer(
        IntegerType::U32Type(U32Type {}),
        2.to_string(),
    ))]);
    let mut cs = TestConstraintSystem::<Fq>::new();
    let _output = program_2_fail.compile_constraints(&mut cs).unwrap();
    assert!(!cs.is_satisfied());
}

#[test]
fn test_mutate() {
    let bytes = include_bytes!("mutate.leo");
    let mut program_1_true = parse_program(bytes).unwrap();
    let mut program_0_pass = program_1_true.clone();

    // Check that an input value of 1 satisfies the constraint system

    program_1_true.set_inputs(vec![Some(InputValue::Integer(
        IntegerType::U32Type(U32Type {}),
        1.to_string(),
    ))]);
    output_one(program_1_true);

    // Check that an input value of 0 satisfies the constraint system

    program_0_pass.set_inputs(vec![Some(InputValue::Integer(
        IntegerType::U32Type(U32Type {}),
        0.to_string(),
    ))]);
    output_zero(program_0_pass);
}

#[test]
fn test_for_loop() {
    let bytes = include_bytes!("for_loop.leo");
    let mut program_true_6 = parse_program(bytes).unwrap();
    let mut program_false_0 = program_true_6.clone();

    // Check that an input value of true satisfies the constraint system

    program_true_6.set_inputs(vec![Some(InputValue::Boolean(true))]);
    output_number(program_true_6, 6u32);

    // Check that an input value of false satisfies the constraint system

    program_false_0.set_inputs(vec![Some(InputValue::Boolean(false))]);
    output_zero(program_false_0);
}

#[test]
fn test_chain() {
    let bytes = include_bytes!("chain.leo");
    let mut program_1_1 = parse_program(bytes).unwrap();
    let mut program_2_2 = program_1_1.clone();
    let mut program_2_3 = program_1_1.clone();

    // Check that an input of 1 outputs true
    program_1_1.set_inputs(vec![Some(InputValue::Integer(
        IntegerType::U32Type(U32Type {}),
        1.to_string(),
    ))]);
    output_number(program_1_1, 1u32);

    // Check that an input of 0 outputs true
    program_2_2.set_inputs(vec![Some(InputValue::Integer(
        IntegerType::U32Type(U32Type {}),
        2.to_string(),
    ))]);
    output_number(program_2_2, 2u32);

    // Check that an input of 0 outputs true
    program_2_3.set_inputs(vec![Some(InputValue::Integer(
        IntegerType::U32Type(U32Type {}),
        5.to_string(),
    ))]);
    output_number(program_2_3, 3u32);
}

#[test]
fn test_nested() {
    let bytes = include_bytes!("nested.leo");
    let mut program_true_true_3 = parse_program(bytes).unwrap();
    let mut program_true_false_1 = program_true_true_3.clone();
    let mut program_false_false_0 = program_true_true_3.clone();

    // Check that an input value of true true satisfies the constraint system

    program_true_true_3.set_inputs(vec![Some(InputValue::Boolean(true)); 2]);
    output_number(program_true_true_3, 3u32);

    // Check that an input value of true false satisfies the constraint system

    program_true_false_1.set_inputs(vec![Some(InputValue::Boolean(true)), Some(InputValue::Boolean(false))]);
    output_number(program_true_false_1, 1u32);

    // Check that an input value of false false satisfies the constraint system

    program_false_false_0.set_inputs(vec![Some(InputValue::Boolean(false)), Some(InputValue::Boolean(false))]);
    output_number(program_false_false_0, 0u32);
}

#[test]
fn test_multiple_returns() {
    let bytes = include_bytes!("multiple_returns.leo");
    let mut program_true_1 = parse_program(bytes).unwrap();
    let mut program_false_0 = program_true_1.clone();

    // Check that an input value of true returns 1 and satisfies the constraint system

    program_true_1.set_inputs(vec![Some(InputValue::Boolean(true))]);
    output_number(program_true_1, 1u32);

    // Check that an input value of false returns 0 and satisfies the constraint system

    program_false_0.set_inputs(vec![Some(InputValue::Boolean(false))]);
    output_number(program_false_0, 0u32);
}
