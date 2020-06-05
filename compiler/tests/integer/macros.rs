macro_rules! test_uint {
    ($name: ident, $_type: ty, $gadget: ty, $directory: expr) => {
        pub struct $name {}

        impl $name {
            fn test_min(min: $_type) {
                let min_allocated = <$gadget>::constant(min);

                let program = compile_program($directory, "min.leo").unwrap();

                output_expected_allocated(program, min_allocated);
            }

            fn test_max(max: $_type) {
                let max_allocated = <$gadget>::constant(max);

                let program = compile_program($directory, "max.leo").unwrap();

                output_expected_allocated(program, max_allocated);
            }
        }

        impl IntegerTester for $name {
            fn test_input() {
                // valid input
                let num: $_type = rand::random();
                let expected = <$gadget>::constant(num);

                let mut program = compile_program($directory, "input.leo").unwrap();
                program.set_inputs(vec![Some(InputValue::Integer(num as usize))]);

                output_expected_allocated(program, expected);

                // invalid input
                let mut program = compile_program($directory, "input.leo").unwrap();
                program.set_inputs(vec![Some(InputValue::Boolean(true))]);
                fail_integer(program);

                // None input
                let mut program = compile_program($directory, "input.leo").unwrap();
                program.set_inputs(vec![None]);
                fail_synthesis(program);
            }

            fn test_add() {
                for _ in 0..10 {
                    let r1: $_type = rand::random();
                    let r2: $_type = rand::random();

                    let sum = r1.wrapping_add(r2);

                    let cs = TestConstraintSystem::<Fq>::new();
                    let sum_allocated = <$gadget>::alloc(cs, || Ok(sum)).unwrap();

                    let mut program = compile_program($directory, "add.leo").unwrap();
                    program.set_inputs(vec![
                        Some(InputValue::Integer(r1 as usize)),
                        Some(InputValue::Integer(r2 as usize)),
                    ]);

                    output_expected_allocated(program, sum_allocated);
                }
            }

            fn test_sub() {
                for _ in 0..10 {
                    let r1: $_type = rand::random();
                    let r2: $_type = rand::random();

                    let sum = r1.wrapping_sub(r2);

                    let cs = TestConstraintSystem::<Fq>::new();
                    let sum_allocated = <$gadget>::alloc(cs, || Ok(sum)).unwrap();

                    let mut program = compile_program($directory, "sub.leo").unwrap();
                    program.set_inputs(vec![
                        Some(InputValue::Integer(r1 as usize)),
                        Some(InputValue::Integer(r2 as usize)),
                    ]);

                    output_expected_allocated(program, sum_allocated);
                }
            }

            fn test_mul() {
                for _ in 0..10 {
                    let r1: $_type = rand::random();
                    let r2: $_type = rand::random();

                    let sum = r1.wrapping_mul(r2);

                    let cs = TestConstraintSystem::<Fq>::new();
                    let sum_allocated = <$gadget>::alloc(cs, || Ok(sum)).unwrap();

                    let mut program = compile_program($directory, "mul.leo").unwrap();
                    program.set_inputs(vec![
                        Some(InputValue::Integer(r1 as usize)),
                        Some(InputValue::Integer(r2 as usize)),
                    ]);

                    output_expected_allocated(program, sum_allocated);
                }
            }

            fn test_div() {
                for _ in 0..10 {
                    let r1: $_type = rand::random();
                    let r2: $_type = rand::random();

                    let sum = r1.wrapping_div(r2);

                    let cs = TestConstraintSystem::<Fq>::new();
                    let sum_allocated = <$gadget>::alloc(cs, || Ok(sum)).unwrap();

                    let mut program = compile_program($directory, "div.leo").unwrap();
                    program.set_inputs(vec![
                        Some(InputValue::Integer(r1 as usize)),
                        Some(InputValue::Integer(r2 as usize)),
                    ]);

                    output_expected_allocated(program, sum_allocated);
                }
            }

            fn test_pow() {
                for _ in 0..10 {
                    let r1: $_type = rand::random();
                    let r2: $_type = rand::random();

                    let sum = r1.wrapping_pow(r2);

                    let cs = TestConstraintSystem::<Fq>::new();
                    let sum_allocated = <$gadget>::alloc(cs, || Ok(sum)).unwrap();

                    let mut program = compile_program($directory, "pow.leo").unwrap();
                    program.set_inputs(vec![
                        Some(InputValue::Integer(r1 as usize)),
                        Some(InputValue::Integer(r2 as usize)),
                    ]);

                    output_expected_allocated(program, sum_allocated);
                }
            }
        }
    };
}
