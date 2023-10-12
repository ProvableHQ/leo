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
    ArrayAccess,
    AssociatedFunction,
    Expression,
    ExpressionReconstructor,
    Member,
    MemberAccess,
    Node,
    Statement,
    StructExpression,
    StructVariableInitializer,
    TernaryExpression,
    Type,
};

// TODO: Clean up logic. To be done in a follow-up PR (feat/tuples)

impl ExpressionReconstructor for Flattener<'_> {
    type AdditionalOutput = Vec<Statement>;

    /// Replaces a tuple access expression with the appropriate expression.
    fn reconstruct_access(&mut self, input: AccessExpression) -> (Expression, Self::AdditionalOutput) {
        let mut statements = Vec::new();
        (
            match input {
                AccessExpression::Array(array) => Expression::Access(AccessExpression::Array(ArrayAccess {
                    array: Box::new(self.reconstruct_expression(*array.array).0),
                    index: Box::new(self.reconstruct_expression(*array.index).0),
                    span: array.span,
                    id: array.id,
                })),
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
                        id: function.id,
                    }))
                }
                AccessExpression::Member(member) => Expression::Access(AccessExpression::Member(MemberAccess {
                    inner: Box::new(self.reconstruct_expression(*member.inner).0),
                    name: member.name,
                    span: member.span,
                    id: member.id,
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
                            self.tuples.get(&identifier.name).unwrap().elements[tuple.index.value()].clone()
                        }
                        _ => unreachable!("SSA guarantees that subexpressions are identifiers or literals."),
                    }
                }
                AccessExpression::AssociatedConstant(access) => {
                    Expression::Access(AccessExpression::AssociatedConstant(access))
                }
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
                id: member.id,
            });
        }

        (Expression::Struct(StructExpression { name: input.name, members, span: input.span, id: input.id }), statements)
    }

    /// Reconstructs ternary expressions over arrays, structs, and tuples, accumulating any statements that are generated.
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
            // If both expressions are identifiers which are arrays, construct ternary expressions for each of the members and an array expression for the result.
            (Expression::Identifier(first), Expression::Identifier(second)) => {
                match (self.type_table.get(&first.id()), self.type_table.get(&second.id())) {
                    (Some(Type::Array(first_type)), Some(Type::Array(second_type))) => {
                        // Note that type checking guarantees that both expressions have the same same type. This is a sanity check.
                        assert_eq!(first_type, second_type);
                        self.ternary_array(&first_type, &input.condition, &first, &second)
                    }
                    (Some(Type::Identifier(first_type)), Some(Type::Identifier(second_type))) => {
                        // Get the struct definitions.
                        let first_type = self.symbol_table.lookup_struct(first_type.name).unwrap();
                        let second_type = self.symbol_table.lookup_struct(second_type.name).unwrap();
                        // Note that type checking guarantees that both expressions have the same same type. This is a sanity check.
                        assert_eq!(first_type, second_type);
                        self.ternary_struct(first_type, &input.condition, &first, &second)
                    }
                    (Some(Type::Tuple(first_type)), Some(Type::Tuple(second_type))) => {
                        // Note that type checking guarantees that both expressions have the same same type. This is a sanity check.
                        assert_eq!(first_type, second_type);
                        self.ternary_tuple(&first_type, &input.condition, &first, &second)
                    }
                    _ => {
                        // Reconstruct the true case.
                        let (if_true, stmts) = self.reconstruct_expression(Expression::Identifier(first));
                        statements.extend(stmts);

                        // Reconstruct the false case.
                        let (if_false, stmts) = self.reconstruct_expression(Expression::Identifier(second));
                        statements.extend(stmts);

                        let (identifier, statement) =
                            self.unique_simple_assign_statement(Expression::Ternary(TernaryExpression {
                                condition: input.condition,
                                if_true: Box::new(if_true),
                                if_false: Box::new(if_false),
                                span: input.span,
                                id: input.id,
                            }));

                        // Accumulate the new assignment statement.
                        statements.push(statement);

                        (Expression::Identifier(identifier), statements)
                    }
                }
            }
            (expr1, expr2) => {
                println!("expr1: {:?}", expr1);
                println!("expr2: {:?}", expr2);
                unreachable!("SSA guarantees that the subexpressions of a ternary expression are identifiers.")
            }
        }
    }
}
