use crate::{
    boolean::{output_expected_boolean, output_true},
    get_error,
    get_output,
    parse_program,
    EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_compiler::{
    errors::{CompilerError, FieldError, FunctionError},
    ConstrainedValue,
    FieldType,
};
use leo_types::InputValue;

use num_bigint::BigUint;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use snarkos_curves::edwards_bls12::Fq;
use snarkos_gadgets::curves::edwards_bls12::FqGadget;
use snarkos_models::{
    curves::{One, PrimeField, Zero},
    gadgets::{
        curves::field::FieldGadget,
        r1cs::{ConstraintSystem, TestConstraintSystem},
    },
};
use snarkos_utilities::{biginteger::BigInteger256, bytes::ToBytes};

fn output_expected_constant(program: EdwardsTestCompiler, expected: Fq) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Field(FieldType::Constant(expected))]).to_string(),
        output.to_string()
    );
}

fn output_expected_allocated(program: EdwardsTestCompiler, expected: FqGadget) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Field(FieldType::Allocated(fp_gadget))] => assert_eq!(*fp_gadget, expected as FqGadget),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

fn output_zero(program: EdwardsTestCompiler) {
    output_expected_constant(program, Fq::zero())
}

fn output_one(program: EdwardsTestCompiler) {
    output_expected_constant(program, Fq::one())
}

fn fail_field(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::FieldError(FieldError::Error(_string))) => {}
        error => panic!("Expected invalid field error, got {}", error),
    }
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

#[test]
fn test_input_pass() {
    let bytes = include_bytes!("input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![Some(InputValue::Field("1".into()))]);

    let cs = TestConstraintSystem::<Fq>::new();
    let expected = FqGadget::one(cs).unwrap();

    output_expected_allocated(program, expected)
}

#[test]
fn test_input_fail_bool() {
    let bytes = include_bytes!("input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![Some(InputValue::Boolean(true))]);
    fail_field(program);
}

#[test]
fn test_input_fail_none() {
    let bytes = include_bytes!("input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![None]);
    fail_field(program);
}

#[test]
fn test_add() {
    use std::ops::Add;

    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let r1: Fq = rng.gen();
        let r2: Fq = rng.gen();

        let mut r1_buf = Vec::new();
        let mut r2_buf = Vec::new();

        r1.write(&mut r1_buf).unwrap();
        r2.write(&mut r2_buf).unwrap();

        let r1_bigint = BigUint::from_bytes_le(&r1_buf);
        let r2_bigint = BigUint::from_bytes_le(&r2_buf);

        let sum = r1.add(&r2);

        let cs = TestConstraintSystem::<Fq>::new();
        let sum_allocated = FqGadget::from(cs, &sum);

        let bytes = include_bytes!("add.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1_bigint.to_str_radix(10))),
            Some(InputValue::Field(r2_bigint.to_str_radix(10))),
        ]);

        output_expected_allocated(program, sum_allocated);
    }
}

#[test]
fn test_sub() {
    use std::ops::Sub;

    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let r1: Fq = rng.gen();
        let r2: Fq = rng.gen();

        let mut r1_buf = Vec::new();
        let mut r2_buf = Vec::new();

        r1.write(&mut r1_buf).unwrap();
        r2.write(&mut r2_buf).unwrap();

        let r1_bigint = BigUint::from_bytes_le(&r1_buf);
        let r2_bigint = BigUint::from_bytes_le(&r2_buf);

        let difference = r1.sub(&r2);

        let cs = TestConstraintSystem::<Fq>::new();
        let difference_allocated = FqGadget::from(cs, &difference);

        let bytes = include_bytes!("sub.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1_bigint.to_str_radix(10))),
            Some(InputValue::Field(r2_bigint.to_str_radix(10))),
        ]);

        output_expected_allocated(program, difference_allocated);
    }
}

#[test]
fn test_mul() {
    use std::ops::Mul;

    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let r1: Fq = rng.gen();
        let r2: Fq = rng.gen();

        let mut r1_buf = Vec::new();
        let mut r2_buf = Vec::new();

        r1.write(&mut r1_buf).unwrap();
        r2.write(&mut r2_buf).unwrap();

        let r1_bigint = BigUint::from_bytes_le(&r1_buf);
        let r2_bigint = BigUint::from_bytes_le(&r2_buf);

        let product = r1.mul(&r2);

        let cs = TestConstraintSystem::<Fq>::new();
        let product_allocated = FqGadget::from(cs, &product);

        let bytes = include_bytes!("mul.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1_bigint.to_str_radix(10))),
            Some(InputValue::Field(r2_bigint.to_str_radix(10))),
        ]);

        output_expected_allocated(program, product_allocated);
    }
}

