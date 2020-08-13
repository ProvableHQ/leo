use crate::{
    assert_satisfied,
    expect_synthesis_error,
    field::field_to_decimal_string,
    generate_main_input,
    parse_program,
    parse_program_with_input,
};
use leo_typed::InputValue;

use snarkos_curves::edwards_bls12::EdwardsAffine;

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

pub fn group_to_decimal_string(g: EdwardsAffine) -> String {
    let x = field_to_decimal_string(g.x);
    let y = field_to_decimal_string(g.y);

    format!("({}, {})", x, y)
}

#[test]
fn test_zero() {
    let bytes = include_bytes!("zero.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_one() {
    let bytes = include_bytes!("one.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program)
}

#[test]
fn test_point() {
    let bytes = include_bytes!("point.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_point_input() {
    let program_bytes = include_bytes!("point_input.leo");
    let input_bytes = include_bytes!("input/point.in");

    let program = parse_program_with_input(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input() {
    let program_bytes = include_bytes!("input.leo");
    let input_bytes_pass = include_bytes!("input/one_one.in");
    let input_bytes_fail = include_bytes!("input/one_zero.in");

    let program = parse_program_with_input(program_bytes, input_bytes_pass).unwrap();

    assert_satisfied(program);

    let program = parse_program_with_input(program_bytes, input_bytes_fail).unwrap();

    expect_synthesis_error(program);
}

#[test]
fn test_negate() {
    use std::ops::Neg;

    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: EdwardsAffine = rng.gen();
        let b = a.neg();

        let a_string = group_to_decimal_string(a);
        let b_string = group_to_decimal_string(b);

        let bytes = include_bytes!("negate.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_string))),
            ("b", Some(InputValue::Group(b_string))),
        ]);
        program.set_main_input(main_input);

        assert_satisfied(program)
    }
}

#[test]
fn test_add() {
    use std::ops::Add;

    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: EdwardsAffine = rng.gen();
        let b: EdwardsAffine = rng.gen();
        let c = a.add(&b);

        let a_string = group_to_decimal_string(a);
        let b_string = group_to_decimal_string(b);
        let c_string = group_to_decimal_string(c);

        let bytes = include_bytes!("add.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_string))),
            ("b", Some(InputValue::Group(b_string))),
            ("c", Some(InputValue::Group(c_string))),
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
        let a: EdwardsAffine = rng.gen();
        let b: EdwardsAffine = rng.gen();
        let c = a.sub(&b);

        let a_string = group_to_decimal_string(a);
        let b_string = group_to_decimal_string(b);
        let c_string = group_to_decimal_string(c);

        let bytes = include_bytes!("sub.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_string))),
            ("b", Some(InputValue::Group(b_string))),
            ("c", Some(InputValue::Group(c_string))),
        ]);
        program.set_main_input(main_input);

        assert_satisfied(program)
    }
}

#[test]
fn test_assert_eq_pass() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: EdwardsAffine = rng.gen();

        let a_string = group_to_decimal_string(a);

        let bytes = include_bytes!("assert_eq.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_string.clone()))),
            ("b", Some(InputValue::Group(a_string))),
        ]);

        program.set_main_input(main_input);

        assert_satisfied(program);
    }
}

#[test]
fn test_assert_eq_fail() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: EdwardsAffine = rng.gen();
        let b: EdwardsAffine = rng.gen();

        if a == b {
            continue;
        }

        let a_string = group_to_decimal_string(a);
        let b_string = group_to_decimal_string(b);

        let bytes = include_bytes!("assert_eq.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_string))),
            ("b", Some(InputValue::Group(b_string))),
        ]);

        program.set_main_input(main_input);

        expect_synthesis_error(program);
    }
}

#[test]
fn test_eq() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: EdwardsAffine = rng.gen();
        let b: EdwardsAffine = rng.gen();

        let a_string = group_to_decimal_string(a);
        let b_string = group_to_decimal_string(b);

        // test equal

        let bytes = include_bytes!("eq.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_string.clone()))),
            ("b", Some(InputValue::Group(a_string.clone()))),
            ("c", Some(InputValue::Boolean(true))),
        ]);

        program.set_main_input(main_input);

        assert_satisfied(program);

        // test not equal

        let c = a.eq(&b);

        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_string))),
            ("b", Some(InputValue::Group(b_string))),
            ("c", Some(InputValue::Boolean(c))),
        ]);

        program.set_main_input(main_input);

        assert_satisfied(program);
    }
}

#[test]
fn test_ternary() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    let a: EdwardsAffine = rng.gen();
    let b: EdwardsAffine = rng.gen();

    let a_string = group_to_decimal_string(a);
    let b_string = group_to_decimal_string(b);

    let bytes = include_bytes!("ternary.leo");
    let mut program = parse_program(bytes).unwrap();

    // true -> field a
    let main_input = generate_main_input(vec![
        ("s", Some(InputValue::Boolean(true))),
        ("a", Some(InputValue::Group(a_string.clone()))),
        ("b", Some(InputValue::Group(b_string.clone()))),
        ("c", Some(InputValue::Group(a_string.clone()))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(bytes).unwrap();

    // false -> field b
    let main_input = generate_main_input(vec![
        ("s", Some(InputValue::Boolean(false))),
        ("a", Some(InputValue::Group(a_string))),
        ("b", Some(InputValue::Group(b_string.clone()))),
        ("c", Some(InputValue::Group(b_string))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}
