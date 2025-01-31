// Copyright (C) 2019-2025 Provable Inc.
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

use crate::TypeChecker;

use leo_ast::*;
use leo_errors::{TypeCheckerError, emitter::Handler};
use leo_span::{Span, Symbol, sym};

use itertools::Itertools as _;

impl ExpressionVisitor for TypeChecker<'_> {
    type AdditionalInput = Option<Type>;
    type Output = Type;

    fn visit_expression(&mut self, input: &Expression, additional: &Self::AdditionalInput) -> Self::Output {
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
        // Add the expression and its associated type to the symbol table.
        self.type_table.insert(input.id(), output.clone());
        output
    }

    fn visit_access(&mut self, input: &AccessExpression, expected: &Self::AdditionalInput) -> Self::Output {
        match input {
            AccessExpression::Array(access) => {
                // Check that the expression is an array.
                let this_type = self.visit_expression(&access.array, &None);
                self.assert_array_type(&this_type, access.array.span());

                // Check that the index is an integer type.
                let index_type = self.visit_expression(&access.index, &None);
                self.assert_int_type(&index_type, access.index.span());

                // Get the element type of the array.
                let Type::Array(array_type) = this_type else {
                    // We must have already reported an error above, in our type assertion.
                    return Type::Err;
                };

                let element_type = array_type.element_type();

                // If the expected type is known, then check that the element type is the same as the expected type.
                self.maybe_assert_type(element_type, expected, input.span());

                // Return the element type of the array.
                element_type.clone()
            }
            AccessExpression::AssociatedFunction(access) => {
                // Check core struct name and function.
                let Some(core_instruction) = self.get_core_function_call(&access.variant, &access.name) else {
                    self.emit_err(TypeCheckerError::invalid_core_function_call(access, access.span()));
                    return Type::Err;
                };
                // Check that operation is not restricted to finalize blocks.
                if self.scope_state.variant != Some(Variant::AsyncFunction) && core_instruction.is_finalize_command() {
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
                self.maybe_assert_type(&return_type, expected, input.span());

                // Await futures here so that can use the argument variable names to lookup.
                if core_instruction == CoreFunction::FutureAwait && access.arguments.len() != 1 {
                    self.emit_err(TypeCheckerError::can_only_await_one_future_at_a_time(access.span));
                }

                return_type
            }
            AccessExpression::Tuple(access) => {
                let type_ = self.visit_expression(&access.tuple, &None);

                match type_ {
                    Type::Err => Type::Err,
                    Type::Tuple(tuple) => {
                        // Check out of range access.
                        let index = access.index.value();
                        let Some(actual) = tuple.elements().get(index) else {
                            self.emit_err(TypeCheckerError::tuple_out_of_range(index, tuple.length(), access.span()));
                            return Type::Err;
                        };

                        self.maybe_assert_type(actual, expected, access.span());

                        actual.clone()
                    }
                    Type::Future(_) => {
                        // Get the fully inferred type.
                        let Some(Type::Future(inferred_f)) = self.type_table.get(&access.tuple.id()) else {
                            // If a future type was not inferred, we will have already reported an error.
                            return Type::Err;
                        };

                        let Some(actual) = inferred_f.inputs().get(access.index.value()) else {
                            self.emit_err(TypeCheckerError::invalid_future_access(
                                access.index.value(),
                                inferred_f.inputs().len(),
                                access.span(),
                            ));
                            return Type::Err;
                        };

                        // If all inferred types weren't the same, the member will be of type `Type::Err`.
                        if let Type::Err = actual {
                            self.emit_err(TypeCheckerError::future_error_member(access.index.value(), access.span()));
                            return Type::Err;
                        }

                        self.maybe_assert_type(actual, expected, access.span());

                        actual.clone()
                    }
                    type_ => {
                        self.emit_err(TypeCheckerError::type_should_be2(type_, "a tuple or future", access.span()));
                        Type::Err
                    }
                }
            }
            AccessExpression::Member(access) => {
                match *access.inner {
                    // If the access expression is of the form `self.<name>`, then check the <name> is valid.
                    Expression::Identifier(identifier) if identifier.name == sym::SelfLower => match access.name.name {
                        sym::caller => {
                            // Check that the operation is not invoked in a `finalize` block.
                            self.check_access_allowed("self.caller", false, access.name.span());
                            Type::Address
                        }
                        sym::signer => {
                            // Check that operation is not invoked in a `finalize` block.
                            self.check_access_allowed("self.signer", false, access.name.span());
                            Type::Address
                        }
                        _ => {
                            self.emit_err(TypeCheckerError::invalid_self_access(access.name.span()));
                            Type::Err
                        }
                    },
                    // If the access expression is of the form `block.<name>`, then check the <name> is valid.
                    Expression::Identifier(identifier) if identifier.name == sym::block => match access.name.name {
                        sym::height => {
                            // Check that the operation is invoked in a `finalize` block.
                            self.check_access_allowed("block.height", true, access.name.span());
                            let ty = Type::Integer(IntegerType::U32);
                            self.maybe_assert_type(&ty, expected, input.span());
                            ty
                        }
                        _ => {
                            self.emit_err(TypeCheckerError::invalid_block_access(access.name.span()));
                            Type::Err
                        }
                    },
                    // If the access expression is of the form `network.<name>`, then check that the <name> is valid.
                    Expression::Identifier(identifier) if identifier.name == sym::network => match access.name.name {
                        sym::id => {
                            // Check that the operation is not invoked outside a `finalize` block.
                            self.check_access_allowed("network.id", true, access.name.span());
                            let ty = Type::Integer(IntegerType::U16);
                            self.maybe_assert_type(&ty, expected, input.span());
                            ty
                        }
                        _ => {
                            self.emit_err(TypeCheckerError::invalid_block_access(access.name.span()));
                            Type::Err
                        }
                    },
                    _ => {
                        // Check that the type of `inner` in `inner.name` is a struct.
                        let ty = self.visit_expression(&access.inner, &None);
                        match ty {
                            Type::Err => Type::Err,
                            Type::Composite(struct_) => {
                                // Retrieve the struct definition associated with `identifier`.
                                let Some(struct_) = self
                                    .lookup_struct(struct_.program.or(self.scope_state.program_name), struct_.id.name)
                                else {
                                    self.emit_err(TypeCheckerError::undefined_type(ty, access.inner.span()));
                                    return Type::Err;
                                };
                                // Check that `access.name` is a member of the struct.
                                match struct_.members.iter().find(|member| member.name() == access.name.name) {
                                    // Case where `access.name` is a member of the struct.
                                    Some(Member { type_, .. }) => {
                                        // Check that the type of `access.name` is the same as `expected`.
                                        self.maybe_assert_type(type_, expected, access.span());
                                        type_.clone()
                                    }
                                    // Case where `access.name` is not a member of the struct.
                                    None => {
                                        self.emit_err(TypeCheckerError::invalid_struct_variable(
                                            access.name,
                                            &struct_,
                                            access.name.span(),
                                        ));
                                        Type::Err
                                    }
                                }
                            }
                            type_ => {
                                self.emit_err(TypeCheckerError::type_should_be2(
                                    type_,
                                    "a struct or record",
                                    access.inner.span(),
                                ));
                                Type::Err
                            }
                        }
                    }
                }
            }
            AccessExpression::AssociatedConstant(access) => {
                // Check associated constant type and constant name
                let Some(core_constant) = self.get_core_constant(&access.ty, &access.name) else {
                    self.emit_err(TypeCheckerError::invalid_associated_constant(access, access.span));
                    return Type::Err;
                };
                let type_ = core_constant.to_type();
                self.maybe_assert_type(&type_, expected, input.span());
                type_
            }
        }
    }

    fn visit_array(&mut self, input: &ArrayExpression, additional: &Self::AdditionalInput) -> Self::Output {
        if input.elements.is_empty() {
            self.emit_err(TypeCheckerError::array_empty(input.span()));
            return Type::Err;
        }

        let element_type = self.visit_expression(&input.elements[0], &None);

        if input.elements.len() > self.limits.max_array_elements {
            self.emit_err(TypeCheckerError::array_too_large(
                input.elements.len(),
                self.limits.max_array_elements,
                input.span(),
            ));
        }

        if element_type == Type::Err {
            return Type::Err;
        }

        for expression in input.elements[1..].iter() {
            let next_type = self.visit_expression(expression, &None);

            if next_type == Type::Err {
                return Type::Err;
            }

            self.assert_type(&next_type, &element_type, expression.span());
        }

        let type_ = Type::Array(ArrayType::new(element_type, NonNegativeNumber::from(input.elements.len())));

        self.maybe_assert_type(&type_, additional, input.span());

        type_
    }

    fn visit_binary(&mut self, input: &BinaryExpression, destination: &Self::AdditionalInput) -> Self::Output {
        let assert_same_type = |slf: &Self, t1: &Type, t2: &Type| -> Type {
            if t1 == &Type::Err || t2 == &Type::Err {
                Type::Err
            } else if !slf.eq_user(t1, t2) {
                slf.emit_err(TypeCheckerError::operation_types_mismatch(input.op, t1, t2, input.span()));
                Type::Err
            } else {
                t1.clone()
            }
        };

        match input.op {
            BinaryOperation::And | BinaryOperation::Or | BinaryOperation::Nand | BinaryOperation::Nor => {
                self.maybe_assert_type(&Type::Boolean, destination, input.span());
                self.visit_expression(&input.left, &Some(Type::Boolean));
                self.visit_expression(&input.right, &Some(Type::Boolean));
                Type::Boolean
            }
            BinaryOperation::BitwiseAnd | BinaryOperation::BitwiseOr | BinaryOperation::Xor => {
                let t1 = self.visit_expression(&input.left, &None);
                self.assert_bool_int_type(&t1, input.left.span());
                let t2 = self.visit_expression(&input.right, &None);
                self.assert_bool_int_type(&t2, input.right.span());
                let result_t = assert_same_type(self, &t1, &t2);
                self.maybe_assert_type(&result_t, destination, input.span());
                result_t
            }
            BinaryOperation::Add => {
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);
                let assert_add_type = |type_: &Type, span: Span| {
                    if !matches!(type_, Type::Err | Type::Field | Type::Group | Type::Scalar | Type::Integer(_)) {
                        self.emit_err(TypeCheckerError::type_should_be2(
                            type_,
                            "a field, group, scalar, or integer",
                            span,
                        ));
                    }
                };

                assert_add_type(&t1, input.left.span());
                assert_add_type(&t2, input.right.span());

                let result_t = assert_same_type(self, &t1, &t2);

                self.maybe_assert_type(&result_t, destination, input.span());

                result_t
            }
            BinaryOperation::Sub => {
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                self.assert_field_group_int_type(&t1, input.left.span());
                self.assert_field_group_int_type(&t2, input.right.span());

                let result_t = assert_same_type(self, &t1, &t2);

                self.maybe_assert_type(&result_t, destination, input.span());

                result_t
            }
            BinaryOperation::Mul => {
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                let result_t = match (&t1, &t2) {
                    (Type::Err, _) | (_, Type::Err) => Type::Err,
                    (Type::Group, Type::Scalar) | (Type::Scalar, Type::Group) => Type::Group,
                    (Type::Field, Type::Field) => Type::Field,
                    (Type::Integer(integer_type1), Type::Integer(integer_type2)) if integer_type1 == integer_type2 => {
                        t1.clone()
                    }
                    _ => {
                        self.emit_err(TypeCheckerError::mul_types_mismatch(t1, t2, input.span()));
                        Type::Err
                    }
                };

                self.maybe_assert_type(&result_t, destination, input.span());

                result_t
            }
            BinaryOperation::Div => {
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                self.assert_field_int_type(&t1, input.left.span());
                self.assert_field_int_type(&t2, input.right.span());

                let result_t = assert_same_type(self, &t1, &t2);

                self.maybe_assert_type(&result_t, destination, input.span());

                result_t
            }
            BinaryOperation::Rem | BinaryOperation::RemWrapped => {
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                self.assert_int_type(&t1, input.left.span());
                self.assert_int_type(&t2, input.right.span());

                let result_t = assert_same_type(self, &t1, &t2);

                self.maybe_assert_type(&result_t, destination, input.span());

                result_t
            }
            BinaryOperation::Mod => {
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                self.assert_unsigned_type(&t1, input.left.span());
                self.assert_unsigned_type(&t1, input.right.span());

                let result_t = assert_same_type(self, &t1, &t2);

                self.maybe_assert_type(&result_t, destination, input.span());

                result_t
            }
            BinaryOperation::Pow => {
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                let ty = match (&t1, &t2) {
                    (Type::Err, _) | (_, Type::Err) => Type::Err,
                    (Type::Field, Type::Field) => Type::Field,
                    (base @ Type::Integer(_), t2) => {
                        if !matches!(
                            t2,
                            Type::Integer(IntegerType::U8)
                                | Type::Integer(IntegerType::U16)
                                | Type::Integer(IntegerType::U32)
                        ) {
                            self.emit_err(TypeCheckerError::pow_types_mismatch(base, t2, input.span()));
                        }
                        base.clone()
                    }
                    _ => {
                        self.emit_err(TypeCheckerError::pow_types_mismatch(t1, t2, input.span()));
                        Type::Err
                    }
                };

                self.maybe_assert_type(&ty, destination, input.span());

                ty
            }
            BinaryOperation::Eq | BinaryOperation::Neq => {
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                let _ = assert_same_type(self, &t1, &t2);

                self.maybe_assert_type(&Type::Boolean, destination, input.span());

                Type::Boolean
            }
            BinaryOperation::Lt | BinaryOperation::Gt | BinaryOperation::Lte | BinaryOperation::Gte => {
                // Assert left and right are equal field, scalar, or integer types.
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                let assert_compare_type = |type_: &Type, span: Span| {
                    if !matches!(type_, Type::Err | Type::Field | Type::Scalar | Type::Integer(_)) {
                        self.emit_err(TypeCheckerError::type_should_be2(type_, "a field, scalar, or integer", span));
                    }
                };

                assert_compare_type(&t1, input.left.span());
                assert_compare_type(&t2, input.right.span());

                let _ = assert_same_type(self, &t1, &t2);

                self.maybe_assert_type(&Type::Boolean, destination, input.span());

                Type::Boolean
            }
            BinaryOperation::AddWrapped
            | BinaryOperation::SubWrapped
            | BinaryOperation::DivWrapped
            | BinaryOperation::MulWrapped => {
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                self.assert_int_type(&t1, input.left.span());
                self.assert_int_type(&t2, input.right.span());

                let result_t = assert_same_type(self, &t1, &t2);

                self.maybe_assert_type(&result_t, destination, input.span());

                result_t
            }
            BinaryOperation::Shl
            | BinaryOperation::ShlWrapped
            | BinaryOperation::Shr
            | BinaryOperation::ShrWrapped
            | BinaryOperation::PowWrapped => {
                let t1 = self.visit_expression(&input.left, &None);
                let t2 = self.visit_expression(&input.right, &None);

                self.assert_int_type(&t1, input.left.span());

                if !matches!(
                    &t2,
                    Type::Err
                        | Type::Integer(IntegerType::U8)
                        | Type::Integer(IntegerType::U16)
                        | Type::Integer(IntegerType::U32)
                ) {
                    self.emit_err(TypeCheckerError::shift_type_magnitude(input.op, t2, input.right.span()));
                }

                t1
            }
        }
    }

    fn visit_call(&mut self, input: &CallExpression, expected: &Self::AdditionalInput) -> Self::Output {
        // Get the function symbol.
        let Expression::Identifier(ident) = &*input.function else {
            unreachable!("Parsing guarantees that a function name is always an identifier.");
        };

        let callee_program = input.program.or(self.scope_state.program_name).unwrap();

        let Some(func_symbol) = self.symbol_table.lookup_function(Location::new(callee_program, ident.name)) else {
            self.emit_err(TypeCheckerError::unknown_sym("function", ident.name, ident.span()));
            return Type::Err;
        };

        let func = func_symbol.function.clone();

        // Check that the call is valid.
        // We always set the variant before entering the body of a function, so this unwrap works.
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
        if func.variant == Variant::Inline && input.program.unwrap() != self.scope_state.program_name.unwrap() {
            self.emit_err(TypeCheckerError::cannot_call_external_inline_function(input.span));
        }

        // Async functions return a single future.
        let mut ret = if func.variant == Variant::AsyncFunction {
            // Type check after resolving the input types.
            if let Some(Type::Future(_)) = expected {
                Type::Future(FutureType::new(Vec::new(), Some(Location::new(callee_program, ident.name)), false))
            } else {
                self.emit_err(TypeCheckerError::return_type_of_finalize_function_is_future(input.span));
                Type::Unit
            }
        } else if func.variant == Variant::AsyncTransition {
            // Fully infer future type.
            let Some(inputs) = self
                .async_function_input_types
                .get(&Location::new(callee_program, Symbol::intern(&format!("finalize/{}", ident.name))))
            else {
                self.emit_err(TypeCheckerError::async_function_not_found(ident.name, input.span));
                return Type::Future(FutureType::new(
                    Vec::new(),
                    Some(Location::new(callee_program, ident.name)),
                    false,
                ));
            };

            let future_type =
                Type::Future(FutureType::new(inputs.clone(), Some(Location::new(callee_program, ident.name)), true));
            let fully_inferred_type = match &func.output_type {
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
            // Get the type of the expression. If the type is not known, do not attempt to attempt any further inference.
            let ty = self.visit_expression(argument, &Some(expected.type_().clone()));
            if ty == Type::Err {
                return Type::Err;
            }
            // Extract information about futures that are being consumed.
            if func.variant == Variant::AsyncFunction && matches!(expected.type_(), Type::Future(_)) {
                // Consume the future.
                let option_name = match argument {
                    Expression::Identifier(id) => Some(id.name),
                    Expression::Access(AccessExpression::Tuple(tuple_access)) => {
                        if let Expression::Identifier(id) = &*tuple_access.tuple { Some(id.name) } else { None }
                    }
                    _ => None,
                };

                if let Some(name) = option_name {
                    match self.scope_state.futures.shift_remove(&name) {
                        Some(future) => {
                            self.scope_state.call_location = Some(future);
                        }
                        None => {
                            self.emit_err(TypeCheckerError::unknown_future_consumed(name, argument.span()));
                        }
                    }
                }

                match argument {
                    Expression::Identifier(_)
                    | Expression::Call(_)
                    | Expression::Access(AccessExpression::Tuple(_)) => {
                        match self.scope_state.call_location {
                            Some(location) => {
                                // Get the external program and function name.
                                input_futures.push(location);
                                // Get the full inferred type.
                                inferred_finalize_inputs.push(ty);
                            }
                            None => {
                                self.emit_err(TypeCheckerError::unknown_future_consumed(argument, argument.span()));
                            }
                        }
                    }
                    _ => {
                        self.emit_err(TypeCheckerError::unknown_future_consumed("unknown", argument.span()));
                    }
                }
            } else {
                inferred_finalize_inputs.push(ty);
            }
        }
        self.scope_state.is_call = false;

        // Add the call to the call graph.
        let Some(caller_name) = self.scope_state.function else {
            unreachable!("`self.function` is set every time a function is visited.");
        };

        // Don't add external functions to call graph. Since imports are acyclic, these can never produce a cycle.
        if input.program.unwrap() == self.scope_state.program_name.unwrap() {
            self.call_graph.add_edge(caller_name, ident.name);
        }

        if func.variant.is_transition()
            && self.scope_state.variant == Some(Variant::AsyncTransition)
            && self.scope_state.has_called_finalize
        {
            // Cannot call an external async transition after having called the async function.
            self.emit_err(TypeCheckerError::external_transition_call_must_be_before_finalize(input.span));
        }

        // Propagate futures from async functions and transitions.
        if func.variant.is_async_function() {
            // Cannot have async calls in a conditional block.
            if self.scope_state.is_conditional {
                self.emit_err(TypeCheckerError::async_call_in_conditional(input.span));
            }

            // Can only call async functions and external async transitions from an async transition body.
            if self.scope_state.variant != Some(Variant::AsyncTransition) {
                self.emit_err(TypeCheckerError::async_call_can_only_be_done_from_async_transition(input.span));
            }

            // Can only call an async function once in a transition function body.
            if self.scope_state.has_called_finalize {
                self.emit_err(TypeCheckerError::must_call_async_function_once(input.span));
            }
            // Check that all futures consumed.
            if !self.scope_state.futures.is_empty() {
                self.emit_err(TypeCheckerError::not_all_futures_consumed(
                    self.scope_state.futures.iter().map(|(f, _)| f).join(", "),
                    input.span,
                ));
            }
            self.symbol_table
                .attach_finalizer(
                    Location::new(callee_program, caller_name),
                    Location::new(callee_program, ident.name),
                    input_futures,
                    inferred_finalize_inputs.clone(),
                )
                .expect("Failed to attach finalizer");
            // Create expectation for finalize inputs that will be checked when checking corresponding finalize function signature.
            self.async_function_callers
                .entry(Location::new(self.scope_state.program_name.unwrap(), ident.name))
                .or_default()
                .insert(self.scope_state.location());

            // Set scope state flag.
            self.scope_state.has_called_finalize = true;

            // Update ret to reflect fully inferred future type.
            ret = Type::Future(FutureType::new(
                inferred_finalize_inputs,
                Some(Location::new(callee_program, ident.name)),
                true,
            ));

            // Type check in case the expected type is known.
            self.assert_and_return_type(ret.clone(), expected, input.span());
        }

        // Set call location so that definition statement knows where future comes from.
        self.scope_state.call_location = Some(Location::new(callee_program, ident.name));

        ret
    }

    fn visit_cast(&mut self, input: &CastExpression, expected: &Self::AdditionalInput) -> Self::Output {
        let expression_type = self.visit_expression(&input.expression, &None);

        let assert_castable_type = |actual: &Type, span: Span| {
            if !matches!(
                actual,
                Type::Integer(_) | Type::Boolean | Type::Field | Type::Group | Type::Scalar | Type::Address | Type::Err,
            ) {
                self.emit_err(TypeCheckerError::type_should_be2(
                    actual,
                    "an integer, bool, field, group, scalar, or address",
                    span,
                ));
            }
        };

        assert_castable_type(&input.type_, input.span());

        assert_castable_type(&expression_type, input.expression.span());

        self.maybe_assert_type(&input.type_, expected, input.span());

        input.type_.clone()
    }

    fn visit_struct_init(&mut self, input: &StructExpression, additional: &Self::AdditionalInput) -> Self::Output {
        let struct_ = self.lookup_struct(self.scope_state.program_name, input.name.name).clone();
        let Some(struct_) = struct_ else {
            self.emit_err(TypeCheckerError::unknown_sym("struct or record", input.name.name, input.name.span()));
            return Type::Err;
        };

        // Note that it is sufficient for the `program` to be `None` as composite types can only be initialized
        // in the program in which they are defined.
        let type_ = Type::Composite(CompositeType { id: input.name, program: None });
        self.maybe_assert_type(&type_, additional, input.name.span());

        // Check number of struct members.
        if struct_.members.len() != input.members.len() {
            self.emit_err(TypeCheckerError::incorrect_num_struct_members(
                struct_.members.len(),
                input.members.len(),
                input.span(),
            ));
        }

        for Member { identifier, type_, .. } in struct_.members.iter() {
            if let Some(actual) = input.members.iter().find(|member| member.identifier.name == identifier.name) {
                match &actual.expression {
                    // If `expression` is None, then the member uses the identifier shorthand, e.g. `Foo { a }`
                    None => self.visit_identifier(&actual.identifier, &Some(type_.clone())),
                    // Otherwise, visit the associated expression.
                    Some(expr) => self.visit_expression(expr, &Some(type_.clone())),
                };
            } else {
                self.emit_err(TypeCheckerError::missing_struct_member(struct_.identifier, identifier, input.span()));
            };
        }

        type_
    }

    // We do not want to panic on `ErrExpression`s in order to propagate as many errors as possible.
    fn visit_err(&mut self, _input: &ErrExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Type::Err
    }

    fn visit_identifier(&mut self, input: &Identifier, expected: &Self::AdditionalInput) -> Self::Output {
        let var = self.symbol_table.lookup_variable(self.scope_state.program_name.unwrap(), input.name);
        if let Some(var) = var {
            self.maybe_assert_type(&var.type_, expected, input.span());
            var.type_.clone()
        } else {
            self.emit_err(TypeCheckerError::unknown_sym("variable", input.name, input.span()));
            Type::Err
        }
    }

    fn visit_literal(&mut self, input: &Literal, expected: &Self::AdditionalInput) -> Self::Output {
        fn parse_integer_literal<I: FromStrRadix>(handler: &Handler, raw_string: &str, span: Span, type_string: &str) {
            let string = raw_string.replace('_', "");
            if I::from_str_by_radix(&string).is_err() {
                handler.emit_err(TypeCheckerError::invalid_int_value(string, type_string, span));
            }
        }

        let type_ = match input {
            Literal::Address(_, _, _) => Type::Address,
            Literal::Boolean(_, _, _) => Type::Boolean,
            Literal::Field(_, _, _) => Type::Field,
            Literal::Integer(IntegerType::U8, string, ..) => {
                parse_integer_literal::<u8>(self.handler, string, input.span(), "u8");
                Type::Integer(IntegerType::U8)
            }
            Literal::Integer(IntegerType::U16, string, ..) => {
                parse_integer_literal::<u16>(self.handler, string, input.span(), "u16");
                Type::Integer(IntegerType::U16)
            }
            Literal::Integer(IntegerType::U32, string, ..) => {
                parse_integer_literal::<u32>(self.handler, string, input.span(), "u32");
                Type::Integer(IntegerType::U32)
            }
            Literal::Integer(IntegerType::U64, string, ..) => {
                parse_integer_literal::<u64>(self.handler, string, input.span(), "u64");
                Type::Integer(IntegerType::U64)
            }
            Literal::Integer(IntegerType::U128, string, ..) => {
                parse_integer_literal::<u128>(self.handler, string, input.span(), "u128");
                Type::Integer(IntegerType::U128)
            }
            Literal::Integer(IntegerType::I8, string, ..) => {
                parse_integer_literal::<i8>(self.handler, string, input.span(), "i8");
                Type::Integer(IntegerType::I8)
            }
            Literal::Integer(IntegerType::I16, string, ..) => {
                parse_integer_literal::<i16>(self.handler, string, input.span(), "i16");
                Type::Integer(IntegerType::I16)
            }
            Literal::Integer(IntegerType::I32, string, ..) => {
                parse_integer_literal::<i32>(self.handler, string, input.span(), "i32");
                Type::Integer(IntegerType::I32)
            }
            Literal::Integer(IntegerType::I64, string, ..) => {
                parse_integer_literal::<i64>(self.handler, string, input.span(), "i64");
                Type::Integer(IntegerType::I64)
            }
            Literal::Integer(IntegerType::I128, string, ..) => {
                parse_integer_literal::<i128>(self.handler, string, input.span(), "i128");
                Type::Integer(IntegerType::I128)
            }
            Literal::Group(..) => Type::Group,
            Literal::Scalar(..) => Type::Scalar,
            Literal::String(..) => {
                self.emit_err(TypeCheckerError::strings_are_not_supported(input.span()));
                Type::String
            }
        };

        self.maybe_assert_type(&type_, expected, input.span());

        type_
    }

    fn visit_locator(&mut self, input: &LocatorExpression, expected: &Self::AdditionalInput) -> Self::Output {
        let maybe_var = self.symbol_table.lookup_global(Location::new(input.program.name.name, input.name)).cloned();
        if let Some(var) = maybe_var {
            self.maybe_assert_type(&var.type_, expected, input.span());
            var.type_
        } else {
            self.emit_err(TypeCheckerError::unknown_sym("variable", input.name, input.span()));
            Type::Err
        }
    }

    fn visit_ternary(&mut self, input: &TernaryExpression, expected: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.condition, &Some(Type::Boolean));

        let t1 = self.visit_expression(&input.if_true, expected);
        let t2 = self.visit_expression(&input.if_false, expected);

        if t1 == Type::Err || t2 == Type::Err {
            Type::Err
        } else if !self.eq_user(&t1, &t2) {
            self.emit_err(TypeCheckerError::ternary_branch_mismatch(t1, t2, input.span()));
            Type::Err
        } else {
            t1
        }
    }

    fn visit_tuple(&mut self, input: &TupleExpression, expected: &Self::AdditionalInput) -> Self::Output {
        // Check the expected tuple types if they are known.
        let Some(Type::Tuple(expected_types)) = expected else {
            self.emit_err(TypeCheckerError::invalid_tuple(input.span()));
            return Type::Err;
        };

        // Check actual length is equal to expected length.
        if expected_types.length() != input.elements.len() {
            self.emit_err(TypeCheckerError::incorrect_tuple_length(
                expected_types.length(),
                input.elements.len(),
                input.span(),
            ));
        }

        for (expr, expected) in input.elements.iter().zip(expected_types.elements().iter()) {
            if matches!(expr, Expression::Tuple(_)) {
                self.emit_err(TypeCheckerError::nested_tuple_expression(expr.span()))
            }

            self.visit_expression(expr, &Some(expected.clone()));
        }

        Type::Tuple(expected_types.clone())
    }

    fn visit_unary(&mut self, input: &UnaryExpression, destination: &Self::AdditionalInput) -> Self::Output {
        let assert_signed_int = |slf: &mut Self, type_: &Type| {
            if !matches!(
                type_,
                Type::Err
                    | Type::Integer(IntegerType::I8)
                    | Type::Integer(IntegerType::I16)
                    | Type::Integer(IntegerType::I32)
                    | Type::Integer(IntegerType::I64)
                    | Type::Integer(IntegerType::I128)
            ) {
                slf.emit_err(TypeCheckerError::type_should_be2(type_, "a signed integer", input.span()));
            }
        };

        let ty = match input.op {
            UnaryOperation::Abs => {
                let type_ = self.visit_expression(&input.receiver, &None);
                assert_signed_int(self, &type_);
                type_
            }
            UnaryOperation::AbsWrapped => {
                let type_ = self.visit_expression(&input.receiver, &None);
                assert_signed_int(self, &type_);
                type_
            }
            UnaryOperation::Double => {
                let type_ = self.visit_expression(&input.receiver, &None);
                if !matches!(&type_, Type::Err | Type::Field | Type::Group) {
                    self.emit_err(TypeCheckerError::type_should_be2(&type_, "a field or group", input.span()));
                }
                type_
            }
            UnaryOperation::Inverse => {
                let type_ = self.visit_expression(&input.receiver, &None);
                self.assert_type(&type_, &Type::Field, input.span());
                type_
            }
            UnaryOperation::Negate => {
                let type_ = self.visit_expression(&input.receiver, &None);
                if !matches!(
                    &type_,
                    Type::Err
                        | Type::Integer(IntegerType::I8)
                        | Type::Integer(IntegerType::I16)
                        | Type::Integer(IntegerType::I32)
                        | Type::Integer(IntegerType::I64)
                        | Type::Integer(IntegerType::I128)
                        | Type::Group
                        | Type::Field
                ) {
                    self.emit_err(TypeCheckerError::type_should_be2(
                        &type_,
                        "a signed integer, group, or field",
                        input.receiver.span(),
                    ));
                }
                type_
            }
            UnaryOperation::Not => {
                let type_ = self.visit_expression(&input.receiver, &None);
                if !matches!(&type_, Type::Err | Type::Boolean | Type::Integer(_)) {
                    self.emit_err(TypeCheckerError::type_should_be2(&type_, "a bool or integer", input.span()));
                }
                type_
            }
            UnaryOperation::Square => {
                let type_ = self.visit_expression(&input.receiver, &None);
                self.assert_type(&type_, &Type::Field, input.span());
                type_
            }
            UnaryOperation::SquareRoot => {
                let type_ = self.visit_expression(&input.receiver, &None);
                self.assert_type(&type_, &Type::Field, input.span());
                type_
            }
            UnaryOperation::ToXCoordinate | UnaryOperation::ToYCoordinate => {
                let _operand_type = self.visit_expression(&input.receiver, &Some(Type::Group));
                self.maybe_assert_type(&Type::Field, destination, input.span());
                Type::Field
            }
        };

        self.maybe_assert_type(&ty, destination, input.span());

        ty
    }

    fn visit_unit(&mut self, _input: &UnitExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Type::Unit
    }
}
