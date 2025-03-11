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

use crate::Destructurer;

use leo_ast::{
    AssignStatement,
    Block,
    ConditionalStatement,
    ConsoleStatement,
    DefinitionPlace,
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
    fn reconstruct_assign(&mut self, _assign: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`AssignStatement`s should not exist in the AST at this phase of compilation.")
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

    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        // Conditional statements can only exist in finalize blocks.
        if !self.is_async {
            unreachable!("`ConditionalStatement`s should not be in the AST at this phase of compilation.")
        } else {
            (
                Statement::Conditional(ConditionalStatement {
                    condition: self.reconstruct_expression(input.condition).0,
                    then: self.reconstruct_block(input.then).0,
                    otherwise: input.otherwise.map(|n| Box::new(self.reconstruct_statement(*n).0)),
                    span: input.span,
                    id: input.id,
                }),
                Default::default(),
            )
        }
    }

    fn reconstruct_console(&mut self, _: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`ConsoleStatement`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_definition(&mut self, definition: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        use DefinitionPlace::*;

        let (value, mut statements) = self.reconstruct_expression(definition.value);
        let ty = self.type_table.get(&value.id()).expect("Expressions should have a type.");
        match (definition.place, value, ty) {
            (Single(identifier), Expression::Tuple(tuple), Type::Tuple(..)) => {
                // Just put the identifier in `self.tuples`. We don't need to keep our definition.
                self.tuples.insert(identifier.name, tuple);
                (Statement::dummy(), statements)
            }
            (Single(identifier), Expression::Identifier(rhs), Type::Tuple(..)) => {
                // Again, just put the identifier in `self.tuples` and we don't need to keep our definition.
                let tuple_expr = self.tuples.get(&rhs.name).expect("We should have encountered this tuple by now");
                self.tuples.insert(identifier.name, tuple_expr.clone());
                (Statement::dummy(), statements)
            }
            (Single(identifier), Expression::Call(rhs), Type::Tuple(tuple_type)) => {
                // Make new identifiers for each member of the tuple.
                let identifiers: Vec<Identifier> = (0..tuple_type.elements().len())
                    .map(|i| {
                        Identifier::new(
                            self.assigner.unique_symbol(identifier.name, format!("$index${i}$")),
                            self.node_builder.next_id(),
                        )
                    })
                    .collect();

                // Make expressions corresponding to those identifiers.
                let expressions: Vec<Expression> = identifiers
                    .iter()
                    .zip_eq(tuple_type.elements().iter())
                    .map(|(identifier, type_)| {
                        let expr = Expression::Identifier(*identifier);
                        self.type_table.insert(expr.id(), type_.clone());
                        expr
                    })
                    .collect();

                // Make a tuple expression.
                let expr = TupleExpression {
                    elements: expressions,
                    span: Default::default(),
                    id: self.node_builder.next_id(),
                };

                // Put it in the type table.
                self.type_table.insert(expr.id(), Type::Tuple(tuple_type));

                // Put it into `self.tuples`.
                self.tuples.insert(identifier.name, expr);

                // Define the new variables. We don't need to keep the old definition.
                let stmt = Statement::Definition(DefinitionStatement {
                    place: Multiple(identifiers),
                    type_: Type::Err,
                    value: Expression::Call(rhs),
                    span: Default::default(),
                    id: self.node_builder.next_id(),
                });

                (stmt, statements)
            }
            (Multiple(identifiers), Expression::Tuple(tuple), Type::Tuple(..)) => {
                // Just make a definition for each tuple element.
                for (identifier, expr) in identifiers.into_iter().zip_eq(tuple.elements) {
                    let stmt = Statement::Definition(DefinitionStatement {
                        place: Single(identifier),
                        type_: Type::Err,
                        value: expr,
                        span: Default::default(),
                        id: self.node_builder.next_id(),
                    });
                    statements.push(stmt);
                }

                // We don't need to keep the original definition.
                (Statement::dummy(), statements)
            }
            (Multiple(identifiers), Expression::Identifier(identifier), Type::Tuple(..)) => {
                // Again, make a definition for each tuple element.
                let tuple = self.tuples.get(&identifier.name).expect("We should have encountered this tuple by now");
                for (identifier, expr) in identifiers.into_iter().zip_eq(tuple.elements.iter()) {
                    let stmt = Statement::Definition(DefinitionStatement {
                        place: Single(identifier),
                        type_: Type::Err,
                        value: expr.clone(),
                        span: Default::default(),
                        id: self.node_builder.next_id(),
                    });
                    statements.push(stmt);
                }

                // We don't need to keep the original definition.
                (Statement::dummy(), statements)
            }
            (m @ Multiple(..), value @ Expression::Call(..), Type::Tuple(..)) => {
                // Just reconstruct the statement.
                let stmt = Statement::Definition(DefinitionStatement {
                    place: m,
                    type_: Type::Err,
                    value,
                    span: definition.span,
                    id: definition.id,
                });
                (stmt, statements)
            }
            (_, Expression::Ternary(..), Type::Tuple(..)) => {
                panic!("Ternary conditionals of tuple type should have been removed by flattening");
            }
            (_, _, Type::Tuple(..)) => {
                panic!("Expressions of tuple type can only be tuple literals, identifiers, or calls.");
            }
            (Single(identifier), rhs, _) => {
                // This isn't a tuple. Just build the definition again.
                (
                    Statement::Definition(DefinitionStatement {
                        place: Single(identifier),
                        type_: Type::Err,
                        value: rhs,
                        span: Default::default(),
                        id: definition.id,
                    }),
                    statements,
                )
            }
            (Multiple(_), _, _) => panic!("A definition with multiple identifiers must have tuple type"),
        }
    }

    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

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

        (Statement::Return(ReturnStatement { expression, span: input.span, id: input.id }), Default::default())
    }
}
