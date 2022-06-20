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

fn return_incorrect_type(
    t1: Option<Type>,
    t2: Option<Type>,
    value: Option<Value>,
    expected: &Option<Type>,
) -> TypeOutput {
    match (t1, t2) {
        (Some(t1), Some(t2)) if t1 == t2 => TypeOutput::Type(t1),
        (Some(t1), Some(t2)) => {
            if let Some(expected) = expected {
                if &t1 != expected {
                    TypeOutput::Type(t1)
                } else {
                    TypeOutput::Type(t2)
                }
            } else {
                TypeOutput::Type(t1)
            }
        }
        _ if value.is_some() => TypeOutput::Const(value.unwrap()),
        _ => TypeOutput::None,
    }
}

pub enum TypeOutput {
    Type(Type),
    Const(Value),
    None,
}

impl From<TypeOutput> for Option<Type> {
    fn from(t: TypeOutput) -> Self {
        t.as_ref().into()
    }
}

impl From<&TypeOutput> for Option<Type> {
    fn from(t: &TypeOutput) -> Self {
        match t {
            TypeOutput::Type(t) => Some(*t),
            TypeOutput::Const(v) => Some(v.into()),
            TypeOutput::None => None,
        }
    }
}

impl AsRef<Self> for TypeOutput {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Default for TypeOutput {
    fn default() -> Self {
        Self::None
    }
}

impl<'a> ExpressionVisitor<'a> for TypeChecker<'a> {
    type AdditionalInput = Option<Type>;
    type Output = TypeOutput;

    fn visit_expression(&mut self, input: &'a Expression, expected: &Self::AdditionalInput) -> Self::Output {
        match input {
            Expression::Identifier(expr) => self.visit_identifier(expr, expected),
            Expression::Value(expr) => self.visit_value(expr, expected),
            Expression::Binary(expr) => self.visit_binary(expr, expected),
            Expression::Unary(expr) => self.visit_unary(expr, expected),
            Expression::Ternary(expr) => self.visit_ternary(expr, expected),
            Expression::Call(expr) => self.visit_call(expr, expected),
            Expression::Err(expr) => self.visit_err(expr, expected),
        }
    }

    fn visit_identifier(&mut self, input: &'a Identifier, expected: &Self::AdditionalInput) -> Self::Output {
        if let Some(var) = self.symbol_table.borrow().lookup_variable(input.name).cloned() {
            self.assert_type(var.type_, var.declaration.get_const_value(), expected, var.span)
        } else {
            self.handler
                .emit_err(TypeCheckerError::unknown_sym("variable", input.name, input.span()).into());
            TypeOutput::None
        }
    }

