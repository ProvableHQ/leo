use crate::{
    boolean::{output_false, output_true},
    compile_program, get_error, get_output, EdwardsConstrainedValue, EdwardsTestCompiler,
};
use leo_compiler::{
    errors::{CompilerError, FunctionError, StatementError},
    group::edwards_bls12::EdwardsGroupType,
    ConstrainedValue, InputValue,
};

use snarkos_curves::edwards_bls12::{EdwardsAffine, Fq};
use snarkos_gadgets::curves::edwards_bls12::EdwardsBlsGadget;
use snarkos_models::{
    curves::Group,
    gadgets::{r1cs::TestConstraintSystem, utilities::alloc::AllocGadget},
};
use std::str::FromStr;

const DIRECTORY_NAME: &str = "tests/group/";

const TEST_POINT_1: &str = "(7374112779530666882856915975292384652154477718021969292781165691637980424078, 3435195339177955418892975564890903138308061187980579490487898366607011481796)";
const TEST_POINT_2: &str = "(1005842117974384149622370061042978581211342111653966059496918451529532134799, 79389132189982034519597104273449021362784864778548730890166152019533697186)";

fn output_expected_constant(program: EdwardsTestCompiler, expected: EdwardsAffine) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Group(EdwardsGroupType::Constant(
            expected
        ))])
        .to_string(),
        output.to_string()
    )
}

fn output_expected_allocated(program: EdwardsTestCompiler, expected: EdwardsBlsGadget) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Group(EdwardsGroupType::Allocated(gadget))] => {
                assert_eq!(*gadget, expected as EdwardsBlsGadget)
            }
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

fn output_zero(program: EdwardsTestCompiler) {
    output_expected_constant(program, EdwardsAffine::zero())
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
    output_expected_constant(program, point);
}

#[test]
fn test_add() {
    use std::ops::Add;

    let point_1 = EdwardsAffine::from_str(TEST_POINT_1).unwrap();
    let point_2 = EdwardsAffine::from_str(TEST_POINT_2).unwrap();

    let sum = point_1.add(&point_2);

    let program = compile_program(DIRECTORY_NAME, "add.leo").unwrap();
    output_expected_constant(program, sum);
}

#[test]
fn test_sub() {
    use std::ops::Sub;

    let point_1 = EdwardsAffine::from_str(TEST_POINT_1).unwrap();
    let point_2 = EdwardsAffine::from_str(TEST_POINT_2).unwrap();

    let sum = point_1.sub(&point_2);

    let program = compile_program(DIRECTORY_NAME, "sub.leo").unwrap();
    output_expected_constant(program, sum);
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

#[test]
fn test_input() {
    let mut program = compile_program(DIRECTORY_NAME, "input.leo").unwrap();
    program.set_inputs(vec![Some(InputValue::Group(TEST_POINT_1.into()))]);

    let mut cs = TestConstraintSystem::<Fq>::new();
    let constant_point = EdwardsAffine::from_str(TEST_POINT_1).unwrap();
    let allocated_point =
        <EdwardsBlsGadget as AllocGadget<EdwardsAffine, Fq>>::alloc(&mut cs, || Ok(constant_point))
            .unwrap();

    output_expected_allocated(program, allocated_point);
}

#[test]
fn test_ternary() {
    let mut program_1 = compile_program(DIRECTORY_NAME, "ternary.leo").unwrap();
    let mut program_2 = program_1.clone();

    // true -> point_1
    program_1.set_inputs(vec![Some(InputValue::Boolean(true))]);

    let mut cs = TestConstraintSystem::<Fq>::new();
    let point_1 = EdwardsAffine::from_str(TEST_POINT_1).unwrap();
    let expected_point_1 =
        <EdwardsBlsGadget as AllocGadget<EdwardsAffine, Fq>>::alloc(&mut cs, || Ok(point_1))
            .unwrap();
    output_expected_allocated(program_1, expected_point_1);

    // false -> point_2
    program_2.set_inputs(vec![Some(InputValue::Boolean(false))]);

    let mut cs = TestConstraintSystem::<Fq>::new();
    let point_2 = EdwardsAffine::from_str(TEST_POINT_2).unwrap();
    let expected_point_2 =
        <EdwardsBlsGadget as AllocGadget<EdwardsAffine, Fq>>::alloc(&mut cs, || Ok(point_2))
            .unwrap();
    output_expected_allocated(program_2, expected_point_2);
}
