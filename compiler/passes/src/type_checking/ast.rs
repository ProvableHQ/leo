// Copyright (C) 2019-2026 Provable Inc.
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

use super::*;
use crate::{BlockToFunctionRewriter, VariableType};

use leo_ast::{
    Type::{Future, Tuple},
    *,
};
use leo_errors::{TypeCheckerError, TypeCheckerWarning};
use leo_span::{Span, Symbol, sym};

use itertools::Itertools as _;

#[derive(Clone, Debug)]
pub struct AssignTargetInfo {
    pub ty: Type,
    pub kind: AssignTargetKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssignTargetKind {
    Local,
    Storage,
    ExternalStorage,
    Err,
}

impl TypeCheckingVisitor<'_> {
    /// Returns information about an expression when used as the LHS of an assignment.
    ///
    /// Specifically, this function returns an `AssignTargetInfo` containing:
    /// - the type of the expression (`ty`),
    /// - the kind of assignment target (`Local`, `Storage`, `ExternalStorage`, or `Err`).
    ///
    /// Supported LHS expressions include:
    /// - `Path` (variables),
    /// - `ArrayAccess`,
    /// - `MemberAccess`,
    /// - `TupleAccess`.
    ///
    /// This function also performs static checks for:
    /// - invalid assignment targets (anything other than the supported LHS expressions),
    /// - assignments to external records or tuples in disallowed scopes,
    /// - reassignment of `Future` or `Mapping` types,
    ///
    /// It also updates the type table with the expression’s type.
    pub fn visit_expression_assign(&mut self, input: &Expression) -> AssignTargetInfo {
        let assign_target_info = match input {
            Expression::ArrayAccess(array_access) => AssignTargetInfo {
                ty: self.visit_array_access_general(array_access, true, &None),
                kind: AssignTargetKind::Local,
            },
            Expression::Path(path) if path.qualifier().is_empty() => self.visit_path_assign(path),
            Expression::MemberAccess(member_access) => AssignTargetInfo {
                ty: self.visit_member_access_general(member_access, true, &None),
                kind: AssignTargetKind::Local,
            },
            Expression::TupleAccess(tuple_access) => AssignTargetInfo {
                ty: self.visit_tuple_access_general(tuple_access, true, &None),
                kind: AssignTargetKind::Local,
            },
            _ => {
                self.emit_err(TypeCheckerError::invalid_assignment_target(input, input.span()));
                AssignTargetInfo { ty: Type::Err, kind: AssignTargetKind::Err }
            }
        };

        // Check external record assignment
        if let Expression::Path(path) = input
            && !self.symbol_in_conditional_scope(path.identifier().name)
            && (self.is_external_record(&assign_target_info.ty)
                || matches!(&assign_target_info.ty, Type::Tuple(tuple) if tuple.elements().iter().any(|ty| self.is_external_record(ty))))
        {
            if self.is_external_record(&assign_target_info.ty) {
                self.emit_err(TypeCheckerError::assignment_to_external_record_cond(
                    &assign_target_info.ty,
                    input.span(),
                ));
            } else {
                // Note that this will cover both assigning to a tuple variable and assigning to a member of a tuple.
                self.emit_err(TypeCheckerError::assignment_to_external_record_tuple_cond(
                    &assign_target_info.ty,
                    input.span(),
                ));
            }
        }

        // Prohibit reassignment of futures or mappings
        match &assign_target_info.ty {
            Type::Future(..) => self.emit_err(TypeCheckerError::cannot_reassign_future_variable(input, input.span())),
            Type::Mapping(_) => self.emit_err(TypeCheckerError::cannot_reassign_mapping(input, input.span())),
            _ => {}
        }

        // Add the expression and its associated type to the type table.
        self.state.type_table.insert(input.id(), assign_target_info.ty.clone());
        assign_target_info
    }

