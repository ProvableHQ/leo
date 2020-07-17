macro_rules! test_int {
    ($name: ident, $type_: ty, $integer_type: expr, $gadget: ty) => {
        pub struct $name {}

        impl $name {
            fn test_min(min: $type_) {
                let min_allocated = <$gadget>::constant(min);

                let bytes = include_bytes!("min.leo");
                let program = parse_program(bytes).unwrap();

                output_expected_allocated(program, min_allocated);
            }

            fn test_max(max: $type_) {
                let max_allocated = <$gadget>::constant(max);

                let bytes = include_bytes!("max.leo");
                let program = parse_program(bytes).unwrap();

                output_expected_allocated(program, max_allocated);
            }
        }

        impl IntegerTester for $name {
            fn test_input() {
                // valid input
                let num: $type_ = rand::random();
                let expected = <$gadget>::constant(num);

                let bytes = include_bytes!("input.leo");
                let mut program = parse_program(bytes).unwrap();

                program.set_inputs(vec![Some(InputValue::Integer($integer_type, num.to_string()))]);

                output_expected_allocated(program, expected);

                // invalid input
                let mut program = parse_program(bytes).unwrap();

                program.set_inputs(vec![Some(InputValue::Boolean(true))]);
                fail_integer(program);

                // None input
                let mut program = parse_program(bytes).unwrap();
                program.set_inputs(vec![None]);
                fail_integer(program);
            }

            fn test_add() {
                for _ in 0..10 {
                    let r1: $type_ = rand::random();
                    let r2: $type_ = rand::random();

                    let sum = match r1.checked_add(r2) {
                        Some(valid) => valid,
                        None => continue,
                    };

                    let cs = TestConstraintSystem::<Fq>::new();
                    let sum_allocated = <$gadget>::alloc(cs, || Ok(sum)).unwrap();

                    let bytes = include_bytes!("add.leo");
                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r2.to_string())),
                    ]);

                    output_expected_allocated(program, sum_allocated);
                }
            }

            fn test_sub() {
                for _ in 0..10 {
                    let r1: $type_ = rand::random();
                    let r2: $type_ = rand::random();

                    if r2.checked_neg().is_none() {
                        continue;
                    }

                    let difference = match r1.checked_sub(r2) {
                        Some(valid) => valid,
                        None => continue,
                    };

                    let cs = TestConstraintSystem::<Fq>::new();
                    let difference_allocated = <$gadget>::alloc(cs, || Ok(difference)).unwrap();

                    let bytes = include_bytes!("sub.leo");
                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r2.to_string())),
                    ]);

                    output_expected_allocated(program, difference_allocated);
                }
            }

            fn test_mul() {
                for _ in 0..10 {
                    let r1: $type_ = rand::random();
                    let r2: $type_ = rand::random();

                    let product = match r1.checked_mul(r2) {
                        Some(valid) => valid,
                        None => continue,
                    };

                    let cs = TestConstraintSystem::<Fq>::new();
                    let product_allocated = <$gadget>::alloc(cs, || Ok(product)).unwrap();

                    let bytes = include_bytes!("mul.leo");
                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r2.to_string())),
                    ]);

                    output_expected_allocated(program, product_allocated);
                }
            }

            fn test_div() {
                // for _ in 0..10 {// these loops take an excessive amount of time
                let r1: $type_ = rand::random();
                let r2: $type_ = rand::random();

                let bytes = include_bytes!("div.leo");
                let mut program = parse_program(bytes).unwrap();

                program.set_inputs(vec![
                    Some(InputValue::Integer($integer_type, r1.to_string())),
                    Some(InputValue::Integer($integer_type, r2.to_string())),
                ]);

                // expect an error when dividing by zero
                if r2 == 0 {
                    let _err = get_error(program);
                } else {
                    let cs = TestConstraintSystem::<Fq>::new();

                    let quotient = match r1.checked_div(r2) {
                        Some(valid) => valid,
                        None => return,
                    };
                    let quotient_allocated = <$gadget>::alloc(cs, || Ok(quotient)).unwrap();

                    output_expected_allocated(program, quotient_allocated);
                }
                // }
            }

            fn test_pow() {
                // for _ in 0..10 {// these loops take an excessive amount of time
                let r1: $type_ = rand::random();
                let r2: $type_ = rand::random();
                let r2 = r2 as u32; // we cast to u32 here because of rust pow() requirements

                let result = match r1.checked_pow(r2) {
                    Some(valid) => valid,
                    None => return,
                };

                let cs = TestConstraintSystem::<Fq>::new();
                let result_allocated = <$gadget>::alloc(cs, || Ok(result)).unwrap();

                let bytes = include_bytes!("pow.leo");
                let mut program = parse_program(bytes).unwrap();

                program.set_inputs(vec![
                    Some(InputValue::Integer($integer_type, r1.to_string())),
                    Some(InputValue::Integer($integer_type, r2.to_string())),
                ]);

                output_expected_allocated(program, result_allocated);
                // }
            }

            fn test_eq() {
                for _ in 0..10 {
                    let r1: $type_ = rand::random();

                    // test equal
                    let bytes = include_bytes!("eq.leo");
                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                    ]);

                    output_true(program);

                    // test not equal
                    let r2: $type_ = rand::random();

                    let result = r1.eq(&r2);

                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r2.to_string())),
                    ]);

                    output_expected_boolean(program, result);
                }
            }

            fn test_ge() {
                for _ in 0..10 {
                    let r1: $type_ = rand::random();

                    // test equal
                    let bytes = include_bytes!("ge.leo");
                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                    ]);

                    output_true(program);

                    // test not equal
                    let r2: $type_ = rand::random();

                    let result = r1.ge(&r2);

                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r2.to_string())),
                    ]);

                    output_expected_boolean(program, result);
                }
            }

            fn test_gt() {
                for _ in 0..10 {
                    let r1: $type_ = rand::random();

                    // test equal
                    let bytes = include_bytes!("gt.leo");
                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                    ]);

                    output_false(program);

                    // test not equal
                    let r2: $type_ = rand::random();

                    let result = r1.gt(&r2);

                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r2.to_string())),
                    ]);

                    output_expected_boolean(program, result);
                }
            }

            fn test_le() {
                for _ in 0..10 {
                    let r1: $type_ = rand::random();

                    // test equal
                    let bytes = include_bytes!("le.leo");
                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                    ]);

                    output_true(program);

                    // test not equal
                    let r2: $type_ = rand::random();

                    let result = r1.le(&r2);

                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r2.to_string())),
                    ]);

                    output_expected_boolean(program, result);
                }
            }

            fn test_lt() {
                for _ in 0..10 {
                    let r1: $type_ = rand::random();

                    // test equal
                    let bytes = include_bytes!("lt.leo");
                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                    ]);

                    output_false(program);

                    // test not equal
                    let r2: $type_ = rand::random();

                    let result = r1.lt(&r2);

                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r2.to_string())),
                    ]);

                    output_expected_boolean(program, result);
                }
            }

            fn test_assert_eq() {
                for _ in 0..10 {
                    let r1: $type_ = rand::random();

                    // test equal
                    let bytes = include_bytes!("assert_eq.leo");
                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                    ]);

                    let _ = get_output(program);

                    // test not equal
                    let r2: $type_ = rand::random();

                    if r1 == r2 {
                        continue;
                    }

                    let mut program = parse_program(bytes).unwrap();

                    program.set_inputs(vec![
                        Some(InputValue::Integer($integer_type, r1.to_string())),
                        Some(InputValue::Integer($integer_type, r2.to_string())),
                    ]);

                    let mut cs = TestConstraintSystem::<Fq>::new();
                    let _ = program.compile_constraints(&mut cs).unwrap();
                    assert!(!cs.is_satisfied());
                }
            }

            fn test_ternary() {
                let r1: $type_ = rand::random();
                let r2: $type_ = rand::random();

                let g1 = <$gadget>::constant(r1);
                let g2 = <$gadget>::constant(r2);

                let bytes = include_bytes!("ternary.leo");
                let mut program_1 = parse_program(bytes).unwrap();

                let mut program_2 = program_1.clone();

                // true -> field 1
                program_1.set_inputs(vec![
                    Some(InputValue::Boolean(true)),
                    Some(InputValue::Integer($integer_type, r1.to_string())),
                    Some(InputValue::Integer($integer_type, r2.to_string())),
                ]);

                output_expected_allocated(program_1, g1);

                // false -> field 2
                program_2.set_inputs(vec![
                    Some(InputValue::Boolean(false)),
                    Some(InputValue::Integer($integer_type, r1.to_string())),
                    Some(InputValue::Integer($integer_type, r2.to_string())),
                ]);

                output_expected_allocated(program_2, g2);
            }
        }
    };
}
