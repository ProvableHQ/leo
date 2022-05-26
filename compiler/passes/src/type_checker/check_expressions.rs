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

impl<'a> ExpressionVisitor<'a> for TypeChecker<'a> {
    type Output = Type;

    fn visit_identifier(&mut self, input: &'a Identifier) -> (VisitResult, Option<Self::Output>) {
        let type_ = if let Some(var) = self.symbol_table.lookup_variable(&input.name) {
            Some(self.assert_type(*var.type_, self.expected_type))
        } else {
            self.handler
                .emit_err(TypeCheckerError::unknown_sym("variable", input.name, self.span).into());
            None
        };

        (VisitResult::VisitChildren, type_)
    }

    fn visit_value(&mut self, input: &'a ValueExpression) -> (VisitResult, Option<Self::Output>) {
        let prev_span = self.span;
        self.span = input.span();

        let type_ = Some(match input {
            ValueExpression::Address(_, _) => self.assert_type(Type::Address, self.expected_type),
            ValueExpression::Boolean(_, _) => self.assert_type(Type::Boolean, self.expected_type),
            ValueExpression::Field(_, _) => self.assert_type(Type::Field, self.expected_type),
            ValueExpression::Integer(type_, str_content, _) => {
                match type_ {
                    IntegerType::I8 => {
                        let int = if self.negate {
                            format!("-{str_content}")
                        } else {
                            str_content.clone()
                        };

                        if int.parse::<i8>().is_err() {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i8", input.span()).into());
                        }
                    }
                    IntegerType::I16 => {
                        let int = if self.negate {
                            format!("-{str_content}")
                        } else {
                            str_content.clone()
                        };

                        if int.parse::<i16>().is_err() {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i16", input.span()).into());
                        }
                    }
                    IntegerType::I32 => {
                        let int = if self.negate {
                            format!("-{str_content}")
                        } else {
                            str_content.clone()
                        };

                        if int.parse::<i32>().is_err() {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i32", input.span()).into());
                        }
                    }
                    IntegerType::I64 => {
                        let int = if self.negate {
                            format!("-{str_content}")
                        } else {
                            str_content.clone()
                        };

                        if int.parse::<i64>().is_err() {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i64", input.span()).into());
                        }
                    }
                    IntegerType::I128 => {
                        let int = if self.negate {
                            format!("-{str_content}")
                        } else {
                            str_content.clone()
                        };

                        if int.parse::<i128>().is_err() {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i128", input.span()).into());
                        }
                    }
                    IntegerType::U8 if str_content.parse::<u8>().is_err() => self
                        .handler
                        .emit_err(TypeCheckerError::invalid_int_value(str_content, "u8", input.span()).into()),
                    IntegerType::U16 if str_content.parse::<u16>().is_err() => self
                        .handler
                        .emit_err(TypeCheckerError::invalid_int_value(str_content, "u16", input.span()).into()),
                    IntegerType::U32 if str_content.parse::<u32>().is_err() => self
                        .handler
                        .emit_err(TypeCheckerError::invalid_int_value(str_content, "u32", input.span()).into()),
                    IntegerType::U64 if str_content.parse::<u64>().is_err() => self
                        .handler
                        .emit_err(TypeCheckerError::invalid_int_value(str_content, "u64", input.span()).into()),
                    IntegerType::U128 if str_content.parse::<u128>().is_err() => self
                        .handler
                        .emit_err(TypeCheckerError::invalid_int_value(str_content, "u128", input.span()).into()),
                    _ => {}
                }
                self.assert_type(Type::IntegerType(*type_), self.expected_type)
            }
            ValueExpression::Group(_) => self.assert_type(Type::Group, self.expected_type),
            ValueExpression::Scalar(_, _) => self.assert_type(Type::Scalar, self.expected_type),
            ValueExpression::String(_, _) => unreachable!("String types are not reachable"),
        });

        self.span = prev_span;
        (VisitResult::VisitChildren, type_)
    }

    fn visit_binary(&mut self, input: &'a BinaryExpression) -> (VisitResult, Option<Self::Output>) {
        let prev_span = self.span;
        self.span = input.span();

        /* let type_ = match input.op {
            BinaryOperation::And | BinaryOperation::Or => {
                self.assert_type(Type::Boolean, self.expected_type);
                let t1 = self.compare_expr_type(&input.left, self.expected_type, input.left.span());
                let t2 = self.compare_expr_type(&input.right, self.expected_type, input.right.span());

                return_incorrect_type(t1, t2, self.expected_type)
            }
            BinaryOperation::Add => {
                self.assert_field_group_scalar_int_type(self.expected_type, input.span());
                let t1 = self.compare_expr_type(&input.left, self.expected_type, input.left.span());
                let t2 = self.compare_expr_type(&input.right, self.expected_type, input.right.span());

                return_incorrect_type(t1, t2, self.expected_type)
            }
            BinaryOperation::Sub => {
                self.assert_field_group_int_type(self.expected_type, input.span());
                let t1 = self.compare_expr_type(&input.left, self.expected_type, input.left.span());
                let t2 = self.compare_expr_type(&input.right, self.expected_type, input.right.span());

                return_incorrect_type(t1, t2, self.expected_type)
            }
            BinaryOperation::Mul => {
                self.assert_field_group_int_type(self.expected_type, input.span());

                let t1 = self.compare_expr_type(&input.left, None, input.left.span());
                let t2 = self.compare_expr_type(&input.right, None, input.right.span());

                // Allow `group` * `scalar` multiplication.
                match (t1.as_ref(), t2.as_ref()) {
                    (Some(Type::Group), Some(other)) => {
                        self.assert_type(Type::Group, self.expected_type);
                        self.assert_type(*other, Some(Type::Scalar));
                        Some(Type::Group)
                    }
                    (Some(other), Some(Type::Group)) => {
                        self.assert_type(Type::Group, self.expected_type);
                        self.assert_type(*other, Some(Type::Scalar));
                        Some(Type::Group)
                    }
                    _ => {
                        self.assert_type(t1.unwrap(), self.expected_type);
                        self.assert_type(t2.unwrap(), self.expected_type);
                        return_incorrect_type(t1, t2, self.expected_type)
                    }
                }
            }
            BinaryOperation::Div => {
                self.assert_field_int_type(self.expected_type, input.span());

                let t1 = self.compare_expr_type(&input.left, self.expected_type, input.left.span());
                let t2 = self.compare_expr_type(&input.right, self.expected_type, input.right.span());
                return_incorrect_type(t1, t2, self.expected_type)
            }
            BinaryOperation::Pow => {
                let t1 = self.compare_expr_type(&input.left, None, input.left.span());
                let t2 = self.compare_expr_type(&input.right, None, input.right.span());

                match (t1.as_ref(), t2.as_ref()) {
                    // Type A must be an int.
                    // Type B must be a unsigned int.
                    (Some(Type::IntegerType(_)), Some(Type::IntegerType(itype))) if !itype.is_signed() => {
                        self.assert_type(t1.unwrap(), self.expected_type);
                    }
                    // Type A was an int.
                    // But Type B was not a unsigned int.
                    (Some(Type::IntegerType(_)), Some(t)) => {
                        self.handler.emit_err(
                            TypeCheckerError::incorrect_pow_exponent_type("unsigned int", t, input.right.span())
                                .into(),
                        );
                    }
                    // Type A must be a field.
                    // Type B must be an int.
                    (Some(Type::Field), Some(Type::IntegerType(_))) => {
                        self.assert_type(Type::Field, self.expected_type);
                    }
                    // Type A was a field.
                    // But Type B was not an int.
                    (Some(Type::Field), Some(t)) => {
                        self.handler.emit_err(
                            TypeCheckerError::incorrect_pow_exponent_type("int", t, input.right.span()).into(),
                        );
                    }
                    // The base is some type thats not an int or field.
                    (Some(t), _) => {
                        self.handler
                            .emit_err(TypeCheckerError::incorrect_pow_base_type(t, input.left.span()).into());
                    }
                    _ => {}
                }

                t1
            }
            BinaryOperation::Eq | BinaryOperation::Ne => {
                let t1 = self.compare_expr_type(&input.left, None, input.left.span());
                let t2 = self.compare_expr_type(&input.right, None, input.right.span());

                self.assert_eq_types(t1, t2, input.span());

                Some(Type::Boolean)
            }
            BinaryOperation::Lt | BinaryOperation::Gt | BinaryOperation::Le | BinaryOperation::Ge => {
                let t1 = self.compare_expr_type(&input.left, None, input.left.span());
                self.assert_field_scalar_int_type(t1, input.left.span());

                let t2 = self.compare_expr_type(&input.right, None, input.right.span());
                self.assert_field_scalar_int_type(t2, input.right.span());

                self.assert_eq_types(t1, t2, input.span());

                Some(Type::Boolean)
            }
        }; */

        self.span = prev_span;
        (VisitResult::VisitChildren, None)
    }
}
