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
use leo_inputs::types::{IntegerType, U64Type};
use leo_types::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{
    r1cs::TestConstraintSystem,
    utilities::{alloc::AllocGadget, uint::UInt64},
};

fn output_expected_allocated(program: EdwardsTestCompiler, expected: UInt64) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Integer(Integer::U64(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

test_uint!(TestU64, u64, IntegerType::U64Type(U64Type {}), UInt64);

#[test]
fn test_u64_min() {
    TestU64::test_min(std::u64::MIN);
}

#[test]
fn test_u64_max() {
    TestU64::test_max(std::u64::MAX);
}

#[test]
fn test_u64_input() {
    TestU64::test_input();
}

#[test]
fn test_u64_add() {
    TestU64::test_add();
}

#[test]
fn test_u64_sub() {
    TestU64::test_sub();
}

#[test]
fn test_u64_mul() {
    TestU64::test_mul();
}

#[test]
fn test_u64_div() {
    TestU64::test_div();
}

#[test]
#[ignore] // this test takes ~7 mins
fn test_u64_pow() {
    TestU64::test_pow();
}

#[test]
fn test_u64_eq() {
    TestU64::test_eq();
}

#[test]
fn test_u64_ge() {
    TestU64::test_ge();
}

#[test]
fn test_u64_gt() {
    TestU64::test_gt();
}

#[test]
fn test_u64_le() {
    TestU64::test_le();
}

#[test]
fn test_u64_lt() {
    TestU64::test_lt();
}

#[test]
fn test_u64_assert_eq() {
    TestU64::test_assert_eq();
}

#[test]
fn test_u64_ternary() {
    TestU64::test_ternary();
}
