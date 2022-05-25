// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_ast::*;
use leo_errors::TypeCheckerError;
use leo_span::Span;

use crate::TypeChecker;

fn return_incorrect_type(t1: Option<Type>, t2: Option<Type>, expected: Option<Type>) -> Option<Type> {
    match (t1, t2) {
        (Some(t1), Some(t2)) if t1 == t2 => Some(t1),
        (Some(t1), Some(t2)) => {
            if let Some(expected) = expected {
                if t1 != expected {
                    Some(t1)
                } else {
                    Some(t2)
                }
            } else {
                Some(t1)
            }
        }
        (None, Some(_)) | (Some(_), None) | (None, None) => None,
    }
}

impl<'a> TypeChecker<'a> {
    pub(crate) fn compare_expr_type(&mut self, expr: &Expression, expected: Option<Type>, span: Span) -> Option<Type> {
        match expr {
            Expression::Identifier(ident) => {
                if let Some(var) = self.symbol_table.lookup_variable(&ident.name) {
                    Some(self.assert_type(*var.type_, expected, span))
                } else {
                    self.handler
                        .emit_err(TypeCheckerError::unknown_sym("variable", ident.name, span).into());
                    None
                }
            }
            Expression::Value(value) => match value {
                ValueExpression::Address(_, _) => Some(self.assert_type(Type::Address, expected, value.span())),
                ValueExpression::Boolean(_, _) => Some(self.assert_type(Type::Boolean, expected, value.span())),
                ValueExpression::Field(_, _) => Some(self.assert_type(Type::Field, expected, value.span())),
                ValueExpression::Integer(type_, str_content, _) => {
                    match type_ {
                        IntegerType::I8 => {
                            let int = if self.negate {
                                self.negate = false;
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i8>().is_err() {
                                self.handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i8", value.span()).into());
                            }
                        }
                        IntegerType::I16 => {
                            let int = if self.negate {
                                self.negate = false;
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i16>().is_err() {
                                self.handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i16", value.span()).into());
                            }
                        }
                        IntegerType::I32 => {
                            let int = if self.negate {
                                self.negate = false;
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i32>().is_err() {
                                self.handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i32", value.span()).into());
                            }
                        }
                        IntegerType::I64 => {
                            let int = if self.negate {
                                self.negate = false;
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i64>().is_err() {
                                self.handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i64", value.span()).into());
                            }
                        }
                        IntegerType::I128 => {
                            let int = if self.negate {
                                self.negate = false;
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i128>().is_err() {
                                self.handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i128", value.span()).into());
                            }
                        }

                        IntegerType::U8 if str_content.parse::<u8>().is_err() => self
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u8", value.span()).into()),
                        IntegerType::U16 if str_content.parse::<u16>().is_err() => self
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u16", value.span()).into()),
                        IntegerType::U32 if str_content.parse::<u32>().is_err() => self
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u32", value.span()).into()),
                        IntegerType::U64 if str_content.parse::<u64>().is_err() => self
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u64", value.span()).into()),
                        IntegerType::U128 if str_content.parse::<u128>().is_err() => self
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u128", value.span()).into()),
                        _ => {}
                    }
                    Some(self.assert_type(Type::IntegerType(*type_), expected, value.span()))
                }
                ValueExpression::Group(_) => Some(self.assert_type(Type::Group, expected, value.span())),
                ValueExpression::Scalar(_, _) => Some(self.assert_type(Type::Scalar, expected, value.span())),
                ValueExpression::String(_, _) => unreachable!("String types are not reachable"),
            },
            Expression::Binary(binary) => match binary.op {
                BinaryOperation::And | BinaryOperation::Or => {
                    self.assert_type(Type::Boolean, expected, binary.span());
                    let t1 = self.compare_expr_type(&binary.left, expected, binary.left.span());
                    let t2 = self.compare_expr_type(&binary.right, expected, binary.right.span());

                    return_incorrect_type(t1, t2, expected)
                }
                BinaryOperation::Add => {
                    self.assert_field_group_scalar_int_type(expected, binary.span());
                    let t1 = self.compare_expr_type(&binary.left, expected, binary.left.span());
                    let t2 = self.compare_expr_type(&binary.right, expected, binary.right.span());

                    return_incorrect_type(t1, t2, expected)
                }
                BinaryOperation::Sub => {
                    self.assert_field_group_int_type(expected, binary.span());
                    let t1 = self.compare_expr_type(&binary.left, expected, binary.left.span());
                    let t2 = self.compare_expr_type(&binary.right, expected, binary.right.span());

                    return_incorrect_type(t1, t2, expected)
                }
                BinaryOperation::Mul => {
                    self.assert_field_group_int_type(expected, binary.span());

                    let t1 = self.compare_expr_type(&binary.left, None, binary.left.span());
                    let t2 = self.compare_expr_type(&binary.right, None, binary.right.span());

                    // Allow `group` * `scalar` multiplication.
                    match (t1.as_ref(), t2.as_ref()) {
                        (Some(Type::Group), Some(other)) | (Some(other), Some(Type::Group)) => {
                            self.assert_type(*other, Some(Type::Scalar), binary.span());
                            Some(Type::Group)
                        }
                        _ => return_incorrect_type(t1, t2, expected),
                    }
                }
                BinaryOperation::Div => {
                    self.assert_field_int_type(expected, binary.span());

                    let t1 = self.compare_expr_type(&binary.left, expected, binary.left.span());
                    let t2 = self.compare_expr_type(&binary.right, expected, binary.right.span());
                    return_incorrect_type(t1, t2, expected)
                }
                BinaryOperation::Pow => {
                    let t1 = self.compare_expr_type(&binary.left, None, binary.left.span());
                    let t2 = self.compare_expr_type(&binary.right, None, binary.right.span());

                    match (t1.as_ref(), t2.as_ref()) {
                        // Type A must be an int.
                        // Type B must be a unsigned int.
                        (Some(Type::IntegerType(_)), Some(Type::IntegerType(itype))) if !itype.is_signed() => {
                            self.assert_type(t1.unwrap(), expected, binary.span());
                        }
                        // Type A was an int.
                        // But Type B was not a unsigned int.
                        (Some(Type::IntegerType(_)), Some(t)) => {
                            self.handler.emit_err(
                                TypeCheckerError::incorrect_pow_exponent_type("unsigned int", t, binary.right.span())
                                    .into(),
                            );
                        }
                        // Type A must be a field.
                        // Type B must be an int.
                        (Some(Type::Field), Some(Type::IntegerType(_))) => {
                            self.assert_type(Type::Field, expected, binary.span());
                        }
                        // Type A was a field.
                        // But Type B was not an int.
                        (Some(Type::Field), Some(t)) => {
                            self.handler.emit_err(
                                TypeCheckerError::incorrect_pow_exponent_type("int", t, binary.right.span()).into(),
                            );
                        }
                        // The base is some type thats not an int or field.
                        (Some(t), _) => {
                            self.handler
                                .emit_err(TypeCheckerError::incorrect_pow_base_type(t, binary.left.span()).into());
                        }
                        _ => {}
                    }

                    t1
                }
                BinaryOperation::Eq | BinaryOperation::Ne => {
                    self.assert_type(Type::Boolean, expected, binary.span());

                    let t1 = self.compare_expr_type(&binary.left, None, binary.left.span());
                    let t2 = self.compare_expr_type(&binary.right, None, binary.right.span());

                    return_incorrect_type(t1, t2, expected)
                }
                BinaryOperation::Lt | BinaryOperation::Gt | BinaryOperation::Le | BinaryOperation::Ge => {
                    self.assert_type(Type::Boolean, expected, binary.span());

                    let t1 = self.compare_expr_type(&binary.left, None, binary.left.span());
                    self.assert_field_scalar_int_type(t1, binary.left.span());

                    let t2 = self.compare_expr_type(&binary.right, None, binary.right.span());
                    self.assert_field_scalar_int_type(t2, binary.right.span());

                    return_incorrect_type(t1, t2, expected)
                }
            },
            Expression::Unary(unary) => match unary.op {
                UnaryOperation::Not => {
                    self.assert_type(Type::Boolean, expected, unary.span());
                    self.compare_expr_type(&unary.inner, expected, unary.inner.span())
                }
                UnaryOperation::Negate => {
                    match expected.as_ref() {
                        Some(
                            Type::IntegerType(
                                IntegerType::I8
                                | IntegerType::I16
                                | IntegerType::I32
                                | IntegerType::I64
                                | IntegerType::I128,
                            )
                            | Type::Field
                            | Type::Group,
                        ) => self.negate = !self.negate,
                        Some(t) => self
                            .handler
                            .emit_err(TypeCheckerError::type_is_not_negatable(t, unary.inner.span()).into()),
                        _ => {}
                    };
                    self.compare_expr_type(&unary.inner, expected, unary.inner.span())
                }
            },
            Expression::Ternary(ternary) => {
                self.compare_expr_type(&ternary.condition, Some(Type::Boolean), ternary.condition.span());
                let t1 = self.compare_expr_type(&ternary.if_true, expected, ternary.if_true.span());
                let t2 = self.compare_expr_type(&ternary.if_false, expected, ternary.if_false.span());
                return_incorrect_type(t1, t2, expected)
            }
            Expression::Call(call) => match &*call.function {
                Expression::Identifier(ident) => {
                    if let Some(func) = self.symbol_table.lookup_fn(&ident.name) {
                        let ret = self.assert_type(func.output, expected, ident.span());

                        if func.input.len() != call.arguments.len() {
                            self.handler.emit_err(
                                TypeCheckerError::incorrect_num_args_to_call(
                                    func.input.len(),
                                    call.arguments.len(),
                                    call.span(),
                                )
                                .into(),
                            );
                        }

                        func.input
                            .iter()
                            .zip(call.arguments.iter())
                            .for_each(|(expected, argument)| {
                                self.compare_expr_type(argument, Some(expected.get_variable().type_), argument.span());
                            });

                        Some(ret)
                    } else {
                        self.handler
                            .emit_err(TypeCheckerError::unknown_sym("function", &ident.name, ident.span()).into());
                        None
                    }
                }
                expr => self.compare_expr_type(expr, expected, call.span()),
            },
            Expression::Err(_) => None,
        }
    }
}

impl<'a> ExpressionVisitor<'a> for TypeChecker<'a> {}
