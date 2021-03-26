// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{
    assert_satisfied, expect_compiler_error, expect_synthesis_error, field::field_to_decimal_string,
    generate_main_input, parse_program, parse_program_with_input,
};
use leo_ast::{GroupCoordinate, GroupTuple, GroupValue, InputValue, Span};

use snarkvm_curves::edwards_bls12::EdwardsAffine;

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

pub fn group_element_to_input_value(g: EdwardsAffine) -> GroupValue {
    let x = field_to_decimal_string(g.x);
    let y = field_to_decimal_string(g.y);

    format!("({}, {})", x, y);

    let fake_span = Span {
        text: "".to_string(),
        line: 0,
        start: 0,
        end: 0,
    };

    GroupValue::Tuple(GroupTuple {
        x: GroupCoordinate::Number(x, fake_span),
        y: GroupCoordinate::Number(y, fake_span),
        span: fake_span,
    })
}

#[test]
fn test_one() {
    let program_string = include_str!("one.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_zero() {
    let program_string = include_str!("zero.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_point() {
    let program_string = include_str!("point.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_x_sign_high() {
    let program_string = include_str!("x_sign_high.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_x_sign_low() {
    let program_string = include_str!("x_sign_low.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_x_sign_inferred() {
    let program_string = include_str!("x_sign_inferred.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_y_sign_high() {
    let program_string = include_str!("y_sign_high.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_y_sign_low() {
    let program_string = include_str!("y_sign_low.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_y_sign_inferred() {
    let program_string = include_str!("y_sign_inferred.leo");
    let program = parse_program(program_string).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_both_sign_high() {
    let program_string = include_str!("both_sign_high.leo");

    let program = parse_program(program_string).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_both_sign_low() {
    let program_string = include_str!("both_sign_low.leo");

    let program = parse_program(program_string).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_both_sign_inferred() {
    let program_string = include_str!("both_sign_inferred.leo");

    let program = parse_program(program_string).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_point_input() {
    let program_string = include_str!("point_input.leo");
    let input_bytes = include_str!("input/point.in");

    let program = parse_program_with_input(program_string, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_input() {
    let program_string = include_str!("input.leo");
    let input_string_pass = include_str!("input/valid.in");
    let input_string_fail = include_str!("input/invalid.in");

    let program = parse_program_with_input(program_string, input_string_pass).unwrap();

    assert_satisfied(program);

    let program = parse_program_with_input(program_string, input_string_fail).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_negate() {
    use std::ops::Neg;

    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: EdwardsAffine = rng.gen();
        let b = a.neg();

        let a_element = group_element_to_input_value(a);
        let b_element = group_element_to_input_value(b);

        let program_string = include_str!("negate.leo");
        let mut program = parse_program(program_string).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_element))),
            ("b", Some(InputValue::Group(b_element))),
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

        let a_element = group_element_to_input_value(a);
        let b_element = group_element_to_input_value(b);
        let c_element = group_element_to_input_value(c);

        let program_string = include_str!("add.leo");
        let mut program = parse_program(program_string).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_element))),
            ("b", Some(InputValue::Group(b_element))),
            ("c", Some(InputValue::Group(c_element))),
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

        let a_element = group_element_to_input_value(a);
        let b_element = group_element_to_input_value(b);
        let c_element = group_element_to_input_value(c);

        let program_string = include_str!("sub.leo");
        let mut program = parse_program(program_string).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_element))),
            ("b", Some(InputValue::Group(b_element))),
            ("c", Some(InputValue::Group(c_element))),
        ]);
        program.set_main_input(main_input);

        assert_satisfied(program)
    }
}

#[test]
fn test_console_assert_pass() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: EdwardsAffine = rng.gen();

        let a_element = group_element_to_input_value(a);

        let program_string = include_str!("assert_eq.leo");
        let mut program = parse_program(program_string).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_element.clone()))),
            ("b", Some(InputValue::Group(a_element))),
        ]);

        program.set_main_input(main_input);

        assert_satisfied(program);
    }
}

#[test]
fn test_console_assert_fail() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: EdwardsAffine = rng.gen();
        let b: EdwardsAffine = rng.gen();

        if a == b {
            continue;
        }

        let a_element = group_element_to_input_value(a);
        let b_element = group_element_to_input_value(b);

        let program_string = include_str!("assert_eq.leo");
        let mut program = parse_program(program_string).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_element))),
            ("b", Some(InputValue::Group(b_element))),
        ]);

        program.set_main_input(main_input);

        expect_compiler_error(program);
    }
}

#[test]
fn test_eq() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: EdwardsAffine = rng.gen();
        let b: EdwardsAffine = rng.gen();

        let a_element = group_element_to_input_value(a);
        let b_element = group_element_to_input_value(b);

        // test equal

        let program_string = include_str!("eq.leo");
        let mut program = parse_program(program_string).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_element.clone()))),
            ("b", Some(InputValue::Group(a_element.clone()))),
            ("c", Some(InputValue::Boolean(true))),
        ]);

        program.set_main_input(main_input);

        assert_satisfied(program);

        // test not equal

        let c = a.eq(&b);

        let mut program = parse_program(program_string).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_element))),
            ("b", Some(InputValue::Group(b_element))),
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

    let a_element = group_element_to_input_value(a);
    let b_element = group_element_to_input_value(b);

    let program_string = include_str!("ternary.leo");
    let mut program = parse_program(program_string).unwrap();

    // true -> field a
    let main_input = generate_main_input(vec![
        ("s", Some(InputValue::Boolean(true))),
        ("a", Some(InputValue::Group(a_element.clone()))),
        ("b", Some(InputValue::Group(b_element.clone()))),
        ("c", Some(InputValue::Group(a_element.clone()))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(program_string).unwrap();

    // false -> field b
    let main_input = generate_main_input(vec![
        ("s", Some(InputValue::Boolean(false))),
        ("a", Some(InputValue::Group(a_element))),
        ("b", Some(InputValue::Group(b_element.clone()))),
        ("c", Some(InputValue::Group(b_element))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}

#[test]
fn test_positive_and_negative() {
    let program_string = include_str!("positive_and_negative.leo");

    let program = parse_program(program_string).unwrap();
    
    assert_satisfied(program);
}
