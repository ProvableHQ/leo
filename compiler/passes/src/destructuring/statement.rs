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

use super::DestructuringVisitor;

use leo_ast::{
    AccessExpression,
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
    Type,
};
use leo_span::Symbol;

use itertools::{Itertools as _, izip};

impl StatementReconstructor for DestructuringVisitor<'_> {
    fn reconstruct_assign(&mut self, mut assign: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let (value, mut statements) = self.reconstruct_expression(assign.value);

        match assign.place {
            Expression::Identifier(identifier) => {
                if let Type::Tuple(tuple_type) =
                    self.state.type_table.get(&value.id()).expect("Expressions should have types.")
                {
                    // It's a variable of tuple type. Aleo VM doesn't know about tuples, so
                    // we'll need to handle this.
                    let new_symbol = self.state.assigner.unique_symbol(identifier, "##");
                    let new_identifier = Identifier::new(new_symbol, self.state.node_builder.next_id());
                    self.state.type_table.insert(new_identifier.id(), Type::Tuple(tuple_type.clone()));

                    let identifiers = self.tuples.get(&identifier.name).expect("Tuple should have been encountered.");

                    let Expression::Identifier(rhs) = value else {
                        panic!("SSA should have ensured this is an identifier.");
                    };

                    let rhs_identifiers = self.tuples.get(&rhs.name).expect("Tuple should have been encountered.");

                    // Again, make an assignment for each identifier.
                    for (identifier, rhs_identifier) in identifiers.iter().zip_eq(rhs_identifiers) {
                        let stmt = Statement::Assign(Box::new(AssignStatement {
                            place: Expression::Identifier(*identifier),
                            value: Expression::Identifier(*rhs_identifier),
                            id: self.state.node_builder.next_id(),
                            span: Default::default(),
                        }));

                        statements.push(stmt);
                    }

                    (Statement::dummy(), statements)
                } else {
                    assign.value = value;
                    (Statement::Assign(Box::new(assign)), statements)
                }
            }

            Expression::Access(AccessExpression::Tuple(access)) => {
                // We're assigning to a tuple member. Again, Aleo VM doesn't know about tuples,
                // so we'll need to handle this.
                let Expression::Identifier(identifier) = &*access.tuple else {
                    panic!("SSA should have ensured this is an identifier.");
                };

                let tuple_ids = self.tuples.get(&identifier.name).expect("Tuple should have been encountered.");

                // This is the correspondig variable name of the member we're assigning to.
                let identifier = tuple_ids[access.index.value()];

                // So just assign to the variable.
                let assign = Statement::Assign(Box::new(AssignStatement {
                    place: Expression::Identifier(identifier),
                    value,
                    span: Default::default(),
                    id: self.state.node_builder.next_id(),
                }));

                (assign, statements)
            }

            _ => {
                assign.value = value;
                (Statement::Assign(Box::new(assign)), statements)
            }
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

        (Block { span: block.span, statements, id: self.state.node_builder.next_id() }, Default::default())
    }

    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        let (condition, mut statements) = self.reconstruct_expression(input.condition);
        let (then, statements2) = self.reconstruct_block(input.then);
        statements.extend(statements2);
        let otherwise = input.otherwise.map(|oth| {
            let (expr, statements3) = self.reconstruct_statement(*oth);
            statements.extend(statements3);
            Box::new(expr)
        });
        (Statement::Conditional(ConditionalStatement { condition, then, otherwise, ..input }), statements)
    }

    fn reconstruct_console(&mut self, _: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`ConsoleStatement`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_definition(&mut self, definition: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        use DefinitionPlace::*;

        let make_identifiers = |slf: &mut Self, single: Symbol, count: usize| -> Vec<Identifier> {
            (0..count)
                .map(|i| {
                    Identifier::new(
                        slf.state.assigner.unique_symbol(format_args!("{single}#tuple{i}"), "$"),
                        slf.state.node_builder.next_id(),
                    )
                })
                .collect()
        };

        let (value, mut statements) = self.reconstruct_expression(definition.value);
        let ty = self.state.type_table.get(&value.id()).expect("Expressions should have a type.");
        match (definition.place, value, ty) {
            (Single(identifier), Expression::Identifier(rhs), Type::Tuple(tuple_type)) => {
                // We need to give the members new names, in case they are assigned to.
                let identifiers = make_identifiers(self, identifier.name, tuple_type.length());

                let rhs_identifiers = self.tuples.get(&rhs.name).unwrap();

                for (identifier, rhs_identifier, ty) in izip!(&identifiers, rhs_identifiers, tuple_type.elements()) {
                    // Make a definition for each.
                    let stmt = Statement::Definition(DefinitionStatement {
                        place: Single(*identifier),
                        type_: ty.clone(),
                        value: Expression::Identifier(*rhs_identifier),
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    });
                    statements.push(stmt);

                    // Put each into the type table.
                    self.state.type_table.insert(identifier.id(), ty.clone());
                }

                // Put the identifier in `self.tuples`. We don't need to keep our definition.
                self.tuples.insert(identifier.name, identifiers);
                (Statement::dummy(), statements)
            }
            (Single(identifier), Expression::Tuple(tuple), Type::Tuple(tuple_type)) => {
                // Name each of the expressions on the right.
                let identifiers = make_identifiers(self, identifier.name, tuple_type.length());

                for (identifier, expr, ty) in izip!(&identifiers, tuple.elements, tuple_type.elements()) {
                    // Make a definition for each.
                    let stmt = Statement::Definition(DefinitionStatement {
                        place: Single(*identifier),
                        type_: ty.clone(),
                        value: expr,
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    });
                    statements.push(stmt);

                    // Put each into the type table.
                    self.state.type_table.insert(identifier.id(), ty.clone());
                }

                // Put the identifier in `self.tuples`. We don't need to keep our definition.
                self.tuples.insert(identifier.name, identifiers);
                (Statement::dummy(), statements)
            }
            (Single(identifier), rhs @ Expression::Call(..), Type::Tuple(tuple_type)) => {
                let definition_stmt = self.assign_tuple(rhs, identifier.name);

                let Statement::Definition(DefinitionStatement {
                    place: DefinitionPlace::Multiple(identifiers), ..
                }) = &definition_stmt
                else {
                    panic!("assign_tuple creates `Multiple`.");
                };

                // Put it into `self.tuples`.
                self.tuples.insert(identifier.name, identifiers.clone());

                // Put each into the type table.
                for (identifier, ty) in identifiers.iter().zip(tuple_type.elements()) {
                    self.state.type_table.insert(identifier.id(), ty.clone());
                }

                (definition_stmt, statements)
            }
            (Multiple(identifiers), Expression::Tuple(tuple), Type::Tuple(..)) => {
                // Just make a definition for each tuple element.
                for (identifier, expr) in identifiers.into_iter().zip_eq(tuple.elements) {
                    let stmt = Statement::Definition(DefinitionStatement {
                        place: Single(identifier),
                        type_: Type::Err,
                        value: expr,
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    });
                    statements.push(stmt);
                }

                // We don't need to keep the original definition.
                (Statement::dummy(), statements)
            }
            (Multiple(identifiers), Expression::Identifier(rhs), Type::Tuple(..)) => {
                // Again, make a definition for each tuple element.
                let rhs_identifiers = self.tuples.get(&rhs.name).expect("We should have encountered this tuple by now");
                for (identifier, rhs_identifier) in identifiers.into_iter().zip_eq(rhs_identifiers.iter()) {
                    let stmt = Statement::Definition(DefinitionStatement {
                        place: Single(identifier),
                        type_: Type::Err,
                        value: Expression::Identifier(*rhs_identifier),
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
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
            (_, _, Type::Tuple(..)) => {
                panic!("Expressions of tuple type can only be tuple literals, identifiers, or calls.");
            }
            (Single(identifier), rhs, _) => {
                // This isn't a tuple. Just build the definition again.
                let stmt = Statement::Definition(DefinitionStatement {
                    place: Single(identifier),
                    type_: Type::Err,
                    value: rhs,
                    span: Default::default(),
                    id: definition.id,
                });
                (stmt, statements)
            }
            (Multiple(_), _, _) => panic!("A definition with multiple identifiers must have tuple type"),
        }
    }

    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    fn reconstruct_return(&mut self, mut input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        let (expression, statements) = self.reconstruct_expression_tuple(input.expression);
        input.expression = expression;
        (Statement::Return(input), statements)
    }
}
