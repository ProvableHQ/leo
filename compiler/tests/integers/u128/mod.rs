use crate::{
    boolean::{output_expected_boolean, output_false, output_true},
    compile_program, get_output,
    integers::{fail_integer, fail_synthesis, IntegerTester},
    EdwardsConstrainedValue, EdwardsTestCompiler,
};
use leo_compiler::ConstrainedValue;
use leo_types::{Integer, InputValue};

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::r1cs::TestConstraintSystem;
use snarkos_models::gadgets::utilities::{alloc::AllocGadget, uint::UInt128};

const DIRECTORY_NAME: &str = "tests/integers/u128/";

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

#[test]
#[ignore] // temporarily ignore memory expensive tests for travis
fn test_u128() {
    test_uint!(TestU128, u128, UInt128, DIRECTORY_NAME);

    TestU128::test_min(std::u128::MIN);
    TestU128::test_max(std::u128::MAX);

    TestU128::test_input();

    TestU128::test_add();
    // TestU128::test_sub(); //Todo: Catch subtraction overflow error in gadget
    TestU128::test_mul();
    TestU128::test_div();
    // TestU128::test_pow(); // takes about 10 minutes

    TestU128::test_eq();
    TestU128::test_ge();
    TestU128::test_gt();
    TestU128::test_le();
    TestU128::test_gt();

    TestU128::test_assert_eq();
    TestU128::test_ternary();
}
