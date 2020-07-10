use crate::{get_error, get_output, parse_program};
use leo_types::InputValue;

#[test]
fn test_print() {
    let bytes = include_bytes!("print.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_output(program);
}

#[test]
fn test_print_fail() {
    let bytes = include_bytes!("print_fail.leo");

    assert!(parse_program(bytes).is_err());
}

#[test]
fn test_print_parameter() {
    let bytes = include_bytes!("print_parameter.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_output(program);
}

#[test]
fn test_print_parameter_many() {
    let bytes = include_bytes!("print_parameter_many.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_output(program);
}

#[test]
fn test_print_parameter_fail_unknown() {
    let bytes = include_bytes!("print_parameter_fail_unknown.leo");
    let program = parse_program(bytes).unwrap();

    let _err = get_error(program);
}

#[test]
fn test_print_parameter_fail_empty() {
    let bytes = include_bytes!("print_parameter_fail_empty.leo");
    let program = parse_program(bytes).unwrap();

    let _err = get_error(program);
}

#[test]
fn test_print_parameter_fail_none() {
    let bytes = include_bytes!("print_parameter_fail_empty.leo");
    let program = parse_program(bytes).unwrap();

    let _err = get_error(program);
}

#[test]
fn test_print_input() {
    let bytes = include_bytes!("print_input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_inputs(vec![Some(InputValue::Boolean(true))]);

    let _output = get_output(program);
}

#[test]
fn test_debug() {
    let bytes = include_bytes!("debug.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_output(program);
}
#[test]
fn test_error() {
    let bytes = include_bytes!("error.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_output(program);
}