    fn visit_value(&mut self, input: &'a ValueExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match input {
            // let x: u8 = 1u32;
            ValueExpression::Address(value, _) => self.assert_type(
                Type::Address,
                Some(Value::Address(value.clone())),
                expected,
                input.span(),
            ),
            ValueExpression::Boolean(value, _) => {
                self.assert_type(Type::Boolean, Some(Value::Boolean(*value)), expected, input.span())
            }
            ValueExpression::Field(value, _) => {
                self.assert_type(Type::Field, Some(Value::Field(value.clone())), expected, input.span())
            }
            ValueExpression::Integer(type_, str_content, _) => {
                let ret_type = self.assert_type(Type::IntegerType(*type_), None, expected, input.span());
                match type_ {
                    IntegerType::I8 => {
                        let int = if self.negate {
                            format!("-{str_content}")
                        } else {
                            str_content.clone()
                        };

                        if let Ok(int) = int.parse::<i8>() {
                            TypeOutput::Const(Value::I8(int))
                        } else {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i8", input.span()).into());
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
                            TypeOutput::Const(Value::I16(int))
                        } else {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i16", input.span()).into());
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
                            TypeOutput::Const(Value::I32(int))
                        } else {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i32", input.span()).into());
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
                            TypeOutput::Const(Value::I64(int))
                        } else {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i64", input.span()).into());
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
                            TypeOutput::Const(Value::I128(int))
                        } else {
                            self.handler
                                .emit_err(TypeCheckerError::invalid_int_value(int, "i128", input.span()).into());
                            ret_type
                        }
                    }
                    IntegerType::U8 if str_content.parse::<u8>().is_err() => {
                        self.handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u8", input.span()).into());
                        ret_type
                    }
                    IntegerType::U16 if str_content.parse::<u16>().is_err() => {
                        self.handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u16", input.span()).into());
                        ret_type
                    }
                    IntegerType::U32 if str_content.parse::<u32>().is_err() => {
                        self.handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u32", input.span()).into());
                        ret_type
                    }
                    IntegerType::U64 if str_content.parse::<u64>().is_err() => {
                        self.handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u64", input.span()).into());
                        ret_type
                    }
                    IntegerType::U128 if str_content.parse::<u128>().is_err() => {
                        self.handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u128", input.span()).into());
                        ret_type
                    }
                    _ => ret_type,
                }
            }
            ValueExpression::Group(_) => self.assert_type(Type::Group, None, expected, input.span()),
            ValueExpression::Scalar(_, _) => self.assert_type(Type::Scalar, None, expected, input.span()),
            ValueExpression::String(_, _) => self.assert_type(Type::String, None, expected, input.span()),
        }
    }

    fn visit_binary(&mut self, input: &'a BinaryExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match input.op {
            BinaryOperation::And | BinaryOperation::Or => {
                self.assert_type(Type::Boolean, None, expected, input.span());
                let t1 = self.visit_expression(&input.left, expected);
                let t2 = self.visit_expression(&input.right, expected);

                return_incorrect_type(t1.into(), t2.into(), None, expected)
            }
            BinaryOperation::Add => {
                self.assert_field_group_scalar_int_type(expected, input.span());
                let t1 = self.visit_expression(&input.left, expected);
                let t2 = self.visit_expression(&input.right, expected);

                return_incorrect_type(t1.into(), t2.into(), None, expected)
            }
            BinaryOperation::Sub => {
                self.assert_field_group_int_type(expected, input.span());
                let t1 = self.visit_expression(&input.left, expected);
                let t2 = self.visit_expression(&input.right, expected);

                return_incorrect_type(t1.into(), t2.into(), None, expected)
            }
            BinaryOperation::Mul => {
                self.assert_field_group_int_type(expected, input.span());

                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                // Allow `group` * `scalar` multiplication.
                match (t1.into(), t2.into()) {
                    (Some(Type::Group), Some(other)) => {
                        self.assert_type(Type::Group, None, expected, input.left.span());
                        self.assert_type(other, None, &Some(Type::Scalar), input.right.span());
                        TypeOutput::Type(Type::Group)
                    }
                    (Some(other), Some(Type::Group)) => {
                        self.assert_type(other, None, &Some(Type::Scalar), input.left.span());
                        self.assert_type(Type::Group, None, expected, input.right.span());
                        TypeOutput::Type(Type::Group)
                    }
                    (Some(t1), Some(t2)) => {
                        self.assert_type(t1, None, expected, input.left.span());
                        self.assert_type(t2, None, expected, input.right.span());
                        return_incorrect_type(Some(t1), Some(t2), None, expected)
                    }
                    (Some(type_), None) => {
                        self.assert_type(type_, None, expected, input.left.span());
                        TypeOutput::None
                    }
                    (None, Some(type_)) => {
                        self.assert_type(type_, None, expected, input.right.span());
                        TypeOutput::None
                    }
                    (None, None) => TypeOutput::None,
                }
            }
            BinaryOperation::Div => {
                self.assert_field_int_type(expected, input.span());

                let t1 = self.visit_expression(&input.left, expected);
                let t2 = self.visit_expression(&input.right, expected);

                return_incorrect_type(t1.into(), t2.into(), None, expected)
            }
            BinaryOperation::Pow => {
                let t1 = self.visit_expression(&input.left, &None);
                let t1_ty: Option<Type> = t1.as_ref().into();
                let t2 = self.visit_expression(&input.right, &None);

                match (t1_ty, t2.into()) {
                    // Type A must be an int.
                    // Type B must be a unsigned int.
                    (Some(Type::IntegerType(_)), Some(Type::IntegerType(itype))) if !itype.is_signed() => {
                        self.assert_type(t1_ty.unwrap(), None, expected, input.left.span());
                    }
                    // Type A was an int.
                    // But Type B was not a unsigned int.
                    (Some(Type::IntegerType(_)), Some(t)) => {
                        self.handler.emit_err(
                            TypeCheckerError::incorrect_pow_exponent_type("unsigned int", t, input.right.span()).into(),
                        );
                    }
                    // Type A must be a field.
                    // Type B must be an int.
                    (Some(Type::Field), Some(Type::IntegerType(_))) => {
                        self.assert_type(Type::Field, None, expected, input.left.span());
                    }
                    // Type A was a field.
                    // But Type B was not an int.
                    (Some(Type::Field), Some(t)) => {
                        self.handler.emit_err(
                            TypeCheckerError::incorrect_pow_exponent_type("int", t, input.right.span()).into(),
                        );
                    }
                    // The base is some type thats not an int or field.
                    (Some(t), _) if !matches!(t, Type::IntegerType(_) | Type::Field) => {
                        self.handler
                            .emit_err(TypeCheckerError::incorrect_pow_base_type(t, input.left.span()).into());
                    }
                    _ => {}
                }

                t1
            }
            BinaryOperation::Eq | BinaryOperation::Ne => {
                let t1 = self.visit_expression(&input.left, &None).into();
                let t2 = self.visit_expression(&input.right, &None).into();

                self.assert_eq_types(t1, t2, input.span());

                TypeOutput::Type(Type::Boolean)
            }
            BinaryOperation::Lt | BinaryOperation::Gt | BinaryOperation::Le | BinaryOperation::Ge => {
                let t1 = self.visit_expression(&input.left, &None).into();
                self.assert_field_scalar_int_type(&t1, input.left.span());

                let t2 = self.visit_expression(&input.right, &None).into();
                self.assert_field_scalar_int_type(&t2, input.right.span());

                self.assert_eq_types(t1, t2, input.span());

                TypeOutput::Type(Type::Boolean)
            }
        }
    }

    fn visit_unary(&mut self, input: &'a UnaryExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match input.op {
            UnaryOperation::Not => {
                self.assert_type(Type::Boolean, None, expected, input.span());
                self.visit_expression(&input.inner, expected)
            }
            UnaryOperation::Negate => {
                let prior_negate_state = self.negate;
                self.negate = true;

                let type_ = self.visit_expression(&input.inner, expected);
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
                        .emit_err(TypeCheckerError::type_is_not_negatable(t, input.inner.span()).into()),
                    _ => {}
                };
                type_
            }
        }
    }

    fn visit_ternary(&mut self, input: &'a TernaryExpression, expected: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.condition, &Some(Type::Boolean));

        let t1 = self.visit_expression(&input.if_true, expected).into();
        let t2 = self.visit_expression(&input.if_false, expected).into();

        return_incorrect_type(t1, t2, None, expected)
    }

    fn visit_call(&mut self, input: &'a CallExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match &*input.function {
            Expression::Identifier(ident) => {
                let f = self.symbol_table.borrow().lookup_fn(ident.name).cloned();
                if let Some(func) = f {
                    let ret = self.assert_type(func.type_, None, expected, func.span);

                    if func.input.len() != input.arguments.len() {
                        self.handler.emit_err(
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
                            self.visit_expression(argument, &Some(expected.get_variable().type_));
                        });

                    ret
                } else {
                    self.handler
                        .emit_err(TypeCheckerError::unknown_sym("function", &ident.name, ident.span()).into());
                    TypeOutput::None
                }
            }
            expr => self.visit_expression(expr, expected),
        }
    }
}
