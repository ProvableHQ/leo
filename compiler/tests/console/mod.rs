use crate::{assert_satisfied, expect_compiler_error, generate_main_input, parse_program};
use leo_typed::InputValue;

#[test]
fn test_log() {
    let bytes = include_bytes!("log.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_log_fail() {
    let bytes = include_bytes!("log_fail.leo");

    assert!(parse_program(bytes).is_err());
}

#[test]
fn test_log_parameter() {
    let bytes = include_bytes!("log_parameter.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_log_parameter_many() {
    let bytes = include_bytes!("log_parameter_many.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_log_parameter_fail_unknown() {
    let bytes = include_bytes!("log_parameter_fail_unknown.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_log_parameter_fail_empty() {
    let bytes = include_bytes!("log_parameter_fail_empty.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_log_parameter_fail_none() {
    let bytes = include_bytes!("log_parameter_fail_empty.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_log_input() {
    let bytes = include_bytes!("log_input.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(true)))]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}

// Debug

#[test]
fn test_debug() {
    let bytes = include_bytes!("debug.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

// Error

#[test]
fn test_error() {
    let bytes = include_bytes!("error.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

// Assertion

#[test]
fn test_assert() {
    let bytes = include_bytes!("assert.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(true)))]);

    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(false)))]);

    program.set_main_input(main_input);

    expect_compiler_error(program);
}

#[test]
fn test_conditional_assert() {
    let bytes = include_bytes!("conditional_assert.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(true)))]);
    program.set_main_input(main_input);

    assert_satisfied(program);

    let mut program = parse_program(bytes).unwrap();

    let main_input = generate_main_input(vec![("a", Some(InputValue::Boolean(false)))]);

    program.set_main_input(main_input);

    assert_satisfied(program);
}
