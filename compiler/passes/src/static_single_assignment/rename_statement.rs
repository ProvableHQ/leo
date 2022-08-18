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

use crate::{RenameTable, StaticSingleAssigner};

use leo_ast::{
    AssignStatement, BinaryExpression, BinaryOperation, Block, ConditionalStatement, ConsoleFunction, ConsoleStatement,
    DefinitionStatement, Expression, ExpressionConsumer, Identifier, IterationStatement, Node, ReturnStatement,
    Statement, StatementConsumer, TernaryExpression, UnaryExpression, UnaryOperation,
};
use leo_span::Symbol;

use indexmap::IndexSet;

impl StatementConsumer for StaticSingleAssigner<'_> {
    type Output = Vec<Statement>;

    /// Transforms a `ReturnStatement` into an empty `BlockStatement`,
    /// storing the expression and the associated guard in `self.early_returns`.
    /// Note that type checking guarantees that there is at most one `ReturnStatement` in a block.
    fn consume_return(&mut self, input: ReturnStatement) -> Self::Output {
        // Construct the associated guard.
        let guard = match self.condition_stack.is_empty() {
            true => None,
            false => {
                let (first, rest) = self.condition_stack.split_first().unwrap();
                Some(rest.iter().cloned().fold(first.clone(), |acc, condition| {
                    Expression::Binary(BinaryExpression {
                        op: BinaryOperation::And,
                        left: Box::new(acc),
                        right: Box::new(condition),
                        span: Default::default(),
                    })
                }))
            }
        };

        // Consume the expression and add it to `early_returns`.
        let (expression, statements) = self.consume_expression(input.expression);
        // Note that this is the only place where `self.early_returns` is mutated.
        // Furthermore, `expression` will always be an identifier or tuple expression.
        self.early_returns.push((guard, expression));

        statements
    }

    /// Consumes the `DefinitionStatement` into an `AssignStatement`, renaming the left-hand-side as appropriate.
    fn consume_definition(&mut self, definition: DefinitionStatement) -> Self::Output {
        // First consume the right-hand-side of the definition.
        let (value, mut statements) = self.consume_expression(definition.value);

        // Then assign a new unique name to the left-hand-side of the definition.
        // Note that this order is necessary to ensure that the right-hand-side uses the correct name when consuming a complex assignment.
        self.is_lhs = true;
        let identifier = match self.consume_identifier(definition.variable_name).0 {
            Expression::Identifier(identifier) => identifier,
            _ => unreachable!("`self.consume_identifier` will always return an `Identifier`."),
        };
        self.is_lhs = false;

        statements.push(Self::simple_assign_statement(Expression::Identifier(identifier), value));

        statements
    }

    /// Consume all `AssignStatement`s, renaming as necessary.
    fn consume_assign(&mut self, assign: AssignStatement) -> Self::Output {
        // First consume the right-hand-side of the assignment.
        let (value, mut statements) = self.consume_expression(assign.value);

        // Then assign a new unique name to the left-hand-side of the assignment.
        // Note that this order is necessary to ensure that the right-hand-side uses the correct name when consuming a complex assignment.
        // TODO: Can lhs have complex expressions?
        self.is_lhs = true;
        let place = self.consume_expression(assign.place).0;
        self.is_lhs = false;

        statements.push(Self::simple_assign_statement(place, value));

        statements
    }

    /// Consumes a `ConditionalStatement`, producing phi functions for variables written in the then-block and otherwise-block.
    /// Furthermore a new `AssignStatement` is introduced for non-trivial expressions in the condition of `ConditionalStatement`s.
    /// For example,
    ///   - `if x > 0 { x = x + 1 }` becomes `let $cond$0 = x > 0; if $cond$0 { x = x + 1; }`
    ///   - `if true { x = x + 1 }` remains the same.
    ///   - `if b { x = x + 1 }` remains the same.
    fn consume_conditional(&mut self, conditional: ConditionalStatement) -> Self::Output {
        // Simplify the condition and add it into the rename table.
        let (condition, mut statements) = self.consume_expression(conditional.condition);

        // Instantiate a `RenameTable` for the then-block.
        self.push();

        // Add condition to the condition stack.
        self.condition_stack.push(condition.clone());

        // Consume the then-block and flatten its constituent statements into the current block.
        statements.extend(self.consume_block(conditional.then));

        // Remove condition from the condition stack.
        self.condition_stack.pop();

        // Remove the `RenameTable` for the then-block.
        let if_table = self.pop();

        // Instantiate a `RenameTable` for the otherwise-block.
        self.push();

        // Consume the otherwise-block and flatten its constituent statements into the current block.
        if let Some(statement) = conditional.otherwise {
            // Add the negated condition to the condition stack.
            self.condition_stack.push(Expression::Unary(UnaryExpression {
                op: UnaryOperation::Not,
                receiver: Box::new(condition.clone()),
                span: condition.span(),
            }));

            statements.extend(self.consume_statement(*statement));

            // Remove the negated condition from the condition stack.
            self.condition_stack.pop();
        };

        // Remove the `RenameTable` for the otherwise-block.
        let else_table = self.pop();

        // Compute the write set for the variables written in the then-block or otherwise-block.
        let if_write_set: IndexSet<&Symbol> = IndexSet::from_iter(if_table.local_names());
        let else_write_set: IndexSet<&Symbol> = IndexSet::from_iter(else_table.local_names());
        let write_set = if_write_set.union(&else_write_set);

        // For each variable in the write set, instantiate a phi function.
        for symbol in write_set {
            // Note that phi functions only need to be instantiated if the variable exists before the `ConditionalStatement`.
            if self.rename_table.lookup(**symbol).is_some() {
                // Helper to lookup a symbol and create an argument for the phi function.
                let create_phi_argument = |table: &RenameTable, symbol: Symbol| {
                    let name = *table
                        .lookup(symbol)
                        .unwrap_or_else(|| panic!("Symbol {} should exist in the program.", symbol));
                    Box::new(Expression::Identifier(Identifier {
                        name,
                        span: Default::default(),
                    }))
                };

                // Create a new name for the variable written to in the `ConditionalStatement`.
                let new_name = self.unique_symbol(symbol);

                // Create a new `AssignStatement` for the phi function.
                let assignment = Self::simple_assign_statement(
                    Expression::Identifier(Identifier {
                        name: new_name,
                        span: Default::default(),
                    }),
                    Expression::Ternary(TernaryExpression {
                        condition: Box::new(condition.clone()),
                        if_true: create_phi_argument(&if_table, **symbol),
                        if_false: create_phi_argument(&else_table, **symbol),
                        span: Default::default(),
                    }),
                );

                // Update the `RenameTable` with the new name of the variable.
                self.rename_table.update(*(*symbol), new_name);

                // Store the generated phi function.
                statements.push(assignment);
            }
        }

        statements
    }

    // TODO: Error message
    fn consume_iteration(&mut self, _input: IterationStatement) -> Self::Output {
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    // TODO: Where do we handle console statements.
    fn consume_console(&mut self, input: ConsoleStatement) -> Self::Output {
        let (function, mut statements) = match input.function {
            ConsoleFunction::Assert(expr) => {
                let (expr, statements) = self.consume_expression(expr);
                (ConsoleFunction::Assert(expr), statements)
            }
            ConsoleFunction::AssertEq(left, right) => {
                // Reconstruct the lhs of the binary expression.
                let (left, mut statements) = self.consume_expression(left);
                // Reconstruct the rhs of the binary expression.
                let (right, mut right_statements) = self.consume_expression(right);
                // Accumulate any statements produced.
                statements.append(&mut right_statements);

                (ConsoleFunction::AssertEq(left, right), statements)
            }
            ConsoleFunction::AssertNeq(left, right) => {
                // Reconstruct the lhs of the binary expression.
                let (left, mut statements) = self.consume_expression(left);
                // Reconstruct the rhs of the binary expression.
                let (right, mut right_statements) = self.consume_expression(right);
                // Accumulate any statements produced.
                statements.append(&mut right_statements);

                (ConsoleFunction::AssertNeq(left, right), statements)
            }
        };

        // Add the console statement to the list of produced statements.
        statements.push(Statement::Console(ConsoleStatement {
            function,
            span: input.span,
        }));

        statements
    }

    /// Consumes a `Block`, flattening its constituent `ConditionalStatement`s.
    fn consume_block(&mut self, block: Block) -> Self::Output {
        block
            .statements
            .into_iter()
            .flat_map(|statement| self.consume_statement(statement))
            .collect()
    }
}