#[test]
fn test_div() {
    use std::ops::Div;

    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let r1: Fq = rng.gen();
        let r2: Fq = rng.gen();

        let mut r1_buf = Vec::new();
        let mut r2_buf = Vec::new();

        r1.write(&mut r1_buf).unwrap();
        r2.write(&mut r2_buf).unwrap();

        let r1_bigint = BigUint::from_bytes_le(&r1_buf);
        let r2_bigint = BigUint::from_bytes_le(&r2_buf);

        let quotient = r1.div(&r2);

        let cs = TestConstraintSystem::<Fq>::new();
        let quotient_allocated = FqGadget::from(cs, &quotient);

        let bytes = include_bytes!("div.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1_bigint.to_str_radix(10))),
            Some(InputValue::Field(r2_bigint.to_str_radix(10))),
        ]);

        output_expected_allocated(program, quotient_allocated);
    }
}

#[test]
fn test_eq() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let r1: Fq = rng.gen();
        let r2: Fq = rng.gen();

        let mut r1_buf = Vec::new();
        let mut r2_buf = Vec::new();

        r1.write(&mut r1_buf).unwrap();
        r2.write(&mut r2_buf).unwrap();

        let r1_bigint = BigUint::from_bytes_le(&r1_buf);
        let r2_bigint = BigUint::from_bytes_le(&r2_buf);

        // test equal

        let bytes = include_bytes!("eq.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1_bigint.to_str_radix(10))),
            Some(InputValue::Field(r1_bigint.to_str_radix(10))),
        ]);

        output_true(program);

        // test not equal

        let result = r1.eq(&r2);

        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1_bigint.to_str_radix(10))),
            Some(InputValue::Field(r2_bigint.to_str_radix(10))),
        ]);

        output_expected_boolean(program, result)
    }
}

#[test]
fn test_assert_eq_pass() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let r1: Fq = rng.gen();
        let mut r1_buf = Vec::new();
        r1.write(&mut r1_buf).unwrap();
        let r1_bigint = BigUint::from_bytes_le(&r1_buf);

        let bytes = include_bytes!("assert_eq.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1_bigint.to_str_radix(10))),
            Some(InputValue::Field(r1_bigint.to_str_radix(10))),
        ]);

        let _ = get_output(program);
    }
}

#[test]
fn test_assert_eq_fail() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let r1: Fq = rng.gen();
        let r2: Fq = rng.gen();

        let mut r1_buf = Vec::new();
        let mut r2_buf = Vec::new();
        r1.write(&mut r1_buf).unwrap();
        r2.write(&mut r2_buf).unwrap();
        let r1_bigint = BigUint::from_bytes_le(&r1_buf);
        let r2_bigint = BigUint::from_bytes_le(&r2_buf);

        if r1 == r2 {
            continue;
        }

        let bytes = include_bytes!("assert_eq.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1_bigint.to_str_radix(10))),
            Some(InputValue::Field(r2_bigint.to_str_radix(10))),
        ]);

        let mut cs = TestConstraintSystem::<Fq>::new();
        let _ = program.compile_constraints(&mut cs).unwrap();
        assert!(!cs.is_satisfied());
    }
}

#[test]
fn test_ternary() {
    let r1: u64 = rand::random();
    let r2: u64 = rand::random();

    let b1 = BigInteger256::from(r1);
    let b2 = BigInteger256::from(r2);

    let f1: Fq = Fq::from_repr(b1).unwrap();
    let f2: Fq = Fq::from_repr(b2).unwrap();

    let mut cs = TestConstraintSystem::<Fq>::new();
    let g1 = FqGadget::from(cs.ns(|| "g1"), &f1);
    let g2 = FqGadget::from(cs.ns(|| "g2"), &f2);

    let bytes = include_bytes!("ternary.leo");
    let mut program_1 = parse_program(bytes).unwrap();
    let mut program_2 = program_1.clone();

    // true -> field 1
    program_1.set_inputs(vec![
        Some(InputValue::Boolean(true)),
        Some(InputValue::Field(r1.to_string())),
        Some(InputValue::Field(r2.to_string())),
    ]);

    output_expected_allocated(program_1, g1);

    // false -> field 2
    program_2.set_inputs(vec![
        Some(InputValue::Boolean(false)),
        Some(InputValue::Field(r1.to_string())),
        Some(InputValue::Field(r2.to_string())),
    ]);

    output_expected_allocated(program_2, g2);
}
