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
use leo_errors::emitter::Handler;
use leo_errors::TypeCheckerError;
use leo_span::{sym, Span};
use std::str::FromStr;

use crate::TypeChecker;

fn return_incorrect_type(t1: Option<Type>, t2: Option<Type>, expected: &Option<Type>) -> Option<Type> {
    match (t1, t2) {
        (Some(t1), Some(t2)) if t1 == t2 => Some(t1),
        (Some(t1), Some(t2)) => {
            if let Some(expected) = expected {
                if &t1 != expected {
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
    type AdditionalInput = Option<Type>;
    type Output = Option<Type>;

    fn visit_access(&mut self, input: &'a AccessExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match input {
            AccessExpression::AssociatedFunction(access) => {
                // Check core circuit name and function.
                if let Some(core_instruction) = self.check_core_circuit_call(&access.ty, &access.name) {
                    // Check num input arguments.
                    if core_instruction.num_args() != access.args.len() {
                        self.emit_err(TypeCheckerError::incorrect_num_args_to_call(
                            core_instruction.num_args(),
                            access.args.len(),
                            input.span(),
                        ));
                    }

                    // Check first argument type.
                    if let Some(first_arg) = access.args.get(0usize) {
                        let first_arg_type = self.visit_expression(first_arg, &None);
                        self.assert_one_of_types(&first_arg_type, core_instruction.first_arg_types(), access.span());
                    }

                    // Check second argument type.
                    if let Some(second_arg) = access.args.get(1usize) {
                        let second_arg_type = self.visit_expression(second_arg, &None);
                        self.assert_one_of_types(&second_arg_type, core_instruction.second_arg_types(), access.span());
                    }

                    // Check return type.
                    return Some(self.assert_and_return_type(core_instruction.return_type(), expected, access.span()));
                } else {
                    self.emit_err(TypeCheckerError::invalid_core_circuit_call(access, access.span()));
                }
            }
            AccessExpression::Tuple(access) => {
                if let Some(type_) = self.visit_expression(&access.tuple, &None) {
                    match type_ {
                        Type::Tuple(tuple) => {
                            // Check out of range access.
                            let index = access.index.to_usize();
                            if index > tuple.len() - 1 {
                                self.emit_err(TypeCheckerError::tuple_out_of_range(index, tuple.len(), access.span()));
                            } else {
                                // Lookup type of tuple index.
                                let actual = tuple.get(index).expect("failed to get tuple index").clone();
                                if let Some(expected) = expected {
                                    // Emit error for mismatched types.
                                    if !actual.eq_flat(expected) {
                                        self.emit_err(TypeCheckerError::type_should_be(
                                            &actual,
                                            expected,
                                            access.span(),
                                        ))
                                    }
                                }

                                // Return type of tuple index.
                                return Some(actual);
                            }
                        }
                        type_ => {
                            self.emit_err(TypeCheckerError::type_should_be(type_, "tuple", access.span()));
                        }
                    }
                    self.emit_err(TypeCheckerError::invalid_core_circuit_call(access, access.span()));
                }
            }
            AccessExpression::Member(access) => {
                match *access.inner {
                    // If the access expression is of the form `self.<name>`, then check the <name> is valid.
                    Expression::Identifier(identifier) if identifier.name == sym::SelfLower => match access.name.name {
                        sym::caller => return Some(Type::Address),
                        _ => {
                            self.emit_err(TypeCheckerError::invalid_self_access(access.name.span()));
                        }
                    },
                    _ => {
                        // Check that the type of `inner` in `inner.name` is a circuit.
                        match self.visit_expression(&access.inner, &None) {
                            Some(Type::Identifier(identifier)) => {
                                // Retrieve the circuit definition associated with `identifier`.
                                let circ = self.symbol_table.borrow().lookup_circuit(identifier.name).cloned();
                                if let Some(circ) = circ {
                                    // Check that `access.name` is a member of the circuit.
                                    match circ
                                        .members
                                        .iter()
                                        .find(|circuit_member| circuit_member.name() == access.name.name)
                                    {
                                        // Case where `access.name` is a member of the circuit.
                                        Some(CircuitMember::CircuitVariable(_, type_)) => return Some(type_.clone()),
                                        // Case where `access.name` is not a member of the circuit.
                                        None => {
                                            self.emit_err(TypeCheckerError::invalid_circuit_variable(
                                                access.name,
                                                &circ,
                                                access.name.span(),
                                            ));
                                        }
                                    }
                                } else {
                                    self.emit_err(TypeCheckerError::undefined_type(&access.inner, access.inner.span()));
                                }
                            }
                            Some(type_) => {
                                self.emit_err(TypeCheckerError::type_should_be(type_, "circuit", access.inner.span()));
                            }
                            None => {
                                self.emit_err(TypeCheckerError::could_not_determine_type(
                                    &access.inner,
                                    access.inner.span(),
                                ));
                            }
                        }
                    }
                }
            }
            AccessExpression::AssociatedConstant(..) => {} // todo: Add support for associated constants (u8::MAX).
        }
        None
    }

    fn visit_binary(&mut self, input: &'a BinaryExpression, destination: &Self::AdditionalInput) -> Self::Output {
        match input.op {
            BinaryOperation::And | BinaryOperation::Or | BinaryOperation::Nand | BinaryOperation::Nor => {
                // Only boolean types.
                self.assert_bool_type(destination, input.span());
                let t1 = self.visit_expression(&input.left, destination);
                let t2 = self.visit_expression(&input.right, destination);

                // Check that both operands have the same type.
                self.check_eq_types(&t1, &t2, input.span());

                return_incorrect_type(t1, t2, destination)
            }
            BinaryOperation::BitwiseAnd | BinaryOperation::BitwiseOr | BinaryOperation::Xor => {
                //  Only boolean or integer types.
                self.assert_bool_int_type(destination, input.span());
                let t1 = self.visit_expression(&input.left, destination);
                let t2 = self.visit_expression(&input.right, destination);

                // Check that both operands have the same type.
                self.check_eq_types(&t1, &t2, input.span());

                return_incorrect_type(t1, t2, destination)
            }
            BinaryOperation::Add => {
                // Only field, group, scalar, or integer types.
                self.assert_field_group_scalar_int_type(destination, input.span());
                let t1 = self.visit_expression(&input.left, destination);
                let t2 = self.visit_expression(&input.right, destination);

                // Check that both operands have the same type.
                self.check_eq_types(&t1, &t2, input.span());

                return_incorrect_type(t1, t2, destination)
            }
            BinaryOperation::Sub => {
                // Only field, group, or integer types.
                self.assert_field_group_int_type(destination, input.span());
                let t1 = self.visit_expression(&input.left, destination);
                let t2 = self.visit_expression(&input.right, destination);

                // Check that both operands have the same type.
                self.check_eq_types(&t1, &t2, input.span());

                return_incorrect_type(t1, t2, destination)
            }
            BinaryOperation::Mul => {
                // Operation returns field, group or integer types.
                self.assert_field_group_int_type(destination, input.span());

                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                // Allow group * scalar multiplication.
                match (t1, input.left.span(), t2, input.right.span()) {
                    (Some(Type::Group), _, other, other_span) | (other, other_span, Some(Type::Group), _) => {
                        // Other type must be scalar.
                        self.assert_scalar_type(&other, other_span);

                        // Operation returns group.
                        self.assert_group_type(destination, input.span());

                        Some(Type::Group)
                    }
                    (Some(Type::Field), _, other, other_span) | (other, other_span, Some(Type::Field), _) => {
                        // Other type must be field.
                        self.assert_field_type(&other, other_span);

                        // Operation returns field.
                        self.assert_field_type(destination, input.span());

                        Some(Type::Field)
                    }
                    (Some(Type::Integer(integer_type)), _, other, other_span)
                    | (other, other_span, Some(Type::Integer(integer_type)), _) => {
                        // Other type must be the same integer type.
                        self.assert_type(&other, &Type::Integer(integer_type), other_span);

                        // Operation returns the same integer type.
                        self.assert_type(destination, &Type::Integer(integer_type), input.span());

                        Some(Type::Integer(integer_type))
                    }
                    (left_type, left_span, right_type, right_span) => {
                        let check_type = |type_: Option<Type>, expression: &Expression, span: Span| match type_ {
                            None => {
                                self.emit_err(TypeCheckerError::could_not_determine_type(expression, span));
                            }
                            Some(type_) => {
                                self.emit_err(TypeCheckerError::type_should_be(
                                    type_,
                                    "field, group, integer, or scalar",
                                    span,
                                ));
                            }
                        };
                        check_type(left_type, &input.left, left_span);
                        check_type(right_type, &input.right, right_span);
                        destination.clone()
                    }
                }
            }
            BinaryOperation::Div => {
                // Only field or integer types.
                self.assert_field_int_type(destination, input.span());

                let t1 = self.visit_expression(&input.left, destination);
                let t2 = self.visit_expression(&input.right, destination);

                // Check that both operands have the same type.
                self.check_eq_types(&t1, &t2, input.span());

                return_incorrect_type(t1, t2, destination)
            }
            BinaryOperation::Rem | BinaryOperation::RemWrapped => {
                // Only integer types.
                self.assert_int_type(destination, input.span());

                let t1 = self.visit_expression(&input.left, destination);
                let t2 = self.visit_expression(&input.right, destination);

                // Check that both operands have the same type.
                self.check_eq_types(&t1, &t2, input.span());

                return_incorrect_type(t1, t2, destination)
            }
            BinaryOperation::Mod => {
                // Only unsigned integer types.
                self.assert_unsigned_int_type(destination, input.span());

                let t1 = self.visit_expression(&input.left, destination);
                let t2 = self.visit_expression(&input.right, destination);

                // Check that both operands have the same type.
                self.check_eq_types(&t1, &t2, input.span());

                return_incorrect_type(t1, t2, destination)
            }
            BinaryOperation::Pow => {
                // Operation returns field or integer types.
                self.assert_field_int_type(destination, input.span());

                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                // Allow field ^ field.
                match (t1, t2) {
                    (Some(Type::Field), right) => {
                        // Right must be field.
                        self.assert_field_type(&right, input.right.span());

                        // Operation returns field.
                        self.assert_field_type(destination, input.span());

                        Some(Type::Field)
                    }
                    (left, Some(Type::Field)) => {
                        // Left must be field.
                        self.assert_field_type(&left, input.left.span());

                        // Operation returns field.
                        self.assert_field_type(destination, input.span());

                        Some(Type::Field)
                    }
                    (Some(left), right) => {
                        // Left type is checked to be an integer by above.
                        // Right type must be magnitude (u8, u16, u32).
                        self.assert_magnitude_type(&right, input.right.span());

                        // Operation returns left type.
                        self.assert_type(destination, &left, input.span());

                        Some(left)
                    }
                    (None, right) => {
                        // Lhs type is checked to be an integer by above.
                        // Rhs type must be magnitude (u8, u16, u32).
                        self.assert_magnitude_type(&right, input.right.span());
                        destination.clone()
                    }
                }
            }
            BinaryOperation::Eq | BinaryOperation::Neq => {
                // Assert first and second address, boolean, field, group, scalar, or integer types.
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                // Check that the types of the operands are equal.
                self.check_eq_types(&t1, &t2, input.span());

                // Operation returns a boolean.
                self.assert_bool_type(destination, input.span());

                Some(Type::Boolean)
            }
            BinaryOperation::Lt | BinaryOperation::Gt | BinaryOperation::Lte | BinaryOperation::Gte => {
                // Assert left and right are equal field, scalar, or integer types.
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                match (&t1, &t2) {
                    (Some(Type::Address), _) | (_, Some(Type::Address)) => {
                        // Emit an error for address comparison.
                        self.emit_err(TypeCheckerError::compare_address(input.op, input.span()));
                    }
                    (t1, t2) => {
                        self.assert_field_scalar_int_type(t1, input.left.span());
                        self.assert_field_scalar_int_type(t2, input.right.span());
                    }
                }

                // Check that the types of the operands are equal.
                self.check_eq_types(&t1, &t2, input.span());

                // Operation returns a boolean.
                self.assert_bool_type(destination, input.span());

                Some(Type::Boolean)
            }
            BinaryOperation::AddWrapped
            | BinaryOperation::SubWrapped
            | BinaryOperation::DivWrapped
            | BinaryOperation::MulWrapped => {
                // Only integer types.
                self.assert_int_type(destination, input.span);
                let t1 = self.visit_expression(&input.left, destination);
                let t2 = self.visit_expression(&input.right, destination);

                // Check that both operands have the same type.
                self.check_eq_types(&t1, &t2, input.span());

                return_incorrect_type(t1, t2, destination)
            }
            BinaryOperation::Shl
            | BinaryOperation::ShlWrapped
            | BinaryOperation::Shr
            | BinaryOperation::ShrWrapped
            | BinaryOperation::PowWrapped => {
                let t1 = self.visit_expression(&input.left, destination);
                let t2 = self.visit_expression(&input.right, &None);

                // Assert left and destination are equal integer types.
                self.assert_int_type(&t1, input.left.span());
                self.assert_int_type(destination, input.span);

                // Assert right type is a magnitude (u8, u16, u32).
                self.assert_magnitude_type(&t2, input.right.span());

                return_incorrect_type(t1, t2, destination)
            }
        }
    }

    fn visit_call(&mut self, input: &'a CallExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match &*input.function {
            Expression::Identifier(ident) => {
                // Note: The function symbol lookup is performed outside of the `if let Some(func) ...` block to avoid a RefCell lifetime bug in Rust.
                // Do not move it into the `if let Some(func) ...` block or it will keep `self.symbol_table` alive for the entire block and will be very memory inefficient!
                let func = self.symbol_table.borrow().lookup_fn_symbol(ident.name).cloned();
                if let Some(func) = func {
                    let ret = self.assert_and_return_type(func.output_type, expected, func.span);

                    // Check number of function arguments.
                    if func.input.len() != input.arguments.len() {
                        self.emit_err(TypeCheckerError::incorrect_num_args_to_call(
                            func.input.len(),
                            input.arguments.len(),
                            input.span(),
                        ));
                    }

                    // Check function argument types.
                    func.input
                        .iter()
                        .zip(input.arguments.iter())
                        .for_each(|(expected, argument)| {
                            self.visit_expression(argument, &Some(expected.type_.clone()));
                        });

                    Some(ret)
                } else {
                    self.emit_err(TypeCheckerError::unknown_sym("function", ident.name, ident.span()));
                    None
                }
            }
            // TODO: Is this case sufficient?
            expr => self.visit_expression(expr, expected),
        }
    }

    fn visit_circuit_init(&mut self, input: &'a CircuitExpression, additional: &Self::AdditionalInput) -> Self::Output {
        let circ = self.symbol_table.borrow().lookup_circuit(input.name.name).cloned();
        if let Some(circ) = circ {
            // Check circuit type name.
            let ret = self.check_expected_circuit(circ.identifier, additional, input.name.span());

            // Check number of circuit members.
            if circ.members.len() != input.members.len() {
                self.emit_err(TypeCheckerError::incorrect_num_circuit_members(
                    circ.members.len(),
                    input.members.len(),
                    input.span(),
                ));
            }

            // Check circuit member types.
            circ.members
                .iter()
                .for_each(|CircuitMember::CircuitVariable(name, ty)| {
                    // Lookup circuit variable name.
                    if let Some(actual) = input.members.iter().find(|member| member.identifier.name == name.name) {
                        if let Some(expr) = &actual.expression {
                            self.visit_expression(expr, &Some(ty.clone()));
                        }
                    } else {
                        self.emit_err(TypeCheckerError::missing_circuit_member(
                            circ.identifier,
                            name,
                            input.span(),
                        ));
                    };
                });

            Some(ret)
        } else {
            self.emit_err(TypeCheckerError::unknown_sym(
                "circuit",
                input.name.name,
                input.name.span(),
            ));
            None
        }
    }

    // We do not want to panic on `ErrExpression`s in order to propagate as many errors as possible.
    fn visit_err(&mut self, _input: &'a ErrExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_identifier(&mut self, var: &'a Identifier, expected: &Self::AdditionalInput) -> Self::Output {
        if let Some(var) = self.symbol_table.borrow().lookup_variable(var.name) {
            Some(self.assert_and_return_type(var.type_.clone(), expected, var.span))
        } else {
            self.emit_err(TypeCheckerError::unknown_sym("variable", var.name, var.span()));
            None
        }
    }

    fn visit_literal(&mut self, input: &'a Literal, expected: &Self::AdditionalInput) -> Self::Output {
        fn parse_integer_literal<I: FromStr>(handler: &Handler, string: &String, span: Span, type_string: &str) {
            if string.parse::<I>().is_err() {
                handler.emit_err(TypeCheckerError::invalid_int_value(string, type_string, span));
            }
        }

        Some(match input {
            Literal::Address(_, _) => self.assert_and_return_type(Type::Address, expected, input.span()),
            Literal::Boolean(_, _) => self.assert_and_return_type(Type::Boolean, expected, input.span()),
            Literal::Field(_, _) => self.assert_and_return_type(Type::Field, expected, input.span()),
            Literal::Integer(integer_type, string, _) => match integer_type {
                IntegerType::U8 => {
                    parse_integer_literal::<u8>(self.handler, string, input.span(), "u8");
                    self.assert_and_return_type(Type::Integer(IntegerType::U8), expected, input.span())
                }
                IntegerType::U16 => {
                    parse_integer_literal::<u16>(self.handler, string, input.span(), "u16");
                    self.assert_and_return_type(Type::Integer(IntegerType::U16), expected, input.span())
                }
                IntegerType::U32 => {
                    parse_integer_literal::<u32>(self.handler, string, input.span(), "u32");
                    self.assert_and_return_type(Type::Integer(IntegerType::U32), expected, input.span())
                }
                IntegerType::U64 => {
                    parse_integer_literal::<u64>(self.handler, string, input.span(), "u64");
                    self.assert_and_return_type(Type::Integer(IntegerType::U64), expected, input.span())
                }
                IntegerType::U128 => {
                    parse_integer_literal::<u128>(self.handler, string, input.span(), "u128");
                    self.assert_and_return_type(Type::Integer(IntegerType::U128), expected, input.span())
                }
                IntegerType::I8 => {
                    parse_integer_literal::<i8>(self.handler, string, input.span(), "i8");
                    self.assert_and_return_type(Type::Integer(IntegerType::I8), expected, input.span())
                }
                IntegerType::I16 => {
                    parse_integer_literal::<i16>(self.handler, string, input.span(), "i16");
                    self.assert_and_return_type(Type::Integer(IntegerType::I16), expected, input.span())
                }
                IntegerType::I32 => {
                    parse_integer_literal::<i32>(self.handler, string, input.span(), "i32");
                    self.assert_and_return_type(Type::Integer(IntegerType::I32), expected, input.span())
                }
                IntegerType::I64 => {
                    parse_integer_literal::<i64>(self.handler, string, input.span(), "i64");
                    self.assert_and_return_type(Type::Integer(IntegerType::I64), expected, input.span())
                }
                IntegerType::I128 => {
                    parse_integer_literal::<i128>(self.handler, string, input.span(), "i128");
                    self.assert_and_return_type(Type::Integer(IntegerType::I128), expected, input.span())
                }
            },
            Literal::Group(_) => self.assert_and_return_type(Type::Group, expected, input.span()),
            Literal::Scalar(_, _) => self.assert_and_return_type(Type::Scalar, expected, input.span()),
            Literal::String(_, _) => self.assert_and_return_type(Type::String, expected, input.span()),
        })
    }

    fn visit_ternary(&mut self, input: &'a TernaryExpression, expected: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.condition, &Some(Type::Boolean));

        let t1 = self.visit_expression(&input.if_true, expected);
        let t2 = self.visit_expression(&input.if_false, expected);

        return_incorrect_type(t1, t2, expected)
    }

    fn visit_tuple(&mut self, input: &'a TupleExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match input.elements.len() {
            0 => Some(self.assert_and_return_type(Type::Unit, expected, input.span())),
            1 => self.visit_expression(&input.elements[0], expected),
            _ => {
                // Check the expected tuple types if they are known.

                if let Some(Type::Tuple(expected_types)) = expected {
                    // Check actual length is equal to expected length.
                    if expected_types.len() != input.elements.len() {
                        self.emit_err(TypeCheckerError::incorrect_tuple_length(
                            expected_types.len(),
                            input.elements.len(),
                            input.span(),
                        ));
                    }

                    expected_types
                        .iter()
                        .zip(input.elements.iter())
                        .for_each(|(expected, expr)| {
                            self.visit_expression(expr, &Some(expected.clone()));
                        });

                    Some(Type::Tuple(expected_types.clone()))
                } else {
                    // Tuples must be explicitly typed in testnet3.
                    self.emit_err(TypeCheckerError::invalid_tuple(input.span()));

                    None
                }
            }
        }
    }

    fn visit_unary(&mut self, input: &'a UnaryExpression, destination: &Self::AdditionalInput) -> Self::Output {
        match input.op {
            UnaryOperation::Abs => {
                // Only signed integer types.
                self.assert_signed_int_type(destination, input.span());
                self.visit_expression(&input.receiver, destination)
            }
            UnaryOperation::AbsWrapped => {
                // Only signed integer types.
                self.assert_signed_int_type(destination, input.span());
                self.visit_expression(&input.receiver, destination)
            }
            UnaryOperation::Double => {
                // Only field or group types.
                self.assert_field_group_type(destination, input.span());
                self.visit_expression(&input.receiver, destination)
            }
            UnaryOperation::Inverse => {
                // Only field types.
                self.assert_field_type(destination, input.span());
                self.visit_expression(&input.receiver, destination)
            }
            UnaryOperation::Negate => {
                let type_ = self.visit_expression(&input.receiver, destination);

                // Only field, group, or signed integer types.
                self.assert_field_group_signed_int_type(&type_, input.receiver.span());
                type_
            }
            UnaryOperation::Not => {
                // Only boolean or integer types.
                self.assert_bool_int_type(destination, input.span());
                self.visit_expression(&input.receiver, destination)
            }
            UnaryOperation::Square => {
                // Only field type.
                self.assert_field_type(destination, input.span());
                self.visit_expression(&input.receiver, destination)
            }
            UnaryOperation::SquareRoot => {
                // Only field type.
                self.assert_field_type(destination, input.span());
                self.visit_expression(&input.receiver, destination)
            }
        }
    }
}
