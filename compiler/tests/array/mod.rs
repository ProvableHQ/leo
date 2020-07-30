use crate::{
    assert_satisfied,
    expect_compiler_error,
    get_outputs,
    parse_program,
    parse_program_with_inputs,
    EdwardsTestCompiler,
};

pub fn output_ones(program: EdwardsTestCompiler) {
    let expected = include_bytes!("outputs_/registers_ones.out");
    let actual = get_outputs(program);

    assert!(expected.eq(actual.bytes().as_slice()));
}

pub fn output_zeros(program: EdwardsTestCompiler) {
    let expected = include_bytes!("outputs_/registers_zeros.out");
    let actual = get_outputs(program);

    assert!(expected.eq(actual.bytes().as_slice()));
}

// Registers

#[test]
fn test_registers() {
    let program_bytes = include_bytes!("registers.leo");
    let ones_input_bytes = include_bytes!("inputs/registers_ones.in");
    let zeros_input_bytes = include_bytes!("inputs/registers_zeros.in");

    // test ones input register => ones output register
    let program = parse_program_with_inputs(program_bytes, ones_input_bytes).unwrap();

    output_ones(program);

    // test zeros input register => zeros output register
    let program = parse_program_with_inputs(program_bytes, zeros_input_bytes).unwrap();

    output_zeros(program);
}

// Expressions

#[test]
fn test_inline() {
    let program_bytes = include_bytes!("inline.leo");
    let input_bytes = include_bytes!("inputs/three_ones.in");
    let program = parse_program_with_inputs(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_inline_fail() {
    let program_bytes = include_bytes!("inline.leo");
    let program = parse_program(program_bytes).unwrap();

    let _err = expect_compiler_error(program);
}

#[test]
fn test_initializer() {
    let program_bytes = include_bytes!("initializer.leo");
    let input_bytes = include_bytes!("inputs/three_ones.in");
    let program = parse_program_with_inputs(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_spread() {
    let program_bytes = include_bytes!("spread.leo");
    let input_bytes = include_bytes!("inputs/three_ones.in");
    let program = parse_program_with_inputs(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_slice() {
    let program_bytes = include_bytes!("slice.leo");
    let input_bytes = include_bytes!("inputs/three_ones.in");
    let program = parse_program_with_inputs(program_bytes, input_bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_multi() {
    let program_bytes = include_bytes!("multi.leo");
    let program = parse_program(program_bytes).unwrap();

    assert_satisfied(program);
}
