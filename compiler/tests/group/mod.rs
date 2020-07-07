use crate::{
    boolean::{output_false, output_true},
    fail_enforce,
    get_output,
    parse_program,
    EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_compiler::{group::edwards_bls12::EdwardsGroupType, ConstrainedValue};
use leo_types::InputValue;

use snarkos_curves::edwards_bls12::{EdwardsAffine, EdwardsParameters, Fq};
use snarkos_gadgets::curves::edwards_bls12::EdwardsBlsGadget;
use snarkos_models::{
    curves::{TEModelParameters, Zero},
    gadgets::{r1cs::TestConstraintSystem, utilities::alloc::AllocGadget},
};
use std::str::FromStr;

const TEST_POINT_1: &str = "(7374112779530666882856915975292384652154477718021969292781165691637980424078, 3435195339177955418892975564890903138308061187980579490487898366607011481796)";
const TEST_POINT_2: &str = "(1005842117974384149622370061042978581211342111653966059496918451529532134799, 79389132189982034519597104273449021362784864778548730890166152019533697186)";

fn output_expected_constant(program: EdwardsTestCompiler, expected: EdwardsAffine) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Group(EdwardsGroupType::Constant(expected))])
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

fn output_one(program: EdwardsTestCompiler) {
    let (x, y) = EdwardsParameters::AFFINE_GENERATOR_COEFFS;
    let one = EdwardsAffine::new(x, y);

    output_expected_constant(program, one)
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

    output_one(program)
}

#[test]
fn test_point() {
    let point = EdwardsAffine::from_str(TEST_POINT_1).unwrap();
    let bytes = include_bytes!("point.leo");
    let program = parse_program(bytes).unwrap();

    output_expected_constant(program, point);
}

#[test]
fn test_input() {
    let bytes = include_bytes!("input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![Some(InputValue::Group(TEST_POINT_1.into()))]);

    let mut cs = TestConstraintSystem::<Fq>::new();
    let constant_point = EdwardsAffine::from_str(TEST_POINT_1).unwrap();
    let allocated_point =
        <EdwardsBlsGadget as AllocGadget<EdwardsAffine, Fq>>::alloc(&mut cs, || Ok(constant_point)).unwrap();

    output_expected_allocated(program, allocated_point);
}

#[test]
fn test_add() {
    use std::ops::Add;

    let point_1 = EdwardsAffine::from_str(TEST_POINT_1).unwrap();
    let point_2 = EdwardsAffine::from_str(TEST_POINT_2).unwrap();

    let sum = point_1.add(&point_2);

    let bytes = include_bytes!("add.leo");
    let program = parse_program(bytes).unwrap();

    output_expected_constant(program, sum);
}

#[test]
fn test_sub() {
    use std::ops::Sub;

    let point_1 = EdwardsAffine::from_str(TEST_POINT_1).unwrap();
    let point_2 = EdwardsAffine::from_str(TEST_POINT_2).unwrap();

    let sum = point_1.sub(&point_2);

    let bytes = include_bytes!("sub.leo");
    let program = parse_program(bytes).unwrap();

    output_expected_constant(program, sum);
}

#[test]
fn test_eq_true() {
    let bytes = include_bytes!("eq_true.leo");
    let program = parse_program(bytes).unwrap();

    output_true(program)
}

#[test]
fn test_eq_false() {
    let bytes = include_bytes!("eq_false.leo");
    let program = parse_program(bytes).unwrap();

    output_false(program)
}

#[test]
fn test_assert_eq_pass() {
    let bytes = include_bytes!("assert_eq_true.leo");
    let program = parse_program(bytes).unwrap();
    let _res = get_output(program);
}

#[test]
fn test_assert_eq_fail() {
    let bytes = include_bytes!("assert_eq_false.leo");
    let program = parse_program(bytes).unwrap();

    fail_enforce(program);
}

#[test]
fn test_ternary() {
    let bytes = include_bytes!("ternary.leo");
    let mut program_1 = parse_program(bytes).unwrap();

    let mut program_2 = program_1.clone();

    // true -> point_1
    program_1.set_inputs(vec![Some(InputValue::Boolean(true))]);

    let mut cs = TestConstraintSystem::<Fq>::new();
    let point_1 = EdwardsAffine::from_str(TEST_POINT_1).unwrap();
    let expected_point_1 =
        <EdwardsBlsGadget as AllocGadget<EdwardsAffine, Fq>>::alloc(&mut cs, || Ok(point_1)).unwrap();
    output_expected_allocated(program_1, expected_point_1);

    // false -> point_2
    program_2.set_inputs(vec![Some(InputValue::Boolean(false))]);

    let mut cs = TestConstraintSystem::<Fq>::new();
    let point_2 = EdwardsAffine::from_str(TEST_POINT_2).unwrap();
    let expected_point_2 =
        <EdwardsBlsGadget as AllocGadget<EdwardsAffine, Fq>>::alloc(&mut cs, || Ok(point_2)).unwrap();
    output_expected_allocated(program_2, expected_point_2);
}
