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

impl<'a> TypeChecker<'a> {
    pub(crate) fn compare_expr_type(&self, expr: &Expression, expected: Type, span: &Span) {
        match expr {
            Expression::Identifier(ident) => {
                if let Some(var) = self.symbol_table.lookup_variable(&ident.name) {
                    self.assert_type(var.type_.clone(), expected, var.span);
                } else {
                    self.handler
                        .emit_err(TypeCheckerError::unknown_sym("variable", ident.name, span).into());
                }
            }
            Expression::Value(value) => match value {
                ValueExpression::Address(_, _) => self.assert_type(Type::Address, expected, value.span()),
                ValueExpression::Boolean(_, _) => self.assert_type(Type::Boolean, expected, value.span()),
                ValueExpression::Char(_) => self.assert_type(Type::Char, expected, value.span()),
                ValueExpression::Field(_, _) => self.assert_type(Type::Field, expected, value.span()),
                ValueExpression::Integer(type_, _, _) => {
                    self.assert_type(Type::IntegerType(*type_), expected, value.span())
                }
                ValueExpression::Group(_) => self.assert_type(Type::Group, expected, value.span()),
                ValueExpression::String(_, _) => {}
            },
            Expression::Binary(binary) => match binary.op.class() {
                // some ops support more types than listed here
                BinaryOperationClass::Boolean => {
                    self.assert_type(Type::Boolean, expected, span);
                    self.compare_expr_type(&binary.left, Type::Boolean, binary.span());
                    self.compare_expr_type(&binary.right, Type::Boolean, binary.span());
                }
                BinaryOperationClass::Numeric => {
                    // depending on operation could also be field or group
                    if !matches!(expected, Type::IntegerType(_)) {
                        self.handler
                            .emit_err(TypeCheckerError::type_should_be_integer(binary.op, expected.clone(), span).into());
                    }

                    self.compare_expr_type(&binary.left, expected.clone(), binary.span());
                    self.compare_expr_type(&binary.right, expected, binary.span());
                }
            },
            Expression::Unary(unary) => match unary.op {
                UnaryOperation::Not => {
                    self.assert_type(Type::Boolean, expected, unary.span());
                    self.compare_expr_type(&unary.inner, Type::Boolean, unary.inner.span());
                }
                UnaryOperation::Negate => {
                    match expected {
                        Type::IntegerType(
                            IntegerType::I8
                            | IntegerType::I16
                            | IntegerType::I32
                            | IntegerType::I64
                            | IntegerType::I128,
                        )
                        | Type::Field
                        | Type::Group => {}
                        _ => self.handler.emit_err(
                            TypeCheckerError::type_is_not_negatable(expected.clone(), unary.inner.span()).into(),
                        ),
                    }
                    self.compare_expr_type(&unary.inner, expected, unary.inner.span());
                }
            },
            Expression::Ternary(ternary) => {
                self.compare_expr_type(&ternary.condition, Type::Boolean, ternary.condition.span());
                self.compare_expr_type(&ternary.if_true, expected.clone(), ternary.if_true.span());
                self.compare_expr_type(&ternary.if_false, expected, ternary.if_false.span());
            }
            Expression::Call(call) => match &*call.function {
                Expression::Identifier(ident) => {
                    if let Some(func) = self.symbol_table.lookup_fn(&ident.name) {
                        self.assert_type(func.output.clone(), expected, ident.span());

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
                                self.compare_expr_type(
                                    argument,
                                    expected.get_variable().type_.clone(),
                                    argument.span(),
                                );
                            });
                    } else {
                        self.handler
                            .emit_err(TypeCheckerError::unknown_sym("function", &ident.name, ident.span()).into());
                    }
                }
                expr => self.compare_expr_type(expr, expected, call.span()),
            },
            Expression::Err(_) => {}
        }
    }
}

impl<'a> ExpressionVisitor<'a> for TypeChecker<'a> {}
