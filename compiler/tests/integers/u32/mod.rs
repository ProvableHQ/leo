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

#[test]
fn test_u32() {
    test_uint!(TestU32, u32, IntegerType::U32Type(U32Type {}), UInt32);

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
    let bytes = include_bytes!("zero.leo");
    let program = parse_program(bytes).unwrap();

    output_zero(program);
}

#[test]
fn test_one() {
    let bytes = include_bytes!("one.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}
