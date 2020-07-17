use crate::{
    boolean::{output_expected_boolean, output_false, output_true},
    get_error,
    get_output,
    integers::{fail_integer, IntegerTester},
    parse_program,
    EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_compiler::{ConstrainedValue, Integer};
use leo_inputs::types::{IntegerType, U32Type};
use leo_types::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{
    r1cs::TestConstraintSystem,
    utilities::{alloc::AllocGadget, uint::UInt32},
};

fn output_expected_allocated(program: EdwardsTestCompiler, expected: UInt32) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Integer(Integer::U32(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

pub(crate) fn output_number(program: EdwardsTestCompiler, number: u32) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Integer(Integer::U32(UInt32::constant(number)))])
            .to_string(),
        output.to_string()
    )
}

pub(crate) fn output_zero(program: EdwardsTestCompiler) {
    output_number(program, 0u32);
}

pub(crate) fn output_one(program: EdwardsTestCompiler) {
    output_number(program, 1u32);
}

test_uint!(TestU32, u32, IntegerType::U32Type(U32Type {}), UInt32);

#[test]
fn test_u32_min() {
    TestU32::test_min(std::u32::MIN);
}

#[test]
fn test_u32_max() {
    TestU32::test_max(std::u32::MAX);
}

#[test]
fn test_u32_input() {
    TestU32::test_input();
}

#[test]
fn test_u32_add() {
    TestU32::test_add();
}

#[test]
fn test_u32_sub() {
    TestU32::test_sub();
}

#[test]
fn test_u32_mul() {
    TestU32::test_mul();
}

#[test]
fn test_u32_div() {
    TestU32::test_div();
}

#[test]
fn test_u32_pow() {
    TestU32::test_pow();
}

#[test]
fn test_u32_eq() {
    TestU32::test_eq();
}

#[test]
fn test_u32_ge() {
    TestU32::test_ge();
}

#[test]
fn test_u32_gt() {
    TestU32::test_gt();
}

#[test]
fn test_u32_le() {
    TestU32::test_le();
}

#[test]
fn test_u32_lt() {
    TestU32::test_lt();
}

#[test]
fn test_u32_assert_eq() {
    TestU32::test_assert_eq();
}

#[test]
fn test_u32_ternary() {
    TestU32::test_ternary();
}
