use crate::{assert_satisfied, expect_compiler_error, generate_main_input, parse_program};
use leo_types::InputValue;

#[test]
fn test_let() {
    let bytes = include_bytes!("let.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_let_mut() {
    let bytes = include_bytes!("let_mut.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_const_fail() {
    let bytes = include_bytes!("const.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_const_mut_fail() {
    let bytes = include_bytes!("const_mut.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_array() {
    let bytes = include_bytes!("array.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_array_mut() {
    let bytes = include_bytes!("array_mut.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_circuit() {
    let bytes = include_bytes!("circuit.leo");
    let program = parse_program(bytes).unwrap();

    expect_compiler_error(program);
}

#[test]
fn test_circuit_mut() {
    let bytes = include_bytes!("circuit_mut.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_function_input() {
    let bytes = include_bytes!("function_input.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_inputs = generate_main_input(vec![("a", Some(InputValue::Boolean(true)))]);

    program.set_main_input(main_inputs);

    expect_compiler_error(program);
}

#[test]
fn test_function_input_mut() {
    let bytes = include_bytes!("function_input_mut.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_inputs = generate_main_input(vec![("a", Some(InputValue::Boolean(true)))]);

    program.set_main_input(main_inputs);

    assert_satisfied(program);
}
