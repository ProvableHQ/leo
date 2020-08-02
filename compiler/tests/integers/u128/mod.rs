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
use leo_inputs::types::{IntegerType, U128Type};
use leo_types::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{
    r1cs::TestConstraintSystem,
    utilities::{alloc::AllocGadget, uint::UInt128},
};

fn output_expected_allocated(program: EdwardsTestCompiler, expected: UInt128) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Integer(Integer::U128(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}
test_uint!(TestU128, u128, IntegerType::U128Type(U128Type {}), UInt128);

#[test]
fn test_u128_min() {
    TestU128::test_min(std::u128::MIN);
}

#[test]
fn test_u128_max() {
    TestU128::test_max(std::u128::MAX);
}

#[test]
fn test_u128_input() {
    TestU128::test_input();
}

#[test]
fn test_u128_add() {
    TestU128::test_add();
}

#[test]
fn test_u128_sub() {
    TestU128::test_sub();
}

#[test] // this test take ~1 min
fn test_u128_mul() {
    TestU128::test_mul();
}

#[test] // this test takes ~30 sec
fn test_u128_div() {
    TestU128::test_div();
}

#[test]
#[ignore] // this test takes ~10 mins
fn test_u128_pow() {
    TestU128::test_pow();
}

#[test]
fn test_u128_eq() {
    TestU128::test_eq();
}

#[test]
fn test_u128_ge() {
    TestU128::test_ge();
}

#[test]
fn test_u128_gt() {
    TestU128::test_gt();
}

#[test]
fn test_u128_le() {
    TestU128::test_le();
}

#[test]
fn test_u128_lt() {
    TestU128::test_lt();
}

#[test]
fn test_u128_assert_eq() {
    TestU128::test_assert_eq();
}

#[test]
fn test_u128_ternary() {
    TestU128::test_ternary();
}
