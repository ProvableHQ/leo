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
use snarkos_models::gadgets::utilities::{alloc::AllocGadget, uint::UInt16};

const DIRECTORY_NAME: &str = "tests/integers/u16/";

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

#[test]
fn test_u16() {
    test_uint!(Testu16, u16, UInt16, DIRECTORY_NAME);

    Testu16::test_min(std::u16::MIN);
    Testu16::test_max(std::u16::MAX);

    Testu16::test_input();

    Testu16::test_add();
    // Testu16::test_sub(); //Todo: Catch subtraction overflow error in gadget
    Testu16::test_mul();
    Testu16::test_div();
    Testu16::test_pow();

    Testu16::test_eq();
    Testu16::test_ge();
    Testu16::test_gt();
    Testu16::test_le();
    Testu16::test_gt();

    Testu16::test_assert_eq();
    Testu16::test_ternary();
}
