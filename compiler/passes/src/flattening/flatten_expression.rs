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

use crate::{Flattener};
use itertools::Itertools;

use leo_ast::{
    AccessExpression, CircuitExpression, CircuitMember,
    CircuitVariableInitializer, ErrExpression, Expression, ExpressionReconstructor,
    MemberAccess, Statement, TernaryExpression, TupleExpression,
};

// TODO: Document

impl ExpressionReconstructor for Flattener<'_> {
    type AdditionalOutput = Vec<Statement>;

    /// Reconstructs ternary expressions over circuits, accumulating any statements that are generated.
    fn reconstruct_ternary(&mut self, input: TernaryExpression) -> (Expression, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        match (*input.if_true, *input.if_false) {
            (Expression::Tuple(first), Expression::Tuple(second)) => {
                let tuple = Expression::Tuple(TupleExpression {
                    elements: first
                        .elements
                        .into_iter()
                        .zip_eq(second.elements.into_iter())
                        .map(|(if_true, if_false)| {
                            let (ternary, stmts) = self.reconstruct_ternary(TernaryExpression {
                                condition: input.condition,
                                if_true: Box::new(if_true),
                                if_false: Box::new(if_false),
                                span: input.span,
                            });
                            statements.extend(stmts);
                            ternary
                        })
                        .collect(),
                    span: Default::default(),
                });
                (tuple, statements)
            }
            // If the `true` and `false` cases are circuits, handle them, appropriately.
            // Note that type checking guarantees that both expressions have the same same type.
            (Expression::Identifier(first), Expression::Identifier(second))
                if self.circuits.contains_key(&first.name) && self.circuits.contains_key(&second.name) =>
            {
                // TODO: Document.
                let first_circuit = self
                    .symbol_table
                    .lookup_circuit(*self.circuits.get(&first.name).unwrap())
                    .unwrap();
                let second_circuit = self
                    .symbol_table
                    .lookup_circuit(*self.circuits.get(&second.name).unwrap())
                    .unwrap();
                assert_eq!(first_circuit, second_circuit);

                // For each circuit member, construct a new ternary expression.
                let members = first_circuit
                    .members
                    .iter()
                    .map(|CircuitMember::CircuitVariable(id, _)| {
                        let (expression, stmts) = self.reconstruct_ternary(TernaryExpression {
                            condition: input.condition,
                            if_true: Box::new(Expression::Access(AccessExpression::Member(MemberAccess {
                                inner: Box::new(Expression::Identifier(first)),
                                name: *id,
                                span: Default::default(),
                            }))),
                            if_false: Box::new(Expression::Access(AccessExpression::Member(MemberAccess {
                                inner: Box::new(Expression::Identifier(second)),
                                name: *id,
                                span: Default::default(),
                            }))),
                            span: Default::default(),
                        });
                        statements.extend(stmts);

                        CircuitVariableInitializer {
                            identifier: *id,
                            expression: Some(expression),
                        }
                    })
                    .collect();

                let (expr, stmts) = self.reconstruct_circuit_init(CircuitExpression {
                    name: first_circuit.identifier,
                    members,
                    span: Default::default(),
                });

                statements.extend(stmts);

                (expr, statements)
            }
            // Otherwise, return the original expression.
            (if_true, if_false) => (Expression::Ternary(TernaryExpression {
                condition: input.condition,
                if_true: Box::new(if_true),
                if_false: Box::new(if_false),
                span: input.span,
            }), Default::default())
        }
    }
}