    pub fn visit_array_access_general(&mut self, input: &ArrayAccess, assign: bool, expected: &Option<Type>) -> Type {
        // Check that the expression is an array.
        let this_type = if assign {
            self.visit_expression_assign(&input.array).ty
        } else {
            self.visit_expression(&input.array, &None)
        };
        self.assert_array_type(&this_type, input.array.span());

        // Check that the index is an integer type.
        let mut index_type = self.visit_expression(&input.index, &None);

        if index_type == Type::Numeric {
            // If the index has type `Numeric`, then it's an unsuffixed literal. Just infer its type to be `u32` and
            // then check it's validity as a `u32`.
            index_type = Type::Integer(IntegerType::U32);
            if let Expression::Literal(literal) = &input.index {
                self.check_numeric_literal(literal, &index_type);
            }
        }

        self.assert_int_type(&index_type, input.index.span());

        // Keep track of the type of the index in the type table.
        // This is important for when the index is an unsuffixed literal.
        self.state.type_table.insert(input.index.id(), index_type.clone());

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

    pub fn visit_member_access_general(&mut self, input: &MemberAccess, assign: bool, expected: &Option<Type>) -> Type {
        let ty = if assign {
            self.visit_expression_assign(&input.inner).ty
        } else {
            self.visit_expression(&input.inner, &None)
        };

        // Make sure we're not assigning to a member of an external record.
        if assign && self.is_external_record(&ty) {
            self.emit_err(TypeCheckerError::assignment_to_external_record_member(&ty, input.span));
        }

        // Check that the type of `inner` in `inner.name` is a composite.
        match ty {
            Type::Err => Type::Err,
            Type::Composite(ref composite) => {
                // Retrieve the composite definition associated with `identifier`.
                let Some(composite) = self.lookup_composite(composite.path.expect_global_location()) else {
                    self.emit_err(TypeCheckerError::undefined_type(ty, input.inner.span()));
                    return Type::Err;
                };
                // Check that `input.name` is a member of the composite.
                match composite.members.iter().find(|member| member.name() == input.name.name) {
                    // Case where `input.name` is a member of the composite.
                    Some(Member { type_, .. }) => {
                        // Check that the type of `input.name` is the same as `expected`.
                        self.maybe_assert_type(type_, expected, input.span());
                        type_.clone()
                    }
                    // Case where `input.name` is not a member of the composite.
                    None => {
                        self.emit_err(TypeCheckerError::invalid_composite_variable(
                            input.name,
                            &composite,
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

    pub fn visit_tuple_access_general(&mut self, input: &TupleAccess, assign: bool, expected: &Option<Type>) -> Type {
        let this_type = if assign {
            self.visit_expression_assign(&input.tuple).ty
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

                if inferred_f.location.is_none() {
                    // This generally means that the `Future` is produced by an `async` block expression and not an
                    // `async function` function call.
                    self.emit_err(TypeCheckerError::invalid_async_block_future_access(input.span()));
                    return Type::Err;
                }

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

    /// Returns information about a `Path` expression when used as the LHS of an assignment.
    ///
    /// Specifically, returns:
    /// - the type of the variable,
    /// - the kind of assignment target (`Local`, `Storage`, `ExternalStorage`, or `Err`).
    ///
    /// This also performs static checks for:
    /// - assignments to constants,
    /// - assignments to storage vectors (invalid),
    /// - assignments in async functions or async blocks outside the allowed conditional scope,
    /// - assignments to variables outside an async block’s scope.
    pub fn visit_path_assign(&mut self, input: &Path) -> AssignTargetInfo {
        let current_program = self.scope_state.program_name.unwrap();

        // Lookup the variable in the symbol table
        let Some(var) = self.state.symbol_table.lookup_path(current_program, input) else {
            self.emit_err(TypeCheckerError::unknown_sym("variable", input, input.span));
            return AssignTargetInfo { ty: Type::Err, kind: AssignTargetKind::Err };
        };

        let ty = var.type_.expect("must be known by now").clone();

        // Cannot assign to storage vectors
        if ty.is_vector() {
            self.emit_err(TypeCheckerError::invalid_assignment_target(input, input.span()));
            return AssignTargetInfo { ty: Type::Err, kind: AssignTargetKind::Err };
        }

        // Check that variable is not a constant or constant input
        match &var.declaration {
            VariableType::Const => self.emit_err(TypeCheckerError::cannot_assign_to_const_var(input, var.span)),
            VariableType::ConstParameter => {
                self.emit_err(TypeCheckerError::cannot_assign_to_generic_const_function_parameter(input, input.span))
            }
            VariableType::Input(Mode::Constant) => {
                self.emit_err(TypeCheckerError::cannot_assign_to_const_input(input, var.span))
            }
            VariableType::Storage => {
                // Determine if this storage variable belongs to another program
                let kind = if input.user_program().is_some() {
                    AssignTargetKind::ExternalStorage
                } else {
                    AssignTargetKind::Storage
                };
                return AssignTargetInfo { ty, kind };
            }
            VariableType::Mut | VariableType::Input(_) => {}
        }

        // Ensure assignment is allowed in async function conditional scopes
        if self.scope_state.variant.unwrap().is_async_function()
            && !self.symbol_in_conditional_scope(input.identifier().name)
        {
            self.emit_err(TypeCheckerError::async_cannot_assign_outside_conditional(input, "function", var.span));
        }

        // Ensure assignment is allowed in async block conditional scopes
        if self.async_block_id.is_some() && !self.symbol_in_conditional_scope(input.identifier().name) {
            self.emit_err(TypeCheckerError::async_cannot_assign_outside_conditional(input, "block", var.span));
        }

        // If inside an async block, cannot assign to variables outside the block or its ancestors
        if let Some(async_block_id) = self.async_block_id
            && !self.state.symbol_table.is_defined_in_scope_or_ancestor_until(async_block_id, input.identifier().name)
        {
            self.emit_err(TypeCheckerError::cannot_assign_to_vars_outside_async_block(
                input.identifier().name,
                input.span,
            ));
        }

        // Default: assignment to a local variable
        AssignTargetInfo { ty, kind: AssignTargetKind::Local }
    }

    /// Infers the type of an expression, but returns Type::Err and emits an error if the result is Type::Numeric.
    /// Used to disallow numeric types in specific contexts where they are not valid or expected.
    pub(crate) fn visit_expression_reject_numeric(&mut self, expr: &Expression, expected: &Option<Type>) -> Type {
        let mut inferred = self.visit_expression(expr, expected);
        match inferred {
            Type::Numeric => {
                self.emit_inference_failure_error(&mut inferred, expr);
                Type::Err
            }
            _ => inferred,
        }
    }

    /// Infers the type of an expression, and if it is `Type::Numeric`, coerces it to `U32`, validates it, and
    /// records it in the type table.
    pub(crate) fn visit_expression_infer_default_u32(&mut self, expr: &Expression) -> Type {
        let mut inferred = self.visit_expression(expr, &None);

        if inferred == Type::Numeric {
            inferred = Type::Integer(IntegerType::U32);

            if let Expression::Literal(literal) = expr
                && !self.check_numeric_literal(literal, &inferred)
            {
                inferred = Type::Err;
            }

            self.state.type_table.insert(expr.id(), inferred.clone());
        }

        inferred
    }
}

impl AstVisitor for TypeCheckingVisitor<'_> {
    type AdditionalInput = Option<Type>;
    type Output = Type;

    /* Types */
    fn visit_array_type(&mut self, input: &ArrayType) {
        self.visit_type(&input.element_type);
        self.visit_expression_infer_default_u32(&input.length);
    }

    fn visit_composite_type(&mut self, input: &CompositeType) {
        let composite = self.lookup_composite(input.path.expect_global_location()).clone();

        if let Some(composite) = composite {
            // Check the number of const arguments against the number of the composite's const parameters
            if composite.const_parameters.len() != input.const_arguments.len() {
                self.emit_err(TypeCheckerError::incorrect_num_const_args(
                    "Composite type",
                    composite.const_parameters.len(),
                    input.const_arguments.len(),
                    input.path.span,
                ));
            }

            // Check the types of const arguments against the types of the composite's const parameters
            for (expected, argument) in composite.const_parameters.iter().zip(input.const_arguments.iter()) {
                self.visit_expression(argument, &Some(expected.type_().clone()));
            }
        } else if !input.const_arguments.is_empty() {
            // This handles erroring out on all non-structs
            self.emit_err(TypeCheckerError::unexpected_const_args(input, input.path.span));
        }
    }

    /* Expressions */
    fn visit_expression(&mut self, input: &Expression, additional: &Self::AdditionalInput) -> Self::Output {
        let output = match input {
            Expression::Array(array) => self.visit_array(array, additional),
            Expression::ArrayAccess(access) => self.visit_array_access_general(access, false, additional),
            Expression::Intrinsic(intr) => self.visit_intrinsic(intr, additional),
            Expression::Async(async_) => self.visit_async(async_, additional),
            Expression::Binary(binary) => self.visit_binary(binary, additional),
            Expression::Call(call) => self.visit_call(call, additional),
            Expression::Cast(cast) => self.visit_cast(cast, additional),
            Expression::Composite(composite) => self.visit_composite_init(composite, additional),
            Expression::Err(err) => self.visit_err(err, additional),
            Expression::Path(path) => self.visit_path(path, additional),
            Expression::Literal(literal) => self.visit_literal(literal, additional),
            Expression::MemberAccess(access) => self.visit_member_access_general(access, false, additional),
            Expression::Repeat(repeat) => self.visit_repeat(repeat, additional),
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
        // Grab the element type from the expected type if the expected type is an array or if it's
        // an optional array
        let element_type = match additional {
            Some(Type::Array(array_ty)) => Some(array_ty.element_type().clone()),
            Some(Type::Optional(opt)) => match &*opt.inner {
                Type::Array(array_ty) => Some(array_ty.element_type().clone()),
                _ => None,
            },
            _ => None,
        };

        let inferred_type = if input.elements.is_empty() {
            if let Some(ty) = element_type.clone() {
                ty
            } else {
                self.emit_err(TypeCheckerError::could_not_determine_type(input, input.span()));
                Type::Err
            }
        } else {
            self.visit_expression_reject_numeric(&input.elements[0], &element_type)
        };

        if input.elements.len() > self.limits.max_array_elements {
            self.emit_err(TypeCheckerError::array_too_large(
                input.elements.len(),
                self.limits.max_array_elements,
                input.span(),
            ));
        }

        for expression in input.elements.iter().skip(1) {
            let next_type = self.visit_expression_reject_numeric(expression, &element_type);

            if next_type == Type::Err {
                return Type::Err;
            }

            if let Some(ref element_type) = element_type {
                self.assert_type(&next_type, element_type, expression.span());
            } else {
                self.assert_type(&next_type, &inferred_type, expression.span());
            }
        }

        if inferred_type == Type::Err {
            return Type::Err;
        }

        let type_ = Type::Array(ArrayType::new(
            inferred_type,
            Expression::Literal(Literal {
                // The default type for array length is `U32`.
                variant: LiteralVariant::Integer(IntegerType::U32, input.elements.len().to_string()),
                id: self.state.node_builder.next_id(),
                span: Span::default(),
            }),
        ));

        self.maybe_assert_type(&type_, additional, input.span());

        type_
    }

    fn visit_repeat(&mut self, input: &RepeatExpression, additional: &Self::AdditionalInput) -> Self::Output {
        // Grab the element type from the expected type if the expected type is an array or if it's
        // an optional array
        let expected_element_type = match additional {
            Some(Type::Array(array_ty)) => Some(array_ty.element_type().clone()),
            Some(Type::Optional(opt)) => match &*opt.inner {
                Type::Array(array_ty) => Some(array_ty.element_type().clone()),
                _ => None,
            },
            _ => None,
        };

        let inferred_element_type = self.visit_expression_reject_numeric(&input.expr, &expected_element_type);

        // Now infer the type of `count`. If it's an unsuffixed literal (i.e. has `Type::Numeric`), then infer it to be
        // a `U32` as the default type.
        self.visit_expression_infer_default_u32(&input.count);

        // If we can already evaluate the repeat count as a `u32`, then make sure it's not 0 or  greater than the array
        // size limit.
        if let Some(count) = input.count.as_u32()
            && count > self.limits.max_array_elements as u32
        {
            self.emit_err(TypeCheckerError::array_too_large(count, self.limits.max_array_elements, input.span()));
        }

        let type_ = Type::Array(ArrayType::new(inferred_element_type, input.count.clone()));

        self.maybe_assert_type(&type_, additional, input.span());
        type_
    }

    fn visit_intrinsic(&mut self, input: &IntrinsicExpression, expected: &Self::AdditionalInput) -> Self::Output {
        // Check core struct name and function.
        let Some(intrinsic) = self.get_intrinsic(input) else {
            return Type::Err;
        };
        // Check that operation is not restricted to finalize blocks.
        if !matches!(self.scope_state.variant, Some(Variant::AsyncFunction) | Some(Variant::Script))
            && self.async_block_id.is_none()
            && intrinsic.is_finalize_command()
        {
            self.emit_err(TypeCheckerError::operation_must_be_in_async_block_or_function(input.span()));
        }

        let return_type = self.check_intrinsic(intrinsic.clone(), &input.arguments, expected, input.span());

        // Check return type if the expected type is known.
        self.maybe_assert_type(&return_type, expected, input.span());

        // Await futures here so that can use the argument variable names to lookup.
        if intrinsic == Intrinsic::FutureAwait && input.arguments.len() != 1 {
            self.emit_err(TypeCheckerError::can_only_await_one_future_at_a_time(input.span));
        }

        return_type
    }

    fn visit_async(&mut self, input: &AsyncExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        // Step into an async block
        self.async_block_id = Some(input.block.id);

        // A few restrictions
        if self.scope_state.is_conditional {
            self.emit_err(TypeCheckerError::async_block_in_conditional(input.span));
        }

        if !matches!(self.scope_state.variant, Some(Variant::AsyncTransition) | Some(Variant::Script)) {
            self.emit_err(TypeCheckerError::illegal_async_block_location(input.span));
        }

        if self.scope_state.already_contains_an_async_block {
            self.emit_err(TypeCheckerError::multiple_async_blocks_not_allowed(input.span));
        }

        if self.scope_state.has_called_finalize {
            self.emit_err(TypeCheckerError::conflicting_async_call_and_block(input.span));
        }

        self.visit_block(&input.block);

        // This scope now already has an async block
        self.scope_state.already_contains_an_async_block = true;

        // Here we convert the async block to an async function using the helper
        // `BlockToFunctionRewriter`. We do not actually replace anything in the original AST. We
        // just inspect how the async block would look like as an async function in order to
        // populate the map `async_function_input_types`.
        let mut block_to_function_rewriter =
            BlockToFunctionRewriter::new(self.state, self.scope_state.program_name.unwrap());
        let (new_function, _) =
            block_to_function_rewriter.rewrite_block(&input.block, Symbol::intern("unused"), Variant::AsyncFunction);
        let input_types = new_function.input.iter().map(|Input { type_, .. }| type_.clone()).collect();
        self.async_function_input_types.insert(
            Location::new(self.scope_state.program_name.unwrap(), vec![Symbol::intern(&format!(
                "finalize/{}",
                self.scope_state.function.unwrap(),
            ))]),
            input_types,
        );

        // Step out of the async block
        self.async_block_id = None;

        // The type of the async block is just a `Future` with no `Location` (i.e. not produced by an explicit `async
        // function`) and no inputs since we're not allowed to access inputs of a `Future` produced by an `async block.
        Type::Future(FutureType::new(Vec::new(), None, false))
    }

    fn visit_binary(&mut self, input: &BinaryExpression, destination: &Self::AdditionalInput) -> Self::Output {
        let assert_same_type = |slf: &Self, t1: &Type, t2: &Type| -> Type {
            if t1 == &Type::Err || t2 == &Type::Err {
                Type::Err
            } else if !t1.eq_user(t2) {
                slf.emit_err(TypeCheckerError::operation_types_mismatch(input.op, t1, t2, input.span()));
                Type::Err
            } else {
                t1.clone()
            }
        };

        // This closure attempts to resolve numeric type inference between two operands.
        // It handles the following cases:
        // - If both types are unknown numeric placeholders (`Numeric`), emit errors for both.
        // - If one type is `Numeric` and the other is an error (`Err`), propagate the error.
        // - If one type is a known numeric type and the other is `Numeric`, infer the unknown type.
        // - If one type is `Numeric` but the other is not a valid numeric type, emit an error.
        // - Otherwise, do nothing (types are already resolved or not subject to inference).
        let infer_numeric_types = |slf: &Self, left_type: &mut Type, right_type: &mut Type| {
            use Type::*;

            match (&*left_type, &*right_type) {
                // Case: Both types are unknown numeric types – cannot infer either side
                (Numeric, Numeric) => {
                    slf.emit_inference_failure_error(left_type, &input.left);
                    slf.emit_inference_failure_error(right_type, &input.right);
                }

                // Case: Left is unknown numeric, right is erroneous – propagate error to left
                (Numeric, Err) => slf.emit_inference_failure_error(left_type, &input.left),

                // Case: Right is unknown numeric, left is erroneous – propagate error to right
                (Err, Numeric) => slf.emit_inference_failure_error(right_type, &input.right),

                // Case: Right type is unknown numeric, infer it from known left type
                (Integer(_) | Field | Group | Scalar, Numeric) => {
                    *right_type = left_type.clone();
                    slf.state.type_table.insert(input.right.id(), right_type.clone());
                    if let Expression::Literal(literal) = &input.right {
                        slf.check_numeric_literal(literal, right_type);
                    }
                }

                // Case: Left type is unknown numeric, infer it from known right type
                (Numeric, Integer(_) | Field | Group | Scalar) => {
                    *left_type = right_type.clone();
                    slf.state.type_table.insert(input.left.id(), left_type.clone());
                    if let Expression::Literal(literal) = &input.left {
                        slf.check_numeric_literal(literal, left_type);
                    }
                }

                // Case: Left type is numeric but right is invalid for numeric inference – error on left
                (Numeric, _) => slf.emit_inference_failure_error(left_type, &input.left),

                // Case: Right type is numeric but left is invalid for numeric inference – error on right
                (_, Numeric) => slf.emit_inference_failure_error(right_type, &input.right),

                // No inference or error needed. Rely on further operator-specific checks.
                _ => {}
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
                let operand_expected = self.unwrap_optional_type(destination);

                // The expected type for both `left` and `right` is the unwrapped type
                let mut t1 = self.visit_expression(&input.left, &operand_expected);
                let mut t2 = self.visit_expression(&input.right, &operand_expected);

                // Infer `Numeric` types if possible
                infer_numeric_types(self, &mut t1, &mut t2);

                // Now sanity check everything
                self.assert_bool_int_type(&t1, input.left.span());
                self.assert_bool_int_type(&t2, input.right.span());

                let result_t = assert_same_type(self, &t1, &t2);
                self.maybe_assert_type(&result_t, destination, input.span());

                result_t
            }
            BinaryOperation::Add => {
                let operand_expected = self.unwrap_optional_type(destination);

                // The expected type for both `left` and `right` is the unwrapped type
                let mut t1 = self.visit_expression(&input.left, &operand_expected);
                let mut t2 = self.visit_expression(&input.right, &operand_expected);

                // Infer `Numeric` types if possible
                infer_numeric_types(self, &mut t1, &mut t2);

                // Now sanity check everything
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
                let operand_expected = self.unwrap_optional_type(destination);

                // The expected type for both `left` and `right` is the unwrapped type
                let mut t1 = self.visit_expression(&input.left, &operand_expected);
                let mut t2 = self.visit_expression(&input.right, &operand_expected);

                // Infer `Numeric` types if possible
                infer_numeric_types(self, &mut t1, &mut t2);

                // Now sanity check everything
                self.assert_field_group_int_type(&t1, input.left.span());
                self.assert_field_group_int_type(&t2, input.right.span());

                let result_t = assert_same_type(self, &t1, &t2);

                self.maybe_assert_type(&result_t, destination, input.span());

                result_t
            }
            BinaryOperation::Mul => {
                let unwrapped_dest = self.unwrap_optional_type(destination);

                // The expected type for both `left` and `right` is the same as unwrapped destination except when it is
                // a `Type::Group`. In that case, the two operands should be a `Type::Group` and `Type::Scalar` but we can't
                // known which one is which.
                let expected = if matches!(unwrapped_dest, Some(Type::Group)) { &None } else { &unwrapped_dest };
                let mut t1 = self.visit_expression(&input.left, expected);
                let mut t2 = self.visit_expression(&input.right, expected);

                // - If one side is `Group` and the other is an unresolved `Numeric`, infer the `Numeric` as a `Scalar`,
                //   since `Group * Scalar = Group`.
                // - Similarly, if one side is `Scalar` and the other is `Numeric`, infer the `Numeric` as `Group`.
                //
                // If no special case applies, default to inferring types between `t1` and `t2` as-is.
                match (&t1, &t2) {
                    (Type::Group, Type::Numeric) => infer_numeric_types(self, &mut Type::Scalar, &mut t2),
                    (Type::Numeric, Type::Group) => infer_numeric_types(self, &mut t1, &mut Type::Scalar),
                    (Type::Scalar, Type::Numeric) => infer_numeric_types(self, &mut Type::Group, &mut t2),
                    (Type::Numeric, Type::Scalar) => infer_numeric_types(self, &mut t1, &mut Type::Group),
                    (_, _) => infer_numeric_types(self, &mut t1, &mut t2),
                }

                // Final sanity checks
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
                let operand_expected = self.unwrap_optional_type(destination);

                // The expected type for both `left` and `right` is the unwrapped type
                let mut t1 = self.visit_expression(&input.left, &operand_expected);
                let mut t2 = self.visit_expression(&input.right, &operand_expected);

                // Infer `Numeric` types if possible
                infer_numeric_types(self, &mut t1, &mut t2);

                // Now sanity check everything
                self.assert_field_int_type(&t1, input.left.span());
                self.assert_field_int_type(&t2, input.right.span());

                let result_t = assert_same_type(self, &t1, &t2);

                self.maybe_assert_type(&result_t, destination, input.span());

                result_t
            }
            BinaryOperation::Rem | BinaryOperation::RemWrapped => {
                let operand_expected = self.unwrap_optional_type(destination);

                // The expected type for both `left` and `right` is the unwrapped type
                let mut t1 = self.visit_expression(&input.left, &operand_expected);
                let mut t2 = self.visit_expression(&input.right, &operand_expected);

                // Infer `Numeric` types if possible
                infer_numeric_types(self, &mut t1, &mut t2);

                // Now sanity check everything
                self.assert_int_type(&t1, input.left.span());
                self.assert_int_type(&t2, input.right.span());

                let result_t = assert_same_type(self, &t1, &t2);

                self.maybe_assert_type(&result_t, destination, input.span());

                result_t
            }
            BinaryOperation::Mod => {
                let operand_expected = self.unwrap_optional_type(destination);

                // The expected type for both `left` and `right` is the unwrapped type
                let mut t1 = self.visit_expression(&input.left, &operand_expected);
                let mut t2 = self.visit_expression(&input.right, &operand_expected);

                // Infer `Numeric` types if possible
                infer_numeric_types(self, &mut t1, &mut t2);

                // Now sanity check everything
                self.assert_unsigned_type(&t1, input.left.span());
                self.assert_unsigned_type(&t2, input.right.span());

                let result_t = assert_same_type(self, &t1, &t2);

                self.maybe_assert_type(&result_t, destination, input.span());

                result_t
            }
            BinaryOperation::Pow => {
                let operand_expected = self.unwrap_optional_type(destination);

                // The expected type of `left` is the unwrapped destination
                let mut t1 = self.visit_expression(&input.left, &operand_expected);

                // The expected type of `right` is `field`, `u8`, `u16`, or `u32` so leave it as `None` for now.
                let mut t2 = self.visit_expression(&input.right, &None);

                // If one side is a `Field` and the other is a `Numeric`, infer the `Numeric` as a `Field.
                // Otherwise, error out for each `Numeric`.
                if matches!((&t1, &t2), (Type::Field, Type::Numeric) | (Type::Numeric, Type::Field)) {
                    infer_numeric_types(self, &mut t1, &mut t2);
                } else {
                    if matches!(t1, Type::Numeric) {
                        self.emit_inference_failure_error(&mut t1, &input.left);
                    }
                    if matches!(t2, Type::Numeric) {
                        self.emit_inference_failure_error(&mut t2, &input.right);
                    }
                }

                // Now sanity check everything
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
                // Handle type inference for `None` as a special case.
                //
                // If either side of the binary expression is the literal `None`, we first type check the other side
                // without any expected type to infer its type. Then we type check the `None` side using that inferred type
                // as context, in hopes of resolving it to a more specific optional type.
                //
                // This helps with cases like `x == None`, allowing us to infer the type of `x` and apply it to `None`.
                // However, this is **not sufficient for the general case**. For instance, in something like `[None] == [x]`,
                // we won't be able to infer the type of `None`.
                let (mut t1, mut t2) =
                    if let Expression::Literal(Literal { variant: LiteralVariant::None, .. }) = input.right {
                        let t1 = self.visit_expression(&input.left, &None);
                        (t1.clone(), self.visit_expression(&input.right, &Some(t1.clone())))
                    } else if let Expression::Literal(Literal { variant: LiteralVariant::None, .. }) = input.left {
                        let t2 = self.visit_expression(&input.right, &None);
                        (self.visit_expression(&input.left, &Some(t2.clone())), t2)
                    } else {
                        (self.visit_expression(&input.left, &None), self.visit_expression(&input.right, &None))
                    };

                // Infer `Numeric` types if possible
                infer_numeric_types(self, &mut t1, &mut t2);

                // Now sanity check everything
                let _ = assert_same_type(self, &t1, &t2);

                self.maybe_assert_type(&Type::Boolean, destination, input.span());

                Type::Boolean
            }
            BinaryOperation::Lt | BinaryOperation::Gt | BinaryOperation::Lte | BinaryOperation::Gte => {
                // Assert left and right are equal field, scalar, or integer types.
                let mut t1 = self.visit_expression(&input.left, &None);
                let mut t2 = self.visit_expression(&input.right, &None);

                // Infer `Numeric` types if possible
                infer_numeric_types(self, &mut t1, &mut t2);

                // Now sanity check everything
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
                let operand_expected = self.unwrap_optional_type(destination);

                // The expected type for both `left` and `right` is the unwrapped type
                let mut t1 = self.visit_expression(&input.left, &operand_expected);
                let mut t2 = self.visit_expression(&input.right, &operand_expected);

                // Infer `Numeric` types if possible
                infer_numeric_types(self, &mut t1, &mut t2);

                // Now sanity check everything
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
                let operand_expected = self.unwrap_optional_type(destination);

                // The expected type of `left` is the unwrapped `destination`
                let t1 = self.visit_expression_reject_numeric(&input.left, &operand_expected);

                // The expected type of `right` is `field`, `u8`, `u16`, or `u32` so leave it as `None` for now.
                let t2 = self.visit_expression_reject_numeric(&input.right, &None);

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
        let current_program = self.scope_state.program_name.unwrap();
        let callee_location = input.function.expect_global_location();
        let callee_program = callee_location.program;
        let callee_path = callee_location.path.clone();

        let Some(func_symbol) = self.state.symbol_table.lookup_function(current_program, callee_location) else {
            self.emit_err(TypeCheckerError::unknown_sym("function", input.function.clone(), input.function.span()));
            return Type::Err;
        };

        let func = func_symbol.function.clone();

        // Check that the call is valid.
        // We always set the variant before entering the body of a function, so this unwrap works.
        match self.scope_state.variant.unwrap() {
            Variant::AsyncFunction | Variant::Function if !matches!(func.variant, Variant::Inline) => self.emit_err(
                TypeCheckerError::can_only_call_inline_function("a `function`, `inline`, or `constructor`", input.span),
            ),
            Variant::Transition | Variant::AsyncTransition
                if matches!(func.variant, Variant::Transition)
                    && callee_program == self.scope_state.program_name.unwrap() =>
            {
                self.emit_err(TypeCheckerError::cannot_invoke_call_to_local_transition_function(input.span))
            }
            _ => {}
        }

        // Check that the call is not to an external `inline` function.
        if func.variant == Variant::Inline && callee_program != self.scope_state.program_name.unwrap() {
            self.emit_err(TypeCheckerError::cannot_call_external_inline_function(input.span));
        }

        // Make sure we're not calling a non-inline from an async block
        if self.async_block_id.is_some() && !matches!(func.variant, Variant::Inline) {
            self.emit_err(TypeCheckerError::can_only_call_inline_function("an async block", input.span));
        }

        // Async functions return a single future.
        let mut ret = if func.variant == Variant::AsyncFunction {
            // Async functions always return futures.
            Type::Future(FutureType::new(Vec::new(), Some(callee_location.clone()), false))
        } else if func.variant == Variant::AsyncTransition {
            // Fully infer future type.
            let Some(inputs) =
                self.async_function_input_types.get(&Location::new(callee_program, vec![Symbol::intern(&format!(
                    "finalize/{}",
                    input.function.identifier().name
                ))]))
            else {
                self.emit_err(TypeCheckerError::async_function_not_found(input.function.clone(), input.span));
                return Type::Future(FutureType::new(Vec::new(), Some(callee_location.clone()), false));
            };

            let future_type = Type::Future(FutureType::new(inputs.clone(), Some(callee_location.clone()), true));
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

        // Check the number of const arguments against the number of the function's const parameters
        if func.const_parameters.len() != input.const_arguments.len() {
            self.emit_err(TypeCheckerError::incorrect_num_const_args(
                "Call",
                func.const_parameters.len(),
                input.const_arguments.len(),
                input.span(),
            ));
        }

        // Check the types of const arguments against the types of the function's const parameters
        for (expected, argument) in func.const_parameters.iter().zip(input.const_arguments.iter()) {
            self.visit_expression(argument, &Some(expected.type_().clone()));
        }

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
                    Expression::Path(path) => Some(path.identifier().name),
                    Expression::TupleAccess(tuple_access) => {
                        if let Expression::Path(path) = &tuple_access.tuple {
                            Some(path.identifier().name)
                        } else {
                            None
                        }
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
                    Expression::Path(_) | Expression::Call(_) | Expression::TupleAccess(_) => {
                        match &self.scope_state.call_location {
                            Some(location) => {
                                // Get the external program and function name.
                                input_futures.push(location.clone());
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

        let caller_program =
            self.scope_state.program_name.expect("`program_name` is always set before traversing a program scope");
        // Note: Constructors are added to the call graph under the `constructor` symbol.
        // This is safe since `constructor` is a reserved token and cannot be used as a function name.
        let caller_function = if self.scope_state.is_constructor {
            sym::constructor
        } else {
            self.scope_state.function.expect("`function` is always set before traversing a function scope")
        };

        // This is the path to the function that we're in
        let caller_path = self
            .scope_state
            .module_name
            .iter()
            .cloned()
            .chain(std::iter::once(caller_function))
            .collect::<Vec<Symbol>>();

        let caller = Location::new(caller_program, caller_path.clone());
        let callee = Location::new(callee_program, callee_path.clone());
        self.state.call_graph.add_edge(caller, callee);

        if func.variant.is_transition() && self.scope_state.variant == Some(Variant::AsyncTransition) {
            if self.scope_state.has_called_finalize {
                self.emit_err(TypeCheckerError::external_call_after_async("function call", input.span));
            }

            if self.scope_state.already_contains_an_async_block {
                self.emit_err(TypeCheckerError::external_call_after_async("block", input.span));
            }
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

            if self.scope_state.already_contains_an_async_block {
                self.emit_err(TypeCheckerError::conflicting_async_call_and_block(input.span));
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
                    Location::new(callee_program, caller_path.clone()),
                    Location::new(callee_program, callee_path.clone()),
                    input_futures,
                    inferred_finalize_inputs.clone(),
                )
                .expect("Failed to attach finalizer");
            // Create expectation for finalize inputs that will be checked when checking corresponding finalize function signature.
            self.async_function_callers
                .entry(Location::new(self.scope_state.program_name.unwrap(), callee_path.clone()))
                .or_default()
                .insert(self.scope_state.location());

            // Set scope state flag.
            self.scope_state.has_called_finalize = true;

            // Update ret to reflect fully inferred future type.
            ret = Type::Future(FutureType::new(
                inferred_finalize_inputs.clone(),
                Some(Location::new(callee_program, callee_path.clone())),
                true,
            ));

            self.async_function_input_types.insert(
                Location::new(callee_program, vec![Symbol::intern(&format!(
                    "finalize/{}",
                    caller_path.last().unwrap()
                ))]),
                inferred_finalize_inputs.clone(),
            );

            // Type check in case the expected type is known.
            self.assert_and_return_type(ret.clone(), expected, input.span());
        }

        // Set call location so that definition statement knows where future comes from.
        self.scope_state.call_location = Some(Location::new(callee_program, callee_path.clone()));

        ret
    }

    fn visit_cast(&mut self, input: &CastExpression, expected: &Self::AdditionalInput) -> Self::Output {
        let expression_type = self.visit_expression_reject_numeric(&input.expression, &None);

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

    fn visit_composite_init(
        &mut self,
        input: &CompositeExpression,
        additional: &Self::AdditionalInput,
    ) -> Self::Output {
        let composite_location = input.path.expect_global_location();
        let composite = self.lookup_composite(composite_location).clone();
        let Some(composite) = composite else {
            self.emit_err(TypeCheckerError::unknown_sym("struct or record", input.path.clone(), input.path.span()));
            return Type::Err;
        };

        // Check the number of const arguments against the number of the composite's const parameters
        if composite.const_parameters.len() != input.const_arguments.len() {
            self.emit_err(TypeCheckerError::incorrect_num_const_args(
                "Composite expression",
                composite.const_parameters.len(),
                input.const_arguments.len(),
                input.span(),
            ));
        }

        // Check the types of const arguments against the types of the composite's const parameters
        for (expected, argument) in composite.const_parameters.iter().zip(input.const_arguments.iter()) {
            self.visit_expression(argument, &Some(expected.type_().clone()));
        }

        let type_ =
            Type::Composite(CompositeType { path: input.path.clone(), const_arguments: input.const_arguments.clone() });
        self.maybe_assert_type(&type_, additional, input.path.span());

        // Check number of composite members.
        if composite.members.len() != input.members.len() {
            self.emit_err(TypeCheckerError::incorrect_num_composite_members(
                composite.members.len(),
                input.members.len(),
                input.span(),
            ));
        }

        for Member { identifier, type_, .. } in composite.members.iter() {
            if let Some(actual) = input.members.iter().find(|member| member.identifier.name == identifier.name) {
                match &actual.expression {
                    None => {
                        // If `expression` is None, then the member uses the identifier shorthand, e.g. `Foo { a }`.
                        // Therefore, lookup a local or a global in the symbol table with the member name.
                        let current_program = self.scope_state.program_name.unwrap();
                        let var = self.state.symbol_table.lookup_local(actual.identifier.name).or_else(|| {
                            self.state
                                .symbol_table
                                .lookup_global(
                                    current_program,
                                    &Location::new(current_program, vec![actual.identifier.name]),
                                )
                                .cloned()
                        });

                        if let Some(var) = var {
                            let ty = var.type_.expect("must be known by now");
                            if var.declaration == VariableType::Storage && !ty.is_vector() && !ty.is_mapping() {
                                self.check_access_allowed("storage access", true, input.span());
                            }

                            self.maybe_assert_type(&ty, &Some(type_.clone()), input.span());
                            ty.clone()
                        } else {
                            self.emit_err(TypeCheckerError::unknown_sym("variable", input, input.span()));
                            Type::Err
                        };
                    }
                    Some(expr) => {
                        // Otherwise, visit the associated expression.
                        self.visit_expression(expr, &Some(type_.clone()));
                    }
                };
            } else {
                self.emit_err(TypeCheckerError::missing_composite_member(
                    composite.identifier,
                    identifier,
                    input.span(),
                ));
            };
        }

        if composite.is_record {
            // Ensure that we're not instantating an external record
            if composite_location.program != self.scope_state.program_name.unwrap() {
                self.state
                    .handler
                    .emit_err(TypeCheckerError::cannot_instantiate_external_record(composite_location, input.span()));
            }

            // First, ensure that the current scope is not an async function. Records should not be instantiated in
            // async functions
            if self.scope_state.variant == Some(Variant::AsyncFunction) {
                self.state
                    .handler
                    .emit_err(TypeCheckerError::records_not_allowed_inside_async("function", input.span()));
            }

            // Similarly, ensure that the current scope is not an async block. Records should not be instantiated in
            // async blocks
            if self.async_block_id.is_some() {
                self.state.handler.emit_err(TypeCheckerError::records_not_allowed_inside_async("block", input.span()));
            }

            // Records where the `owner` is `self.caller` can be problematic because `self.caller` can be a program
            // address and programs can't spend records. Emit a warning in this case.
            //
            // Multiple occurrences of `owner` here is an error but that should be flagged somewhere else.
            input.members.iter().filter(|init| init.identifier.name == sym::owner).for_each(|init| {
                if let Some(Expression::Intrinsic(intr)) = &init.expression
                    && let IntrinsicExpression { name: sym::_self_caller, .. } = &**intr
                {
                    self.emit_warning(TypeCheckerWarning::caller_as_record_owner(input.path.clone(), intr.span()));
                }
            });
        }

        type_
    }

    // We do not want to panic on `ErrExpression`s in order to propagate as many errors as possible.
    fn visit_err(&mut self, _input: &ErrExpression, _additional: &Self::AdditionalInput) -> Self::Output {
        Type::Err
    }

    fn visit_path(&mut self, input: &Path, expected: &Self::AdditionalInput) -> Self::Output {
        let current_program = self.scope_state.program_name.unwrap();
        let var = self.state.symbol_table.lookup_path(current_program, input);

        if let Some(var) = var {
            let ty = var.type_.expect("must be known at this point");
            if var.declaration == VariableType::Storage && !ty.is_vector() && !ty.is_mapping() {
                self.check_access_allowed("storage access", true, input.span());
            }

            self.maybe_assert_type(&ty, expected, input.span());
            ty.clone()
        } else {
            self.emit_err(TypeCheckerError::unknown_sym("variable", input, input.span()));
            Type::Err
        }
    }

    fn visit_literal(&mut self, input: &Literal, expected: &Self::AdditionalInput) -> Self::Output {
        let span = input.span();

        macro_rules! parse_and_return {
            ($ty:ty, $variant:expr, $str:expr, $label:expr) => {{
                self.parse_integer_literal::<$ty>($str, span, $label);
                Type::Integer($variant)
            }};
        }

        let type_ = match &input.variant {
            LiteralVariant::Address(..) => Type::Address,
            LiteralVariant::Boolean(..) => Type::Boolean,
            LiteralVariant::Field(..) => Type::Field,
            LiteralVariant::Scalar(..) => Type::Scalar,
            LiteralVariant::String(..) => Type::String,
            LiteralVariant::Integer(kind, string) => match kind {
                IntegerType::U8 => parse_and_return!(u8, IntegerType::U8, string, "u8"),
                IntegerType::U16 => parse_and_return!(u16, IntegerType::U16, string, "u16"),
                IntegerType::U32 => parse_and_return!(u32, IntegerType::U32, string, "u32"),
                IntegerType::U64 => parse_and_return!(u64, IntegerType::U64, string, "u64"),
                IntegerType::U128 => parse_and_return!(u128, IntegerType::U128, string, "u128"),
                IntegerType::I8 => parse_and_return!(i8, IntegerType::I8, string, "i8"),
                IntegerType::I16 => parse_and_return!(i16, IntegerType::I16, string, "i16"),
                IntegerType::I32 => parse_and_return!(i32, IntegerType::I32, string, "i32"),
                IntegerType::I64 => parse_and_return!(i64, IntegerType::I64, string, "i64"),
                IntegerType::I128 => parse_and_return!(i128, IntegerType::I128, string, "i128"),
            },
            LiteralVariant::Group(s) => {
                let trimmed = s.trim_start_matches('-').trim_start_matches('0');
                if !trimmed.is_empty()
                    && format!("{trimmed}group")
                        .parse::<snarkvm::prelude::Group<snarkvm::prelude::TestnetV0>>()
                        .is_err()
                {
                    self.emit_err(TypeCheckerError::invalid_int_value(trimmed, "group", span));
                }
                Type::Group
            }
            LiteralVariant::Unsuffixed(_) => match expected {
                Some(ty @ Type::Integer(_) | ty @ Type::Field | ty @ Type::Group | ty @ Type::Scalar) => {
                    self.check_numeric_literal(input, ty);
                    ty.clone()
                }
                Some(ty @ Type::Optional(opt)) => {
                    // Handle optional expected type, e.g., u32?
                    let inner = &opt.inner;
                    match &**inner {
                        Type::Integer(_) | Type::Field | Type::Group | Type::Scalar => {
                            self.check_numeric_literal(input, inner);
                            *inner.clone()
                        }
                        _ => {
                            self.emit_err(TypeCheckerError::unexpected_unsuffixed_numeral(
                                format!("type `{ty}`"),
                                span,
                            ));
                            Type::Err
                        }
                    }
                }
                Some(ty) => {
                    self.emit_err(TypeCheckerError::unexpected_unsuffixed_numeral(format!("type `{ty}`"), span));
                    Type::Err
                }
                None => Type::Numeric,
            },
            LiteralVariant::None => {
                if let Some(ty @ Type::Optional(_)) = expected {
                    ty.clone()
                } else if let Some(ty) = expected {
                    self.emit_err(TypeCheckerError::none_found_non_optional(format!("{ty}"), span));
                    Type::Err
                } else {
                    self.emit_err(TypeCheckerError::could_not_determine_type(format!("{input}"), span));
                    Type::Err
                }
            }
        };

        self.maybe_assert_type(&type_, expected, span);

        type_
    }

    fn visit_ternary(&mut self, input: &TernaryExpression, expected: &Self::AdditionalInput) -> Self::Output {
        self.visit_expression(&input.condition, &Some(Type::Boolean));

        // We try to coerce one side to another in the ternary operator whenever possible and/or needed.
        let (t1, t2) = if expected.is_some() {
            (
                self.visit_expression_reject_numeric(&input.if_true, expected),
                self.visit_expression_reject_numeric(&input.if_false, expected),
            )
        } else if input.if_false.is_none_expr() {
            let t1 = self.visit_expression(&input.if_true, &None);
            if matches!(t1, Type::Optional(_)) {
                (t1.clone(), self.visit_expression(&input.if_false, &Some(t1.clone())))
            } else {
                (
                    t1.clone(),
                    self.visit_expression(
                        &input.if_false,
                        &Some(Type::Optional(OptionalType { inner: Box::new(t1.clone()) })),
                    ),
                )
            }
        } else if input.if_true.is_none_expr() {
            let t2 = self.visit_expression(&input.if_false, &None);
            if matches!(t2, Type::Optional(_)) {
                (t2.clone(), self.visit_expression(&input.if_true, &Some(t2.clone())))
            } else {
                (
                    t2.clone(),
                    self.visit_expression(
                        &input.if_true,
                        &Some(Type::Optional(OptionalType { inner: Box::new(t2.clone()) })),
                    ),
                )
            }
        } else {
            (
                self.visit_expression_reject_numeric(&input.if_true, &None),
                self.visit_expression_reject_numeric(&input.if_false, &None),
            )
        };

        let typ = if t1 == Type::Err || t2 == Type::Err {
            Type::Err
        } else if !t1.can_coerce_to(&t2) && !t2.can_coerce_to(&t1) {
            self.emit_err(TypeCheckerError::ternary_branch_mismatch(t1, t2, input.span()));
            Type::Err
        } else if let Some(expected) = expected {
            expected.clone()
        } else if t1.can_coerce_to(&t2) {
            t2
        } else {
            t1
        };

        // Make sure this isn't an external record type - won't work as we can't construct it.
        if self.is_external_record(&typ) {
            self.emit_err(TypeCheckerError::ternary_over_external_records(&typ, input.span));
        }

        // None of its members may be external record types either.
        if let Type::Tuple(tuple) = &typ
            && tuple.elements().iter().any(|ty| self.is_external_record(ty))
        {
            self.emit_err(TypeCheckerError::ternary_over_external_records(&typ, input.span));
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
                let field_types = input
                    .elements
                    .iter()
                    .map(|field| {
                        let ty = self.visit_expression(field, &None);
                        if ty == Type::Numeric {
                            self.emit_err(TypeCheckerError::could_not_determine_type(field.clone(), field.span()));
                            Type::Err
                        } else {
                            ty
                        }
                    })
                    .collect::<Vec<_>>();
                if field_types.iter().all(|f| *f != Type::Err) {
                    let tuple_type = Type::Tuple(TupleType::new(field_types));
                    self.emit_err(TypeCheckerError::type_should_be2(tuple_type, expected, input.span()));
                }

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
                input
                    .elements
                    .iter()
                    .map(|field| {
                        let ty = self.visit_expression(field, &None);
                        if ty == Type::Numeric {
                            self.emit_err(TypeCheckerError::could_not_determine_type(field.clone(), field.span()));
                            Type::Err
                        } else {
                            ty
                        }
                    })
                    .collect::<Vec<_>>(),
            ))
        }
    }

    fn visit_unary(&mut self, input: &UnaryExpression, destination: &Self::AdditionalInput) -> Self::Output {
        let operand_expected = self.unwrap_optional_type(destination);

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
                let type_ = self.visit_expression_reject_numeric(&input.receiver, &operand_expected);
                assert_signed_int(self, &type_);
                type_
            }
            UnaryOperation::AbsWrapped => {
                let type_ = self.visit_expression_reject_numeric(&input.receiver, &operand_expected);
                assert_signed_int(self, &type_);
                type_
            }
            UnaryOperation::Double => {
                let type_ = self.visit_expression_reject_numeric(&input.receiver, &operand_expected);
                if !matches!(&type_, Type::Err | Type::Field | Type::Group) {
                    self.emit_err(TypeCheckerError::type_should_be2(&type_, "a field or group", input.span()));
                }
                type_
            }
            UnaryOperation::Inverse => {
                let mut type_ = self.visit_expression(&input.receiver, &operand_expected);
                if type_ == Type::Numeric {
                    // We can actually infer to `field` here because only fields can be inverted
                    type_ = Type::Field;
                    self.state.type_table.insert(input.receiver.id(), Type::Field);
                } else {
                    self.assert_type(&type_, &Type::Field, input.span());
                }
                type_
            }
            UnaryOperation::Negate => {
                let type_ = self.visit_expression_reject_numeric(&input.receiver, &operand_expected);
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
                let type_ = self.visit_expression_reject_numeric(&input.receiver, &operand_expected);
                if !matches!(&type_, Type::Err | Type::Boolean | Type::Integer(_)) {
                    self.emit_err(TypeCheckerError::type_should_be2(&type_, "a bool or integer", input.span()));
                }
                type_
            }
            UnaryOperation::Square => {
                let mut type_ = self.visit_expression(&input.receiver, &operand_expected);
                if type_ == Type::Numeric {
                    // We can actually infer to `field` here because only fields can be squared
                    type_ = Type::Field;
                    self.state.type_table.insert(input.receiver.id(), Type::Field);
                } else {
                    self.assert_type(&type_, &Type::Field, input.span());
                }
                type_
            }
            UnaryOperation::SquareRoot => {
                let mut type_ = self.visit_expression(&input.receiver, &operand_expected);
                if type_ == Type::Numeric {
                    // We can actually infer to `field` here because only fields can be square-rooted
                    type_ = Type::Field;
                    self.state.type_table.insert(input.receiver.id(), Type::Field);
                } else {
                    self.assert_type(&type_, &Type::Field, input.span());
                }
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

    /* Statements */
    fn visit_statement(&mut self, input: &Statement) {
        // No statements can follow a return statement.
        if self.scope_state.has_return {
            self.emit_err(TypeCheckerError::unreachable_code_after_return(input.span()));
            return;
        }

        match input {
            Statement::Assert(stmt) => self.visit_assert(stmt),
            Statement::Assign(stmt) => self.visit_assign(stmt),
            Statement::Block(stmt) => self.visit_block(stmt),
            Statement::Conditional(stmt) => self.visit_conditional(stmt),
            Statement::Const(stmt) => self.visit_const(stmt),
            Statement::Definition(stmt) => self.visit_definition(stmt),
            Statement::Expression(stmt) => self.visit_expression_statement(stmt),
            Statement::Iteration(stmt) => self.visit_iteration(stmt),
            Statement::Return(stmt) => self.visit_return(stmt),
        }
    }

    fn visit_assert(&mut self, input: &AssertStatement) {
        match &input.variant {
            AssertVariant::Assert(expr) => {
                let _type = self.visit_expression(expr, &Some(Type::Boolean));
            }
            AssertVariant::AssertEq(left, right) | AssertVariant::AssertNeq(left, right) => {
                let t1 = self.visit_expression_reject_numeric(left, &None);
                let t2 = self.visit_expression_reject_numeric(right, &None);

                if t1 != Type::Err && t2 != Type::Err && !t1.eq_user(&t2) {
                    let op =
                        if matches!(input.variant, AssertVariant::AssertEq(..)) { "assert_eq" } else { "assert_neq" };
                    self.emit_err(TypeCheckerError::operation_types_mismatch(op, &t1, &t2, input.span()));
                }
            }
        }
    }

    fn visit_assign(&mut self, input: &AssignStatement) {
        let assign_target_info = self.visit_expression_assign(&input.place);
        let value = &input.value;

        if assign_target_info.kind == AssignTargetKind::Err {
            self.visit_expression(value, &None);
            return;
        }

        if assign_target_info.kind == AssignTargetKind::Storage
            && !assign_target_info.ty.is_vector()
            && !assign_target_info.ty.is_mapping()
        {
            self.check_access_allowed("storage write", true, input.place.span())
        }

        if assign_target_info.kind == AssignTargetKind::ExternalStorage {
            self.emit_err(TypeCheckerError::cannot_modify_external_storage_variable(input.span()));
        }

        let expected_rhs_ty = match (&assign_target_info.kind, value.is_none_expr(), &assign_target_info.ty) {
            (AssignTargetKind::Storage, false, Type::Optional(OptionalType { inner })) => {
                // For storage variables that are being assigned to a value other than `none` where
                // assign_target_info.ty` is an optional, we want the expected type to be the wrapped type
                // in that optional. For example:
                // ```leo
                // storage x: u8; // actually optional
                //
                // x = 5` // expected type of the RHS is just `u8` (not `u8`?)
                // ```
                // Note that this means that this would fail with a type mismatch:
                // ```leo
                // let y: u8? = 5;
                // x = y; // Assigning an optional to a storage variable.
                // ```
                // We should probably eventually support this for completeness, but it add some
                // complications that makes it not worth it as this stage.
                Some(*inner.clone())
            }
            _ => {
                // Fore everything else, just proceed as normal.
                Some(assign_target_info.ty)
            }
        };

        self.visit_expression(value, &expected_rhs_ty);
    }

    fn visit_block(&mut self, input: &Block) {
        self.in_scope(input.id, |slf| {
            input.statements.iter().for_each(|stmt| slf.visit_statement(stmt));
        });
    }

    fn visit_conditional(&mut self, input: &ConditionalStatement) {
        self.visit_expression(&input.condition, &Some(Type::Boolean));

        let mut then_block_has_return = false;
        let mut otherwise_block_has_return = false;

        // Set the `has_return` flag for the then-block.
        let previous_has_return = core::mem::replace(&mut self.scope_state.has_return, then_block_has_return);
        // Set the `is_conditional` flag.
        let previous_is_conditional = core::mem::replace(&mut self.scope_state.is_conditional, true);

        // Visit block.
        self.in_conditional_scope(|slf| slf.visit_block(&input.then));

        // Store the `has_return` flag for the then-block.
        then_block_has_return = self.scope_state.has_return;

        if let Some(otherwise) = &input.otherwise {
            // Set the `has_return` flag for the otherwise-block.
            self.scope_state.has_return = otherwise_block_has_return;

            match &**otherwise {
                Statement::Block(stmt) => {
                    // Visit the otherwise-block.
                    self.in_conditional_scope(|slf| slf.visit_block(stmt));
                }
                Statement::Conditional(stmt) => self.visit_conditional(stmt),
                _ => unreachable!("Else-case can only be a block or conditional statement."),
            }

            // Store the `has_return` flag for the otherwise-block.
            otherwise_block_has_return = self.scope_state.has_return;
        }

        // Restore the previous `has_return` flag.
        self.scope_state.has_return = previous_has_return || (then_block_has_return && otherwise_block_has_return);
        // Restore the previous `is_conditional` flag.
        self.scope_state.is_conditional = previous_is_conditional;
    }

    fn visit_const(&mut self, input: &ConstDeclaration) {
        self.visit_type(&input.type_);

        // For now, consts that contain optional types are not supported.
        // TODO: remove this restriction by supporting const evaluation of optionals including `None`.
        if self.contains_optional_type(&input.type_) {
            self.emit_err(TypeCheckerError::const_cannot_be_optional(input.span));
        }

        // Check that the type of the definition is not a unit type, singleton tuple type, or nested tuple type.
        match &input.type_ {
            // If the type is an empty tuple, return an error.
            Type::Unit => self.emit_err(TypeCheckerError::lhs_must_be_identifier_or_tuple(input.span)),
            // If the type is a singleton tuple, return an error.
            Type::Tuple(tuple) => match tuple.length() {
                0 | 1 => unreachable!("Parsing guarantees that tuple types have at least two elements."),
                _ => {
                    if tuple.elements().iter().any(|type_| matches!(type_, Type::Tuple(_))) {
                        self.emit_err(TypeCheckerError::nested_tuple_type(input.span))
                    }
                }
            },
            Type::Mapping(_) | Type::Err => unreachable!(
                "Parsing guarantees that `mapping` and `err` types are not present at this location in the AST."
            ),
            // Otherwise, the type is valid.
            _ => (), // Do nothing
        }

        // Check the expression on the right-hand side.
        self.visit_expression(&input.value, &Some(input.type_.clone()));

        if self.scope_state.function.is_some() {
            // Global consts have already been handled, so only set the type here if it's local
            self.state.symbol_table.set_local_type(input.place.name, input.type_.clone());
        }
    }

    fn visit_definition(&mut self, input: &DefinitionStatement) {
        // Check that the type annotation of the definition is valid, if provided.
        if let Some(ty) = &input.type_ {
            self.visit_type(ty);
            self.assert_type_is_valid(ty, input.span);
        }

        // Check that the type of the definition is not a unit type, singleton tuple type, or nested tuple type.
        match &input.type_ {
            // If the type is a singleton tuple, return an error.
            Some(Type::Tuple(tuple)) => match tuple.length() {
                0 | 1 => unreachable!("Parsing guarantees that tuple types have at least two elements."),
                _ => {
                    for type_ in tuple.elements() {
                        if matches!(type_, Type::Tuple(_)) {
                            self.emit_err(TypeCheckerError::nested_tuple_type(input.span))
                        }
                    }
                }
            },
            Some(Type::Mapping(_)) | Some(Type::Err) => unreachable!(
                "Parsing guarantees that `mapping` and `err` types are not present at this location in the AST."
            ),
            // Otherwise, the type is valid.
            _ => (), // Do nothing
        }

        // Check the expression on the right-hand side. If we could not resolve `Type::Numeric`, then just give up.
        // We could do better in the future by potentially looking at consumers of this variable and inferring type
        // information from them.
        let inferred_type = self.visit_expression_reject_numeric(&input.value, &input.type_);

        // If the RHS is a storage vector, error out. Storage vectors cannot be assigned to a variable.
        // They can only be accessed directly via `push`, `pop`, etc.
        if inferred_type.is_vector() {
            self.emit_err(TypeCheckerError::storage_vectors_cannot_be_moved_or_assigned(input.value.span()));
        }

        // Set the types of variables in the symbol.
        match &input.place {
            DefinitionPlace::Single(identifier) => {
                self.set_local_type(
                    Some(inferred_type.clone()),
                    identifier,
                    input.type_.clone().unwrap_or(inferred_type),
                );
            }
            DefinitionPlace::Multiple(identifiers) => {
                // Get the tuple type either from `input.type_` or from `inferred_type`.
                let tuple_type = match (&input.type_, inferred_type.clone()) {
                    (Some(Type::Tuple(tuple_type)), _) => tuple_type.clone(),
                    (None, Type::Tuple(tuple_type)) => tuple_type.clone(),
                    _ => {
                        // This is an error but should have been emitted earlier. Just exit here.
                        return;
                    }
                };

                // Ensure the number of identifiers we're defining is the same as the number of tuple elements, as
                // indicated by `tuple_type`
                if identifiers.len() != tuple_type.length() {
                    return self.emit_err(TypeCheckerError::incorrect_num_tuple_elements(
                        identifiers.len(),
                        tuple_type.length(),
                        input.span(),
                    ));
                }

                // Now just insert each tuple element as a separate variable
                for (i, identifier) in identifiers.iter().enumerate() {
                    let inferred = if let Type::Tuple(inferred_tuple) = &inferred_type {
                        inferred_tuple.elements().get(i).cloned().unwrap_or_default()
                    } else {
                        Type::Err
                    };
                    self.set_local_type(Some(inferred), identifier, tuple_type.elements()[i].clone());
                }
            }
        }
    }

    fn visit_expression_statement(&mut self, input: &ExpressionStatement) {
        // Expression statements can only be function calls.
        if !matches!(input.expression, Expression::Call(_) | Expression::Intrinsic(_) | Expression::Unit(_)) {
            self.emit_err(TypeCheckerError::expression_statement_must_be_function_call(input.span()));
        } else {
            // Check the expression.
            self.visit_expression(&input.expression, &None);
        }
    }

    fn visit_iteration(&mut self, input: &IterationStatement) {
        // Ensure the type annotation is an integer type
        if let Some(ty) = &input.type_ {
            self.visit_type(ty);
            self.assert_int_type(ty, input.variable.span);
        }

        // These are the types of the start and end expressions of the iterator range. `visit_expression` will make
        // sure they match `input.type_` (i.e. the iterator type annotation) if available.
        let start_ty = self.visit_expression(&input.start, &input.type_.clone());
        let stop_ty = self.visit_expression(&input.stop, &input.type_.clone());

        // Ensure both types are integer types
        self.assert_int_type(&start_ty, input.start.span());
        self.assert_int_type(&stop_ty, input.stop.span());

        if start_ty != stop_ty {
            // Emit an error if the types of the range bounds do not match
            self.emit_err(TypeCheckerError::range_bounds_type_mismatch(input.start.span() + input.stop.span()));
        }

        // Now, just set the type of the iterator variable to `start_ty` if `input.type_` is not available. If `stop_ty`
        // does not match `start_ty` and `input.type_` is not available, the we just recover with `start_ty` anyways
        // and continue.
        let iterator_ty = input.type_.clone().unwrap_or(start_ty);
        self.state.type_table.insert(input.variable.id(), iterator_ty.clone());

        self.in_scope(input.id(), |slf| {
            // Set the type of the loop variable in the scope of the loop body.
            slf.state.symbol_table.set_local_type(input.variable.name, iterator_ty.clone());

            let prior_has_return = core::mem::take(&mut slf.scope_state.has_return);
            let prior_has_finalize = core::mem::take(&mut slf.scope_state.has_called_finalize);

            slf.visit_block(&input.block);

            if slf.scope_state.has_return {
                slf.emit_err(TypeCheckerError::loop_body_contains_return(input.span()));
            }

            if slf.scope_state.has_called_finalize {
                slf.emit_err(TypeCheckerError::loop_body_contains_async("function call", input.span()));
            }

            if slf.scope_state.already_contains_an_async_block {
                slf.emit_err(TypeCheckerError::loop_body_contains_async("block expression", input.span()));
            }

            slf.scope_state.has_return = prior_has_return;
            slf.scope_state.has_called_finalize = prior_has_finalize;
        });
    }

    fn visit_return(&mut self, input: &ReturnStatement) {
        if self.async_block_id.is_some() {
            return self.emit_err(TypeCheckerError::async_block_cannot_return(input.span()));
        }

        if self.scope_state.is_constructor {
            // It must return a unit value; nothing else to check.
            if !matches!(input.expression, Expression::Unit(..)) {
                self.emit_err(TypeCheckerError::constructor_can_only_return_unit(&input.expression, input.span));
            }
            return;
        }

        let caller_name = self.scope_state.function.expect("`self.function` is set every time a function is visited.");
        let caller_path =
            self.scope_state.module_name.iter().cloned().chain(std::iter::once(caller_name)).collect::<Vec<Symbol>>();

        let current_program = self.scope_state.program_name.unwrap();

        let func_symbol = self
            .state
            .symbol_table
            .lookup_function(current_program, &Location::new(current_program, caller_path.clone()))
            .expect("The symbol table creator should already have visited all functions.");

        let mut return_type = func_symbol.function.output_type.clone();

        if self.scope_state.variant == Some(Variant::AsyncTransition) && self.scope_state.has_called_finalize {
            let inferred_future_type = Future(FutureType::new(
                if let Some(finalizer) = &func_symbol.finalizer { finalizer.inferred_inputs.clone() } else { vec![] },
                Some(Location::new(current_program, caller_path)),
                true,
            ));

            // Need to modify return type since the function signature is just default future, but the actual return
            // type is the fully inferred future of the finalize input type.
            let inferred = match return_type.clone() {
                Future(_) => inferred_future_type,
                Tuple(tuple) => Tuple(TupleType::new(
                    tuple
                        .elements()
                        .iter()
                        .map(|t| if matches!(t, Future(_)) { inferred_future_type.clone() } else { t.clone() })
                        .collect::<Vec<Type>>(),
                )),
                _ => {
                    return self.emit_err(TypeCheckerError::async_transition_missing_future_to_return(input.span()));
                }
            };

            // Check that the explicit type declared in the function output signature matches the inferred type.
            return_type = self.assert_and_return_type(inferred, &Some(return_type), input.span());
        }

        if matches!(input.expression, Expression::Unit(..)) {
            // Manually type check rather than using one of the assert functions for a better error message.
            if return_type != Type::Unit {
                // TODO - This is a bit hackish. We're reusing an existing error, because
                // we have too many errors in TypeCheckerError without hitting the recursion
                // limit for macros. But the error message to the user should still be pretty clear.
                return self.emit_err(TypeCheckerError::missing_return(input.span()));
            }
        }

        self.visit_expression(&input.expression, &Some(return_type));

        // Set the `has_return` flag after processing `input.expression` so that we don't error out
        // on something like `return async { .. }`.
        self.scope_state.has_return = true;
    }
}
