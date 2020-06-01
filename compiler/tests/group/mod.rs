use crate::{compile_program, get_error, get_output, EdwardsConstrainedValue, EdwardsTestCompiler};
use leo_compiler::group::edwards_bls12::EdwardsGroupType;
use leo_compiler::ConstrainedValue;

use snarkos_curves::edwards_bls12::EdwardsAffine;
use snarkos_models::curves::Group;

use crate::boolean::{output_false, output_true};
use leo_compiler::errors::{CompilerError, FunctionError, StatementError};
use std::str::FromStr;

const DIRECTORY_NAME: &str = "tests/group/";

const TEST_POINT_1: &str = "(7374112779530666882856915975292384652154477718021969292781165691637980424078, 3435195339177955418892975564890903138308061187980579490487898366607011481796)";
const TEST_POINT_2: &str = "(1005842117974384149622370061042978581211342111653966059496918451529532134799, 79389132189982034519597104273449021362784864778548730890166152019533697186)";

fn output_expected(program: EdwardsTestCompiler, expected: EdwardsAffine) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Group(EdwardsGroupType::Constant(
            expected
        ))])
        .to_string(),
        output.to_string()
    )
}

fn output_zero(program: EdwardsTestCompiler) {
    output_expected(program, EdwardsAffine::zero())
}

fn fail_enforce(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(
            StatementError::SynthesisError(_),
        )) => {}
        error => panic!("Expected evaluate error, got {}", error),
    }
}

#[test]
fn test_zero() {
    let program = compile_program(DIRECTORY_NAME, "zero.leo").unwrap();
    output_zero(program);
}

#[test]
fn test_point() {
    let point = EdwardsAffine::from_str(TEST_POINT_1).unwrap();
    let program = compile_program(DIRECTORY_NAME, "point.leo").unwrap();
    output_expected(program, point);
}

#[test]
fn test_add() {
    use std::ops::Add;

    let point_1 = EdwardsAffine::from_str(TEST_POINT_1).unwrap();
    let point_2 = EdwardsAffine::from_str(TEST_POINT_2).unwrap();

    let sum = point_1.add(&point_2);

    let program = compile_program(DIRECTORY_NAME, "add.leo").unwrap();
    output_expected(program, sum);
}

#[test]
fn test_sub() {
    use std::ops::Sub;

    let point_1 = EdwardsAffine::from_str(TEST_POINT_1).unwrap();
    let point_2 = EdwardsAffine::from_str(TEST_POINT_2).unwrap();

    let sum = point_1.sub(&point_2);

    let program = compile_program(DIRECTORY_NAME, "sub.leo").unwrap();
    output_expected(program, sum);
}

#[test]
fn test_eq_true() {
    let program = compile_program(DIRECTORY_NAME, "eq_true.leo").unwrap();
    output_true(program)
}

#[test]
fn test_eq_false() {
    let program = compile_program(DIRECTORY_NAME, "eq_false.leo").unwrap();
    output_false(program)
}

#[test]
fn test_assert_eq_true() {
    let program = compile_program(DIRECTORY_NAME, "assert_eq_true.leo").unwrap();
    let _res = get_output(program);
}

#[test]
fn test_assert_eq_false() {
    let program = compile_program(DIRECTORY_NAME, "assert_eq_false.leo").unwrap();
    fail_enforce(program);
}
