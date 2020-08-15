macro_rules! test_uint {
    ($name: ident, $type_: ty, $integer_type: expr, $gadget: ty) => {
        pub struct $name {}

        impl IntegerTester for $name {
            fn test_min() {
                let bytes = include_bytes!("min.leo");
                let program = parse_program(bytes).unwrap();

                assert_satisfied(program);
            }

            fn test_min_fail() {
                let bytes = include_bytes!("min_fail.leo");
                let program = parse_program(bytes).unwrap();

                expect_parsing_error(program);
            }

            fn test_max() {
                let bytes = include_bytes!("max.leo");
                let program = parse_program(bytes).unwrap();

                assert_satisfied(program);
            }

            fn test_max_fail() {
                let bytes = include_bytes!("max_fail.leo");
                let program = parse_program(bytes).unwrap();

                expect_parsing_error(program);
            }

            fn test_add() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();
                    let b: $type_ = rand::random();

                    let c = match a.checked_add(b) {
                        Some(valid) => valid,
                        None => continue,
                    };

                    let bytes = include_bytes!("add.leo");
                    let mut program = parse_program(bytes).unwrap();

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

                    let c = match a.checked_sub(b) {
                        Some(valid) => valid,
                        None => continue,
                    };

                    let bytes = include_bytes!("sub.leo");
                    let mut program = parse_program(bytes).unwrap();

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

                    let bytes = include_bytes!("mul.leo");
                    let mut program = parse_program(bytes).unwrap();

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

                    let c = match a.checked_div(b) {
                        Some(valid) => valid,
                        None => continue,
                    };

                    let bytes = include_bytes!("div.leo");
                    let mut program = parse_program(bytes).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                        ("c", Some(InputValue::Integer($integer_type, c.to_string()))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);
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

                    let bytes = include_bytes!("pow.leo");
                    let mut program = parse_program(bytes).unwrap();

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

                    let bytes = include_bytes!("eq.leo");
                    let mut program = parse_program(bytes).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("c", Some(InputValue::Boolean(true))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);

                    // test not equal

                    let c = a.eq(&b);

                    let mut program = parse_program(bytes).unwrap();

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

                    let bytes = include_bytes!("ne.leo");
                    let mut program = parse_program(bytes).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("c", Some(InputValue::Boolean(false))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);

                    // test not equal

                    let c = a.ne(&b);

                    let mut program = parse_program(bytes).unwrap();

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

                    let bytes = include_bytes!("ge.leo");
                    let mut program = parse_program(bytes).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("c", Some(InputValue::Boolean(true))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);

                    // test greater or equal

                    let c = a.ge(&b);

                    let mut program = parse_program(bytes).unwrap();

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

                    let bytes = include_bytes!("gt.leo");
                    let mut program = parse_program(bytes).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("c", Some(InputValue::Boolean(false))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);

                    // test greater than

                    let c = a.gt(&b);

                    let mut program = parse_program(bytes).unwrap();

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

                    let bytes = include_bytes!("le.leo");
                    let mut program = parse_program(bytes).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("c", Some(InputValue::Boolean(true))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);

                    // test less or equal

                    let c = a.le(&b);

                    let mut program = parse_program(bytes).unwrap();

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

                    let bytes = include_bytes!("lt.leo");
                    let mut program = parse_program(bytes).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("c", Some(InputValue::Boolean(false))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);

                    // test less or equal

                    let c = a.lt(&b);

                    let mut program = parse_program(bytes).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                        ("c", Some(InputValue::Boolean(c))),
                    ]);

                    program.set_main_input(main_input);

                    assert_satisfied(program);
                }
            }

            fn test_assert_eq() {
                for _ in 0..10 {
                    let a: $type_ = rand::random();

                    // test equal
                    let bytes = include_bytes!("assert_eq.leo");
                    let mut program = parse_program(bytes).unwrap();

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

                    let mut program = parse_program(bytes).unwrap();

                    let main_input = generate_main_input(vec![
                        ("a", Some(InputValue::Integer($integer_type, a.to_string()))),
                        ("b", Some(InputValue::Integer($integer_type, b.to_string()))),
                    ]);

                    program.set_main_input(main_input);

                    expect_synthesis_error(program);
                }
            }

            fn test_ternary() {
                let a: $type_ = rand::random();
                let b: $type_ = rand::random();

                let bytes = include_bytes!("ternary.leo");
                let mut program = parse_program(bytes).unwrap();

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
                let mut program = parse_program(bytes).unwrap();

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
