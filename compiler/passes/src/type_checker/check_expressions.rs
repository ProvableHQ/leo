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

use crate::TypeChecker;

use super::director::Director;

impl<'a> ExpressionVisitor<'a> for TypeChecker<'a> {}

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

impl<'a> ExpressionVisitorDirector<'a> for Director<'a> {
    type Output = Type;

    fn visit_expression(&mut self, input: &'a Expression) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor.visit_expression(input) {
            return match input {
                Expression::Identifier(expr) => self.visit_identifier(expr),
                Expression::Value(expr) => self.visit_value(expr),
                Expression::Binary(expr) => self.visit_binary(expr),
                Expression::Unary(expr) => self.visit_unary(expr),
                Expression::Ternary(expr) => self.visit_ternary(expr),
                Expression::Call(expr) => self.visit_call(expr),
                Expression::Err(expr) => self.visit_err(expr),
            };
        }

        None
    }

    fn visit_identifier(&mut self, input: &'a Identifier) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor.visit_identifier(input) {
            return if let Some(var) = self.visitor.symbol_table.clone().lookup_variable(&input.name) {
                Some(self.visitor.assert_type(*var.type_, self.visitor.expected_type))
            } else {
                self.visitor
                    .handler
                    .emit_err(TypeCheckerError::unknown_sym("variable", input.name, input.span()).into());
                None
            };
        }

        None
    }

    fn visit_value(&mut self, input: &'a ValueExpression) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor.visit_value(input) {
            return Some(match input {
                ValueExpression::Address(_, _) => self.visitor.assert_type(Type::Address, self.visitor.expected_type),
                ValueExpression::Boolean(_, _) => self.visitor.assert_type(Type::Boolean, self.visitor.expected_type),
                ValueExpression::Field(_, _) => self.visitor.assert_type(Type::Field, self.visitor.expected_type),
                ValueExpression::Integer(type_, str_content, _) => {
                    match type_ {
                        IntegerType::I8 => {
                            let int = if self.visitor.negate {
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i8>().is_err() {
                                self.visitor
                                    .handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i8", input.span()).into());
                            }
                        }
                        IntegerType::I16 => {
                            let int = if self.visitor.negate {
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i16>().is_err() {
                                self.visitor
                                    .handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i16", input.span()).into());
                            }
                        }
                        IntegerType::I32 => {
                            let int = if self.visitor.negate {
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i32>().is_err() {
                                self.visitor
                                    .handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i32", input.span()).into());
                            }
                        }
                        IntegerType::I64 => {
                            let int = if self.visitor.negate {
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i64>().is_err() {
                                self.visitor
                                    .handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i64", input.span()).into());
                            }
                        }
                        IntegerType::I128 => {
                            let int = if self.visitor.negate {
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i128>().is_err() {
                                self.visitor
                                    .handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i128", input.span()).into());
                            }
                        }
                        IntegerType::U8 if str_content.parse::<u8>().is_err() => self
                            .visitor
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u8", input.span()).into()),
                        IntegerType::U16 if str_content.parse::<u16>().is_err() => self
                            .visitor
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u16", input.span()).into()),
                        IntegerType::U32 if str_content.parse::<u32>().is_err() => self
                            .visitor
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u32", input.span()).into()),
                        IntegerType::U64 if str_content.parse::<u64>().is_err() => self
                            .visitor
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u64", input.span()).into()),
                        IntegerType::U128 if str_content.parse::<u128>().is_err() => self
                            .visitor
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u128", input.span()).into()),
                        _ => {}
                    }
                    self.visitor
                        .assert_type(Type::IntegerType(*type_), self.visitor.expected_type)
                }
                ValueExpression::Group(_) => self.visitor.assert_type(Type::Group, self.visitor.expected_type),
                ValueExpression::Scalar(_, _) => self.visitor.assert_type(Type::Scalar, self.visitor.expected_type),
                ValueExpression::String(_, _) => unreachable!("String types are not reachable"),
            });
        }

        None
    }

    fn visit_binary(&mut self, input: &'a BinaryExpression) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor.visit_binary(input) {
            return match input.op {
                BinaryOperation::And | BinaryOperation::Or => {
                    self.visitor.assert_type(Type::Boolean, self.visitor.expected_type);
                    let t1 = self.visit_expression(&input.left);
                    let t2 = self.visit_expression(&input.right);

                    return_incorrect_type(t1, t2, self.visitor.expected_type)
                }
                BinaryOperation::Add => {
                    self.visitor
                        .assert_field_group_scalar_int_type(self.visitor.expected_type, input.span());
                    let t1 = self.visit_expression(&input.left);
                    let t2 = self.visit_expression(&input.right);

                    return_incorrect_type(t1, t2, self.visitor.expected_type)
                }
                BinaryOperation::Sub => {
                    self.visitor
                        .assert_field_group_int_type(self.visitor.expected_type, input.span());
                    let t1 = self.visit_expression(&input.left);
                    let t2 = self.visit_expression(&input.right);

                    return_incorrect_type(t1, t2, self.visitor.expected_type)
                }
                BinaryOperation::Mul => {
                    self.visitor
                        .assert_field_group_int_type(self.visitor.expected_type, input.span());

                    let prev_expected_type = self.visitor.expected_type;
                    self.visitor.expected_type = None;
                    let t1 = self.visit_expression(&input.left);
                    let t2 = self.visit_expression(&input.right);
                    self.visitor.expected_type = prev_expected_type;

                    // Allow `group` * `scalar` multiplication.
                    match (t1.as_ref(), t2.as_ref()) {
                        (Some(Type::Group), Some(other))
                        | (Some(other), Some(Type::Group)) => {
                            self.visitor.assert_type(Type::Group, self.visitor.expected_type);
                            self.visitor.assert_type(*other, Some(Type::Scalar));
                            Some(Type::Group)
                        }
                        _ => {
                            self.visitor.assert_type(t1.unwrap(), self.visitor.expected_type);
                            self.visitor.assert_type(t2.unwrap(), self.visitor.expected_type);
                            return_incorrect_type(t1, t2, self.visitor.expected_type)
                        }
                    }
                }
                BinaryOperation::Div => {
                    self.visitor
                        .assert_field_int_type(self.visitor.expected_type, input.span());

                    let t1 = self.visit_expression(&input.left);
                    let t2 = self.visit_expression(&input.right);
                    
                    return_incorrect_type(t1, t2, self.visitor.expected_type)
                }
                BinaryOperation::Pow => {
                    let prev_expected_type = self.visitor.expected_type;
                    self.visitor.expected_type = None;
                    
                    let t1 = self.visit_expression(&input.left);
                    let t2 = self.visit_expression(&input.right);
                    
                    self.visitor.expected_type = prev_expected_type;

                    match (t1.as_ref(), t2.as_ref()) {
                        // Type A must be an int.
                        // Type B must be a unsigned int.
                        (Some(Type::IntegerType(_)), Some(Type::IntegerType(itype))) if !itype.is_signed() => {
                            self.visitor.assert_type(t1.unwrap(), self.visitor.expected_type);
                        }
                        // Type A was an int.
                        // But Type B was not a unsigned int.
                        (Some(Type::IntegerType(_)), Some(t)) => {
                            self.visitor.handler.emit_err(
                                TypeCheckerError::incorrect_pow_exponent_type("unsigned int", t, input.right.span())
                                    .into(),
                            );
                        }
                        // Type A must be a field.
                        // Type B must be an int.
                        (Some(Type::Field), Some(Type::IntegerType(_))) => {
                            self.visitor.assert_type(Type::Field, self.visitor.expected_type);
                        }
                        // Type A was a field.
                        // But Type B was not an int.
                        (Some(Type::Field), Some(t)) => {
                            self.visitor.handler.emit_err(
                                TypeCheckerError::incorrect_pow_exponent_type("int", t, input.right.span()).into(),
                            );
                        }
                        // The base is some type thats not an int or field.
                        (Some(t), _) => {
                            self.visitor
                                .handler
                                .emit_err(TypeCheckerError::incorrect_pow_base_type(t, input.left.span()).into());
                        }
                        _ => {}
                    }

                    t1
                }
                BinaryOperation::Eq | BinaryOperation::Ne => {
                    let prev_expected_type = self.visitor.expected_type;
                    self.visitor.expected_type = None;
                    
                    let t1 = self.visit_expression(&input.left);
                    let t2 = self.visit_expression(&input.right);
                    
                    self.visitor.expected_type = prev_expected_type;
                    self.visitor.assert_eq_types(t1, t2, input.span());

                    Some(Type::Boolean)
                }
                BinaryOperation::Lt | BinaryOperation::Gt | BinaryOperation::Le | BinaryOperation::Ge => {
                    let prev_expected_type = self.visitor.expected_type;
                    self.visitor.expected_type = None;
                    
                    let t1 = self.visit_expression(&input.left);
                    self.visitor.assert_field_scalar_int_type(t1, input.left.span());

                    let t2 = self.visit_expression(&input.right);
                    self.visitor.assert_field_scalar_int_type(t2, input.right.span());

                    self.visitor.expected_type = prev_expected_type;
                    self.visitor.assert_eq_types(t1, t2, input.span());

                    Some(Type::Boolean)
                }
            };
        }

        None
    }

    fn visit_unary(&mut self, input: &'a UnaryExpression) -> Option<Self::Output> {
        match input.op {
            UnaryOperation::Not => {
                self.visitor.assert_type(Type::Boolean, self.visitor.expected_type);
                self.visit_expression(&input.inner)
            }
            UnaryOperation::Negate => {
                let prior_negate_state = self.visitor.negate;
                self.visitor.negate = true;

                let type_ = self.visit_expression(&input.inner);
                self.visitor.negate = prior_negate_state;
                match type_.as_ref() {
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
                    ) => {}
                    Some(t) => self
                        .visitor
                        .handler
                        .emit_err(TypeCheckerError::type_is_not_negatable(t, input.inner.span()).into()),
                    _ => {}
                };
                type_
            }
        }
    }

    fn visit_ternary(&mut self, input: &'a TernaryExpression) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor.visit_ternary(input) {
            let prev_expected_type = self.visitor.expected_type;
            self.visitor.expected_type = Some(Type::Boolean);
            self.visit_expression(&input.condition);
            self.visitor.expected_type = prev_expected_type;

            let t1 = self.visit_expression(&input.if_true);
            let t2 = self.visit_expression(&input.if_false);

            return return_incorrect_type(t1, t2, self.visitor.expected_type);
        }

        None
    }

    fn visit_call(&mut self, input: &'a CallExpression) -> Option<Self::Output> {
        match &*input.function {
            Expression::Identifier(ident) => {
                if let Some(func) = self.visitor.symbol_table.clone().lookup_fn(&ident.name) {
                    let ret = self.visitor.assert_type(func.output, self.visitor.expected_type);

                    if func.input.len() != input.arguments.len() {
                        self.visitor.handler.emit_err(
                            TypeCheckerError::incorrect_num_args_to_call(
                                func.input.len(),
                                input.arguments.len(),
                                input.span(),
                            )
                            .into(),
                        );
                    }

                    func.input
                        .iter()
                        .zip(input.arguments.iter())
                        .for_each(|(expected, argument)| {
                            let prev_expected_type = self.visitor.expected_type;
                            self.visitor.expected_type = Some(expected.get_variable().type_);
                            self.visit_expression(argument);
                            self.visitor.expected_type = prev_expected_type;
                        });

                    Some(ret)
                } else {
                    self.visitor
                        .handler
                        .emit_err(TypeCheckerError::unknown_sym("function", &ident.name, ident.span()).into());
                    None
                }
            }
            expr => self.visit_expression(expr),
        }
    }
}
