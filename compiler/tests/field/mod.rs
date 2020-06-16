use crate::{
    boolean::{output_expected_boolean, output_false, output_true},
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

use snarkos_curves::edwards_bls12::Fq;
use snarkos_gadgets::curves::edwards_bls12::FqGadget;
use snarkos_models::{
    curves::{One, PrimeField, Zero},
    gadgets::{
        curves::field::FieldGadget,
        r1cs::{ConstraintSystem, TestConstraintSystem},
    },
};
use snarkos_utilities::biginteger::BigInteger256;

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
        CompilerError::FunctionError(FunctionError::FieldError(FieldError::Invalid(_string))) => {}
        error => panic!("Expected invalid field error, got {}", error),
    }
}

fn fail_synthesis(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::FieldError(FieldError::SynthesisError(_string))) => {}
        error => panic!("Expected synthesis error, got {}", error),
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
    fail_synthesis(program);
}

#[test]
fn test_add() {
    use std::ops::Add;

    for _ in 0..10 {
        let r1: u64 = rand::random();
        let r2: u64 = rand::random();

        let b1 = BigInteger256::from(r1);
        let b2 = BigInteger256::from(r2);

        let f1: Fq = Fq::from_repr(b1);
        let f2: Fq = Fq::from_repr(b2);

        let sum = f1.add(&f2);

        let cs = TestConstraintSystem::<Fq>::new();
        let sum_allocated = FqGadget::from(cs, &sum);

        let bytes = include_bytes!("add.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r2.to_string())),
        ]);

        output_expected_allocated(program, sum_allocated);
    }
}

#[test]
fn test_sub() {
    use std::ops::Sub;

    for _ in 0..10 {
        let r1: u64 = rand::random();
        let r2: u64 = rand::random();

        let b1 = BigInteger256::from(r1);
        let b2 = BigInteger256::from(r2);

        let f1: Fq = Fq::from_repr(b1);
        let f2: Fq = Fq::from_repr(b2);

        let difference = f1.sub(&f2);

        let cs = TestConstraintSystem::<Fq>::new();
        let difference_allocated = FqGadget::from(cs, &difference);

        let bytes = include_bytes!("sub.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r2.to_string())),
        ]);

        output_expected_allocated(program, difference_allocated);
    }
}

#[test]
fn test_mul() {
    use std::ops::Mul;

    for _ in 0..10 {
        let r1: u64 = rand::random();
        let r2: u64 = rand::random();

        let b1 = BigInteger256::from(r1);
        let b2 = BigInteger256::from(r2);

        let f1: Fq = Fq::from_repr(b1);
        let f2: Fq = Fq::from_repr(b2);

        let product = f1.mul(&f2);

        let cs = TestConstraintSystem::<Fq>::new();
        let product_allocated = FqGadget::from(cs, &product);

        let bytes = include_bytes!("mul.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r2.to_string())),
        ]);

        output_expected_allocated(program, product_allocated);
    }
}

#[test]
fn test_div() {
    use std::ops::Div;

    for _ in 0..10 {
        let r1: u64 = rand::random();
        let r2: u64 = rand::random();

        let b1 = BigInteger256::from(r1);
        let b2 = BigInteger256::from(r2);

        let f1: Fq = Fq::from_repr(b1);
        let f2: Fq = Fq::from_repr(b2);

        let quotient = f1.div(&f2);

        let cs = TestConstraintSystem::<Fq>::new();
        let quotient_allocated = FqGadget::from(cs, &quotient);

        let bytes = include_bytes!("div.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r2.to_string())),
        ]);

        output_expected_allocated(program, quotient_allocated);
    }
}

#[test]
fn test_eq() {
    for _ in 0..10 {
        let r1: u64 = rand::random();

        // test equal
        let bytes = include_bytes!("eq.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r1.to_string())),
        ]);

        output_true(program);

        // test not equal
        let r2: u64 = rand::random();

        let result = r1.eq(&r2);

        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r2.to_string())),
        ]);

        output_expected_boolean(program, result)
    }
}

#[test]
fn test_ge() {
    for _ in 0..10 {
        let r1: u64 = rand::random();

        // test equal
        let bytes = include_bytes!("ge.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r1.to_string())),
        ]);

        output_true(program);

        // test greater than
        let r2: u64 = rand::random();

        let result = r1.ge(&r2);

        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r2.to_string())),
        ]);

        output_expected_boolean(program, result)
    }
}

#[test]
fn test_gt() {
    for _ in 0..10 {
        let r1: u64 = rand::random();

        // test equal
        let bytes = include_bytes!("gt.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r1.to_string())),
        ]);

        output_false(program);

        // test greater than
        let r2: u64 = rand::random();

        let result = r1.gt(&r2);

        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r2.to_string())),
        ]);

        output_expected_boolean(program, result)
    }
}

#[test]
fn test_le() {
    for _ in 0..10 {
        let r1: u64 = rand::random();

        // test equal
        let bytes = include_bytes!("le.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r1.to_string())),
        ]);

        output_true(program);

        // test greater than
        let r2: u64 = rand::random();

        let result = r1.le(&r2);

        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r2.to_string())),
        ]);

        output_expected_boolean(program, result)
    }
}

#[test]
fn test_lt() {
    for _ in 0..10 {
        let r1: u64 = rand::random();

        // test equal
        let bytes = include_bytes!("lt.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r1.to_string())),
        ]);

        output_false(program);

        // test greater than
        let r2: u64 = rand::random();

        let result = r1.lt(&r2);

        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r2.to_string())),
        ]);

        output_expected_boolean(program, result)
    }
}

#[test]
fn test_assert_eq_pass() {
    for _ in 0..10 {
        let r1: u64 = rand::random();

        let bytes = include_bytes!("assert_eq.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r1.to_string())),
        ]);

        let _ = get_output(program);
    }
}

#[test]
fn test_assert_eq_fail() {
    for _ in 0..10 {
        let r1: u64 = rand::random();
        let r2: u64 = rand::random();

        if r1 == r2 {
            continue;
        }

        let bytes = include_bytes!("assert_eq.leo");
        let mut program = parse_program(bytes).unwrap();

        program.set_inputs(vec![
            Some(InputValue::Field(r1.to_string())),
            Some(InputValue::Field(r2.to_string())),
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

    let f1: Fq = Fq::from_repr(b1);
    let f2: Fq = Fq::from_repr(b2);

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
