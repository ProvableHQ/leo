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
use leo_inputs::types::{IntegerType, U8Type};
use leo_types::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{
    r1cs::TestConstraintSystem,
    utilities::{alloc::AllocGadget, uint::UInt8},
};

fn output_expected_allocated(program: EdwardsTestCompiler, expected: UInt8) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Integer(Integer::U8(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

#[test]
fn test_u8() {
    test_uint!(Testu8, u8, IntegerType::U8Type(U8Type {}), UInt8);

    Testu8::test_min(std::u8::MIN);
    Testu8::test_max(std::u8::MAX);

    Testu8::test_input();

    Testu8::test_add();
    // Testu8::test_sub(); //Todo: Catch subtraction overflow error in gadget
    Testu8::test_mul();
    Testu8::test_div();
    Testu8::test_pow();

    Testu8::test_eq();
    Testu8::test_ge();
    Testu8::test_gt();
    Testu8::test_le();
    Testu8::test_gt();

    Testu8::test_assert_eq();
    Testu8::test_ternary();
}
