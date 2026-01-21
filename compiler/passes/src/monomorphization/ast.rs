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

use super::MonomorphizationVisitor;
use crate::{ConstPropagationVisitor, Replacer};

use leo_ast::{
    AstReconstructor,
    CallExpression,
    CompositeExpression,
    CompositeFieldInitializer,
    CompositeType,
    Expression,
    Identifier,
    Node as _,
    ProgramReconstructor,
    Type,
    Variant,
};

use indexmap::IndexMap;
use itertools::{Either, Itertools};

impl<'a> MonomorphizationVisitor<'a> {
    /// Evaluates the given constant arguments if possible.
    ///
    /// Returns `Some` with all evaluated expressions if all are constants, or `None` if any argument is not constant.
    fn try_evaluate_const_args(&mut self, const_args: &[Expression]) -> Option<Vec<Expression>> {
        let mut const_evaluator = ConstPropagationVisitor::new(self.state, self.program);

        let (evaluated_const_args, non_const_args): (Vec<_>, Vec<_>) = const_args
            .iter()
            .map(|arg| const_evaluator.reconstruct_expression(arg.clone(), &()))
            .partition_map(|(evaluated_arg, evaluated_value)| match (evaluated_value, evaluated_arg) {
                (Some(_), expr @ Expression::Literal(_)) => Either::Left(expr),
                _ => Either::Right(()),
            });

        if !non_const_args.is_empty() { None } else { Some(evaluated_const_args) }
    }
}

