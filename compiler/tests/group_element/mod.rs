use crate::{compile_program, get_output};

use leo_compiler::{compiler::Compiler, ConstrainedValue};
use snarkos_curves::{bls12_377::Fr, edwards_bls12::EdwardsProjective};
use snarkos_models::curves::Group;

const DIRECTORY_NAME: &str = "tests/group_element/";

pub(crate) fn output_zero(program: Compiler<Fr, EdwardsProjective>) {
    let output = get_output(program);
    assert_eq!(
        ConstrainedValue::<Fr, EdwardsProjective>::Return(vec![ConstrainedValue::GroupElement(
            EdwardsProjective::zero()
        )]),
        output
    );
}

#[test]
fn test_zero() {
    let program = compile_program(DIRECTORY_NAME, "zero.leo").unwrap();
    output_zero(program);
}
