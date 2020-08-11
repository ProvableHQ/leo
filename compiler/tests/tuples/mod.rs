use crate::{assert_satisfied, parse_program};

#[test]
fn test_tuple_basic() {
    let program_bytes = include_bytes!("basic.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_tuple_access() {
    let program_bytes = include_bytes!("access.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_tuple_typed() {
    let program_bytes = include_bytes!("typed.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_multiple() {
    let program_bytes = include_bytes!("multiple.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_multiple_typed() {
    let program_bytes = include_bytes!("multiple_typed.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_function() {
    let program_bytes = include_bytes!("function.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_function_typed() {
    let program_bytes = include_bytes!("function_typed.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_function_multiple() {
    let progam_bytes = include_bytes!("function_multiple.leo");

    let program = parse_program(progam_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_nested() {
    let program_bytes = include_bytes!("nested.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_nested_access() {
    let program_bytes = include_bytes!("nested_access.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_nested_typed() {
    let program_bytes = include_bytes!("nested_typed.leo");

    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}

// #[test]
// fn test_input() {
//     let input_bytes = include_bytes!("inputs/input.in");
//     let program_bytes = include_bytes!("")
// }
