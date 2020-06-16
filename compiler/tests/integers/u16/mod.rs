use crate::{
    boolean::{output_expected_boolean, output_false, output_true},
    get_error,
    get_output,
    integers::{fail_integer, fail_synthesis, IntegerTester},
    parse_program,
    EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_compiler::ConstrainedValue;
use leo_inputs::types::{IntegerType, U16Type};
use leo_types::{InputValue, Integer};

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

#[test]
fn test_u16() {
    test_uint!(Testu16, u16, IntegerType::U16Type(U16Type {}), UInt16);

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
