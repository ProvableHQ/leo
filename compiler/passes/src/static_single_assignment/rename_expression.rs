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

use crate::StaticSingleAssigner;

use leo_ast::{
    CallExpression, CircuitExpression, CircuitVariableInitializer, Expression, ExpressionReconstructor, Identifier,
};

impl ExpressionReconstructor for StaticSingleAssigner<'_> {
    type AdditionalOutput = ();

    /// Reconstructs `CallExpression` without visiting the function name.
    fn reconstruct_call(&mut self, input: CallExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Call(CallExpression {
                // Note that we do not rename the function name.
                function: input.function,
                arguments: input
                    .arguments
                    .into_iter()
                    .map(|arg| self.reconstruct_expression(arg).0)
                    .collect(),
                span: input.span,
            }),
            Default::default(),
        )
    }

    /// Produces a new `CircuitExpression` with renamed variables.
    fn reconstruct_circuit_init(&mut self, input: CircuitExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Circuit(CircuitExpression {
                name: input.name,
                span: input.span,
                members: input
                    .members
                    .into_iter()
                    .map(|arg| CircuitVariableInitializer {
                        identifier: arg.identifier,
                        expression: Some(match &arg.expression.is_some() {
                            // If the expression is None, then `arg` is a `CircuitVariableInitializer` of the form `<id>,`.
                            // In this case, we must reconstruct the identifier and produce an initializer of the form `<id>: <renamed_id>`.
                            false => self.reconstruct_identifier(arg.identifier).0,
                            // If expression is `Some(..)`, then `arg is a `CircuitVariableInitializer` of the form `<id>: <expr>,`.
                            // In this case, we must reconstruct the expression.
                            true => self.reconstruct_expression(arg.expression.unwrap()).0,
                        }),
                    })
                    .collect(),
            }),
            Default::default(),
        )
    }

    /// Produces a new `Identifier` with a unique name.
    fn reconstruct_identifier(&mut self, identifier: Identifier) -> (Expression, Self::AdditionalOutput) {
        let name = match self.is_lhs {
            // If reconstructing the left-hand side of a definition or assignment, a new unique name is introduced.
            true => {
                let new_name = self.unique_symbol(identifier.name);
                self.rename_table.update(identifier.name, new_name);
                new_name
            }
            // Otherwise, we look up the previous name in the `RenameTable`.
            false => *self.rename_table.lookup(identifier.name).unwrap_or_else(|| {
                panic!(
                    "SSA Error: An entry in the `RenameTable` for {} should exist.",
                    identifier.name
                )
            }),
        };

        (
            Expression::Identifier(Identifier {
                name,
                span: identifier.span,
            }),
            Default::default(),
        )
    }
}
