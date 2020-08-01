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
use leo_inputs::types::{IntegerType, U16Type};
use leo_typed::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{
    r1cs::TestConstraintSystem,
    utilities::{alloc::AllocGadget, uint::UInt16},
};

fn output_expected_allocated(program: EdwardsTestCompiler, expected: UInt16) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Integer(Integer::U16(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

test_uint!(TestU16, u16, IntegerType::U16Type(U16Type {}), UInt16);

#[test]
fn test_u16_min() {
    TestU16::test_min(std::u16::MIN);
}

#[test]
fn test_u16_max() {
    TestU16::test_max(std::u16::MAX);
}

#[test]
fn test_u16_input() {
    TestU16::test_input();
}

#[test]
fn test_u16_add() {
    TestU16::test_add();
}

#[test]
fn test_u16_sub() {
    TestU16::test_sub();
}

#[test]
fn test_u16_mul() {
    TestU16::test_mul();
}

#[test]
fn test_u16_div() {
    TestU16::test_div();
}

#[test]
fn test_u16_pow() {
    TestU16::test_pow();
}

#[test]
fn test_u16_eq() {
    TestU16::test_eq();
}

#[test]
fn test_u16_ge() {
    TestU16::test_ge();
}

#[test]
fn test_u16_gt() {
    TestU16::test_gt();
}

#[test]
fn test_u16_le() {
    TestU16::test_le();
}

#[test]
fn test_u16_lt() {
    TestU16::test_lt();
}

#[test]
fn test_u16_assert_eq() {
    TestU16::test_assert_eq();
}

#[test]
fn test_u16_ternary() {
    TestU16::test_ternary();
}
