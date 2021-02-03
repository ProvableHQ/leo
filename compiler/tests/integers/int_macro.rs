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

macro_rules! test_int {
    ($name: ident, $type_: ty, $integer_type: expr, $gadget: ty) => {
        pub struct $name {}

        impl $name {
            fn test_negate() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();

                    let b = match a.checked_neg() {
                        Some(valid) => valid,
                        None => continue,
                    };

                    let program_string = include_str!("negate.leo");
                    let mut program = parse_program(program_string).unwrap();
                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);
                }
            }

            fn test_negate_min_fail() {
                let program_string = include_str!("negate_min.leo");
                let program = parse_program(program_string).unwrap();

                expect_computation_error(program);
            }

            fn test_negate_zero() {
                let program_string = include_str!("negate_zero.leo");
                let program = parse_program(program_string).unwrap();

                assert_satisfied(program);
            }
        }

        impl IntegerTester for $name {
            fn test_min() {
                let program_string = include_str!("min.leo");
                let program = parse_program(program_string).unwrap();

                assert_satisfied(program);
            }

            fn test_min_fail() {
                let program_string = include_str!("min_fail.leo");
                let error = parse_program(program_string).err().unwrap();

                expect_asg_error(error);
            }

            fn test_max() {
                let program_string = include_str!("max.leo");
                let program = parse_program(program_string).unwrap();

                assert_satisfied(program);
            }

            fn test_max_fail() {
                let program_string = include_str!("max_fail.leo");
                let error = parse_program(program_string).err().unwrap();

                expect_asg_error(error);
            }

            fn test_add() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();
                    let b: $type_ = rand::random();

                    let c = match a.checked_add(b) {
                        Some(valid) => valid,
                        None => continue,
                    };

                    let program_string = include_str!("add.leo");
                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                        ("c", Some(InputValue::Integer($integer_type, c.to_string()))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);
                }
            }

            fn test_sub() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();
                    let b: $type_ = rand::random();

                    if b.checked_neg().is_none() {
                        continue;
                    }

                    let c = match a.checked_sub(b) {
                        Some(valid) => valid,
                        None => continue,
                    };

                    let program_string = include_str!("sub.leo");
                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                        ("c", Some(InputValue::Integer($integer_type, c.to_string()))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);
                }
            }

            fn test_mul() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();
                    let b: $type_ = rand::random();

                    let c = match a.checked_mul(b) {
                        Some(valid) => valid,
                        None => continue,
                    };

                    let program_string = include_str!("mul.leo");
                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                        ("c", Some(InputValue::Integer($integer_type, c.to_string()))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);
                }
            }

            fn test_div() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();
                    let b: $type_ = rand::random();

                    // make sure that we can calculate the inverse of each number
                    // Leo signed integer division is non-wrapping. Thus attempting to calculate a
                    // division result that wraps should be ignored here.
                    if a.checked_neg().is_none() {
                        continue;
                    }

                    let program_string = include_str!("div.leo");
                    let mut program = parse_program(program_string).unwrap();

                    // expect an error when dividing by zero
                    if b == 0 {
                        let main_input = generate_main_input(vec![
                            ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                            ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                            ("c", Some(InputValue::Integer($integer_type, b.to_string()))),
                        ]);

                        program.set_main_input(main_input);

                        expect_compiler_error(program);
                    } else {
                        let c = match a.checked_div(b) {
                            Some(valid) => valid,
                            None => continue,
                        };

                        let main_input = generate_main_input(vec![
                            ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                            ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                            ("c", Some(InputValue::Integer($integer_type, c.to_string()))),
                        ]);

                        program.set_main_input(main_input);

                        assert_satisfied(program);
                    }
                }
            }

            fn test_pow() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();
                    let b: $type_ = rand::random();

                    // rust specific conversion see https://doc.rust-lang.org/std/primitive.u8.html#method.checked_pow
                    let c = match a.checked_pow(b as u32) {
                        Some(valid) => valid,
                        None => continue,
                    };

                    let program_string = include_str!("pow.leo");
                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                        ("c", Some(InputValue::Integer($integer_type, c.to_string()))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);
                }
            }

            fn test_eq() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();
                    let b: $type_ = rand::random();

                    // test equal

                    let program_string = include_str!("eq.leo");
                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("c", Some(InputValue::Boolean(true))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);

                    // test not equal

                    let c = a.eq(&b);

                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                        ("c", Some(InputValue::Boolean(c))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);
                }
            }

            fn test_ne() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();
                    let b: $type_ = rand::random();

                    // test a != a == false

                    let program_string = include_str!("ne.leo");
                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("c", Some(InputValue::Boolean(false))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);

                    // test not equal

                    let c = a.ne(&b);

                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                        ("c", Some(InputValue::Boolean(c))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);
                }
            }

            fn test_ge() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();
                    let b: $type_ = rand::random();

                    // test equal

                    let program_string = include_str!("ge.leo");
                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("c", Some(InputValue::Boolean(true))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);

                    // test greater or equal

                    let c = a.ge(&b);

                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                        ("c", Some(InputValue::Boolean(c))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);
                }
            }

            fn test_gt() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();
                    let b: $type_ = rand::random();

                    // test equal

                    let program_string = include_str!("gt.leo");
                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("c", Some(InputValue::Boolean(false))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);

                    // test greater than

                    let c = a.gt(&b);

                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                        ("c", Some(InputValue::Boolean(c))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);
                }
            }

            fn test_le() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();
                    let b: $type_ = rand::random();

                    // test equal

                    let program_string = include_str!("le.leo");
                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("c", Some(InputValue::Boolean(true))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);

                    // test less or equal

                    let c = a.le(&b);

                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                        ("c", Some(InputValue::Boolean(c))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);
                }
            }

            fn test_lt() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();
                    let b: $type_ = rand::random();

                    // test equal

                    let program_string = include_str!("lt.leo");
                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("c", Some(InputValue::Boolean(false))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);

                    // test less or equal

                    let c = a.lt(&b);

                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                        ("c", Some(InputValue::Boolean(c))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);
                }
            }

            fn test_console_assert() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();

                    // test equal
                    let program_string = include_str!("console_assert.leo");
                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, a.to_string()))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);

                    // test not equal
                    let b: $type_ = rand::random();

                    if a == b {
                        continue;
                    }

                    let mut program = parse_program(program_string).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                    ]);

                    program.set_main_input(main_input);

                    expect_compiler_error(program);
                }
            }

            fn test_ternary() {
                let a: $type_ = rand::random();
                let b: $type_ = rand::random();

                let program_string = include_str!("ternary.leo");
                let mut program = parse_program(program_string).unwrap();

                // true -> field 1
                let main_input = generate_main_input(vec![
                    ("s", Some(InputValue::Boolean(true))),
                    ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                    ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                    ("c", Some(InputValue::Integer($integer_type, a.to_string()))),
                ]);

                program.set_main_input(main_input);

                assert_satisfied(program);

                // false -> field 2
                let mut program = parse_program(program_string).unwrap();

                let main_input = generate_main_input(vec![
                    ("s", Some(InputValue::Boolean(false))),
                    ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                    ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                    ("c", Some(InputValue::Integer($integer_type, b.to_string()))),
                ]);

                program.set_main_input(main_input);

                assert_satisfied(program);
            }
        }
    };
}
