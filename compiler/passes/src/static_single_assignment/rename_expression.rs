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
    AccessExpression, AssociatedFunction, BinaryExpression, CallExpression, CircuitExpression,
    CircuitVariableInitializer, ErrExpression, Expression, ExpressionConsumer, Identifier, Literal, MemberAccess,
    Statement, TernaryExpression, TupleAccess, TupleExpression, UnaryExpression,
};

impl ExpressionConsumer for StaticSingleAssigner<'_> {
    type Output = (Expression, Vec<Statement>);

    /// Consumes an access expression, accumulating any statements that are generated.
    fn consume_access(&mut self, input: AccessExpression) -> Self::Output {
        let mut additional_output = Vec::new();

        let expr = Expression::Access(match input {
            AccessExpression::AssociatedFunction(function) => {
                AccessExpression::AssociatedFunction(AssociatedFunction {
                    ty: function.ty,
                    name: function.name,
                    args: function
                        .args
                        .into_iter()
                        .map(|arg| {
                            let (place, statement) = self.consume_expression(arg);
                            additional_output.extend(statement);
                            place
                        })
                        .collect(),
                    span: function.span,
                })
            }
            AccessExpression::Member(member) => AccessExpression::Member(MemberAccess {
                inner: {
                    let (place, statement) = self.consume_expression(*member.inner);
                    additional_output.extend(statement);
                    Box::new(place)
                },
                name: member.name,
                span: member.span,
            }),
            AccessExpression::Tuple(tuple) => AccessExpression::Tuple(TupleAccess {
                tuple: {
                    let (place, statement) = self.consume_expression(*tuple.tuple);
                    additional_output.extend(statement);
                    Box::new(place)
                },
                index: tuple.index,
                span: tuple.span,
            }),
            expr => expr,
        });
        let (place, statement) = self.simple_expr_assign_statement(expr);
        additional_output.push(statement);

        (place, additional_output)
    }

    /// Consumes a binary expression, accumulating any statements that are generated.
    fn consume_binary(&mut self, input: BinaryExpression) -> Self::Output {
        let mut additional_output = Vec::new();

        let expr = Expression::Binary(BinaryExpression {
            left: {
                let (expression, statement) = self.consume_expression(*input.left);
                additional_output.extend(statement);
                Box::new(expression)
            },
            right: {
                let (expression, statement) = self.consume_expression(*input.right);
                additional_output.extend(statement);
                Box::new(expression)
            },
            op: input.op,
            span: input.span,
        });
        let (place, statement) = self.simple_expr_assign_statement(expr);
        additional_output.push(statement);

        (place, additional_output)
    }

    /// Consumes a call expression without visiting the function name, accumulating any statements that are generated.
    fn consume_call(&mut self, input: CallExpression) -> Self::Output {
        let mut additional_output = Vec::new();

        // Create a new assignment statement for the call expression.
        let expr = Expression::Call(CallExpression {
            // Note that we do not rename the function name.
            function: input.function,
            // Consume the arguments.
            arguments: input
                .arguments
                .into_iter()
                .map(|argument| {
                    let (argument, output) = self.consume_expression(argument);
                    additional_output.extend(output);
                    argument
                })
                .collect(),
            span: input.span,
        });
        let (place, statement) = self.simple_expr_assign_statement(expr);
        additional_output.push(statement);

        (place, additional_output)
    }

    /// Consumes a circuit initialization expression with renamed variables, accumulating any statements that are generated.
    fn consume_circuit_init(&mut self, input: CircuitExpression) -> Self::Output {
        let mut additional_output = Vec::new();

        // Create a new assignment statement for the circuit init expression.
        let expr = Expression::Circuit(CircuitExpression {
            name: input.name,
            span: input.span,
            // Consume the circuit members.
            members: input
                .members
                .into_iter()
                .map(|arg| {
                    let (expression, output) = match &arg.expression.is_some() {
                        // If the expression is None, then `arg` is a `CircuitVariableInitializer` of the form `<id>,`.
                        // In this case, we must consume the identifier and produce an initializer of the form `<id>: <renamed_id>`.
                        false => self.consume_identifier(arg.identifier),
                        // If expression is `Some(..)`, then `arg is a `CircuitVariableInitializer` of the form `<id>: <expr>,`.
                        // In this case, we must consume the expression.
                        true => self.consume_expression(arg.expression.unwrap()),
                    };
                    // Add the output to the additional output.
                    additional_output.extend(output);

                    // Return the new member.
                    CircuitVariableInitializer {
                        identifier: arg.identifier,
                        expression: Some(expression),
                    }
                })
                .collect(),
        });
        let (place, statement) = self.simple_expr_assign_statement(expr);
        additional_output.push(statement);

        (place, additional_output)
    }

    fn consume_err(&mut self, input: ErrExpression) -> Self::Output {
        (Expression::Err(input), Default::default())
    }

    /// Produces a new `Identifier` with a unique name.
    fn consume_identifier(&mut self, identifier: Identifier) -> Self::Output {
        let name = match self.is_lhs {
            // If consumeing the left-hand side of a definition or assignment, a new unique name is introduced.
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

    fn consume_literal(&mut self, input: Literal) -> Self::Output {
        (Expression::Literal(input), Default::default())
    }

    /// Consumes a ternary expression, accumulating any statements that are generated.
    fn consume_ternary(&mut self, input: TernaryExpression) -> Self::Output {
        let mut additional_output = Vec::new();

        let expr = Expression::Ternary(TernaryExpression {
            condition: Box::new(self.consume_expression(*input.condition).0),
            if_true: Box::new(self.consume_expression(*input.if_true).0),
            if_false: Box::new(self.consume_expression(*input.if_false).0),
            span: input.span,
        });
        let (place, statement) = self.simple_expr_assign_statement(expr);
        additional_output.push(statement);

        (place, additional_output)
    }

    /// Consumes a tuple expression, accumulating any statements that are generated
    fn consume_tuple(&mut self, input: TupleExpression) -> Self::Output {
        let mut additional_output = Vec::new();

        let expr = Expression::Tuple(TupleExpression {
            elements: input
                .elements
                .into_iter()
                .map(|element| {
                    let (element, statements) = self.consume_expression(element);
                    additional_output.extend(statements);
                    element
                })
                .collect(),
            span: input.span,
        });
        let (place, statement) = self.simple_expr_assign_statement(expr);
        additional_output.push(statement);

        (place, additional_output)
    }

    /// Consumes a unary expression, accumulating any statements that are generated.
    fn consume_unary(&mut self, input: UnaryExpression) -> Self::Output {
        let mut additional_output = Vec::new();

        let expr = Expression::Unary(UnaryExpression {
            receiver: {
                let (expression, statement) = self.consume_expression(*input.receiver);
                additional_output.extend(statement);
                Box::new(expression)
            },
            op: input.op,
            span: input.span,
        });
        let (place, statement) = self.simple_expr_assign_statement(expr);
        additional_output.push(statement);

        (place, additional_output)
    }
}
