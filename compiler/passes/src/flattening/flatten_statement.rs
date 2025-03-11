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

use crate::{Flattener, Guard, ReturnGuard};

use leo_ast::{
    AssertStatement,
    AssertVariant,
    AssignStatement,
    BinaryExpression,
    BinaryOperation,
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

        // If we are traversing an async function, then we can return the assert as it.
        if self.is_async {
            return (Statement::Assert(input), statements);
        }

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

        let mut guards: Vec<Expression> = Vec::new();

        if let Some((guard, guard_statements)) = self.construct_guard() {
            statements.extend(guard_statements);

            // The not_guard is true if we didn't follow the condition chain
            // that led to this assertion.
            let not_guard = Expression::Unary(UnaryExpression {
                op: UnaryOperation::Not,
                receiver: Box::new(Expression::Identifier(guard)),
                span: Default::default(),
                id: {
                    // Create a new node ID for the unary expression.
                    let id = self.node_builder.next_id();
                    // Update the type table with the type of the unary expression.
                    self.type_table.insert(id, Type::Boolean);
                    id
                },
            });
            let (identifier, statement) = self.unique_simple_definition(not_guard);
            statements.push(statement);
            guards.push(Expression::Identifier(identifier));
        }

        // We also need to guard against early returns.
        if let Some((guard, guard_statements)) = self.construct_early_return_guard() {
            guards.push(Expression::Identifier(guard));
            statements.extend(guard_statements);
        }

        if guards.is_empty() {
            return (Statement::Assert(assert), statements);
        }

        let is_eq = matches!(assert.variant, AssertVariant::AssertEq(..));

        // We need to `or` the asserted expression with the guards,
        // so extract an appropriate expression.
        let mut expression = match assert.variant {
            // If the assert statement is an `assert`, use the expression as is.
            AssertVariant::Assert(expression) => expression,

            // For `assert_eq` or `assert_neq`, construct a new expression.
            AssertVariant::AssertEq(left, right) | AssertVariant::AssertNeq(left, right) => {
                let binary = Expression::Binary(BinaryExpression {
                    left: Box::new(left),
                    op: if is_eq { BinaryOperation::Eq } else { BinaryOperation::Neq },
                    right: Box::new(right),
                    span: Default::default(),
                    id: {
                        // Create a new node ID.
                        let id = self.node_builder.next_id();
                        // Update the type table.
                        self.type_table.insert(id, Type::Boolean);
                        id
                    },
                });
                let (identifier, statement) = self.unique_simple_definition(binary);
                statements.push(statement);
                Expression::Identifier(identifier)
            }
        };

        // The assertion will be that the original assert statement is true or one of the guards is true
        // (ie, we either didn't follow the condition chain that led to this assert, or else we took an
        // early return).
        for guard in guards.into_iter() {
            let binary = Expression::Binary(BinaryExpression {
                op: BinaryOperation::Or,
                span: Default::default(),
                id: {
                    // Create a new node ID.
                    let id = self.node_builder.next_id();
                    // Update the type table.
                    self.type_table.insert(id, Type::Boolean);
                    id
                },
                left: Box::new(expression),
                right: Box::new(guard),
            });
            let (identifier, statement) = self.unique_simple_definition(binary);
            statements.push(statement);
            expression = Expression::Identifier(identifier);
        }

        let assert_statement = Statement::Assert(AssertStatement {
            span: input.span,
            id: input.id,
            variant: AssertVariant::Assert(expression),
        });

        (assert_statement, statements)
    }

    fn reconstruct_assign(&mut self, _assign: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`AssignStatement`s should not be in the AST at this phase of compilation");
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

        // If we are traversing an async function, reconstruct the if and else blocks, but do not flatten them.
        if self.is_async {
            let then_block = self.reconstruct_block(conditional.then).0;
            let otherwise_block = match conditional.otherwise {
                Some(statement) => match *statement {
                    Statement::Block(block) => self.reconstruct_block(block).0,
                    _ => panic!("SSA guarantees that the `otherwise` is always a `Block`"),
                },
                None => Block { span: Default::default(), statements: Vec::new(), id: self.node_builder.next_id() },
            };

            return (
                Statement::Conditional(ConditionalStatement {
                    condition: conditional.condition,
                    then: then_block,
                    otherwise: Some(Box::new(Statement::Block(otherwise_block))),
                    span: conditional.span,
                    id: conditional.id,
                }),
                statements,
            );
        }

        // Assign the condition to a variable, as it may be used multiple times.
        let place = Identifier {
            name: self.assigner.unique_symbol("condition", "$"),
            span: Default::default(),
            id: {
                let id = self.node_builder.next_id();
                self.type_table.insert(id, Type::Boolean);
                id
            },
        };

        statements.push(self.simple_definition(place, conditional.condition.clone()));

        // Add condition to the condition stack.
        self.condition_stack.push(Guard::Unconstructed(place));

        // Reconstruct the then-block and accumulate it constituent statements.
        statements.extend(self.reconstruct_block(conditional.then).0.statements);

        // Remove condition from the condition stack.
        self.condition_stack.pop();

        // Consume the otherwise-block and flatten its constituent statements into the current block.
        if let Some(statement) = conditional.otherwise {
            // Apply Not to the condition, assign it, and put it on the condition stack.
            let not_condition = Expression::Unary(UnaryExpression {
                op: UnaryOperation::Not,
                receiver: Box::new(conditional.condition.clone()),
                span: conditional.condition.span(),
                id: conditional.condition.id(),
            });
            let not_place = Identifier {
                name: self.assigner.unique_symbol("condition", "$"),
                span: Default::default(),
                id: {
                    let id = self.node_builder.next_id();
                    self.type_table.insert(id, Type::Boolean);
                    id
                },
            };
            statements.push(self.simple_definition(not_place, not_condition));
            self.condition_stack.push(Guard::Unconstructed(not_place));

            // Reconstruct the otherwise-block and accumulate it constituent statements.
            match *statement {
                Statement::Block(block) => statements.extend(self.reconstruct_block(block).0.statements),
                _ => panic!("SSA guarantees that the `otherwise` is always a `Block`"),
            }

            // Remove the negated condition from the condition stack.
            self.condition_stack.pop();
        };

        (Statement::dummy(), statements)
    }

    fn reconstruct_console(&mut self, _: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`ConsoleStatement`s should not be in the AST at this phase of compilation.")
    }

    /// Flattens a definition, if necessary.
    /// Marks variables as structs as necessary.
    /// Note that new statements are only produced if the right hand side is a ternary expression over structs.
    /// Otherwise, the statement is returned as is.
    fn reconstruct_definition(&mut self, definition: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        // Flatten the rhs of the assignment.
        let (value, statements) = self.reconstruct_expression(definition.value);
        match (definition.place, &value) {
            (DefinitionPlace::Single(identifier), _) => (self.simple_definition(identifier, value), statements),
            (DefinitionPlace::Multiple(identifiers), expression) => {
                let output_type = match &self.type_table.get(&expression.id()) {
                    Some(Type::Tuple(tuple_type)) => tuple_type.clone(),
                    _ => panic!("Type checking guarantees that the output type is a tuple."),
                };

                for (identifier, type_) in identifiers.iter().zip_eq(output_type.elements().iter()) {
                    // Add the type of each identifier to the type table.
                    self.type_table.insert(identifier.id, type_.clone());
                }

                (
                    Statement::Definition(DefinitionStatement {
                        place: DefinitionPlace::Multiple(identifiers),
                        type_: Type::Err,
                        value,
                        span: Default::default(),
                        id: self.node_builder.next_id(),
                    }),
                    statements,
                )
            }
        }
    }

    fn reconstruct_iteration(&mut self, _input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    /// Transforms a return statement into an empty block statement.
    /// Stores the arguments to the return statement, which are later folded into a single return statement at the end of the function.
    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        // If we are traversing an async function, return as is.
        if self.is_async {
            return (Statement::Return(input), Default::default());
        }
        // Construct the associated guard.
        let (guard_identifier, statements) = self.construct_guard().unzip();

        let return_guard = guard_identifier.map_or(ReturnGuard::None, ReturnGuard::Unconstructed);

        match input.expression {
            Expression::Unit(_) | Expression::Identifier(_) | Expression::AssociatedConstant(_) => {
                self.returns.push((return_guard, input))
            }
            _ => panic!("SSA guarantees that the expression is always an identifier or unit expression."),
        };

        (Statement::dummy(), statements.unwrap_or_default())
    }
}
