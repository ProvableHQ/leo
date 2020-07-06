use crate::{get_error, get_output, parse_program};
use leo_types::InputValue;

#[test]
fn test_valid() {
    let bytes = include_bytes!("valid.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_output(program);
}

#[test]
fn test_invalid() {
    let bytes = include_bytes!("invalid.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_error(program);
}

#[test]
fn test_implicit_valid() {
    let bytes = include_bytes!("implicit_valid.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_output(program);
}

#[test]
fn test_implicit_invalid() {
    let bytes = include_bytes!("implicit_invalid.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_error(program);
}

#[test]
fn test_assert_eq_pass() {
    let bytes = include_bytes!("assert_eq_pass.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_output(program);
}

#[test]
fn test_assert_eq_fail() {
    let bytes = include_bytes!("assert_eq_fail.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_error(program);
}

#[test]
fn test_input_pass() {
    let bytes = include_bytes!("input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![Some(InputValue::Address(
        "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8".to_string(),
    ))]);

    let _output = get_output(program);
}

#[test]
fn test_input_fail_bool() {
    let bytes = include_bytes!("input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![Some(InputValue::Boolean(true))]);

    let _err = get_error(program);
}
