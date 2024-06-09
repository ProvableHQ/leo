// Copyright (C) 2019-2023 Aleo Systems Inc.
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

use crate::{TypeChecker, VariableSymbol};

use leo_ast::*;
use leo_errors::{emitter::Handler, TypeCheckerError};
use leo_span::{sym, Span, Symbol};

use snarkvm::console::network::Network;

use itertools::Itertools;
use std::str::FromStr;

fn return_incorrect_type(t1: Option<Type>, t2: Option<Type>, expected: &Option<Type>) -> Option<Type> {
    match (t1, t2) {
        (Some(t1), Some(t2)) if t1 == t2 => Some(t1),
        (Some(t1), Some(t2)) => {
            if let Some(expected) = expected {
                if &t1 != expected { Some(t1) } else { Some(t2) }
            } else {
                Some(t1)
            }
        }
        (None, Some(_)) | (Some(_), None) | (None, None) => None,
    }
}

impl<'a, N: Network> ExpressionVisitor<'a> for TypeChecker<'a, N> {
    type AdditionalInput = Option<Type>;
    type Output = Option<Type>;

    fn visit_expression(&mut self, input: &'a Expression, additional: &Self::AdditionalInput) -> Self::Output {
        let output = match input {
            Expression::Access(access) => self.visit_access(access, additional),
            Expression::Array(array) => self.visit_array(array, additional),
            Expression::Binary(binary) => self.visit_binary(binary, additional),
            Expression::Call(call) => self.visit_call(call, additional),
            Expression::Cast(cast) => self.visit_cast(cast, additional),
            Expression::Struct(struct_) => self.visit_struct_init(struct_, additional),
            Expression::Err(err) => self.visit_err(err, additional),
            Expression::Identifier(identifier) => self.visit_identifier(identifier, additional),
            Expression::Literal(literal) => self.visit_literal(literal, additional),
            Expression::Locator(locator) => self.visit_locator(locator, additional),
            Expression::Ternary(ternary) => self.visit_ternary(ternary, additional),
            Expression::Tuple(tuple) => self.visit_tuple(tuple, additional),
            Expression::Unary(unary) => self.visit_unary(unary, additional),
            Expression::Unit(unit) => self.visit_unit(unit, additional),
        };
        // If the output type is known, add the expression and its associated type to the symbol table.
        if let Some(type_) = &output {
            self.type_table.insert(input.id(), type_.clone());
        }
        // Return the output type.
        output
    }

    fn visit_access(&mut self, input: &'a AccessExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match input {
            AccessExpression::Array(access) => {
                // Check that the expression is an array.
                let array_type = self.visit_expression(&access.array, &None);
                self.assert_array_type(&array_type, access.array.span());

                // Check that the index is an integer type.
                let index_type = self.visit_expression(&access.index, &None);
                self.assert_int_type(&index_type, access.index.span());

                // Get the element type of the array.
                let element_type = match array_type {
                    Some(Type::Array(array_type)) => Some(array_type.element_type().clone()),
                    _ => None,
                };

                // If the expected type is known, then check that the element type is the same as the expected type.
                if let Some(expected) = expected {
                    self.assert_type(&element_type, expected, input.span());
                }

                // Return the element type of the array.
                return element_type;
            }
            AccessExpression::AssociatedFunction(access) => {
                // Check core struct name and function.
                if let Some(core_instruction) = self.get_core_function_call(&access.variant, &access.name) {
                    // Check that operation is not restricted to finalize blocks.
                    if self.scope_state.variant != Some(Variant::AsyncFunction)
                        && core_instruction.is_finalize_command()
                    {
                        self.emit_err(TypeCheckerError::operation_must_be_in_finalize_block(input.span()));
                    }

                    // Get the types of the arguments.
                    let argument_types = access
                        .arguments
                        .iter()
                        .map(|arg| (self.visit_expression(arg, &None), arg.span()))
                        .collect::<Vec<_>>();

                    // Check that the types of the arguments are valid.
                    let return_type =
                        self.check_core_function_call(core_instruction.clone(), &argument_types, input.span());

                    // Check return type if the expected type is known.
                    if let Some(expected) = expected {
                        self.assert_type(&return_type, expected, input.span());
                    }

                    // Await futures here so that can use the argument variable names to lookup.
                    if core_instruction == CoreFunction::FutureAwait {
                        if access.arguments.len() != 1 {
                            self.emit_err(TypeCheckerError::can_only_await_one_future_at_a_time(access.span));
                            return Some(Type::Unit);
                        }
                        self.assert_future_await(&access.arguments.first(), input.span());
                    }

                    return return_type;
                } else {
                    self.emit_err(TypeCheckerError::invalid_core_function_call(access, access.span()));
                }
            }
            AccessExpression::Tuple(access) => {
                if let Some(type_) = self.visit_expression(&access.tuple, &None) {
                    match type_ {
                        Type::Tuple(tuple) => {
                            // Check out of range access.
                            let index = access.index.value();
                            if index > tuple.length() - 1 {
                                self.emit_err(TypeCheckerError::tuple_out_of_range(
                                    index,
                                    tuple.length(),
                                    access.span(),
                                ));
                            } else {
                                // Lookup type of tuple index.
                                let actual = tuple.elements().get(index).expect("failed to get tuple index").clone();
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
                        Type::Future(_) => {
                            // Get the fully inferred type.
                            if let Some(Type::Future(inferred_f)) = self.type_table.get(&access.tuple.id()) {
                                // Make sure in range.
                                if access.index.value() >= inferred_f.inputs().len() {
                                    self.emit_err(TypeCheckerError::invalid_future_access(
                                        access.index.value(),
                                        inferred_f.inputs().len(),
                                        access.span(),
                                    ));
                                } else {
                                    // Return the type of the input parameter.
                                    return Some(self.assert_and_return_type(
                                        inferred_f.inputs().get(access.index.value()).unwrap().clone(),
                                        expected,
                                        access.span(),
                                    ));
                                }
                            }
                        }
                        type_ => {
                            self.emit_err(TypeCheckerError::type_should_be(type_, "tuple", access.span()));
                        }
                    }
                    self.emit_err(TypeCheckerError::invalid_core_function_call(access, access.span()));
                }
            }
            AccessExpression::Member(access) => {
                match *access.inner {
                    // If the access expression is of the form `self.<name>`, then check the <name> is valid.
                    Expression::Identifier(identifier) if identifier.name == sym::SelfLower => match access.name.name {
                        sym::caller => {
                            // Check that the operation is not invoked in a `finalize` block.
                            self.check_access_allowed("self.caller", false, access.name.span());
                            return Some(Type::Address);
                        }
                        sym::signer => {
                            // Check that operation is not invoked in a `finalize` block.
                            self.check_access_allowed("self.signer", false, access.name.span());
                            return Some(Type::Address);
                        }
                        _ => {
                            self.emit_err(TypeCheckerError::invalid_self_access(access.name.span()));
                        }
                    },
                    // If the access expression is of the form `block.<name>`, then check the <name> is valid.
                    Expression::Identifier(identifier) if identifier.name == sym::block => match access.name.name {
                        sym::height => {
                            // Check that the operation is invoked in a `finalize` block.
                            self.check_access_allowed("block.height", true, access.name.span());
                            return Some(Type::Integer(IntegerType::U32));
                        }
                        _ => {
                            self.emit_err(TypeCheckerError::invalid_block_access(access.name.span()));
                        }
                    },
                    // If the access expression is of the form `network.<name>`, then check that the <name> is valid.
                    Expression::Identifier(identifier) if identifier.name == sym::network => match access.name.name {
                        sym::id => {
                            // Check that the operation is not invoked outside a `finalize` block.
                            self.check_access_allowed("network.id", true, access.name.span());
                            return Some(Type::Integer(IntegerType::U16));
                        }
                        _ => {
                            self.emit_err(TypeCheckerError::invalid_block_access(access.name.span()));
                        }
                    },
                    _ => {
                        // Check that the type of `inner` in `inner.name` is a struct.
                        match self.visit_expression(&access.inner, &None) {
                            Some(Type::Composite(struct_)) => {
                                // Retrieve the struct definition associated with `identifier`.
                                let struct_ = self.lookup_struct(struct_.program, struct_.id.name);
                                if let Some(struct_) = struct_ {
                                    // Check that `access.name` is a member of the struct.
                                    match struct_.members.iter().find(|member| member.name() == access.name.name) {
                                        // Case where `access.name` is a member of the struct.
                                        Some(Member { type_, .. }) => {
                                            // Check that the type of `access.name` is the same as `expected`.
                                            return Some(self.assert_and_return_type(
                                                type_.clone(),
                                                expected,
                                                access.span(),
                                            ));
                                        }
                                        // Case where `access.name` is not a member of the struct.
                                        None => {
                                            self.emit_err(TypeCheckerError::invalid_struct_variable(
                                                access.name,
                                                &struct_,
                                                access.name.span(),
                                            ));
                                        }
                                    }
                                } else {
                                    self.emit_err(TypeCheckerError::undefined_type(&access.inner, access.inner.span()));
                                }
                            }
                            Some(type_) => {
                                self.emit_err(TypeCheckerError::type_should_be(type_, "struct", access.inner.span()));
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
            AccessExpression::AssociatedConstant(access) => {
                // Check associated constant type and constant name
                if let Some(core_constant) = self.get_core_constant(&access.ty, &access.name) {
                    // Check return type if the expected type is known.
                    let return_type = Some(core_constant.to_type());
                    if let Some(expected) = expected {
                        self.assert_type(&return_type, expected, input.span());
                    }
                    return return_type;
                } else {
                    self.emit_err(TypeCheckerError::invalid_associated_constant(access, access.span))
                }
            }
        }
        None
    }

    fn visit_array(&mut self, input: &'a ArrayExpression, additional: &Self::AdditionalInput) -> Self::Output {
        // Get the types of each element expression.
        let element_types =
            input.elements.iter().map(|element| self.visit_expression(element, &None)).collect::<Vec<_>>();

        // Construct the array type.
        let return_type = match element_types.len() {
            // The array cannot be empty.
            0 => {
                self.emit_err(TypeCheckerError::array_empty(input.span()));
                None
            }
            num_elements => {
                if num_elements <= N::MAX_ARRAY_ELEMENTS {
                    // Check that the element types match.
                    let mut element_types = element_types.into_iter();
                    // Note that this unwrap is safe because we already checked that the array is not empty.
                    element_types.next().unwrap().map(|first_type| {
                        // Check that all elements have the same type.
                        for (element_type, element) in element_types.zip_eq(input.elements.iter().skip(1)) {
                            self.assert_type(&element_type, &first_type, element.span());
                        }
                        // Return the array type.
                        Type::Array(ArrayType::new(first_type, NonNegativeNumber::from(input.elements.len())))
                    })
                } else {
                    // The array cannot have more than `MAX_ARRAY_ELEMENTS` elements.
                    self.emit_err(TypeCheckerError::array_too_large(num_elements, N::MAX_ARRAY_ELEMENTS, input.span()));
                    None
                }
            }
        };

        // If the expected type is known, then check that the array type is the same as the expected type.
        if let Some(expected) = additional {
            self.assert_type(&return_type, expected, input.span());
        }

        // Return the array type.
        return_type
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

                t1
            }
        }
    }

    fn visit_call(&mut self, input: &'a CallExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match &*input.function {
            // Note that the parser guarantees that `input.function` is always an identifier.
            Expression::Identifier(ident) => {
                // Note: The function symbol lookup is performed outside of the `if let Some(func) ...` block to avoid a RefCell lifetime bug in Rust.
                // Do not move it into the `if let Some(func) ...` block or it will keep `self.symbol_table_creation` alive for the entire block and will be very memory inefficient!
                let func =
                    self.symbol_table.borrow().lookup_fn_symbol(Location::new(input.program, ident.name)).cloned();
                if let Some(func) = func {
                    // Check that the call is valid.
                    // Note that this unwrap is safe since we always set the variant before traversing the body of the function.
                    match self.scope_state.variant.unwrap() {
                        Variant::AsyncFunction | Variant::Function if !matches!(func.variant, Variant::Inline) => {
                            self.emit_err(TypeCheckerError::can_only_call_inline_function(input.span))
                        }
                        Variant::Transition | Variant::AsyncTransition
                            if matches!(func.variant, Variant::Transition)
                                && input.program.unwrap() == self.scope_state.program_name.unwrap() =>
                        {
                            self.emit_err(TypeCheckerError::cannot_invoke_call_to_local_transition_function(input.span))
                        }
                        _ => {}
                    }

                    // Check that the call is not to an external `inline` function.
                    if func.variant == Variant::Inline
                        && input.program.unwrap() != self.scope_state.program_name.unwrap()
                    {
                        self.emit_err(TypeCheckerError::cannot_call_external_inline_function(input.span));
                    }
                    // Async functions return a single future.
                    let mut ret = if func.variant == Variant::AsyncFunction {
                        // Type check after resolving the input types.
                        if let Some(Type::Future(_)) = expected {
                            Type::Future(FutureType::new(
                                Vec::new(),
                                Some(Location::new(input.program, ident.name)),
                                false,
                            ))
                        } else {
                            self.emit_err(TypeCheckerError::return_type_of_finalize_function_is_future(input.span));
                            Type::Unit
                        }
                    } else if func.variant == Variant::AsyncTransition {
                        // Fully infer future type.
                        let future_type = match self
                            .async_function_input_types
                            .get(&Location::new(input.program, Symbol::intern(&format!("finalize/{}", ident.name))))
                        {
                            Some(inputs) => Type::Future(FutureType::new(
                                inputs.clone(),
                                Some(Location::new(input.program, ident.name)),
                                true,
                            )),
                            None => {
                                self.emit_err(TypeCheckerError::async_function_not_found(ident.name, input.span));
                                return Some(Type::Future(FutureType::new(
                                    Vec::new(),
                                    Some(Location::new(input.program, ident.name)),
                                    false,
                                )));
                            }
                        };
                        let fully_inferred_type = match func.output_type {
                            Type::Tuple(tup) => Type::Tuple(TupleType::new(
                                tup.elements()
                                    .iter()
                                    .map(|t| if matches!(t, Type::Future(_)) { future_type.clone() } else { t.clone() })
                                    .collect::<Vec<Type>>(),
                            )),
                            Type::Future(_) => future_type,
                            _ => panic!("Invalid output type for async transition."),
                        };
                        self.assert_and_return_type(fully_inferred_type, expected, input.span())
                    } else {
                        self.assert_and_return_type(func.output_type, expected, input.span())
                    };

                    // Check number of function arguments.
                    if func.input.len() != input.arguments.len() {
                        self.emit_err(TypeCheckerError::incorrect_num_args_to_call(
                            func.input.len(),
                            input.arguments.len(),
                            input.span(),
                        ));
                    }

                    // Check function argument types.
                    self.scope_state.is_call = true;
                    let (mut input_futures, mut inferred_finalize_inputs) = (Vec::new(), Vec::new());
                    for (expected, argument) in func.input.iter().zip(input.arguments.iter()) {
                        // Get the type of the expression. If the type is not known, do not attempt to attempt any futher inference.
                        let ty = self.visit_expression(argument, &Some(expected.type_().clone()))?;
                        // Extract information about futures that are being consumed.
                        if func.variant == Variant::AsyncFunction && matches!(expected.type_(), Type::Future(_)) {
                            match argument {
                                Expression::Identifier(_)
                                | Expression::Call(_)
                                | Expression::Access(AccessExpression::Tuple(_)) => {
                                    match self.scope_state.call_location.clone() {
                                        Some(location) => {
                                            // Get the external program and function name.
                                            input_futures.push(location);
                                            // Get the full inferred type.
                                            inferred_finalize_inputs.push(ty);
                                        }
                                        None => {
                                            self.emit_err(TypeCheckerError::unknown_future_consumed(
                                                argument,
                                                argument.span(),
                                            ));
                                        }
                                    }
                                }
                                _ => {
                                    self.emit_err(TypeCheckerError::unknown_future_consumed(
                                        "unknown",
                                        argument.span(),
                                    ));
                                }
                            }
                        } else {
                            inferred_finalize_inputs.push(ty);
                        }
                    }
                    self.scope_state.is_call = false;

                    // Add the call to the call graph.
                    let caller_name = match self.scope_state.function {
                        None => unreachable!("`self.function` is set every time a function is visited."),
                        Some(func) => func,
                    };

                    // Don't add external functions to call graph. Since imports are acyclic, these can never produce a cycle.
                    if input.program.unwrap() == self.scope_state.program_name.unwrap() {
                        self.call_graph.add_edge(caller_name, ident.name);
                    }

                    // Propagate futures from async functions and transitions.
                    if func.variant.is_async_function() {
                        // Cannot have async calls in a conditional block.
                        if self.scope_state.is_conditional {
                            self.emit_err(TypeCheckerError::async_call_in_conditional(input.span));
                        }

                        // Can only call async functions and external async transitions from an async transition body.
                        if self.scope_state.variant != Some(Variant::AsyncTransition) {
                            self.emit_err(TypeCheckerError::async_call_can_only_be_done_from_async_transition(
                                input.span,
                            ));
                        }

                        if func.variant.is_transition() {
                            // Cannot call an external async transition after having called the async function.
                            if self.scope_state.has_called_finalize {
                                self.emit_err(TypeCheckerError::external_transition_call_must_be_before_finalize(
                                    input.span,
                                ));
                            }
                        } else if func.variant.is_function() {
                            // Can only call an async function once in a transition function body.
                            if self.scope_state.has_called_finalize {
                                self.emit_err(TypeCheckerError::must_call_async_function_once(input.span));
                            }
                            // Check that all futures consumed.
                            if !self.scope_state.futures.is_empty() {
                                self.emit_err(TypeCheckerError::not_all_futures_consumed(
                                    self.scope_state.futures.iter().map(|(f, _)| f.to_string()).join(", "),
                                    input.span,
                                ));
                            }
                            // Add future locations to symbol table. Unwrap safe since insert function into symbol table during previous pass.
                            let mut st = self.symbol_table.borrow_mut();
                            // Insert futures into symbol table.
                            st.insert_futures(input.program.unwrap(), ident.name, input_futures).unwrap();
                            // Link async transition to the async function that finalizes it.
                            st.attach_finalize(
                                self.scope_state.location(),
                                Location::new(self.scope_state.program_name, ident.name),
                            )
                            .unwrap();
                            drop(st);
                            // Create expectation for finalize inputs that will be checked when checking corresponding finalize function signature.
                            self.async_function_input_types.insert(
                                Location::new(self.scope_state.program_name, ident.name),
                                inferred_finalize_inputs.clone(),
                            );

                            // Set scope state flag.
                            self.scope_state.has_called_finalize = true;

                            // Update ret to reflect fully inferred future type.
                            ret = Type::Future(FutureType::new(
                                inferred_finalize_inputs,
                                Some(Location::new(input.program, ident.name)),
                                true,
                            ));

                            // Type check in case the expected type is known.
                            self.assert_and_return_type(ret.clone(), expected, input.span());
                        }
                    }

                    // Set call location so that definition statement knows where future comes from.
                    self.scope_state.call_location = Some(Location::new(input.program, ident.name));

                    Some(ret)
                } else {
                    self.emit_err(TypeCheckerError::unknown_sym("function", ident.name, ident.span()));
                    None
                }
            }
            _ => unreachable!("Parsing guarantees that a function name is always an identifier."),
        }
    }

    fn visit_cast(&mut self, input: &'a CastExpression, expected: &Self::AdditionalInput) -> Self::Output {
        // Check that the target type of the cast expression is a castable type.
        self.assert_castable_type(&Some(input.type_.clone()), input.span());

        // Check that the expression type is a primitive type.
        let expression_type = self.visit_expression(&input.expression, &None);
        self.assert_castable_type(&expression_type, input.expression.span());

        // Check that the expected type matches the target type.
        Some(self.assert_and_return_type(input.type_.clone(), expected, input.span()))
    }

    fn visit_struct_init(&mut self, input: &'a StructExpression, additional: &Self::AdditionalInput) -> Self::Output {
        let struct_ = self.lookup_struct(self.scope_state.program_name, input.name.name).clone();
        if let Some(struct_) = struct_ {
            // Check struct type name.
            let ret = self.check_expected_struct(&struct_, additional, input.name.span());

            // Check number of struct members.
            if struct_.members.len() != input.members.len() {
                self.emit_err(TypeCheckerError::incorrect_num_struct_members(
                    struct_.members.len(),
                    input.members.len(),
                    input.span(),
                ));
            }

            // Check struct member types.
            struct_.members.iter().for_each(|Member { identifier, type_, .. }| {
                // Lookup struct variable name.
                if let Some(actual) = input.members.iter().find(|member| member.identifier.name == identifier.name) {
                    match &actual.expression {
                        // If `expression` is None, then the member uses the identifier shorthand, e.g. `Foo { a }`
                        None => self.visit_identifier(&actual.identifier, &Some(type_.clone())),
                        // Otherwise, visit the associated expression.
                        Some(expr) => self.visit_expression(expr, &Some(type_.clone())),
                    };
                } else {
                    self.emit_err(TypeCheckerError::missing_struct_member(
                        struct_.identifier,
                        identifier,
                        input.span(),
                    ));
                };
            });

            Some(ret)
        } else {
            self.emit_err(TypeCheckerError::unknown_sym("struct", input.name.name, input.name.span()));
            None
        }
    }

    // We do not want to panic on `ErrExpression`s in order to propagate as many errors as possible.
    fn visit_err(&mut self, _input: &'a ErrExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Default::default()
    }

    fn visit_identifier(&mut self, input: &'a Identifier, expected: &Self::AdditionalInput) -> Self::Output {
        let var = self.symbol_table.borrow().lookup_variable(Location::new(None, input.name)).cloned();
        if let Some(var) = &var {
            if matches!(var.type_, Type::Future(_)) && matches!(expected, Some(Type::Future(_))) {
                if self.scope_state.variant == Some(Variant::AsyncTransition) && self.scope_state.is_call {
                    // Consume future.
                    match self.scope_state.futures.remove(&input.name) {
                        Some(future) => {
                            self.scope_state.call_location = Some(future.clone());
                            return Some(var.type_.clone());
                        }
                        None => {
                            self.emit_err(TypeCheckerError::unknown_future_consumed(input.name, input.span));
                        }
                    }
                } else {
                    // Case where accessing input argument of future. Ex `f.1`.
                    return Some(var.type_.clone());
                }
            }
            Some(self.assert_and_return_type(var.type_.clone(), expected, input.span()))
        } else {
            self.emit_err(TypeCheckerError::unknown_sym("variable", input.name, input.span()));
            None
        }
    }

    fn visit_literal(&mut self, input: &'a Literal, expected: &Self::AdditionalInput) -> Self::Output {
        fn parse_integer_literal<I: FromStr>(handler: &Handler, raw_string: &str, span: Span, type_string: &str) {
            let string = raw_string.replace('_', "");
            if string.parse::<I>().is_err() {
                handler.emit_err(TypeCheckerError::invalid_int_value(string, type_string, span));
            }
        }

        Some(match input {
            Literal::Address(_, _, _) => self.assert_and_return_type(Type::Address, expected, input.span()),
            Literal::Boolean(_, _, _) => self.assert_and_return_type(Type::Boolean, expected, input.span()),
            Literal::Field(_, _, _) => self.assert_and_return_type(Type::Field, expected, input.span()),
            Literal::Integer(integer_type, string, _, _) => match integer_type {
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
            Literal::Scalar(_, _, _) => self.assert_and_return_type(Type::Scalar, expected, input.span()),
            Literal::String(_, _, _) => {
                self.emit_err(TypeCheckerError::strings_are_not_supported(input.span()));
                self.assert_and_return_type(Type::String, expected, input.span())
            }
        })
    }

    fn visit_locator(&mut self, input: &'a LocatorExpression, expected: &Self::AdditionalInput) -> Self::Output {
        // Check that the locator points to a valid resource in the ST.
        let loc_: VariableSymbol;
        if let Some(var) =
            self.symbol_table.borrow().lookup_variable(Location::new(Some(input.program.name.name), input.name))
        {
            loc_ = var.clone();
        } else {
            self.emit_err(TypeCheckerError::unknown_sym("variable", input.name, input.span()));
            return None;
        }
        Some(self.assert_and_return_type(loc_.type_.clone(), expected, input.span()))
    }

    fn visit_ternary(&mut self, input: &'a TernaryExpression, expected: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.condition, &Some(Type::Boolean));

        let t1 = self.visit_expression(&input.if_true, expected);
        let t2 = self.visit_expression(&input.if_false, expected);

        return_incorrect_type(t1, t2, expected)
    }

    fn visit_tuple(&mut self, input: &'a TupleExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match input.elements.len() {
            0 | 1 => unreachable!("Parsing guarantees that tuple expressions have at least two elements."),
            _ => {
                // Check the expected tuple types if they are known.
                if let Some(Type::Tuple(expected_types)) = expected {
                    // Check actual length is equal to expected length.
                    if expected_types.length() != input.elements.len() {
                        self.emit_err(TypeCheckerError::incorrect_tuple_length(
                            expected_types.length(),
                            input.elements.len(),
                            input.span(),
                        ));
                    }

                    expected_types.elements().iter().zip(input.elements.iter()).for_each(|(expected, expr)| {
                        // Check that the component expression is not a tuple.
                        if matches!(expr, Expression::Tuple(_)) {
                            self.emit_err(TypeCheckerError::nested_tuple_expression(expr.span()))
                        }
                        self.visit_expression(expr, &Some(expected.clone()));
                    });

                    Some(Type::Tuple(expected_types.clone()))
                } else {
                    // Tuples must be explicitly typed.
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
            UnaryOperation::ToXCoordinate | UnaryOperation::ToYCoordinate => {
                // Only field type.
                self.assert_field_type(destination, input.span());
                self.visit_expression(&input.receiver, &Some(Type::Group))
            }
        }
    }

    fn visit_unit(&mut self, input: &'a UnitExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        // Unit expression are only allowed inside a return statement.
        if !self.scope_state.is_return {
            self.emit_err(TypeCheckerError::unit_expression_only_in_return_statements(input.span()));
        }
        Some(Type::Unit)
    }
}
