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
    CallExpression,
    Expression,
    ExpressionReconstructor,
    Function,
    Identifier,
    StatementReconstructor,
    Variant,
};

use indexmap::IndexMap;
use itertools::Itertools;

impl ExpressionReconstructor for MonomorphizationVisitor<'_> {
    type AdditionalOutput = ();

    fn reconstruct_call(&mut self, input_call: CallExpression) -> (Expression, Self::AdditionalOutput) {
        // Skip calls to functions from other programs.
        if input_call.program.unwrap() != self.program {
            return (input_call.into(), Default::default());
        }

        // Extract the function name from the call expression.
        let Expression::Identifier(Identifier { name: callee_name, .. }) = &input_call.function else {
            panic!("Parser ensures `function` is always an identifier.")
        };

        // Look up the already reconstructed function by name.
        let callee_fn = self
            .reconstructed_functions
            .get(callee_name)
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
        // For a function `fn foo::[x: u32, y: u32](..)`, the generated name would be `foo::[1u32, 2u32]` for a call
        // that sets `x` to `1u32` and `y` to `2u32`. We know this name is safe to use because it's not a valid
        // identifier in the user code.
        let new_callee_name = leo_span::Symbol::intern(&format!(
            "{}::[{}]",
            callee_name,
            input_call.const_arguments.iter().map(|arg| arg.to_string()).format(", ")
        ));

        // Check if the new callee name is not already present in `reconstructed_functions`. This ensures that we do not
        // add a duplicate entry for the same function and only insert a new version with a unique name.
        if self.reconstructed_functions.get(&new_callee_name).is_none() {
            // Build mapping from const parameters to const argument values.
            let const_param_map: IndexMap<_, _> = callee_fn
                .const_parameters
                .iter()
                .map(|param| param.identifier().name)
                .zip_eq(&input_call.const_arguments)
                .collect();

            // Function to replace identifiers with their corresponding const argument or keep them unchanged.
            let replace_identifier = |ident: &Identifier| {
                const_param_map.get(&ident.name).map_or(Expression::Identifier(*ident), |&expr| expr.clone())
            };

            // Add a new copy of `callee_fn` with a new name, no const parameters, and the monomorphized block
            self.reconstructed_functions.insert(new_callee_name, Function {
                identifier: Identifier {
                    name: new_callee_name,
                    span: leo_span::Span::default(),
                    id: self.state.node_builder.next_id(),
                },
                annotations: callee_fn.annotations.clone(),
                variant: callee_fn.variant,
                const_parameters: Vec::new(), // Remove const parameters
                input: callee_fn.input.clone(),
                output: callee_fn.output.clone(),
                output_type: callee_fn.output_type.clone(),
                block: Replacer::new(replace_identifier).reconstruct_block(callee_fn.block.clone()).0,
                span: callee_fn.span,
                id: callee_fn.id,
            });

            // Now keep track of the function we just monomorphized
            self.monomorphized_functions.insert(*callee_name);
        }

        // Update call graph with edges for the monomorphized function. We do this by basically cloning the edges in
        // and out of `callee_name` and replicating them for a new node that contains `new_callee_name`.
        if let Some(neighbors) = self.state.call_graph.neighbors(callee_name) {
            for neighbor in neighbors {
                if neighbor != *callee_name {
                    self.state.call_graph.add_edge(new_callee_name, neighbor);
                }
            }
        }
        self.state.call_graph.add_edge(self.function, new_callee_name);

        // Finally, construct the updated call expression that points to the monomorphized version and return it.
        (
            CallExpression {
                function: Expression::Identifier(Identifier {
                    name: new_callee_name, // use the new name
                    span: leo_span::Span::default(),
                    id: self.state.node_builder.next_id(),
                }),
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
}
