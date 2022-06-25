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

use crate::{TypeChecker, Value};

use super::type_output::TypeOutput;

impl<'a> ExpressionVisitor<'a> for TypeChecker<'a> {
    type AdditionalInput = Option<Type>;
    type Output = TypeOutput;

    fn visit_expression(&mut self, input: &'a Expression, expected: &Self::AdditionalInput) -> Self::Output {
        match input {
            Expression::Identifier(expr) => self.visit_identifier(expr, expected),
            Expression::Literal(expr) => self.visit_literal(expr, expected),
            Expression::Binary(expr) => self.visit_binary(expr, expected),
            Expression::Unary(expr) => self.visit_unary(expr, expected),
            Expression::Ternary(expr) => self.visit_ternary(expr, expected),
            Expression::Call(expr) => self.visit_call(expr, expected),
            Expression::Err(expr) => self.visit_err(expr, expected),
        }
    }

    fn visit_identifier(&mut self, input: &'a Identifier, expected: &Self::AdditionalInput) -> Self::Output {
        if let Some(var) = self.symbol_table.borrow().lookup_variable(&input.name) {
            self.assert_expected_option(var.type_, var, expected, var.span)
        } else {
            self.handler
                .emit_err(TypeCheckerError::unknown_sym("variable", input.name, input.span()));
            TypeOutput::None
        }
    }

    fn visit_literal(&mut self, input: &'a LiteralExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match input {
            LiteralExpression::Address(value, span) => self.assert_expected_option(
                Type::Address,
                Value::Address(value.clone(), *span),
                expected,
                input.span(),
            ),
            LiteralExpression::Boolean(value, span) => {
                self.assert_expected_option(Type::Boolean, Value::Boolean(*value, *span), expected, input.span())
            }
            LiteralExpression::Field(value, span) => {
                self.assert_expected_option(Type::Field, Value::Field(value.clone(), *span), expected, input.span())
            }
            LiteralExpression::Integer(type_, str_content, _) => {
                let ret_type =
                    self.assert_expected_option(Type::IntegerType(*type_), TypeOutput::None, expected, input.span());
                match type_ {
                    IntegerType::I8 => {
                        let int = if self.negate {
                            format!("-{str_content}")
                        } else {
                            str_content.clone()
                        };

                        if let Ok(int) = int.parse::<i8>() {
                            Value::I8(int, input.span()).into()
                        } else {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i8", input.span()));
                            ret_type
                        }
                    }
                    IntegerType::I16 => {
                        let int = if self.negate {
                            format!("-{str_content}")
                        } else {
                            str_content.clone()
                        };

                        if let Ok(int) = int.parse::<i16>() {
                            Value::I16(int, input.span()).into()
                        } else {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i16", input.span()));
                            ret_type
                        }
                    }
                    IntegerType::I32 => {
                        let int = if self.negate {
                            format!("-{str_content}")
                        } else {
                            str_content.clone()
                        };

                        if let Ok(int) = int.parse::<i32>() {
                            Value::I32(int, input.span()).into()
                        } else {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i32", input.span()));
                            ret_type
                        }
                    }
                    IntegerType::I64 => {
                        let int = if self.negate {
                            format!("-{str_content}")
                        } else {
                            str_content.clone()
                        };

                        if let Ok(int) = int.parse::<i64>() {
                            Value::I64(int, input.span()).into()
                        } else {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i64", input.span()));
                            ret_type
                        }
                    }
                    IntegerType::I128 => {
                        let int = if self.negate {
                            format!("-{str_content}")
                        } else {
                            str_content.clone()
                        };

                        if let Ok(int) = int.parse::<i128>() {
                            Value::I128(int, input.span()).into()
                        } else {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i128", input.span()));
                            ret_type
                        }
                    }
                    IntegerType::U8 => match str_content.parse::<u8>() {
                        Ok(int) => Value::U8(int, input.span()).into(),
                        Err(_) => {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(str_content, "u8", input.span()));
                            ret_type
                        }
                    },
                    IntegerType::U16 => match str_content.parse::<u16>() {
                        Ok(int) => Value::U16(int, input.span()).into(),
                        Err(_) => {
                            self.handler.emit_err(TypeCheckerError::invalid_int_value(
                                str_content,
                                "u16",
                                input.span(),
                            ));
                            ret_type
                        }
                    },
                    IntegerType::U32 => match str_content.parse::<u32>() {
                        Ok(int) => Value::U32(int, input.span()).into(),
                        Err(_) => {
                            self.handler.emit_err(TypeCheckerError::invalid_int_value(
                                str_content,
                                "u32",
                                input.span(),
                            ));
                            ret_type
                        }
                    },
                    IntegerType::U64 => match str_content.parse::<u64>() {
                        Ok(int) => Value::U64(int, input.span()).into(),
                        Err(_) => {
                            self.handler.emit_err(TypeCheckerError::invalid_int_value(
                                str_content,
                                "u64",
                                input.span(),
                            ));
                            ret_type
                        }
                    },
                    IntegerType::U128 => match str_content.parse::<u128>() {
                        Ok(int) => Value::U128(int, input.span()).into(),
                        Err(_) => {
                            self.handler.emit_err(TypeCheckerError::invalid_int_value(
                                str_content,
                                "u128",
                                input.span(),
                            ));
                            ret_type
                        }
                    },
                }
            }
            LiteralExpression::Group(value) => {
                self.assert_expected_option(Type::Group, Value::Group(value.clone()), expected, input.span())
            }
            LiteralExpression::Scalar(value, span) => self.assert_expected_option(
                Type::Scalar,
                Value::Scalar(value.clone(), *span),
                expected,
                input.span(),
            ),
            LiteralExpression::String(value, span) => self.assert_expected_option(
                Type::String,
                Value::String(value.clone(), *span),
                expected,
                input.span(),
            ),
        }
    }

    fn visit_binary(&mut self, input: &'a BinaryExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match input.op {
            BinaryOperation::And | BinaryOperation::Or | BinaryOperation::Nand | BinaryOperation::Nor => {
                self.assert_expected_option(Type::Boolean, TypeOutput::None, expected, input.span());
                let t1 = self.visit_expression(&input.left, expected);
                let t2 = self.visit_expression(&input.right, expected);

                t1.return_incorrect_type(&t2, expected)
            }
            BinaryOperation::BitwiseAnd | BinaryOperation::BitwiseOr | BinaryOperation::Xor => {
                // Assert equal boolean or integer types.
                self.assert_bool_int_type(expected, input.span());
                let t1 = self.visit_expression(&input.left, expected);
                let t2 = self.visit_expression(&input.right, expected);

                t1.return_incorrect_type(&t2, expected)
            }
            BinaryOperation::Add => {
                self.assert_field_group_scalar_int_type(expected, input.span());
                let t1 = self.visit_expression(&input.left, expected);
                let t2 = self.visit_expression(&input.right, expected);

                t1.return_incorrect_type(&t2, expected)
            }
            BinaryOperation::Sub => {
                self.assert_field_group_int_type(expected, input.span());
                let t1 = self.visit_expression(&input.left, expected);
                let t2 = self.visit_expression(&input.right, expected);

                t1.return_incorrect_type(&t2, expected)
            }
            BinaryOperation::Mul => {
                self.assert_field_group_int_type(expected, input.span());

                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);
                let combined = t1.return_incorrect_type(&t2, expected);

                // Allow `group` * `scalar` multiplication.
                match (t1.as_ref().into(), t2.as_ref().into()) {
                    (Some(Type::Group), other) => {
                        self.assert_expected_type(&other, TypeOutput::None, Type::Scalar, input.right.span());
                        self.assert_expected_type(expected, combined, Type::Group, input.span())
                    }
                    (other, Some(Type::Group)) => {
                        self.assert_expected_type(&other, TypeOutput::None, Type::Scalar, input.left.span());
                        self.assert_expected_type(expected, combined, Type::Group, input.span())
                    }
                    (_, _) => {
                        // Assert equal field or integer types.
                        self.assert_field_int_type(expected, input.span());

                        t1.return_incorrect_type(&t2, expected)
                    }
                }
            }
            BinaryOperation::Div => {
                self.assert_field_int_type(expected, input.span());

                let t1 = self.visit_expression(&input.left, expected);
                let t2 = self.visit_expression(&input.right, expected);

                t1.return_incorrect_type(&t2, expected)
            }
            BinaryOperation::Pow => {
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);
                let combined = t1.return_incorrect_type(&t2, expected);

                match (t1.into(), t2.as_ref().into()) {
                    (Some(Type::Field), type_) => {
                        self.assert_expected_type(&type_, TypeOutput::None, Type::Field, input.right.span());
                        self.assert_expected_type(expected, combined, Type::Field, input.span())
                    }
                    (type_, Some(Type::Field)) => {
                        self.assert_expected_type(&type_, TypeOutput::None, Type::Field, input.left.span());
                        self.assert_expected_type(expected, combined, Type::Field, input.span())
                    }
                    (Some(t1), t2) => {
                        // Allow integer t2 magnitude (u8, u16, u32)
                        self.assert_magnitude_type(&t2, input.right.span());
                        self.assert_expected_type(expected, combined, t1, input.span())
                    }
                    (None, t2_type) => {
                        // Allow integer t2 magnitude (u8, u16, u32)
                        self.assert_magnitude_type(&t2_type, input.right.span());
                        t2
                    }
                }
            }
            BinaryOperation::Eq | BinaryOperation::Neq => {
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                self.assert_eq_types(t1.as_ref().into(), t2.as_ref().into(), input.span());

                // Forces this to return a Boolean as the correct type output variation.
                t1.return_incorrect_type(&t2, &None).replace(Type::Boolean)
            }
            BinaryOperation::Lt | BinaryOperation::Gt | BinaryOperation::Le | BinaryOperation::Ge => {
                // address, fields, int, scalar
                let t1 = self.visit_expression(&input.left, &None);
                let t1_type = t1.as_ref().into();
                self.assert_address_field_scalar_int_type(&t1_type, input.left.span());

                let t2 = self.visit_expression(&input.right, &None);
                let t2_type = t2.as_ref().into();
                self.assert_address_field_scalar_int_type(&t2_type, input.right.span());

                self.assert_eq_types(t1_type, t2_type, input.span());

                // Forces this to return a Boolean as the correct type output variation.
                t1.return_incorrect_type(&t2, &None).replace(Type::Boolean)
            }
            BinaryOperation::AddWrapped
            | BinaryOperation::SubWrapped
            | BinaryOperation::DivWrapped
            | BinaryOperation::MulWrapped => {
                // Assert equal integer types.
                self.assert_int_type(expected, input.span);
                let t1 = self.visit_expression(&input.left, expected);
                let t2 = self.visit_expression(&input.right, expected);

                t1.return_incorrect_type(&t2, expected)
            }
            BinaryOperation::Shl
            | BinaryOperation::ShlWrapped
            | BinaryOperation::Shr
            | BinaryOperation::ShrWrapped
            | BinaryOperation::PowWrapped => {
                // Assert left and expected are equal integer types.
                self.assert_int_type(expected, input.span);
                let t1 = self.visit_expression(&input.left, expected);

                // Assert right type is a magnitude (u8, u16, u32).
                let t2 = self.visit_expression(&input.right, &None);
                let t2_ty = t2.as_ref().into();
                self.assert_magnitude_type(&t2_ty, input.right.span());

                t1.return_incorrect_type(&t2, expected)
            }
        }
    }

    fn visit_unary(&mut self, input: &'a UnaryExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match input.op {
            UnaryOperation::Abs => {
                // Assert integer type only.
                self.assert_int_type(expected, input.span());
                self.visit_expression(&input.receiver, expected)
            }
            UnaryOperation::AbsWrapped => {
                // Assert integer type only.
                self.assert_int_type(expected, input.span());
                self.visit_expression(&input.receiver, expected)
            }
            UnaryOperation::Double => {
                // Assert field and group type only.
                self.assert_field_group_type(expected, input.span());
                self.visit_expression(&input.receiver, expected)
            }
            UnaryOperation::Inverse => {
                // Assert field type only.
                self.assert_expected_type(expected, TypeOutput::None, Type::Field, input.span());
                self.visit_expression(&input.receiver, expected)
            }
            UnaryOperation::Negate => {
                let prior_negate_state = self.negate;
                self.negate = !self.negate;

                let type_ = self.visit_expression(&input.receiver, expected);
                self.negate = prior_negate_state;
                match type_.as_ref().into() {
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
                        .handler
                        .emit_err(TypeCheckerError::type_is_not_negatable(t, input.receiver.span())),
                    _ => {}
                };
                type_
            }
            UnaryOperation::Not => {
                // Assert boolean, integer types only.
                self.assert_bool_int_type(expected, input.span());
                self.visit_expression(&input.receiver, expected)
            }
            UnaryOperation::Square => {
                // Assert field type only.
                self.assert_expected_type(expected, TypeOutput::None, Type::Field, input.span());
                self.visit_expression(&input.receiver, expected)
            }
            UnaryOperation::SquareRoot => {
                // Assert field and scalar types only.
                self.assert_field_scalar_type(expected, input.span());
                self.visit_expression(&input.receiver, expected)
            }
        }
    }

    fn visit_ternary(&mut self, input: &'a TernaryExpression, expected: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.condition, &Some(Type::Boolean));

        let t1 = self.visit_expression(&input.if_true, expected);
        let t2 = self.visit_expression(&input.if_false, expected);

        t1.return_incorrect_type(&t2, &None)
    }

    fn visit_call(&mut self, input: &'a CallExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match &*input.function {
            Expression::Identifier(ident) => {
                // the function symbol lookup is purposely done outside of the `if let` to avoid a RefCell lifetime bug in rust.
                // dont move it into the `if let` or it will keep the `symbol_table` alive for the entire block and will be very memory inefficient!
                let f = self.symbol_table.borrow().lookup_fn(&ident.name).cloned();
                if let Some(func) = f {
                    let ret = if expected.is_some() {
                        self.assert_expected_option(func.type_, TypeOutput::None, expected, func.span)
                    } else {
                        // For now we assume that functions with no expected type return a mutable type.
                        // Can always be double checked during flattening.
                        TypeOutput::MutType(func.type_)
                    };

                    if func.input.len() != input.arguments.len() {
                        self.handler.emit_err(TypeCheckerError::incorrect_num_args_to_call(
                            func.input.len(),
                            input.arguments.len(),
                            input.span(),
                        ));
                    }

                    func.input
                        .iter()
                        .zip(input.arguments.iter())
                        .for_each(|(expected, argument)| {
                            self.visit_expression(argument, &Some(expected.get_variable().type_));
                        });

                    ret
                } else {
                    self.handler
                        .emit_err(TypeCheckerError::unknown_sym("function", &ident.name, ident.span()));
                    TypeOutput::None
                }
            }
            expr => self.visit_expression(expr, expected),
        }
    }
}