impl AstReconstructor for MonomorphizationVisitor<'_> {
    type AdditionalInput = ();
    type AdditionalOutput = ();

    /* Types */
    fn reconstruct_composite_type(&mut self, input: leo_ast::CompositeType) -> (leo_ast::Type, Self::AdditionalOutput) {
        // Proceed only if there are some const arguments.
        if input.const_arguments.is_empty() {
            return (Type::Composite(input), Default::default());
        }

        // Ensure all const arguments can be evaluated to literals; if not, we skip this composite type instantiation for
        // now and mark it as unresolved.
        //
        // The types of the const arguments are already checked in the type checking pass.
        let Some(evaluated_const_args) = self.try_evaluate_const_args(&input.const_arguments) else {
            self.unresolved_composite_types.push(input.clone());
            return (Type::Composite(input), Default::default());
        };

        // At this stage, we know that we're going to modify the program
        self.changed = true;
        (
            Type::Composite(CompositeType {
                path: self.monomorphize_composite(&input.path, &evaluated_const_args),
                const_arguments: vec![], // remove const arguments
            }),
            Default::default(),
        )
    }

    /* Expressions */
    fn reconstruct_expression(&mut self, input: Expression, _additional: &()) -> (Expression, Self::AdditionalOutput) {
        let opt_old_type = self.state.type_table.get(&input.id());
        let (new_expr, opt_value) = match input {
            Expression::Array(array) => self.reconstruct_array(array, &()),
            Expression::ArrayAccess(access) => self.reconstruct_array_access(*access, &()),
            Expression::Intrinsic(intr) => self.reconstruct_intrinsic(*intr, &()),
            Expression::Async(async_) => self.reconstruct_async(async_, &()),
            Expression::Binary(binary) => self.reconstruct_binary(*binary, &()),
            Expression::Call(call) => self.reconstruct_call(*call, &()),
            Expression::Cast(cast) => self.reconstruct_cast(*cast, &()),
            Expression::Composite(composite) => self.reconstruct_composite_init(composite, &()),
            Expression::Err(err) => self.reconstruct_err(err, &()),
            Expression::Path(path) => self.reconstruct_path(path, &()),
            Expression::Literal(value) => self.reconstruct_literal(value, &()),
            Expression::Locator(locator) => self.reconstruct_locator(locator, &()),
            Expression::MemberAccess(access) => self.reconstruct_member_access(*access, &()),
            Expression::Repeat(repeat) => self.reconstruct_repeat(*repeat, &()),
            Expression::Ternary(ternary) => self.reconstruct_ternary(*ternary, &()),
            Expression::Tuple(tuple) => self.reconstruct_tuple(tuple, &()),
            Expression::TupleAccess(access) => self.reconstruct_tuple_access(*access, &()),
            Expression::Unary(unary) => self.reconstruct_unary(*unary, &()),
            Expression::Unit(unit) => self.reconstruct_unit(unit, &()),
        };

        // If the expression was in the type table before, make an entry for the new expression.
        if let Some(old_type) = opt_old_type {
            self.state.type_table.insert(new_expr.id(), old_type);
        }

        (new_expr, opt_value)
    }

    fn reconstruct_call(
        &mut self,
        input_call: CallExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        // Skip calls to functions from other programs.
        if input_call.function.expect_global_location().program != self.program {
            return (input_call.into(), Default::default());
        }

        // Proceed only if there are some const arguments.
        if input_call.const_arguments.is_empty() {
            return (input_call.into(), Default::default());
        }

        // Ensure all const arguments can be evaluated to literals; if not, we skip this call for now and mark it as
        // unresolved.
        //
        // The types of the const arguments are already checked in the type checking pass.
        let Some(evaluated_const_args) = self.try_evaluate_const_args(&input_call.const_arguments) else {
            self.unresolved_calls.push(input_call.clone());
            return (input_call.into(), Default::default());
        };

        // Look up the already reconstructed function by name.
        let callee_fn = self
            .reconstructed_functions
            .get(&input_call.function.expect_global_location().path)
            .expect("Callee should already be reconstructed (post-order traversal).");

        // Proceed only if the function variant is `inline`.
        if !matches!(callee_fn.variant, Variant::Inline) {
            return (input_call.into(), Default::default());
        }

        // Generate a unique name for the monomorphized function based on const arguments.
        //
        // For a function `fn foo::[x: u32, y: u32](..)`, the generated name would be `"foo::[1u32, 2u32]"` for a call
        // that sets `x` to `1u32` and `y` to `2u32`. We know this name is safe to use because it's not a valid
        // identifier in the user code.
        let new_callee_path = input_call.function.clone().with_updated_last_symbol(leo_span::Symbol::intern(&format!(
            "\"{}::[{}]\"",
            input_call.function.identifier().name,
            evaluated_const_args.iter().format(", ")
        )));

        // Check if the new callee name is not already present in `reconstructed_functions`. This ensures that we do not
        // add a duplicate definition for the same function.
        if self.reconstructed_functions.get(&new_callee_path.expect_global_location().path).is_none() {
            // Build mapping from const parameters to const argument values.
            let const_param_map: IndexMap<_, _> = callee_fn
                .const_parameters
                .iter()
                .map(|param| param.identifier().name)
                .zip_eq(&evaluated_const_args)
                .collect();

            // Function to replace identifier expressions with their corresponding const argument or keep them unchanged.
            let replace_identifier = |expr: &Expression| match expr {
                Expression::Path(path) => const_param_map
                    .get(&path.identifier().name)
                    .map_or(Expression::Path(path.clone()), |&expr| expr.clone()),
                _ => expr.clone(),
            };

            let mut replacer = Replacer::new(replace_identifier, true /* refresh IDs */, self.state);

            // Create a new version of `callee_fn` that has a new name, no const parameters, and a new function ID.

            // First, reconstruct the function by changing all instances of const generic parameters to literals
            // according to `const_param_map`.
            let mut function = replacer.reconstruct_function(callee_fn.clone());

            // Now, reconstruct the function to actually monomorphize its content such as generic struct expressions.
            function = self.reconstruct_function(function);
            function.identifier = Identifier {
                name: new_callee_path.identifier().name,
                span: leo_span::Span::default(),
                id: self.state.node_builder.next_id(),
            };
            function.const_parameters = vec![];
            function.id = self.state.node_builder.next_id();

            // Keep track of the new function in case other functions need it.
            self.reconstructed_functions.insert(new_callee_path.expect_global_location().path.clone(), function);

            // Now keep track of the function we just monomorphized
            self.monomorphized_functions.insert(input_call.function.expect_global_location().path.clone());
        }

        // At this stage, we know that we're going to modify the program
        self.changed = true;

        // Finally, construct the updated call expression that points to a monomorphized version and return it.
        (
            CallExpression {
                function: new_callee_path,
                const_arguments: vec![], // remove const arguments
                arguments: input_call.arguments,
                span: input_call.span, // Keep pointing to the original call expression
                id: input_call.id,
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_composite_init(
        &mut self,
        mut input: CompositeExpression,
        _additional: &(),
    ) -> (Expression, Self::AdditionalOutput) {
        // Handle all the composite members first
        let members = input
            .members
            .clone()
            .into_iter()
            .map(|member| CompositeFieldInitializer {
                identifier: member.identifier,
                expression: member.expression.map(|expr| self.reconstruct_expression(expr, &()).0),
                span: member.span,
                id: member.id,
            })
            .collect();

        // Proceed only if there are some const arguments.
        if input.const_arguments.is_empty() {
            input.members = members;
            return (input.into(), Default::default());
        }

        // Ensure all const arguments can be evaluated to literals; if not, we skip this composite expression for now and
        // mark it as unresolved.
        //
        // The types of the const arguments are already checked in the type checking pass.
        let Some(evaluated_const_args) = self.try_evaluate_const_args(&input.const_arguments) else {
            self.unresolved_composite_exprs.push(input.clone());
            input.members = members;
            return (input.into(), Default::default());
        };

        // At this stage, we know that we're going to modify the program
        self.changed = true;

        // Finally, construct the updated composite expression that points to a monomorphized version and return it.
        (
            CompositeExpression {
                path: self.monomorphize_composite(&input.path, &evaluated_const_args),
                members,
                const_arguments: vec![], // remove const arguments
                span: input.span,        // Keep pointing to the original composite expression
                id: input.id,
            }
            .into(),
            Default::default(),
        )
    }
}
