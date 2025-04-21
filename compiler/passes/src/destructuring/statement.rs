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
    AssignStatement,
    Block,
    ConditionalStatement,
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
    /// Modify assignments to tuples to become assignments to the corresponding variables.
    ///
    /// There are two cases we handle:
    /// 1. An assignment to a tuple x, like `x = rhs;`.
    ///    This we need to transform into individual assignments
    ///    `x_i = rhs_i;`
    ///    of the variables corresponding to members of `x` and `rhs`.
    /// 2. An assignment to a tuple member, like `x.2[i].member = rhs;`.
    ///    This we need to change into
    ///    `x_2[i].member = rhs;`
    ///    where `x_2` is the variable corresponding to `x.2`.
    fn reconstruct_assign(&mut self, mut assign: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let (value, mut statements) = self.reconstruct_expression(assign.value);

        if let Expression::Identifier(identifier) = assign.place {
            if let Type::Tuple(..) = self.state.type_table.get(&value.id()).expect("Expressions should have types.") {
                // This is the first case, assigning to a variable of tuple type.
                let identifiers = self.tuples.get(&identifier.name).expect("Tuple should have been encountered.");

                let Expression::Identifier(rhs) = value else {
                    panic!("SSA should have ensured this is an identifier.");
                };

                let rhs_identifiers = self.tuples.get(&rhs.name).expect("Tuple should have been encountered.");

                // Again, make an assignment for each identifier.
                for (&identifier, &rhs_identifier) in identifiers.iter().zip_eq(rhs_identifiers) {
                    let stmt = AssignStatement {
                        place: identifier.into(),
                        value: rhs_identifier.into(),
                        id: self.state.node_builder.next_id(),
                        span: Default::default(),
                    }
                    .into();

                    statements.push(stmt);
                }

                // We don't need the original assignment, just the ones we've created.
                return (Statement::dummy(), statements);
            }
        }

        // We need to check for case 2, so we loop and see if we find a tuple access.

        assign.value = value;
        let mut place = &mut assign.place;

        loop {
            // Loop through the places in the assignment to the top-level expression until an identifier or tuple access is reached.
            match place {
                Expression::TupleAccess(access) => {
                    // We're assigning to a tuple member, case 2 mentioned above.
                    let Expression::Identifier(identifier) = &access.tuple else {
                        panic!("SSA should have ensured this is an identifier.");
                    };

                    let tuple_ids = self.tuples.get(&identifier.name).expect("Tuple should have been encountered.");

                    // This is the corresponding variable name of the member we're assigning to.
                    let identifier = tuple_ids[access.index.value()];

                    *place = identifier.into();

                    return (assign.into(), statements);
                }

                Expression::ArrayAccess(access) => {
                    // We need to investigate the array, as maybe it's inside a tuple access, like `tupl.0[1u8]`.
                    place = &mut access.array;
                }

                Expression::MemberAccess(access) => {
                    // We need to investigate the struct, as maybe it's inside a tuple access, like `tupl.0.mem`.
                    place = &mut access.inner;
                }

                Expression::Identifier(..) => {
                    // There was no tuple access, so this is neither case 1 nor 2; there's nothing to do.
                    return (assign.into(), statements);
                }

                _ => panic!("Type checking should have prevented this."),
            }
        }
    }

    fn reconstruct_block(&mut self, block: Block) -> (Block, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(block.statements.len());

        // Reconstruct the statements in the block, accumulating any additional statements.
        for statement in block.statements {
            let (reconstructed_statement, additional_statements) = self.reconstruct_statement(statement);
            statements.extend(additional_statements);
            if !reconstructed_statement.is_empty() {
                statements.push(reconstructed_statement);
            }
        }

        (Block { statements, ..block }, Default::default())
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
        (ConditionalStatement { condition, then, otherwise, ..input }.into(), statements)
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
                    let stmt = DefinitionStatement {
                        place: Single(*identifier),
                        type_: ty.clone(),
                        value: Expression::Identifier(*rhs_identifier),
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into();
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
                    let stmt = DefinitionStatement {
                        place: Single(*identifier),
                        type_: ty.clone(),
                        value: expr,
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into();
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
                    let stmt = DefinitionStatement {
                        place: Single(identifier),
                        type_: Type::Err,
                        value: expr,
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into();
                    statements.push(stmt);
                }

                // We don't need to keep the original definition.
                (Statement::dummy(), statements)
            }
            (Multiple(identifiers), Expression::Identifier(rhs), Type::Tuple(..)) => {
                // Again, make a definition for each tuple element.
                let rhs_identifiers = self.tuples.get(&rhs.name).expect("We should have encountered this tuple by now");
                for (identifier, rhs_identifier) in identifiers.into_iter().zip_eq(rhs_identifiers.iter()) {
                    let stmt = DefinitionStatement {
                        place: Single(identifier),
                        type_: Type::Err,
                        value: Expression::Identifier(*rhs_identifier),
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into();
                    statements.push(stmt);
                }

                // We don't need to keep the original definition.
                (Statement::dummy(), statements)
            }
            (m @ Multiple(..), value @ Expression::Call(..), Type::Tuple(..)) => {
                // Just reconstruct the statement.
                let stmt =
                    DefinitionStatement { place: m, type_: Type::Err, value, span: definition.span, id: definition.id }
                        .into();
                (stmt, statements)
            }
            (_, _, Type::Tuple(..)) => {
                panic!("Expressions of tuple type can only be tuple literals, identifiers, or calls.");
            }
            (s @ Single(..), rhs, _) => {
                // This isn't a tuple. Just build the definition again.
                let stmt = DefinitionStatement {
                    place: s,
                    type_: Type::Err,
                    value: rhs,
                    span: Default::default(),
                    id: definition.id,
                }
                .into();
                (stmt, statements)
            }
            (Multiple(_), _, _) => panic!("A definition with multiple identifiers must have tuple type"),
        }
    }

    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        let (expression, statements) = self.reconstruct_expression_tuple(input.expression);
        (ReturnStatement { expression, ..input }.into(), statements)
    }
}
