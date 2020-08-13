use crate::{assert_satisfied, parse_input_and_state, parse_program_with_input_and_state};

#[test]
fn test_basic() {
    let input_bytes = include_bytes!("input/basic.in");
    let state_bytes = include_bytes!("input/basic.state");

    parse_input_and_state(input_bytes, state_bytes).unwrap();
}

#[test]
fn test_full() {
    let input_bytes = include_bytes!("input/token_withdraw.in");
    let state_bytes = include_bytes!("input/token_withdraw.state");

    parse_input_and_state(input_bytes, state_bytes).unwrap();
}

#[test]
fn test_access() {
    let program_bytes = include_bytes!("access.leo");
    let input_bytes = include_bytes!("input/token_withdraw.in");
    let state_bytes = include_bytes!("input/token_withdraw.state");

    let program = parse_program_with_input_and_state(program_bytes, input_bytes, state_bytes).unwrap();

    assert_satisfied(program);
}
