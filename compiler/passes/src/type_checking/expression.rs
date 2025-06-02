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

use super::TypeCheckingVisitor;
use crate::VariableType;

use leo_ast::*;
use leo_errors::{Handler, TypeCheckerError, TypeCheckerWarning};
use leo_span::{Span, Symbol, sym};

use itertools::Itertools as _;

impl TypeCheckingVisitor<'_> {
    pub fn visit_expression_assign(&mut self, input: &Expression) -> Type {
        let ty = match input {
            Expression::ArrayAccess(array_access) => self.visit_array_access_general(array_access, true, &None),
            Expression::Identifier(ident) => self.visit_identifier_assign(ident),
            Expression::MemberAccess(member_access) => self.visit_member_access_general(member_access, true, &None),
            Expression::TupleAccess(tuple_access) => self.visit_tuple_access_general(tuple_access, true, &None),
            _ => {
                self.emit_err(TypeCheckerError::invalid_assignment_target(input, input.span()));
                Type::Err
            }
        };

        // Prohibit assignment to an external record or a member thereof.
        // This is necessary as an assignment in a conditional branch would become a
        // ternary, which can't happen.
        if self.is_external_record(&ty) {
            self.emit_err(TypeCheckerError::assignment_to_external_record(&ty, input.span()));
        }

        // Similarly prohibit assignment to a tuple with an external record member.
        if let Type::Tuple(tuple) = &ty {
            if tuple.elements().iter().any(|ty| self.is_external_record(ty)) {
                self.emit_err(TypeCheckerError::assignment_to_external_record(&ty, input.span()));
            }
        }

        // Prohibit reassignment of futures.
        if let Type::Future(..) = ty {
            self.emit_err(TypeCheckerError::cannot_reassign_future_variable(input, input.span()));
        }

        // Prohibit reassignment of mappings.
        if let Type::Mapping(_) = ty {
            self.emit_err(TypeCheckerError::cannot_reassign_mapping(input, input.span()));
        }

        // Add the expression and its associated type to the type table.
        self.state.type_table.insert(input.id(), ty.clone());
        ty
    }

    fn visit_array_access_general(&mut self, input: &ArrayAccess, assign: bool, expected: &Option<Type>) -> Type {
        // Check that the expression is an array.
        let this_type = if assign {
            self.visit_expression_assign(&input.array)
        } else {
            self.visit_expression(&input.array, &None)
        };
        self.assert_array_type(&this_type, input.array.span());

        // Check that the index is an integer type.
        let index_type = self.visit_expression(&input.index, &None);
        self.assert_int_type(&index_type, input.index.span());

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

    fn visit_member_access_general(&mut self, input: &MemberAccess, assign: bool, expected: &Option<Type>) -> Type {
        // Handler member access expressions that correspond to valid operands in AVM code.
        if !assign {
            match input.inner {
                // If the access expression is of the form `self.<name>`, then check the <name> is valid.
                Expression::Identifier(identifier) if identifier.name == sym::SelfLower => {
                    match input.name.name {
                        sym::caller => {
                            // Check that the operation is not invoked in a `finalize` block.
                            self.check_access_allowed("self.caller", false, input.name.span());
                            let ty = Type::Address;
                            self.maybe_assert_type(&ty, expected, input.span());
                            return ty;
                        }
                        sym::signer => {
                            // Check that operation is not invoked in a `finalize` block.
                            self.check_access_allowed("self.signer", false, input.name.span());
                            let ty = Type::Address;
                            self.maybe_assert_type(&ty, expected, input.span());
                            return ty;
                        }
                        _ => {
                            self.emit_err(TypeCheckerError::invalid_self_access(input.name.span()));
                            return Type::Err;
                        }
                    }
                }
                // If the access expression is of the form `block.<name>`, then check the <name> is valid.
                Expression::Identifier(identifier) if identifier.name == sym::block => match input.name.name {
                    sym::height => {
                        // Check that the operation is invoked in a `finalize` block.
                        self.check_access_allowed("block.height", true, input.name.span());
                        let ty = Type::Integer(IntegerType::U32);
                        self.maybe_assert_type(&ty, expected, input.span());
                        return ty;
                    }
                    _ => {
                        self.emit_err(TypeCheckerError::invalid_block_access(input.name.span()));
                        return Type::Err;
                    }
                },
                // If the access expression is of the form `network.<name>`, then check that the <name> is valid.
                Expression::Identifier(identifier) if identifier.name == sym::network => match input.name.name {
                    sym::id => {
                        // Check that the operation is not invoked outside a `finalize` block.
                        self.check_access_allowed("network.id", true, input.name.span());
                        let ty = Type::Integer(IntegerType::U16);
                        self.maybe_assert_type(&ty, expected, input.span());
                        return ty;
                    }
                    _ => {
                        self.emit_err(TypeCheckerError::invalid_block_access(input.name.span()));
                        return Type::Err;
                    }
                },
                _ => {}
            }
        }

        let ty = if assign {
            self.visit_expression_assign(&input.inner)
        } else {
            self.visit_expression(&input.inner, &None)
        };

        // Check that the type of `inner` in `inner.name` is a struct.
        match ty {
            Type::Err => Type::Err,
            Type::Composite(struct_) => {
                // Retrieve the struct definition associated with `identifier`.
                let Some(struct_) =
                    self.lookup_struct(struct_.program.or(self.scope_state.program_name), struct_.id.name)
                else {
                    self.emit_err(TypeCheckerError::undefined_type(ty, input.inner.span()));
                    return Type::Err;
                };
                // Check that `input.name` is a member of the struct.
                match struct_.members.iter().find(|member| member.name() == input.name.name) {
                    // Case where `input.name` is a member of the struct.
                    Some(Member { type_, .. }) => {
                        // Check that the type of `input.name` is the same as `expected`.
                        self.maybe_assert_type(type_, expected, input.span());
                        type_.clone()
                    }
                    // Case where `input.name` is not a member of the struct.
                    None => {
                        self.emit_err(TypeCheckerError::invalid_struct_variable(
                            input.name,
                            &struct_,
                            input.name.span(),
                        ));
                        Type::Err
                    }
                }
            }
            type_ => {
                self.emit_err(TypeCheckerError::type_should_be2(type_, "a struct or record", input.inner.span()));
                Type::Err
            }
        }
    }

    fn visit_tuple_access_general(&mut self, input: &TupleAccess, assign: bool, expected: &Option<Type>) -> Type {
        let this_type = if assign {
            self.visit_expression_assign(&input.tuple)
        } else {
            self.visit_expression(&input.tuple, &None)
        };
        match this_type {
            Type::Err => Type::Err,
            Type::Tuple(tuple) => {
                // Check out of range input.
                let index = input.index.value();
                let Some(actual) = tuple.elements().get(index) else {
                    self.emit_err(TypeCheckerError::tuple_out_of_range(index, tuple.length(), input.span()));
                    return Type::Err;
                };

                self.maybe_assert_type(actual, expected, input.span());

                actual.clone()
            }
            Type::Future(_) => {
                // Get the fully inferred type.
                let Some(Type::Future(inferred_f)) = self.state.type_table.get(&input.tuple.id()) else {
                    // If a future type was not inferred, we will have already reported an error.
                    return Type::Err;
                };

                let Some(actual) = inferred_f.inputs().get(input.index.value()) else {
                    self.emit_err(TypeCheckerError::invalid_future_access(
                        input.index.value(),
                        inferred_f.inputs().len(),
                        input.span(),
                    ));
                    return Type::Err;
                };

                // If all inferred types weren't the same, the member will be of type `Type::Err`.
                if let Type::Err = actual {
                    self.emit_err(TypeCheckerError::future_error_member(input.index.value(), input.span()));
                    return Type::Err;
                }

                self.maybe_assert_type(actual, expected, input.span());

                actual.clone()
            }
            type_ => {
                self.emit_err(TypeCheckerError::type_should_be2(type_, "a tuple or future", input.span()));
                Type::Err
            }
        }
    }

    fn visit_identifier_assign(&mut self, input: &Identifier) -> Type {
        // Lookup the variable in the symbol table and retrieve its type.
        let Some(var) = self.state.symbol_table.lookup_variable(self.scope_state.program_name.unwrap(), input.name)
        else {
            self.emit_err(TypeCheckerError::unknown_sym("variable", input.name, input.span));
            return Type::Err;
        };

        // If the variable exists, then check that it is not a constant.
        match &var.declaration {
            VariableType::Const => self.emit_err(TypeCheckerError::cannot_assign_to_const_var(input, var.span)),
            VariableType::Input(Mode::Constant) => {
                self.emit_err(TypeCheckerError::cannot_assign_to_const_input(input, var.span))
            }
            VariableType::Mut | VariableType::Input(_) => {}
        }

        // If the variable exists and it's in an async function, then check that it is in the current conditional scope.
        if self.scope_state.variant.unwrap().is_async_function() && !self.symbol_in_conditional_scope(input.name) {
            self.emit_err(TypeCheckerError::async_cannot_assign_outside_conditional(input, var.span));
        }

        var.type_.clone()
    }
}

