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
use leo_span::Symbol;

impl<'a> ExpressionReconstructor for StaticSingleAssigner<'a> {
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

    // TODO: Consider moving this functionality to the Reconstructor.
    /// Produces a new `CircuitExpression` with renamed variables.
    fn reconstruct_circuit_init(&mut self, input: CircuitExpression) -> (Expression, Self::AdditionalOutput) {
        (
            Expression::Circuit(CircuitExpression {
                name: input.name,
                span: input.span,
                members: input
                    .members
                    .into_iter()
                    .map(|arg| match &arg.expression {
                        // If the expression is None, then it is a `CircuitVariableInitializer` of the form `<id>,`.
                        // In this case, we must reconstruct the identifier.
                        None => match self.reconstruct_identifier(arg.identifier).0 {
                            Expression::Identifier(identifier) => CircuitVariableInitializer {
                                identifier,
                                expression: None,
                            },
                            _ => unreachable!("`self.reconstruct_identifier` always returns a `Identifier`."),
                        },
                        // If expression is not `None`, then it is a `CircuitVariableInitializer` of the form `<id>: <expr>,`.
                        // In this case, we must reconstruct the expression.
                        Some(_) => CircuitVariableInitializer {
                            identifier: arg.identifier,
                            expression: Some(self.reconstruct_expression(arg.expression.unwrap()).0),
                        },
                    })
                    .collect(),
            }),
            Default::default(),
        )
    }

    /// Produces a new `Identifier` with a unique name.
    /// If this function is invoked on the left-hand side of a definition or assignment, a new unique name is introduced.
    /// Otherwise, we look up the previous name in the `RenameTable`.
    fn reconstruct_identifier(&mut self, identifier: Identifier) -> (Expression, Self::AdditionalOutput) {
        match self.is_lhs {
            true => {
                let new_name = Symbol::intern(&format!("{}${}", identifier.name, self.get_unique_id()));
                self.rename_table.update(identifier.name, new_name);
                (
                    Expression::Identifier(Identifier {
                        name: new_name,
                        span: identifier.span,
                    }),
                    Default::default(),
                )
            }
            false => {
                match self.rename_table.lookup(&identifier.name) {
                    // TODO: Better error.
                    None => panic!(
                        "Error: A unique name for the variable {} is not defined.",
                        identifier.name
                    ),
                    Some(name) => (
                        Expression::Identifier(Identifier {
                            name: *name,
                            span: identifier.span,
                        }),
                        Default::default(),
                    ),
                }
            }
        }
    }
}
