use crate::{assert_satisfied, parse_program_with_state, parse_state};

#[test]
fn test_basic() {
    let bytes = include_bytes!("inputs/basic.state");

    parse_state(bytes).unwrap();
}

#[test]
fn test_token_withdraw() {
    let bytes = include_bytes!("inputs/token_withdraw.state");

    parse_state(bytes).unwrap();
}

#[test]
fn test_access_state() {
    let program_bytes = include_bytes!("access_state.leo");
    let state_bytes = include_bytes!("inputs/token_withdraw.state");

    let program = parse_program_with_state(program_bytes, state_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_access_all() {
    let program_bytes = include_bytes!("access_all.leo");
    let state_bytes = include_bytes!("inputs/token_withdraw.state");

    let program = parse_program_with_state(program_bytes, state_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_visibility_fail() {
    let state_bytes = include_bytes!("inputs/visibility_fail.state");

    let is_err = parse_state(state_bytes).is_err();

    assert!(is_err);
}

#[test]
fn test_section_undefined() {
    let state_bytes = include_bytes!("inputs/section_undefined.state");

    let is_err = parse_state(state_bytes).is_err();

    assert!(is_err);
}

#[test]
fn test_section_invalid() {
    let state_bytes = include_bytes!("inputs/section_invalid.state");

    let is_err = parse_state(state_bytes).is_err();

    assert!(is_err);
}
