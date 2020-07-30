use crate::{assert_satisfied, expect_compiler_error, parse_program, EdwardsTestCompiler};
use leo_compiler::errors::{CompilerError, ExpressionError, FunctionError, StatementError};
// use leo_compiler::{
//     errors::{CompilerError, ExpressionError, FunctionError, StatementError},
//     ConstrainedCircuitMember,
//     ConstrainedValue,
//     Integer,
// };
// use leo_types::{Expression, Function, Identifier, Span, Statement, Type};

// use snarkos_models::gadgets::utilities::uint::UInt32;
//
// // Foo { x: 1u32 }
// fn output_circuit(program: EdwardsTestCompiler) {
//     let output = get_output(program);
//     assert_eq!(
//         EdwardsConstrainedValue::Return(vec![ConstrainedValue::CircuitExpression(
//             Identifier {
//                 name: "Foo".to_string(),
//                 span: Span {
//                     text: "".to_string(),
//                     line: 0,
//                     start: 0,
//                     end: 0
//                 }
//             },
//             vec![ConstrainedCircuitMember(
//                 Identifier {
//                     name: "x".to_string(),
//                     span: Span {
//                         text: "".to_string(),
//                         line: 0,
//                         start: 0,
//                         end: 0
//                     }
//                 },
//                 ConstrainedValue::Integer(Integer::U32(UInt32::constant(1u32)))
//             )]
//         )])
//         .to_string(),
//         output.to_string()
//     );
// }
//
fn expect_fail(program: EdwardsTestCompiler) {
    match expect_compiler_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::Error(_string),
        ))) => {}
        error => panic!("Expected invalid circuit member error, got {}", error),
    }
}

// Expressions

#[test]
fn test_inline() {
    let bytes = include_bytes!("inline.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_inline_fail() {
    let bytes = include_bytes!("inline_fail.leo");
    let program = parse_program(bytes).unwrap();

    expect_fail(program);
}

#[test]
fn test_inline_undefined() {
    let bytes = include_bytes!("inline_undefined.leo");
    let program = parse_program(bytes).unwrap();

    expect_fail(program);
}

// Members

#[test]
fn test_member_field() {
    let bytes = include_bytes!("member_field.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_field_fail() {
    let bytes = include_bytes!("member_field_fail.leo");
    let program = parse_program(bytes).unwrap();

    expect_fail(program);
}

#[test]
fn test_member_field_and_function() {
    let bytes = include_bytes!("member_field_and_function.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_function() {
    let bytes = include_bytes!("member_function.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_function_fail() {
    let bytes = include_bytes!("member_function_fail.leo");
    let program = parse_program(bytes).unwrap();

    expect_fail(program);
}

#[test]
fn test_member_function_invalid() {
    let bytes = include_bytes!("member_function_invalid.leo");
    let program = parse_program(bytes).unwrap();

    expect_fail(program);
}

#[test]
fn test_member_function_nested() {
    let bytes = include_bytes!("member_function_nested.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_static_function() {
    let bytes = include_bytes!("member_static_function.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_member_static_function_invalid() {
    let bytes = include_bytes!("member_static_function_invalid.leo");
    let program = parse_program(bytes).unwrap();

    expect_fail(program)
}

#[test]
fn test_member_static_function_undefined() {
    let bytes = include_bytes!("member_static_function_undefined.leo");
    let program = parse_program(bytes).unwrap();

    expect_fail(program)
}

// Self
#[test]
fn test_self_member_pass() {
    let bytes = include_bytes!("self_member.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}

#[test]
fn test_self_member_invalid() {
    let bytes = include_bytes!("self_member_invalid.leo");
    let program = parse_program(bytes).unwrap();

    let _err = expect_compiler_error(program);
}

#[test]
fn test_self_member_undefined() {
    let bytes = include_bytes!("self_member_undefined.leo");
    let program = parse_program(bytes).unwrap();

    let _err = expect_compiler_error(program);
}

// All

#[test]
fn test_pedersen_mock() {
    let bytes = include_bytes!("pedersen_mock.leo");
    let program = parse_program(bytes).unwrap();

    assert_satisfied(program);
}
