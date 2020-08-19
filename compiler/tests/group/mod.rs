// Copyright (C) 2019-2020 Aleo Systems Inc.
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
    assert_satisfied,
    expect_compiler_error,
    expect_synthesis_error,
    field::field_to_decimal_string,
    generate_main_input,
    parse_program,
    parse_program_with_input,
};
use leo_typed::{GroupCoordinate, GroupValue, InputValue, Span};

use snarkos_curves::edwards_bls12::EdwardsAffine;

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

    GroupValue {
        x: GroupCoordinate::Number(x, fake_span.clone()),
        y: GroupCoordinate::Number(y, fake_span.clone()),
        span: fake_span,
    }
}

#[test]
fn test_point() {
    let bytes = include_bytes!("point.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_x_sign_high() {
    let bytes = include_bytes!("x_sign_high.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_x_sign_low() {
    let bytes = include_bytes!("x_sign_low.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_x_sign_inferred() {
    let bytes = include_bytes!("x_sign_inferred.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_y_sign_high() {
    let bytes = include_bytes!("y_sign_high.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_y_sign_low() {
    let bytes = include_bytes!("y_sign_low.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_y_sign_inferred() {
    let bytes = include_bytes!("y_sign_inferred.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_double_high() {
    let bytes = include_bytes!("double_high.leo");

    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_double_low() {
    let bytes = include_bytes!("double_low.leo");

    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_double_inferred() {
    let bytes = include_bytes!("double_inferred.leo");

    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
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
    let input_bytes_pass = include_bytes!("input/valid.in");
    let input_bytes_fail = include_bytes!("input/invalid.in");

    let program = parse_program_with_input(program_bytes, input_bytes_pass).unwrap();

    assert_satisfied(program);

    let program = parse_program_with_input(program_bytes, input_bytes_fail).unwrap();

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

        let bytes = include_bytes!("negate.leo");
        let mut program = parse_program(bytes).unwrap();

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

        let bytes = include_bytes!("add.leo");
        let mut program = parse_program(bytes).unwrap();

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

        let bytes = include_bytes!("sub.leo");
        let mut program = parse_program(bytes).unwrap();

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
fn test_assert_eq_pass() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: EdwardsAffine = rng.gen();

        let a_element = group_element_to_input_value(a);

        let bytes = include_bytes!("assert_eq.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_element.clone()))),
            ("b", Some(InputValue::Group(a_element))),
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

        let a_element = group_element_to_input_value(a);
        let b_element = group_element_to_input_value(b);

        let bytes = include_bytes!("assert_eq.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_element))),
            ("b", Some(InputValue::Group(b_element))),
        ]);

        program.set_main_input(main_input);

        expect_synthesis_error(program);
    }
}

#[test]
#[ignore]
fn test_eq() {
    let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..10 {
        let a: EdwardsAffine = rng.gen();
        let b: EdwardsAffine = rng.gen();

        let a_element = group_element_to_input_value(a);
        let b_element = group_element_to_input_value(b);

        // test equal

        let bytes = include_bytes!("eq.leo");
        let mut program = parse_program(bytes).unwrap();

        let main_input = generate_main_input(vec![
            ("a", Some(InputValue::Group(a_element.clone()))),
            ("b", Some(InputValue::Group(a_element.clone()))),
            ("c", Some(InputValue::Boolean(true))),
        ]);

        program.set_main_input(main_input);

        assert_satisfied(program);

        // test not equal

        let c = a.eq(&b);

        let mut program = parse_program(bytes).unwrap();

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

    let bytes = include_bytes!("ternary.leo");
    let mut program = parse_program(bytes).unwrap();

    // true -> field a
    let main_input = generate_main_input(vec![
        ("s", Some(InputValue::Boolean(true))),
        ("a", Some(InputValue::Group(a_element.clone()))),
        ("b", Some(InputValue::Group(b_element.clone()))),
        ("c", Some(InputValue::Group(a_element.clone()))),
    ]);

    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(bytes).unwrap();

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