impl ExpressionVisitor for TypeCheckingVisitor<'_> {
    type AdditionalInput = Option<Type>;
    type Output = Type;

    fn visit_expression(&mut self, input: &Expression, additional: &Self::AdditionalInput) -> Self::Output {
        let output = match input {
            Expression::Array(array) => self.visit_array(array, additional),
            Expression::ArrayAccess(access) => self.visit_array_access_general(access, false, additional),
            Expression::AssociatedConstant(constant) => self.visit_associated_constant(constant, additional),
            Expression::AssociatedFunction(function) => self.visit_associated_function(function, additional),
            Expression::Binary(binary) => self.visit_binary(binary, additional),
            Expression::Call(call) => self.visit_call(call, additional),
            Expression::Cast(cast) => self.visit_cast(cast, additional),
            Expression::Struct(struct_) => self.visit_struct_init(struct_, additional),
            Expression::Err(err) => self.visit_err(err, additional),
            Expression::Identifier(identifier) => self.visit_identifier(identifier, additional),
            Expression::Literal(literal) => self.visit_literal(literal, additional),
            Expression::Locator(locator) => self.visit_locator(locator, additional),
            Expression::MemberAccess(access) => self.visit_member_access_general(access, false, additional),
            Expression::Ternary(ternary) => self.visit_ternary(ternary, additional),
            Expression::Tuple(tuple) => self.visit_tuple(tuple, additional),
            Expression::TupleAccess(access) => self.visit_tuple_access_general(access, false, additional),
            Expression::Unary(unary) => self.visit_unary(unary, additional),
            Expression::Unit(unit) => self.visit_unit(unit, additional),
        };
        // Add the expression and its associated type to the symbol table.
        self.state.type_table.insert(input.id(), output.clone());
        output
    }

    fn visit_array_access(&mut self, _input: &ArrayAccess, _additional: &Self::AdditionalInput) -> Self::Output {
        panic!("Should not be called.");
    }

    fn visit_member_access(&mut self, _input: &MemberAccess, _additional: &Self::AdditionalInput) -> Self::Output {
        panic!("Should not be called.");
    }

    fn visit_tuple_access(&mut self, _input: &TupleAccess, _additional: &Self::AdditionalInput) -> Self::Output {
        panic!("Should not be called.");
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

    fn visit_associated_constant(
        &mut self,
        input: &AssociatedConstantExpression,
        expected: &Self::AdditionalInput,
    ) -> Self::Output {
        // Check associated constant type and constant name
        let Some(core_constant) = self.get_core_constant(&input.ty, &input.name) else {
            self.emit_err(TypeCheckerError::invalid_associated_constant(input, input.span));
            return Type::Err;
        };
        let type_ = core_constant.to_type();
        self.maybe_assert_type(&type_, expected, input.span());
        type_
    }

    fn visit_associated_function(
        &mut self,
        input: &AssociatedFunctionExpression,
        expected: &Self::AdditionalInput,
    ) -> Self::Output {
        // Check core struct name and function.
        let Some(core_instruction) = self.get_core_function_call(&input.variant, &input.name) else {
            self.emit_err(TypeCheckerError::invalid_core_function_call(input, input.span()));
            return Type::Err;
        };
        // Check that operation is not restricted to finalize blocks.
        if !matches!(self.scope_state.variant, Some(Variant::AsyncFunction) | Some(Variant::Script))
            && core_instruction.is_finalize_command()
        {
            self.emit_err(TypeCheckerError::operation_must_be_in_finalize_block(input.span()));
        }

        // Get the types of the arguments.
        let argument_types =
            input.arguments.iter().map(|arg| (self.visit_expression(arg, &None), arg.span())).collect::<Vec<_>>();

        // Check that the types of the arguments are valid.
        let return_type = self.check_core_function_call(core_instruction.clone(), &argument_types, input.span());

        // Check return type if the expected type is known.
        self.maybe_assert_type(&return_type, expected, input.span());

        // Await futures here so that can use the argument variable names to lookup.
        if core_instruction == CoreFunction::FutureAwait && input.arguments.len() != 1 {
            self.emit_err(TypeCheckerError::can_only_await_one_future_at_a_time(input.span));
        }

        return_type
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
        let Expression::Identifier(ident) = &input.function else {
            panic!("Parsing guarantees that a function name is always an identifier.");
        };

        let callee_program = input.program.or(self.scope_state.program_name).unwrap();

        let Some(func_symbol) = self.state.symbol_table.lookup_function(Location::new(callee_program, ident.name))
        else {
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
            let actual =
                Type::Future(FutureType::new(Vec::new(), Some(Location::new(callee_program, ident.name)), false));
            match expected {
                Some(Type::Future(_)) | None => {
                    // If the expected type is a `Future` or if it's not set, then just return the
                    // actual type of the future from the expression itself
                    actual
                }
                Some(_) => {
                    // Otherwise, error out. There is a mismatch in types.
                    self.maybe_assert_type(&actual, expected, input.span());
                    Type::Unit
                }
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
                let option_name =
                    match argument {
                        Expression::Identifier(id) => Some(id.name),
                        Expression::TupleAccess(tuple_access) => {
                            if let Expression::Identifier(id) = &tuple_access.tuple { Some(id.name) } else { None }
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
                    Expression::Identifier(_) | Expression::Call(_) | Expression::TupleAccess(_) => {
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
            panic!("`self.function` is set every time a function is visited.");
        };

        // Don't add external functions to call graph. Since imports are acyclic, these can never produce a cycle.
        if input.program.unwrap() == self.scope_state.program_name.unwrap() {
            self.state.call_graph.add_edge(caller_name, ident.name);
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
            if !matches!(self.scope_state.variant, Some(Variant::AsyncTransition) | Some(Variant::Script)) {
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
            self.state
                .symbol_table
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
                    None => {
                        // If `expression` is None, then the member uses the identifier shorthand, e.g. `Foo { a }`
                        // We visit it as an expression rather than just calling `visit_identifier` so it will get
                        // put into the type table.
                        self.visit_expression(&actual.identifier.into(), &Some(type_.clone()));
                    }
                    Some(expr) => {
                        // Otherwise, visit the associated expression.
                        self.visit_expression(expr, &Some(type_.clone()));
                    }
                };
            } else {
                self.emit_err(TypeCheckerError::missing_struct_member(struct_.identifier, identifier, input.span()));
            };
        }

        if struct_.is_record {
            // First, ensure that the current scope is not an async function. Records should not be instantiated in
            // async functions
            if self.scope_state.variant == Some(Variant::AsyncFunction) {
                self.state.handler.emit_err(TypeCheckerError::records_not_allowed_inside_finalize(input.span()));
            }

            // Records where the `owner` is `self.caller` can be problematic because `self.caller` can be a program
            // address and programs can't spend records. Emit a warning in this case.
            //
            // Multiple occurrences of `owner` here is an error but that should be flagged somewhere else.
            input.members.iter().filter(|init| init.identifier.name == sym::owner).for_each(|init| {
                if let Some(Expression::MemberAccess(access)) = &init.expression {
                    if let MemberAccess {
                        inner: Expression::Identifier(Identifier { name: sym::SelfLower, .. }),
                        name: Identifier { name: sym::caller, .. },
                        ..
                    } = &**access
                    {
                        self.emit_warning(TypeCheckerWarning::caller_as_record_owner(input.name, access.span()));
                    }
                }
            });
        }

        type_
    }

    // We do not want to panic on `ErrExpression`s in order to propagate as many errors as possible.
    fn visit_err(&mut self, _input: &ErrExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Type::Err
    }

    fn visit_identifier(&mut self, input: &Identifier, expected: &Self::AdditionalInput) -> Self::Output {
        let var = self.state.symbol_table.lookup_variable(self.scope_state.program_name.unwrap(), input.name);
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

        let type_ = match &input.variant {
            LiteralVariant::Address(..) => Type::Address,
            LiteralVariant::Boolean(..) => Type::Boolean,
            LiteralVariant::Field(..) => Type::Field,
            LiteralVariant::Integer(IntegerType::U8, string) => {
                parse_integer_literal::<u8>(&self.state.handler, string, input.span(), "u8");
                Type::Integer(IntegerType::U8)
            }
            LiteralVariant::Integer(IntegerType::U16, string) => {
                parse_integer_literal::<u16>(&self.state.handler, string, input.span(), "u16");
                Type::Integer(IntegerType::U16)
            }
            LiteralVariant::Integer(IntegerType::U32, string) => {
                parse_integer_literal::<u32>(&self.state.handler, string, input.span(), "u32");
                Type::Integer(IntegerType::U32)
            }
            LiteralVariant::Integer(IntegerType::U64, string) => {
                parse_integer_literal::<u64>(&self.state.handler, string, input.span(), "u64");
                Type::Integer(IntegerType::U64)
            }
            LiteralVariant::Integer(IntegerType::U128, string) => {
                parse_integer_literal::<u128>(&self.state.handler, string, input.span(), "u128");
                Type::Integer(IntegerType::U128)
            }
            LiteralVariant::Integer(IntegerType::I8, string) => {
                parse_integer_literal::<i8>(&self.state.handler, string, input.span(), "i8");
                Type::Integer(IntegerType::I8)
            }
            LiteralVariant::Integer(IntegerType::I16, string) => {
                parse_integer_literal::<i16>(&self.state.handler, string, input.span(), "i16");
                Type::Integer(IntegerType::I16)
            }
            LiteralVariant::Integer(IntegerType::I32, string) => {
                parse_integer_literal::<i32>(&self.state.handler, string, input.span(), "i32");
                Type::Integer(IntegerType::I32)
            }
            LiteralVariant::Integer(IntegerType::I64, string) => {
                parse_integer_literal::<i64>(&self.state.handler, string, input.span(), "i64");
                Type::Integer(IntegerType::I64)
            }
            LiteralVariant::Integer(IntegerType::I128, string) => {
                parse_integer_literal::<i128>(&self.state.handler, string, input.span(), "i128");
                Type::Integer(IntegerType::I128)
            }
            LiteralVariant::Group(s) => {
                // Get rid of leading - and 0 and see if it parses
                let s = s.trim_start_matches('-').trim_start_matches('0');
                if !s.is_empty()
                    && format!("{s}group").parse::<snarkvm::prelude::Group<snarkvm::prelude::TestnetV0>>().is_err()
                {
                    self.emit_err(TypeCheckerError::invalid_int_value(s, "group", input.span()));
                }
                Type::Group
            }
            LiteralVariant::Scalar(..) => Type::Scalar,
            LiteralVariant::String(..) => {
                self.emit_err(TypeCheckerError::strings_are_not_supported(input.span()));
                Type::String
            }
        };

        self.maybe_assert_type(&type_, expected, input.span());

        type_
    }

    fn visit_locator(&mut self, input: &LocatorExpression, expected: &Self::AdditionalInput) -> Self::Output {
        let maybe_var =
            self.state.symbol_table.lookup_global(Location::new(input.program.name.name, input.name)).cloned();
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

        let typ = if t1 == Type::Err || t2 == Type::Err {
            Type::Err
        } else if !self.eq_user(&t1, &t2) {
            self.emit_err(TypeCheckerError::ternary_branch_mismatch(t1, t2, input.span()));
            Type::Err
        } else {
            t1
        };

        // Make sure this isn't an external record type - won't work as we can't construct it.
        if self.is_external_record(&typ) {
            self.emit_err(TypeCheckerError::ternary_over_external_records(&typ, input.span));
        }

        // None of its members may be external record types either.
        if let Type::Tuple(tuple) = &typ {
            if tuple.elements().iter().any(|ty| self.is_external_record(ty)) {
                self.emit_err(TypeCheckerError::ternary_over_external_records(&typ, input.span));
            }
        }

        typ
    }

    fn visit_tuple(&mut self, input: &TupleExpression, expected: &Self::AdditionalInput) -> Self::Output {
        if let Some(expected) = expected {
            if let Type::Tuple(expected_types) = expected {
                // If the expected type is a tuple, then ensure it's compatible with `input`

                // First, make sure that the number of tuple elements is correct
                if expected_types.length() != input.elements.len() {
                    self.emit_err(TypeCheckerError::incorrect_tuple_length(
                        expected_types.length(),
                        input.elements.len(),
                        input.span(),
                    ));
                }

                // Now make sure that none of the tuple elements is a tuple
                input.elements.iter().zip(expected_types.elements()).for_each(|(expr, expected_el_ty)| {
                    if matches!(expr, Expression::Tuple(_)) {
                        self.emit_err(TypeCheckerError::nested_tuple_expression(expr.span()));
                    }
                    self.visit_expression(expr, &Some(expected_el_ty.clone()));
                });

                // Just return the expected type since we proved it's correct
                expected.clone()
            } else {
                // If the expected type is not a tuple, then we just error out

                // This is the expected type of the tuple based on its individual fields
                let inferred_type = Type::Tuple(TupleType::new(
                    input.elements.iter().map(|field| self.visit_expression(field, &None)).collect::<Vec<_>>(),
                ));
                self.emit_err(TypeCheckerError::type_should_be2(inferred_type.clone(), expected, input.span()));

                // Recover with the expected type anyways
                expected.clone()
            }
        } else {
            // If no `expected` type is provided, then we analyze the tuple itself and infer its type

            // We still need to check that none of the tuple elements is a tuple
            input.elements.iter().for_each(|expr| {
                if matches!(expr, Expression::Tuple(_)) {
                    self.emit_err(TypeCheckerError::nested_tuple_expression(expr.span()));
                }
            });

            Type::Tuple(TupleType::new(
                input.elements.iter().map(|field| self.visit_expression(field, &None)).collect::<Vec<_>>(),
            ))
        }
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
