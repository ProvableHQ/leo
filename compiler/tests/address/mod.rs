use crate::{
    boolean::{output_false, output_true},
    get_error,
    get_output,
    parse_program,
    EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_compiler::{Address, ConstrainedValue};
use leo_types::InputValue;

use snarkos_dpc::base_dpc::instantiated::Components;
use snarkos_objects::AccountPublicKey;
use std::str::FromStr;

static TEST_ADDRESS_1: &'static str = "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8";
static TEST_ADDRESS_2: &'static str = "aleo18qgam03qe483tdrcc3fkqwpp38ehff4a2xma6lu7hams6lfpgcpq3dq05r";

fn output_test_address(program: EdwardsTestCompiler, address: &str) {
    let output = get_output(program);

    let address_1 = AccountPublicKey::<Components>::from_str(address).unwrap();

    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::Address(Address(Some(address_1)))]).to_string(),
        output.to_string()
    );
}

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

    program.set_main_inputs(vec![Some(InputValue::Address(TEST_ADDRESS_1.to_string()))]);

    let _output = get_output(program);
}

#[test]
fn test_input_fail_bool() {
    let bytes = include_bytes!("input.leo");
    let mut program = parse_program(bytes).unwrap();

    program.set_main_inputs(vec![Some(InputValue::Boolean(true))]);

    let _err = get_error(program);
}

#[test]
fn test_ternary() {
    let bytes = include_bytes!("ternary.leo");
    let mut program_1 = parse_program(bytes).unwrap();
    let mut program_2 = program_1.clone();

    program_1.set_main_inputs(vec![Some(InputValue::Boolean(true))]);

    output_test_address(program_1, TEST_ADDRESS_1);

    program_2.set_main_inputs(vec![Some(InputValue::Boolean(false))]);

    output_test_address(program_2, TEST_ADDRESS_2);
}

#[test]
fn test_equal() {
    let bytes = include_bytes!("equal.leo");
    let mut program_1 = parse_program(bytes).unwrap();
    let mut program_2 = program_1.clone();

    program_1.set_main_inputs(vec![
        Some(InputValue::Address(TEST_ADDRESS_1.to_string())),
        Some(InputValue::Address(TEST_ADDRESS_1.to_string())),
    ]);

    output_true(program_1);

    program_2.set_main_inputs(vec![
        Some(InputValue::Address(TEST_ADDRESS_1.to_string())),
        Some(InputValue::Address(TEST_ADDRESS_2.to_string())),
    ]);

    output_false(program_2);
}
