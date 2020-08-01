use crate::{assert_satisfied, expect_synthesis_error, generate_main_input, parse_program};
use leo_types::InputValue;

use snarkos_curves::edwards_bls12::Fq;
use snarkos_utilities::bytes::ToBytes;

use num_bigint::BigUint;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

// Helper function to convert field element into decimal base 10 string
pub fn field_to_decimal_string(f: Fq) -> String {
    // write field to buffer

    let mut buf = Vec::new();

    f.write(&mut buf).unwrap();

    // convert to big integer

    let f_bigint = BigUint::from_bytes_le(&buf);

    f_bigint.to_str_radix(10)
}

#[test]
fn test_add() {
    use std::ops::Add;

    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: Fq = rng.gen();
        let b: Fq = rng.gen();
        let c = a.add(&b);

        let a_string = field_to_decimal_string(a);
        let b_string = field_to_decimal_string(b);
        let c_string = field_to_decimal_string(c);

        let bytes = include_bytes!("add.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Field(a_string))),
            ("b", Some(InputValue::Field(b_string))),
            ("c", Some(InputValue::Field(c_string))),
        ]);

        program.set_main_input(main_input);

        assert_satisfied(program)
    }
}

#[test]
fn test_sub() {
    use std::ops::Sub;

    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: Fq = rng.gen();
        let b: Fq = rng.gen();
        let c = a.sub(&b);

        let a_string = field_to_decimal_string(a);
        let b_string = field_to_decimal_string(b);
        let c_string = field_to_decimal_string(c);

        let bytes = include_bytes!("sub.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Field(a_string))),
            ("b", Some(InputValue::Field(b_string))),
            ("c", Some(InputValue::Field(c_string))),
        ]);
        program.set_main_input(main_input);

        assert_satisfied(program)
    }
}

#[test]
fn test_div() {
    use std::ops::Div;

    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: Fq = rng.gen();
        let b: Fq = rng.gen();
        let c = a.div(&b);

        let a_string = field_to_decimal_string(a);
        let b_string = field_to_decimal_string(b);
        let c_string = field_to_decimal_string(c);

        let bytes = include_bytes!("div.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Field(a_string))),
            ("b", Some(InputValue::Field(b_string))),
            ("c", Some(InputValue::Field(c_string))),
        ]);
        program.set_main_input(main_input);

        assert_satisfied(program)
    }
}

#[test]
fn test_mul() {
    use std::ops::Mul;

    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: Fq = rng.gen();
        let b: Fq = rng.gen();
        let c = a.mul(&b);

        let a_string = field_to_decimal_string(a);
        let b_string = field_to_decimal_string(b);
        let c_string = field_to_decimal_string(c);

        let bytes = include_bytes!("mul.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Field(a_string))),
            ("b", Some(InputValue::Field(b_string))),
            ("c", Some(InputValue::Field(c_string))),
        ]);

        program.set_main_input(main_input);

        assert_satisfied(program)
    }
}

#[test]
fn test_eq() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: Fq = rng.gen();
        let b: Fq = rng.gen();

        let a_string = field_to_decimal_string(a);
        let b_string = field_to_decimal_string(b);

        // test equal

        let bytes = include_bytes!("eq.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Field(a_string.clone()))),
            ("b", Some(InputValue::Field(a_string.clone()))),
            ("c", Some(InputValue::Boolean(true))),
        ]);

        program.set_main_input(main_input);

        assert_satisfied(program);

        // test not equal

        let c = a.eq(&b);

        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Field(a_string))),
            ("b", Some(InputValue::Field(b_string))),
            ("c", Some(InputValue::Boolean(c))),
        ]);

        program.set_main_input(main_input);

        assert_satisfied(program);
    }
}

#[test]
fn test_assert_eq_pass() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: Fq = rng.gen();

        let a_string = field_to_decimal_string(a);

        let bytes = include_bytes!("assert_eq.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Field(a_string.clone()))),
            ("b", Some(InputValue::Field(a_string))),
        ]);

        program.set_main_input(main_input);

        assert_satisfied(program);
    }
}

#[test]
fn test_assert_eq_fail() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: Fq = rng.gen();
        let b: Fq = rng.gen();

        if a == b {
            continue;
        }

        let a_string = field_to_decimal_string(a);
        let b_string = field_to_decimal_string(b);

        let bytes = include_bytes!("assert_eq.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Field(a_string))),
            ("b", Some(InputValue::Field(b_string))),
        ]);

        program.set_main_input(main_input);

        expect_synthesis_error(program);
    }
}

#[test]
fn test_ternary() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    let a: Fq = rng.gen();
    let b: Fq = rng.gen();

    let a_string = field_to_decimal_string(a);
    let b_string = field_to_decimal_string(b);

    let bytes = include_bytes!("ternary.leo");
    let mut program = parse_program(bytes).unwrap();

    // true -> field a
    let main_input = generate_main_input(vec![
        ("s", Some(InputValue::Boolean(true))),
        ("a", Some(InputValue::Field(a_string.clone()))),
        ("b", Some(InputValue::Field(b_string.clone()))),
        ("c", Some(InputValue::Field(a_string.clone()))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(bytes).unwrap();

    // false -> field b
    let main_input = generate_main_input(vec![
        ("s", Some(InputValue::Boolean(false))),
        ("a", Some(InputValue::Field(a_string))),
        ("b", Some(InputValue::Field(b_string.clone()))),
        ("c", Some(InputValue::Field(b_string))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}

//
// pub fn output_one(program: EdwardsTestCompiler) {
//     let expected = include_bytes!("outputs_/register_one.out");
//     let actual = get_outputs(program);
//
//     assert_eq!(expected, actual.bytes().as_slice());
// }
//
// pub fn output_zero(program: EdwardsTestCompiler) {
//     let expected = include_bytes!("outputs_/register_zero.out");
//     let actual = get_outputs(program);
//
//     assert_eq!(expected, actual.bytes().as_slice());
// }
//
// #[test]
// fn test_registers() {
//     let program_bytes = include_bytes!("output_register.leo");
//     let one_input_bytes = include_bytes!("input/register_one.in");
//     let zero_input_bytes = include_bytes!("input/register_zero.in");
//
//     // test 1field input register => 1field output register
//     let program = parse_program_with_input(program_bytes, one_input_bytes).unwrap();
//
//     output_one(program);
//
//     // test 0field input register => 0field output register
//     let program = parse_program_with_input(program_bytes, zero_input_bytes).unwrap();
//
//     output_zero(program);
// }
