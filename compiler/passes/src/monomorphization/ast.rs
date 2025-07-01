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

use super::MonomorphizationVisitor;
use crate::Replacer;

use leo_ast::{
    AstReconstructor,
    CallExpression,
    CompositeType,
    Expression,
    Identifier,
    ProgramReconstructor,
    StructExpression,
    StructVariableInitializer,
    Type,
    Variant,
};

use indexmap::IndexMap;
use itertools::Itertools;

impl AstReconstructor for MonomorphizationVisitor<'_> {
    type AdditionalOutput = ();

    /* Types */
    fn reconstruct_composite_type(&mut self, input: leo_ast::CompositeType) -> (leo_ast::Type, Self::AdditionalOutput) {
        // Proceed only if there are some const arguments.
        if input.const_arguments.is_empty() {
            return (Type::Composite(input), Default::default());
        }

        // Ensure all const arguments are literals; if not, we skip this struct type instantiation for now and mark it
        // as unresolved.
        //
        // The types of the const arguments are checked in the type checking pass.
        if input.const_arguments.iter().any(|arg| !matches!(arg, Expression::Literal(_))) {
            self.unresolved_struct_types.push(input.clone());
            return (Type::Composite(input), Default::default());
        }

        // At this stage, we know that we're going to modify the program
        self.changed = true;
        (
            Type::Composite(CompositeType {
                id: Identifier {
                    name: self.monomorphize_struct(&input.id.name, &input.const_arguments), // use the new name
                    span: input.id.span,
                    id: self.state.node_builder.next_id(),
                },
                const_arguments: vec![], // remove const arguments
                program: input.program,
            }),
            Default::default(),
        )
    }

    /* Expressions */
    fn reconstruct_call(&mut self, input_call: CallExpression) -> (Expression, Self::AdditionalOutput) {
        // Skip calls to functions from other programs.
        if input_call.program.unwrap() != self.program {
            return (input_call.into(), Default::default());
        }

        // Look up the already reconstructed function by name.
        let callee_fn = self
            .reconstructed_functions
            .get(&input_call.function.name)
            .expect("Callee should already be reconstructed (post-order traversal).");

        // Proceed only if the function variant is `inline` and if there are some const arguments.
        if !matches!(callee_fn.variant, Variant::Inline) || input_call.const_arguments.is_empty() {
            return (input_call.into(), Default::default());
        }

        // Ensure all const arguments are literals; if not, we skip this call for now and mark it as unresolved.
        //
        // The types of the const arguments are checked in the type checking pass.
        if input_call.const_arguments.iter().any(|arg| !matches!(arg, Expression::Literal(_))) {
            self.unresolved_calls.push(input_call.clone());
            return (input_call.into(), Default::default());
        }

        // Generate a unique name for the monomorphized function based on const arguments.
        //
        // For a function `fn foo::[x: u32, y: u32](..)`, the generated name would be `"foo::[1u32, 2u32]"` for a call
        // that sets `x` to `1u32` and `y` to `2u32`. We know this name is safe to use because it's not a valid
        // identifier in the user code.
        let new_callee_name = leo_span::Symbol::intern(&format!(
            "\"{}::[{}]\"",
            input_call.function.name,
            input_call.const_arguments.iter().format(", ")
        ));

        // Check if the new callee name is not already present in `reconstructed_functions`. This ensures that we do not
        // add a duplicate definition for the same function.
        if self.reconstructed_functions.get(&new_callee_name).is_none() {
            // Build mapping from const parameters to const argument values.
            let const_param_map: IndexMap<_, _> = callee_fn
                .const_parameters
                .iter()
                .map(|param| param.identifier().name)
                .zip_eq(&input_call.const_arguments)
                .collect();

            // Function to replace identifier expressions with their corresponding const argument or keep them unchanged.
            let replace_identifier = |expr: &Expression| match expr {
                Expression::Identifier(ident) => {
                    const_param_map.get(&ident.name).map_or(Expression::Identifier(*ident), |&expr| expr.clone())
                }
                _ => expr.clone(),
            };

            let mut replacer = Replacer::new(replace_identifier, &self.state.node_builder);

            // Create a new version of `callee_fn` that has a new name, no const parameters, and a new function ID.

            // First, reconstruct the function by changing all instances of const generic parameters to literals
            // according to `const_param_map`.
            let mut function = replacer.reconstruct_function(callee_fn.clone());

            // Now, reconstruct the function to actually monomorphize its content such as generic struct expressions.
            function = self.reconstruct_function(function);
            function.identifier = Identifier {
                name: new_callee_name,
                span: leo_span::Span::default(),
                id: self.state.node_builder.next_id(),
            };
            function.const_parameters = vec![];
            function.id = self.state.node_builder.next_id();

            // Keep track of the new function in case other functions need it.
            self.reconstructed_functions.insert(new_callee_name, function);

            // Now keep track of the function we just monomorphized
            self.monomorphized_functions.insert(input_call.function.name);
        }

        // At this stage, we know that we're going to modify the program
        self.changed = true;

        // Finally, construct the updated call expression that points to a monomorphized version and return it.
        (
            CallExpression {
                function: Identifier {
                    name: new_callee_name, // use the new name
                    span: leo_span::Span::default(),
                    id: self.state.node_builder.next_id(),
                },
                const_arguments: vec![], // remove const arguments
                arguments: input_call.arguments,
                program: input_call.program,
                span: input_call.span, // Keep pointing to the original call expression
                id: input_call.id,
            }
            .into(),
            Default::default(),
        )
    }

    fn reconstruct_struct_init(&mut self, mut input: StructExpression) -> (Expression, Self::AdditionalOutput) {
        // Handle all the struct members first
        let members = input
            .members
            .clone()
            .into_iter()
            .map(|member| StructVariableInitializer {
                identifier: member.identifier,
                expression: member.expression.map(|expr| self.reconstruct_expression(expr).0),
                span: member.span,
                id: member.id,
            })
            .collect();

        // Proceed only if there are some const arguments.
        if input.const_arguments.is_empty() {
            input.members = members;
            return (input.into(), Default::default());
        }

        // Ensure all const arguments are literals; if not, we skip this struct expression for now and mark it as
        // unresolved.
        //
        // The types of the const arguments are checked in the type checking pass.
        if input.const_arguments.iter().any(|arg| !matches!(arg, Expression::Literal(_))) {
            self.unresolved_struct_exprs.push(input.clone());
            input.members = members;
            return (input.into(), Default::default());
        }

        // At this stage, we know that we're going to modify the program
        self.changed = true;

        // Finally, construct the updated struct expression that points to a monomorphized version and return it.
        (
            StructExpression {
                name: Identifier {
                    name: self.monomorphize_struct(&input.name.name, &input.const_arguments),
                    span: input.name.span,
                    id: self.state.node_builder.next_id(),
                },
                members,
                const_arguments: vec![], // remove const arguments
                span: input.span,        // Keep pointing to the original struct expression
                id: input.id,
            }
            .into(),
            Default::default(),
        )
    }
}
