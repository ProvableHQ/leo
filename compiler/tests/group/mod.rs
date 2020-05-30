use crate::{compile_program, get_output, EdwardsConstrainedValue, EdwardsTestCompiler};
use leo_compiler::group::edwards_bls12::EdwardsGroupType;
use leo_compiler::ConstrainedValue;
use snarkos_curves::edwards_bls12::EdwardsAffine;
use snarkos_models::curves::Group;

const DIRECTORY_NAME: &str = "tests/group/";

fn output_zero(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Group(EdwardsGroupType::Constant(
            EdwardsAffine::zero()
        ))])
        .to_string(),
        output.to_string()
    );
}

#[test]
fn test_zero() {
    let program = compile_program(DIRECTORY_NAME, "zero.leo").unwrap();
    output_zero(program);
}

#[test]
fn test_add() {
    let program = compile_program(DIRECTORY_NAME, "add.leo").unwrap();
    output_zero(program);
}
