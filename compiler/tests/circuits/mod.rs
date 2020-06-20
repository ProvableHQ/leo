use crate::{
    get_error,
    get_output,
    integers::u32::output_one,
    parse_program,
    EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_compiler::{
    errors::{CompilerError, ExpressionError, FunctionError, StatementError},
    ConstrainedCircuitMember,
    ConstrainedValue,
};
use leo_types::{Expression, Function, Identifier, Integer, Span, Statement, Type};

use snarkos_models::gadgets::utilities::uint::UInt32;

// Circ { x: 1u32 }
fn output_circuit(program: EdwardsTestCompiler) {
    let output = get_output(program);
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::CircuitExpression(
            Identifier {
                name: "Circ".to_string(),
                span: Span {
                    text: "".to_string(),
                    line: 0,
                    start: 0,
                    end: 0
                }
            },
            vec![ConstrainedCircuitMember(
                Identifier {
                    name: "x".to_string(),
                    span: Span {
                        text: "".to_string(),
                        line: 0,
                        start: 0,
                        end: 0
                    }
                },
                ConstrainedValue::Integer(Integer::U32(UInt32::constant(1u32)))
            )]
        )])
        .to_string(),
        output.to_string()
    );
}

fn fail_expected_member(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::Error(_string),
        ))) => {}
        error => panic!("Expected invalid circuit member error, got {}", error),
    }
}

fn fail_undefined_member(program: EdwardsTestCompiler) {
    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::Error(_),
        ))) => {}
        error => panic!("Expected undefined circuit member error, got {}", error),
    }
}

// Expressions

#[test]
fn test_inline() {
    let bytes = include_bytes!("inline.leo");
    let program = parse_program(bytes).unwrap();

    output_circuit(program);
}

#[test]
fn test_inline_fail() {
    let bytes = include_bytes!("inline_fail.leo");
    let program = parse_program(bytes).unwrap();

    fail_expected_member(program)
}

#[test]
fn test_inline_undefined() {
    let bytes = include_bytes!("inline_undefined.leo");
    let program = parse_program(bytes).unwrap();

    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::Error(_),
        ))) => {}
        error => panic!("Expected undefined circuit error, got {}", error),
    }
}

// Members

#[test]
fn test_member_field() {
    let bytes = include_bytes!("member_field.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}

#[test]
fn test_member_field_fail() {
    let bytes = include_bytes!("member_field_fail.leo");
    let program = parse_program(bytes).unwrap();

    fail_undefined_member(program);
}

#[test]
fn test_member_function() {
    let bytes = include_bytes!("member_function.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}

#[test]
fn test_member_function_fail() {
    let bytes = include_bytes!("member_function_fail.leo");
    let program = parse_program(bytes).unwrap();

    fail_undefined_member(program);
}

#[test]
fn test_member_function_invalid() {
    let bytes = include_bytes!("member_function_invalid.leo");
    let program = parse_program(bytes).unwrap();

    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::Error(_),
        ))) => {}
        error => panic!("Expected invalid function error, got {}", error),
    }
}

#[test]
fn test_member_static_function() {
    let bytes = include_bytes!("member_static_function.leo");
    let program = parse_program(bytes).unwrap();

    output_one(program);
}

#[test]
fn test_member_static_function_undefined() {
    let bytes = include_bytes!("member_static_function_undefined.leo");
    let program = parse_program(bytes).unwrap();

    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::Error(_),
        ))) => {}
        error => panic!("Expected undefined static function error, got {}", error),
    }
}

#[test]
fn test_member_static_function_invalid() {
    let bytes = include_bytes!("member_static_function_invalid.leo");
    let program = parse_program(bytes).unwrap();

    match get_error(program) {
        CompilerError::FunctionError(FunctionError::StatementError(StatementError::ExpressionError(
            ExpressionError::Error(_),
        ))) => {}
        error => panic!("Expected invalid static function error, got {}", error),
    }
}

// Self
#[test]
fn test_self() {
    let bytes = include_bytes!("self.leo");
    let program = parse_program(bytes).unwrap();

    let output = get_output(program);

    // circuit Circ {
    //   static function new() -> Self {
    //     return Self { }
    //   }
    // }
    assert_eq!(
        EdwardsConstrainedValue::Return(vec![ConstrainedValue::CircuitExpression(
            Identifier {
                name: "Circ".to_string(),
                span: Span {
                    text: "".to_string(),
                    line: 0,
                    start: 0,
                    end: 0
                }
            },
            vec![ConstrainedCircuitMember(
                Identifier {
                    name: "new".to_string(),
                    span: Span {
                        text: "".to_string(),
                        line: 0,
                        start: 0,
                        end: 0
                    }
                },
                ConstrainedValue::Static(Box::new(ConstrainedValue::Function(
                    Some(Identifier {
                        name: "new".to_string(),
                        span: Span {
                            text: "".to_string(),
                            line: 0,
                            start: 0,
                            end: 0
                        }
                    }),
                    Function {
                        function_name: Identifier {
                            name: "new".to_string(),
                            span: Span {
                                text: "".to_string(),
                                line: 0,
                                start: 0,
                                end: 0
                            }
                        },
                        inputs: vec![],
                        returns: vec![Type::SelfType],
                        statements: vec![Statement::Return(
                            vec![Expression::Circuit(
                                Identifier {
                                    name: "Self".to_string(),
                                    span: Span {
                                        text: "".to_string(),
                                        line: 0,
                                        start: 0,
                                        end: 0
                                    }
                                },
                                vec![],
                                Span {
                                    text: "".to_string(),
                                    line: 0,
                                    start: 0,
                                    end: 0
                                }
                            )],
                            Span {
                                text: "".to_string(),
                                line: 0,
                                start: 0,
                                end: 0
                            }
                        )],
                        span: Span {
                            text: "".to_string(),
                            line: 0,
                            start: 0,
                            end: 0
                        }
                    }
                )))
            )]
        )])
        .to_string(),
        output.to_string()
    );
}

// All

// #[test]
// fn test_pedersen_mock() {
// use crate::integers::u32::output_zero;
//     let bytes = include_bytes!("pedersen_mock.leo");
//     let program = parse_program(bytes).unwrap();
//
//     let err = get_error(program);
//
//     println!("{}", err);
//     // output_zero(program);
// }
