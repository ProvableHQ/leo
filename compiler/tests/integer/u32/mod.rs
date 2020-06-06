use crate::{
    boolean::{output_expected_boolean, output_false, output_true},
    compile_program,
    get_output,
    integer::{fail_integer, fail_synthesis, IntegerTester},
    EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_compiler::{types::Integer, ConstrainedValue, InputValue};

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{
    r1cs::TestConstraintSystem,
    utilities::{alloc::AllocGadget, uint::UInt32},
};

const DIRECTORY_NAME: &str = "tests/integer/u32/";

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

pub(crate) fn output_zero(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Integer(Integer::U32(UInt32::constant(0u32)))])
            .to_string(),
        output.to_string()
    )
}

pub(crate) fn output_one(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Integer(Integer::U32(UInt32::constant(1u32)))])
            .to_string(),
        output.to_string()
    )
}

#[test]
fn test_u32() {
    test_uint!(TestU32, u32, UInt32, DIRECTORY_NAME);

    TestU32::test_min(std::u32::MIN);
    TestU32::test_max(std::u32::MAX);

    TestU32::test_input();

    TestU32::test_add();
    // TestU32::test_sub(); //Todo: Catch subtraction overflow error in gadget
    TestU32::test_mul();
    TestU32::test_div();
    TestU32::test_pow(); // takes about 2 mins

    TestU32::test_eq();
    TestU32::test_ge();
    TestU32::test_gt();
    TestU32::test_le();
    TestU32::test_gt();

    TestU32::test_assert_eq();
    TestU32::test_ternary();
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
