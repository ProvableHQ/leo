use crate::{
    boolean::{output_false, output_true},
    get_output,
    integers::u32::{output_one, output_zero},
    parse_program,
    EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_inputs::types::{IntegerType, U32Type};
use leo_types::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::r1cs::TestConstraintSystem;

fn empty_output_satisfied(program: EdwardsTestCompiler) {
    let output = get_output(program);

    assert_eq!(EdwardsConstrainedValue::Return(vec![]).to_string(), output.to_string());
}

// Tests a conditional enforceBit() program
//
// function main(bit: private u8) {
//     if bit == 1u8 {
//       assert_eq!(bit, 1u8);
//     } else {
//       assert_eq!(bit, 0u8);
//     }
// }
#[test]
fn conditional_basic() {
    let bytes = include_bytes!("conditional_basic.leo");
    let mut program_1_pass = parse_program(bytes).unwrap();
    let mut program_0_pass = program_1_pass.clone();
    let mut program_2_fail = program_1_pass.clone();

    // Check that an input value of 1 satisfies the constraint system

    program_1_pass.set_inputs(vec![Some(InputValue::Integer(IntegerType::U32Type(U32Type {}), 1))]);
    empty_output_satisfied(program_1_pass);

    // Check that an input value of 0 satisfies the constraint system

    program_0_pass.set_inputs(vec![Some(InputValue::Integer(IntegerType::U32Type(U32Type {}), 0))]);
    empty_output_satisfied(program_0_pass);

    // Check that an input value of 2 does not satisfy the constraint system

    program_2_fail.set_inputs(vec![Some(InputValue::Integer(IntegerType::U32Type(U32Type {}), 2))]);
    let mut cs = TestConstraintSystem::<Fq>::new();
    let _output = program_2_fail.compile_constraints(&mut cs).unwrap();
    assert!(!cs.is_satisfied());
}

#[test]
fn conditional_mutate() {
    let bytes = include_bytes!("conditional_mutate.leo");
    let mut program_1_true = parse_program(bytes).unwrap();
    let mut program_0_pass = program_1_true.clone();

    // Check that an input value of 1 satisfies the constraint system

    program_1_true.set_inputs(vec![Some(InputValue::Integer(IntegerType::U32Type(U32Type {}), 1))]);
    output_one(program_1_true);

    // Check that an input value of 0 satisfies the constraint system

    program_0_pass.set_inputs(vec![Some(InputValue::Integer(IntegerType::U32Type(U32Type {}), 0))]);
    output_zero(program_0_pass);
}
