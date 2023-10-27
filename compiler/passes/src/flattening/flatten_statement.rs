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

use leo_ast::{
    AssertStatement,
    AssertVariant,
    AssignStatement,
    BinaryExpression,
    BinaryOperation,
    Block,
    ConditionalStatement,
    ConsoleStatement,
    DefinitionStatement,
    Expression,
    ExpressionReconstructor,
    IterationStatement,
    Node,
    ReturnStatement,
    Statement,
    StatementReconstructor,
    Type,
    UnaryExpression,
    UnaryOperation,
};

use itertools::Itertools;

impl StatementReconstructor for Flattener<'_> {
    /// Rewrites an assert statement into a flattened form.
    /// Assert statements at the top level only have their arguments flattened.
    /// Assert statements inside a conditional statement are flattened to such that the check is conditional on
    /// the execution path being valid.
    /// For example, the following snippet:
    /// ```leo
    /// if condition1 {
    ///    if condition2 {
    ///        assert(foo);
    ///    }
    /// }
    /// ```
    /// is flattened to:
    /// ```leo
    /// assert(!(condition1 && condition2) || foo);
    /// ```
    /// which is equivalent to the logical formula `(condition1 /\ condition2) ==> foo`.
    fn reconstruct_assert(&mut self, input: AssertStatement) -> (Statement, Self::AdditionalOutput) {
        let mut statements = Vec::new();

        // Flatten the arguments of the assert statement.
        let assert = AssertStatement {
            span: input.span,
            id: input.id,
            variant: match input.variant {
                AssertVariant::Assert(expression) => {
                    let (expression, additional_statements) = self.reconstruct_expression(expression);
                    statements.extend(additional_statements);
                    AssertVariant::Assert(expression)
                }
                AssertVariant::AssertEq(left, right) => {
                    let (left, additional_statements) = self.reconstruct_expression(left);
                    statements.extend(additional_statements);
                    let (right, additional_statements) = self.reconstruct_expression(right);
                    statements.extend(additional_statements);
                    AssertVariant::AssertEq(left, right)
                }
                AssertVariant::AssertNeq(left, right) => {
                    let (left, additional_statements) = self.reconstruct_expression(left);
                    statements.extend(additional_statements);
                    let (right, additional_statements) = self.reconstruct_expression(right);
                    statements.extend(additional_statements);
                    AssertVariant::AssertNeq(left, right)
                }
            },
        };

        // Add the appropriate guards.
        match self.construct_guard() {
            // If the condition stack is empty, we can return the flattened assert statement.
            None => (Statement::Assert(assert), statements),
            // Otherwise, we need to join the guard with the expression in the flattened assert statement.
            // Note given the guard and the expression, we construct the logical formula `guard => expression`,
            // which is equivalent to `!guard || expression`.
            Some(guard) => (
                Statement::Assert(AssertStatement {
                    span: input.span,
                    id: input.id,
                    variant: AssertVariant::Assert(Expression::Binary(BinaryExpression {
                        op: BinaryOperation::Or,
                        span: Default::default(),
                        id: {
                            // Create a new node ID for the binary expression.
                            let id = self.node_builder.next_id();
                            // Update the type table with the type of the binary expression.
                            self.type_table.insert(id, Type::Boolean);
                            id
                        },
                        // Take the logical negation of the guard.
                        left: Box::new(Expression::Unary(UnaryExpression {
                            op: UnaryOperation::Not,
                            receiver: Box::new(guard),
                            span: Default::default(),
                            id: {
                                // Create a new node ID for the unary expression.
                                let id = self.node_builder.next_id();
                                // Update the type table with the type of the unary expression.
                                self.type_table.insert(id, Type::Boolean);
                                id
                            },
                        })),
                        right: Box::new(match assert.variant {
                            // If the assert statement is an `assert`, use the expression as is.
                            AssertVariant::Assert(expression) => expression,
                            // If the assert statement is an `assert_eq`, construct a new equality expression.
                            AssertVariant::AssertEq(left, right) => Expression::Binary(BinaryExpression {
                                left: Box::new(left),
                                op: BinaryOperation::Eq,
                                right: Box::new(right),
                                span: Default::default(),
                                id: {
                                    // Create a new node ID for the unary expression.
                                    let id = self.node_builder.next_id();
                                    // Update the type table with the type of the unary expression.
                                    self.type_table.insert(id, Type::Boolean);
                                    id
                                },
                            }),
                            // If the assert statement is an `assert_ne`, construct a new inequality expression.
                            AssertVariant::AssertNeq(left, right) => Expression::Binary(BinaryExpression {
                                left: Box::new(left),
                                op: BinaryOperation::Neq,
                                right: Box::new(right),
                                span: Default::default(),
                                id: {
                                    // Create a new node ID for the unary expression.
                                    let id = self.node_builder.next_id();
                                    // Update the type table with the type of the unary expression.
                                    self.type_table.insert(id, Type::Boolean);
                                    id
                                },
                            }),
                        }),
                    })),
                }),
                statements,
            ),
        }
    }

    /// Flattens an assign statement, if necessary.
    /// Marks variables as structs as necessary.
    /// Note that new statements are only produced if the right hand side is a ternary expression over structs.
    /// Otherwise, the statement is returned as is.
    fn reconstruct_assign(&mut self, assign: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        // Flatten the rhs of the assignment.
        let (value, statements) = self.reconstruct_expression(assign.value);
        match (assign.place, &value) {
            (Expression::Identifier(identifier), _) => (self.simple_assign_statement(identifier, value), statements),
            (Expression::Tuple(tuple), expression) => {
                let output_type = match &self.type_table.get(&expression.id()) {
                    Some(Type::Tuple(tuple_type)) => tuple_type.clone(),
                    _ => unreachable!("Type checking guarantees that the output type is a tuple."),
                };

                tuple.elements.iter().zip_eq(output_type.elements().iter()).for_each(|(identifier, type_)| {
                    let identifier = match identifier {
                        Expression::Identifier(identifier) => identifier,
                        _ => unreachable!("Type checking guarantees that a tuple element on the lhs is an identifier."),
                    };
                    // Add the type of each identifier to the type table.
                    self.type_table.insert(identifier.id, type_.clone());
                });

                // Set the type of the tuple expression.
                self.type_table.insert(tuple.id, Type::Tuple(output_type.clone()));

                (
                    Statement::Assign(Box::new(AssignStatement {
                        place: Expression::Tuple(tuple),
                        value,
                        span: Default::default(),
                        id: self.node_builder.next_id(),
                    })),
                    statements,
                )
            }
            _ => unreachable!("`AssignStatement`s can only have `Identifier`s or `Tuple`s on the left hand side."),
        }
    }

    // TODO: Do we want to flatten nested blocks? They do not affect code generation but it would regularize the AST structure.
    /// Flattens the statements inside a basic block.
    /// The resulting block does not contain any conditional statements.
    fn reconstruct_block(&mut self, block: Block) -> (Block, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(block.statements.len());

        // Flatten each statement, accumulating any new statements produced.
        for statement in block.statements {
            let (reconstructed_statement, additional_statements) = self.reconstruct_statement(statement);
            statements.extend(additional_statements);
            statements.push(reconstructed_statement);
        }

        (Block { span: block.span, statements, id: self.node_builder.next_id() }, Default::default())
    }

    /// Flatten a conditional statement into a list of statements.
    fn reconstruct_conditional(&mut self, conditional: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(conditional.then.statements.len());

        // Add condition to the condition stack.
        self.condition_stack.push(conditional.condition.clone());

        // Reconstruct the then-block and accumulate it constituent statements.
        statements.extend(self.reconstruct_block(conditional.then).0.statements);

        // Remove condition from the condition stack.
        self.condition_stack.pop();

        // Consume the otherwise-block and flatten its constituent statements into the current block.
        if let Some(statement) = conditional.otherwise {
            // Add the negated condition to the condition stack.
            self.condition_stack.push(Expression::Unary(UnaryExpression {
                op: UnaryOperation::Not,
                receiver: Box::new(conditional.condition.clone()),
                span: conditional.condition.span(),
                id: conditional.condition.id(),
            }));

            // Reconstruct the otherwise-block and accumulate it constituent statements.
            match *statement {
                Statement::Block(block) => statements.extend(self.reconstruct_block(block).0.statements),
                _ => unreachable!("SSA guarantees that the `otherwise` is always a `Block`"),
            }

            // Remove the negated condition from the condition stack.
            self.condition_stack.pop();
        };

        (Statement::dummy(Default::default(), self.node_builder.next_id()), statements)
    }

    fn reconstruct_console(&mut self, _: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`ConsoleStatement`s should not be in the AST at this phase of compilation.")
    }

    fn reconstruct_definition(&mut self, _definition: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`DefinitionStatement`s should not exist in the AST at this phase of compilation.")
    }

    fn reconstruct_iteration(&mut self, _input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    /// Transforms a return statement into an empty block statement.
    /// Stores the arguments to the return statement, which are later folded into a single return statement at the end of the function.
    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        // Construct the associated guard.
        let guard = self.construct_guard();

        match input.expression {
            Expression::Unit(_) | Expression::Identifier(_) => self.returns.push((guard, input)),
            _ => unreachable!("SSA guarantees that the expression is always an identifier or unit expression."),
        };

        (Statement::dummy(Default::default(), self.node_builder.next_id()), Default::default())
    }
}
