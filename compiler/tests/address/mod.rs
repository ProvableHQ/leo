use crate::{assert_satisfied, generate_main_inputs, get_compiler_error, parse_program};
use leo_types::InputValue;

static TEST_ADDRESS_1: &'static str = "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8";
static TEST_ADDRESS_2: &'static str = "aleo18qgam03qe483tdrcc3fkqwpp38ehff4a2xma6lu7hams6lfpgcpq3dq05r";

#[test]
fn test_valid() {
    let bytes = include_bytes!("valid.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program)
}

#[test]
fn test_invalid() {
    let bytes = include_bytes!("invalid.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_compiler_error(program);
}

#[test]
fn test_implicit_valid() {
    let bytes = include_bytes!("implicit_valid.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_implicit_invalid() {
    let bytes = include_bytes!("implicit_invalid.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_compiler_error(program);
}

#[test]
fn test_assert_eq_pass() {
    let bytes = include_bytes!("assert_eq_pass.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_assert_eq_fail() {
    let bytes = include_bytes!("assert_eq_fail.leo");
    let program = parse_program(bytes).unwrap();

    let _output = get_compiler_error(program);
}

#[test]
fn test_ternary() {
    let bytes = include_bytes!("ternary.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_inputs = generate_main_inputs(vec![
        ("s", Some(InputValue::Boolean(true))),
        ("c", Some(InputValue::Address(TEST_ADDRESS_1.to_string()))),
    ]);

    program.set_main_inputs(main_inputs);

    assert_satisfied(program);

    let mut program = parse_program(bytes).unwrap();

    let main_inputs = generate_main_inputs(vec![
        ("s", Some(InputValue::Boolean(false))),
        ("c", Some(InputValue::Address(TEST_ADDRESS_2.to_string()))),
    ]);

    program.set_main_inputs(main_inputs);

    assert_satisfied(program);
}

#[test]
fn test_equal() {
    let bytes = include_bytes!("equal.leo");
    let mut program = parse_program(bytes).unwrap();

    let main_inputs = generate_main_inputs(vec![
        ("a", Some(InputValue::Address(TEST_ADDRESS_1.to_string()))),
        ("b", Some(InputValue::Address(TEST_ADDRESS_1.to_string()))),
        ("c", Some(InputValue::Boolean(true))),
    ]);

    program.set_main_inputs(main_inputs);

    assert_satisfied(program);

    let mut program = parse_program(bytes).unwrap();

    let main_inputs = generate_main_inputs(vec![
        ("a", Some(InputValue::Address(TEST_ADDRESS_1.to_string()))),
        ("b", Some(InputValue::Address(TEST_ADDRESS_2.to_string()))),
        ("c", Some(InputValue::Boolean(false))),
    ]);

    program.set_main_inputs(main_inputs);

    assert_satisfied(program);
}
