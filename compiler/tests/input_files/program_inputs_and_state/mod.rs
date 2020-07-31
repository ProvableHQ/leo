use crate::{assert_satisfied, parse_inputs_and_state, parse_program_with_inputs_and_state};

#[test]
fn test_basic() {
    let inputs_bytes = include_bytes!("inputs/basic.in");
    let state_bytes = include_bytes!("inputs/basic.state");

    parse_inputs_and_state(inputs_bytes, state_bytes).unwrap();
}

#[test]
fn test_full() {
    let inputs_bytes = include_bytes!("inputs/token_withdraw.in");
    let state_bytes = include_bytes!("inputs/token_withdraw.state");

    parse_inputs_and_state(inputs_bytes, state_bytes).unwrap();
}

#[test]
fn test_access() {
    let program_bytes = include_bytes!("access.leo");
    let inputs_bytes = include_bytes!("inputs/token_withdraw.in");
    let state_bytes = include_bytes!("inputs/token_withdraw.state");

    let program = parse_program_with_inputs_and_state(program_bytes, inputs_bytes, state_bytes).unwrap();

    assert_satisfied(program);
}
