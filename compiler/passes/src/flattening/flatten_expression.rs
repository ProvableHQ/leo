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

use crate::Flattener;
use itertools::Itertools;

use leo_ast::{
    AccessExpression,
    AssociatedFunction,
    Expression,
    ExpressionReconstructor,
    Member,
    MemberAccess,
    NodeID,
    Statement,
    StructExpression,
    StructVariableInitializer,
    TernaryExpression,
    TupleExpression,
};

// TODO: Clean up logic. To be done in a follow-up PR (feat/tuples)

impl ExpressionReconstructor for Flattener<'_> {
    type AdditionalOutput = Vec<Statement>;

    /// Replaces a tuple access expression with the appropriate expression.
    fn reconstruct_access(&mut self, input: AccessExpression) -> (Expression, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        (
            match input {
                AccessExpression::AssociatedFunction(function) => {
                    Expression::Access(AccessExpression::AssociatedFunction(AssociatedFunction {
                        ty: function.ty,
                        name: function.name,
                        arguments: function
                            .arguments
                            .into_iter()
                            .map(|arg| self.reconstruct_expression(arg).0)
                            .collect(),
                        span: function.span,
                        id: NodeID::default(),
                    }))
                }
                AccessExpression::Member(member) => Expression::Access(AccessExpression::Member(MemberAccess {
                    inner: Box::new(self.reconstruct_expression(*member.inner).0),
                    name: member.name,
                    span: member.span,
                    id: NodeID::default(),
                })),
                AccessExpression::Tuple(tuple) => {
                    // Reconstruct the tuple expression.
                    let (expr, stmts) = self.reconstruct_expression(*tuple.tuple);

                    // Accumulate any statements produced.
                    statements.extend(stmts);

                    // Lookup the expression in the tuple map.
                    match expr {
                        Expression::Identifier(identifier) => {
                            // Note that this unwrap is safe since TYC guarantees that all tuples are declared and indices are valid.
                            self.tuples.get(&identifier.name).unwrap().elements[tuple.index.to_usize()].clone()
                        }
                        _ => unreachable!("SSA guarantees that subexpressions are identifiers or literals."),
                    }
                }
                expr => Expression::Access(expr),
            },
            statements,
        )
    }

    /// Reconstructs a struct init expression, flattening any tuples in the expression.
    fn reconstruct_struct_init(&mut self, input: StructExpression) -> (Expression, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        let mut members = Vec::with_capacity(input.members.len());

        // Reconstruct and flatten the argument expressions.
        for member in input.members.into_iter() {
            // Note that this unwrap is safe since SSA guarantees that all struct variable initializers are of the form `<name>: <expr>`.
            let (expr, stmts) = self.reconstruct_expression(member.expression.unwrap());
            // Accumulate any statements produced.
            statements.extend(stmts);
            // Accumulate the struct members.
            members.push(StructVariableInitializer {
                identifier: member.identifier,
                expression: Some(expr),
                span: member.span,
                id: NodeID::default(),
            });
        }

        (
            Expression::Struct(StructExpression { name: input.name, members, span: input.span, id: NodeID::default() }),
            statements,
        )
    }

    /// Reconstructs ternary expressions over tuples and structs, accumulating any statements that are generated.
    /// This is necessary because Aleo instructions does not support ternary expressions over composite data types.
    /// For example, the ternary expression `cond ? (a, b) : (c, d)` is flattened into the following:
    /// ```leo
    /// let var$0 = cond ? a : c;
    /// let var$1 = cond ? b : d;
    /// (var$0, var$1)
    /// ```
    /// For structs, the ternary expression `cond ? a : b`, where `a` and `b` are both structs `Foo { bar: u8, baz: u8 }`, is flattened into the following:
    /// ```leo
    /// let var$0 = cond ? a.bar : b.bar;
    /// let var$1 = cond ? a.baz : b.baz;
    /// let var$2 = Foo { bar: var$0, baz: var$1 };
    /// var$2
    /// ```
    fn reconstruct_ternary(&mut self, input: TernaryExpression) -> (Expression, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        match (*input.if_true, *input.if_false) {
            // Folds ternary expressions over tuples into a tuple of ternary expression.
            // Note that this branch is only invoked when folding a conditional returns.
            (Expression::Tuple(first), Expression::Tuple(second)) => {
                let tuple = Expression::Tuple(TupleExpression {
                    elements: first
                        .elements
                        .into_iter()
                        .zip_eq(second.elements.into_iter())
                        .map(|(if_true, if_false)| {
                            // Reconstruct the true case.
                            let (if_true, stmts) = self.reconstruct_expression(if_true);
                            statements.extend(stmts);

                            // Reconstruct the false case.
                            let (if_false, stmts) = self.reconstruct_expression(if_false);
                            statements.extend(stmts);

                            // Construct a new ternary expression for the tuple element.
                            let (ternary, stmts) = self.reconstruct_ternary(TernaryExpression {
                                condition: input.condition.clone(),
                                if_true: Box::new(if_true),
                                if_false: Box::new(if_false),
                                span: input.span,
                                id: NodeID::default(),
                            });

                            // Accumulate any statements generated.
                            statements.extend(stmts);

                            // Create and accumulate an intermediate assignment statement for the ternary expression corresponding to the tuple element.
                            let (identifier, statement) = self.unique_simple_assign_statement(ternary);
                            statements.push(statement);

                            // Return the identifier associated with the folded tuple element.
                            Expression::Identifier(identifier)
                        })
                        .collect(),
                    span: Default::default(),
                    id: NodeID::default(),
                });
                (tuple, statements)
            }
            // If both expressions are access expressions which themselves are structs, construct ternary expression for nested struct member.
            (
                Expression::Access(AccessExpression::Member(first)),
                Expression::Access(AccessExpression::Member(second)),
            ) => {
                // Lookup the struct symbols associated with the expressions.
                // TODO: Remove clones
                let first_struct_symbol =
                    self.lookup_struct_symbol(&Expression::Access(AccessExpression::Member(first.clone())));
                let second_struct_symbol =
                    self.lookup_struct_symbol(&Expression::Access(AccessExpression::Member(second.clone())));

                match (first_struct_symbol, second_struct_symbol) {
                    (Some(first_struct_symbol), Some(second_struct_symbol)) => {
                        let first_member_struct = self.symbol_table.lookup_struct(first_struct_symbol).unwrap();
                        let second_member_struct = self.symbol_table.lookup_struct(second_struct_symbol).unwrap();
                        // Note that type checking guarantees that both expressions have the same same type. This is a sanity check.
                        assert_eq!(first_member_struct, second_member_struct);

                        // For each struct member, construct a new ternary expression.
                        let members = first_member_struct
                            .members
                            .iter()
                            .map(|Member { identifier, .. }| {
                                // Construct a new ternary expression for the struct member.
                                let (expression, stmts) = self.reconstruct_ternary(TernaryExpression {
                                    condition: input.condition.clone(),
                                    if_true: Box::new(Expression::Access(AccessExpression::Member(MemberAccess {
                                        inner: Box::new(Expression::Access(AccessExpression::Member(first.clone()))),
                                        name: *identifier,
                                        span: Default::default(),
                                        id: NodeID::default(),
                                    }))),
                                    if_false: Box::new(Expression::Access(AccessExpression::Member(MemberAccess {
                                        inner: Box::new(Expression::Access(AccessExpression::Member(second.clone()))),
                                        name: *identifier,
                                        span: Default::default(),
                                        id: NodeID::default(),
                                    }))),
                                    span: Default::default(),
                                    id: NodeID::default(),
                                });

                                // Accumulate any statements generated.
                                statements.extend(stmts);

                                // Create and accumulate an intermediate assignment statement for the ternary expression corresponding to the struct member.
                                let (result, statement) = self.unique_simple_assign_statement(expression);
                                statements.push(statement);

                                StructVariableInitializer {
                                    identifier: *identifier,
                                    expression: Some(Expression::Identifier(result)),
                                    span: Default::default(),
                                    id: NodeID::default(),
                                }
                            })
                            .collect();

                        let (expr, stmts) = self.reconstruct_struct_init(StructExpression {
                            name: first_member_struct.identifier,
                            members,
                            span: Default::default(),
                            id: NodeID::default(),
                        });

                        // Accumulate any statements generated.
                        statements.extend(stmts);

                        // Create a new assignment statement for the struct expression.
                        let (identifier, statement) = self.unique_simple_assign_statement(expr);

                        // Mark the lhs of the assignment as a struct.
                        self.structs.insert(identifier.name, first_member_struct.identifier.name);

                        statements.push(statement);

                        (Expression::Identifier(identifier), statements)
                    }
                    _ => {
                        let if_true = Expression::Access(AccessExpression::Member(first));
                        let if_false = Expression::Access(AccessExpression::Member(second));
                        // Reconstruct the true case.
                        let (if_true, stmts) = self.reconstruct_expression(if_true);
                        statements.extend(stmts);

                        // Reconstruct the false case.
                        let (if_false, stmts) = self.reconstruct_expression(if_false);
                        statements.extend(stmts);

                        let (identifier, statement) =
                            self.unique_simple_assign_statement(Expression::Ternary(TernaryExpression {
                                condition: input.condition,
                                if_true: Box::new(if_true),
                                if_false: Box::new(if_false),
                                span: input.span,
                                id: NodeID::default(),
                            }));

                        // Accumulate the new assignment statement.
                        statements.push(statement);

                        (Expression::Identifier(identifier), statements)
                    }
                }
            }
            // If both expressions are identifiers which are structs, construct ternary expression for each of the members and a struct expression for the result.
            (Expression::Identifier(first), Expression::Identifier(second))
                if self.structs.contains_key(&first.name) && self.structs.contains_key(&second.name) =>
            {
                let first_struct = self.symbol_table.lookup_struct(*self.structs.get(&first.name).unwrap()).unwrap();
                let second_struct = self.symbol_table.lookup_struct(*self.structs.get(&second.name).unwrap()).unwrap();
                // Note that type checking guarantees that both expressions have the same same type. This is a sanity check.
                assert_eq!(first_struct, second_struct);

                // For each struct member, construct a new ternary expression.
                let members = first_struct
                    .members
                    .iter()
                    .map(|Member { identifier, .. }| {
                        // Construct a new ternary expression for the struct member.
                        let (expression, stmts) = self.reconstruct_ternary(TernaryExpression {
                            condition: input.condition.clone(),
                            if_true: Box::new(Expression::Access(AccessExpression::Member(MemberAccess {
                                inner: Box::new(Expression::Identifier(first)),
                                name: *identifier,
                                span: Default::default(),
                                id: NodeID::default(),
                            }))),
                            if_false: Box::new(Expression::Access(AccessExpression::Member(MemberAccess {
                                inner: Box::new(Expression::Identifier(second)),
                                name: *identifier,
                                span: Default::default(),
                                id: NodeID::default(),
                            }))),
                            span: Default::default(),
                            id: NodeID::default(),
                        });

                        // Accumulate any statements generated.
                        statements.extend(stmts);

                        // Create and accumulate an intermediate assignment statement for the ternary expression corresponding to the struct member.
                        let (result, statement) = self.unique_simple_assign_statement(expression);
                        statements.push(statement);

                        StructVariableInitializer {
                            identifier: *identifier,
                            expression: Some(Expression::Identifier(result)),
                            span: Default::default(),
                            id: NodeID::default(),
                        }
                    })
                    .collect();

                let (expr, stmts) = self.reconstruct_struct_init(StructExpression {
                    name: first_struct.identifier,
                    members,
                    span: Default::default(),
                    id: NodeID::default(),
                });

                // Accumulate any statements generated.
                statements.extend(stmts);

                // Create a new assignment statement for the struct expression.
                let (identifier, statement) = self.unique_simple_assign_statement(expr);

                // Mark the lhs of the assignment as a struct.
                self.structs.insert(identifier.name, first_struct.identifier.name);

                statements.push(statement);

                (Expression::Identifier(identifier), statements)
            }
            // If both expressions are identifiers which map to tuples, construct ternary expression over the tuples.
            (Expression::Identifier(first), Expression::Identifier(second))
                if self.tuples.contains_key(&first.name) && self.tuples.contains_key(&second.name) =>
            {
                // Note that this unwrap is safe since we check that `self.tuples` contains the key.
                let first_tuple = self.tuples.get(&first.name).unwrap();
                // Note that this unwrap is safe since we check that `self.tuples` contains the key.
                let second_tuple = self.tuples.get(&second.name).unwrap();
                // Note that type checking guarantees that both expressions have the same same type.
                self.reconstruct_ternary(TernaryExpression {
                    condition: input.condition,
                    if_true: Box::new(Expression::Tuple(first_tuple.clone())),
                    if_false: Box::new(Expression::Tuple(second_tuple.clone())),
                    span: input.span,
                    id: NodeID::default(),
                })
            }
            // Otherwise, create a new intermediate assignment for the ternary expression are return the assigned variable.
            // Note that a new assignment must be created to flattened nested ternary expressions.
            (if_true, if_false) => {
                // Reconstruct the true case.
                let (if_true, stmts) = self.reconstruct_expression(if_true);
                statements.extend(stmts);

                // Reconstruct the false case.
                let (if_false, stmts) = self.reconstruct_expression(if_false);
                statements.extend(stmts);

                let (identifier, statement) =
                    self.unique_simple_assign_statement(Expression::Ternary(TernaryExpression {
                        condition: input.condition,
                        if_true: Box::new(if_true),
                        if_false: Box::new(if_false),
                        span: input.span,
                        id: NodeID::default(),
                    }));

                // Accumulate the new assignment statement.
                statements.push(statement);

                (Expression::Identifier(identifier), statements)
            }
        }
    }
}
