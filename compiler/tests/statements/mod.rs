use crate::{assert_satisfied, expect_compiler_error, expect_synthesis_error, generate_main_inputs, parse_program};
use leo_types::InputValue;

pub mod conditional;

// Ternary if {bool}? {expression} : {expression};

#[test]
fn test_ternary_basic() {
    let bytes = include_bytes!("ternary_basic.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_inputs = generate_main_inputs(vec![
        ("a", Some(InputValue::Boolean(true))),
        ("b", Some(InputValue::Boolean(true))),
    ]);

    program.set_main_inputs(main_inputs);

    assert_satisfied(program);

    let mut program = parse_program(bytes).unwrap();

    let main_inputs = generate_main_inputs(vec![
        ("a", Some(InputValue::Boolean(false))),
        ("b", Some(InputValue::Boolean(false))),
    ]);

    program.set_main_inputs(main_inputs);

    assert_satisfied(program);
}

// Iteration for i {start}..{stop} { statements }

#[test]
fn test_iteration_basic() {
    let bytes = include_bytes!("iteration_basic.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

// Assertion

#[test]
fn test_assertion_basic() {
    let bytes = include_bytes!("assertion_basic.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_inputs = generate_main_inputs(vec![("a", Some(InputValue::Boolean(true)))]);

    program.set_main_inputs(main_inputs);

    assert_satisfied(program);

    let mut program = parse_program(bytes).unwrap();

    let main_inputs = generate_main_inputs(vec![("a", Some(InputValue::Boolean(false)))]);

    program.set_main_inputs(main_inputs);

    expect_synthesis_error(program);
}

#[test]
fn test_num_returns_fail() {
    let bytes = include_bytes!("num_returns_fail.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}
