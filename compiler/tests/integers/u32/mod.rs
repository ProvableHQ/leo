use crate::{
    compile_program,
    // get_error,
    get_output,
};

use leo_compiler::{compiler::Compiler, types::Integer, ConstrainedValue};

use snarkos_curves::{bls12_377::Fr, edwards_bls12::EdwardsProjective};
use snarkos_models::gadgets::utilities::uint32::UInt32;

const DIRECTORY_NAME: &str = "tests/integers/u32/";

fn output_zero(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Integer(
            Integer::U32(UInt32::constant(0u32))
        )]),
        output
    )
}

fn output_one(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Integer(
            Integer::U32(UInt32::constant(1u32))
        )]),
        output
    )
}

fn output_two(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::Integer(
            Integer::U32(UInt32::constant(2u32))
        )]),
        output
    )
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
