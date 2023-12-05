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

use crate::Destructurer;

use leo_ast::{
    AssignStatement,
    Block,
    ConditionalStatement,
    ConsoleStatement,
    DefinitionStatement,
    Expression,
    ExpressionReconstructor,
    Identifier,
    IterationStatement,
    Node,
    ReturnStatement,
    Statement,
    StatementReconstructor,
    TupleExpression,
    Type,
};

use itertools::Itertools;

impl StatementReconstructor for Destructurer<'_> {
    /// Flattens an assign statement, if necessary.
    /// Marks variables as structs as necessary.
    /// Note that new statements are only produced if the right hand side is a ternary expression over structs.
    /// Otherwise, the statement is returned as is.
    fn reconstruct_assign(&mut self, assign: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        // Flatten the rhs of the assignment.
        let value = self.reconstruct_expression(assign.value).0;
        match (assign.place, value.clone()) {
            // If the lhs is an identifier and the rhs is a tuple, then add the tuple to `self.tuples`.
            // Return a dummy statement in its place.
            (Expression::Identifier(identifier), Expression::Tuple(tuple)) => {
                self.tuples.insert(identifier.name, tuple);
                // Note that tuple assignments are removed from the AST.
                (Statement::dummy(Default::default(), self.node_builder.next_id()), Default::default())
            }
            // If the lhs is an identifier and the rhs is an identifier that is a tuple, then add it to `self.tuples`.
            // Return a dummy statement in its place.
            (Expression::Identifier(lhs_identifier), Expression::Identifier(rhs_identifier))
                if self.tuples.contains_key(&rhs_identifier.name) =>
            {
                // Lookup the entry in `self.tuples` and add it for the lhs of the assignment.
                // Note that the `unwrap` is safe since the match arm checks that the entry exists.
                self.tuples.insert(lhs_identifier.name, self.tuples.get(&rhs_identifier.name).unwrap().clone());
                // Note that tuple assignments are removed from the AST.
                (Statement::dummy(Default::default(), self.node_builder.next_id()), Default::default())
            }
            // If the lhs is an identifier and the rhs is a function call that produces a tuple, then add it to `self.tuples`.
            (Expression::Identifier(lhs_identifier), Expression::Call(call)) => {
                // Retrieve the entry in the type table for the function call.
                let value_type = match self.type_table.get(&call.id()) {
                    Some(type_) => type_,
                    None => unreachable!("Type checking guarantees that the type of the rhs is in the type table."),
                };

                match &value_type {
                    // If the function returns a tuple, reconstruct the assignment and add an entry to `self.tuples`.
                    Type::Tuple(tuple) => {
                        // Create a new tuple expression with unique identifiers for each index of the lhs.
                        let tuple_expression = TupleExpression {
                            elements: (0..tuple.length())
                                .zip_eq(tuple.elements().iter())
                                .map(|(i, type_)| {
                                    // Return the identifier as an expression.
                                    Expression::Identifier(Identifier::new(
                                        self.assigner.unique_symbol(lhs_identifier.name, format!("$index${i}$")),
                                        {
                                            // Construct a node ID for the identifier.
                                            let id = self.node_builder.next_id();
                                            // Update the type table with the type.
                                            self.type_table.insert(id, type_.clone());
                                            id
                                        },
                                    ))
                                })
                                .collect(),
                            span: Default::default(),
                            id: {
                                // Construct a node ID for the tuple expression.
                                let id = self.node_builder.next_id();
                                // Update the type table with the type.
                                self.type_table.insert(id, Type::Tuple(tuple.clone()));
                                id
                            },
                        };
                        // Add the `tuple_expression` to `self.tuples`.
                        self.tuples.insert(lhs_identifier.name, tuple_expression.clone());

                        // Update the type table with the type of the tuple expression.
                        self.type_table.insert(tuple_expression.id, Type::Tuple(tuple.clone()));

                        // Construct a new assignment statement with a tuple expression on the lhs.
                        (
                            Statement::Assign(Box::new(AssignStatement {
                                place: Expression::Tuple(tuple_expression),
                                value: Expression::Call(call),
                                span: Default::default(),
                                id: self.node_builder.next_id(),
                            })),
                            Default::default(),
                        )
                    }
                    // Otherwise, reconstruct the assignment as is.
                    _ => (self.simple_assign_statement(lhs_identifier, Expression::Call(call)), Default::default()),
                }
            }
            (Expression::Identifier(identifier), expression) => {
                (self.simple_assign_statement(identifier, expression), Default::default())
            }
            // If the lhs is a tuple and the rhs is a function call, then return the reconstructed statement.
            (Expression::Tuple(tuple), Expression::Call(call)) => (
                Statement::Assign(Box::new(AssignStatement {
                    place: Expression::Tuple(tuple),
                    value: Expression::Call(call),
                    span: Default::default(),
                    id: self.node_builder.next_id(),
                })),
                Default::default(),
            ),
            // If the lhs is a tuple and the rhs is a tuple, create a new assign statement for each tuple element.
            (Expression::Tuple(lhs_tuple), Expression::Tuple(rhs_tuple)) => {
                let statements = lhs_tuple
                    .elements
                    .into_iter()
                    .zip_eq(rhs_tuple.elements)
                    .map(|(lhs, rhs)| {
                        // Get the type of the rhs.
                        let type_ = match self.type_table.get(&lhs.id()) {
                            Some(type_) => type_.clone(),
                            None => {
                                unreachable!("Type checking guarantees that the type of the lhs is in the type table.")
                            }
                        };
                        // Set the type of the lhs.
                        self.type_table.insert(rhs.id(), type_);
                        // Return the assign statement.
                        Statement::Assign(Box::new(AssignStatement {
                            place: lhs,
                            value: rhs,
                            span: Default::default(),
                            id: self.node_builder.next_id(),
                        }))
                    })
                    .collect();
                (Statement::dummy(Default::default(), self.node_builder.next_id()), statements)
            }
            // If the lhs is a tuple and the rhs is an identifier that is a tuple, create a new assign statement for each tuple element.
            (Expression::Tuple(lhs_tuple), Expression::Identifier(identifier))
                if self.tuples.contains_key(&identifier.name) =>
            {
                // Lookup the entry in `self.tuples`.
                // Note that the `unwrap` is safe since the match arm checks that the entry exists.
                let rhs_tuple = self.tuples.get(&identifier.name).unwrap().clone();
                // Create a new assign statement for each tuple element.
                let statements = lhs_tuple
                    .elements
                    .into_iter()
                    .zip_eq(rhs_tuple.elements)
                    .map(|(lhs, rhs)| {
                        // Get the type of the rhs.
                        let type_ = match self.type_table.get(&lhs.id()) {
                            Some(type_) => type_.clone(),
                            None => {
                                unreachable!("Type checking guarantees that the type of the lhs is in the type table.")
                            }
                        };
                        // Set the type of the lhs.
                        self.type_table.insert(rhs.id(), type_);
                        // Return the assign statement.
                        Statement::Assign(Box::new(AssignStatement {
                            place: lhs,
                            value: rhs,
                            span: Default::default(),
                            id: self.node_builder.next_id(),
                        }))
                    })
                    .collect();
                (Statement::dummy(Default::default(), self.node_builder.next_id()), statements)
            }
            // If the lhs of an assignment is a tuple, then the rhs can be one of the following:
            //  - A function call that produces a tuple. (handled above)
            //  - A tuple. (handled above)
            //  - An identifier that is a tuple. (handled above)
            //  - A ternary expression that produces a tuple. (handled when the rhs is flattened above)
            (Expression::Tuple(_), _) => {
                unreachable!("`Type checking guarantees that the rhs of an assignment to a tuple is a tuple.`")
            }
            _ => unreachable!("`AssignStatement`s can only have `Identifier`s or `Tuple`s on the left hand side."),
        }
    }

    fn reconstruct_block(&mut self, block: Block) -> (Block, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(block.statements.len());

        // Reconstruct the statements in the block, accumulating any additional statements.
        for statement in block.statements {
            let (reconstructed_statement, additional_statements) = self.reconstruct_statement(statement);
            statements.extend(additional_statements);
            statements.push(reconstructed_statement);
        }

        (Block { span: block.span, statements, id: self.node_builder.next_id() }, Default::default())
    }

    fn reconstruct_conditional(&mut self, _: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`ConditionalStatement`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_console(&mut self, _: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`ConsoleStatement`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_definition(&mut self, _: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`DefinitionStatement`s should not exist in the AST at this phase of compilation.")
    }

    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    /// Reconstructs
    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        // Note that SSA guarantees that `input.expression` is either a literal, identifier, or unit expression.
        let expression = match input.expression {
            // If the input is an identifier that maps to a tuple, use the tuple expression.
            Expression::Identifier(identifier) if self.tuples.contains_key(&identifier.name) => {
                // Note that the `unwrap` is safe since the match arm checks that the entry exists in `self.tuples`.
                let tuple = self.tuples.get(&identifier.name).unwrap().clone();
                Expression::Tuple(tuple)
            }
            // Otherwise, use the original expression.
            _ => input.expression,
        };

        // TODO: Do finalize args need to be destructured.
        (
            Statement::Return(ReturnStatement {
                expression,
                finalize_arguments: input.finalize_arguments,
                span: input.span,
                id: input.id,
            }),
            Default::default(),
        )
    }
}
