use crate::{get_error, get_output, parse_program};

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
