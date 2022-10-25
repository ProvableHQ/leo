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
    AssignStatement, Block, ConditionalStatement, ConsoleFunction, ConsoleStatement, DecrementStatement,
    DefinitionStatement, Expression, ExpressionConsumer, FinalizeStatement, Identifier, IncrementStatement,
    IterationStatement, ReturnStatement, Statement, StatementConsumer, TernaryExpression,
};
use leo_span::Symbol;

use indexmap::IndexSet;

impl StatementConsumer for StaticSingleAssigner<'_> {
    type Output = Vec<Statement>;

    /// Consume all `AssignStatement`s, renaming as necessary.
    fn consume_assign(&mut self, assign: AssignStatement) -> Self::Output {
        // First consume the right-hand-side of the assignment.
        let (value, mut statements) = self.consume_expression(assign.value);

        // Then assign a new unique name to the left-hand-side of the assignment.
        // Note that this order is necessary to ensure that the right-hand-side uses the correct name when consuming a complex assignment.
        self.is_lhs = true;
        let place = match self.consume_expression(assign.place).0 {
            Expression::Identifier(identifier) => identifier,
            _ => panic!("Type checking guarantees that the left-hand-side of an assignment is an identifier."),
        };
        self.is_lhs = false;

        statements.push(self.assigner.simple_assign_statement(place, value));

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

    /// Consumes a `ConditionalStatement`, producing phi functions (assign statements) for variables written in the then-block and otherwise-block.
    /// For more information on phi functions, see https://en.wikipedia.org/wiki/Static_single_assignment_form.
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

        // Consume the then-block.
        let then = Block {
            span: conditional.then.span,
            statements: self.consume_block(conditional.then),
        };

        // Remove the `RenameTable` for the then-block.
        let if_table = self.pop();

        // Instantiate a `RenameTable` for the otherwise-block.
        self.push();

        // Consume the otherwise-block and flatten its constituent statements into the current block.
        let otherwise = conditional.otherwise.map(|otherwise| Box::new(Statement::Block(match *otherwise {
            Statement::Block(block) => Block {
                span: block.span,
                statements: self.consume_block(block),
            },
            Statement::Conditional(conditional) => Block {
                span: conditional.span,
                statements: self.consume_conditional(conditional),
            },
            _ => unreachable!("Type checking guarantees that the otherwise-block of a conditional statement is a block or another conditional statement."),
        })));

        // Remove the `RenameTable` for the otherwise-block.
        let else_table = self.pop();

        // Add reconstructed conditional statement to the list of produced statements.
        statements.push(Statement::Conditional(ConditionalStatement {
            span: conditional.span,
            condition: condition.clone(),
            then,
            otherwise,
        }));

        // Compute the write set for the variables written in the then-block or otherwise-block.
        let if_write_set: IndexSet<&Symbol> = IndexSet::from_iter(if_table.local_names());
        let else_write_set: IndexSet<&Symbol> = IndexSet::from_iter(else_table.local_names());
        let write_set = if_write_set.union(&else_write_set);

        // For each variable in the write set, instantiate and add a phi function to the list of produced statements.
        for symbol in write_set {
            // Note that phi functions only need to be instantiated if the variable exists before the `ConditionalStatement`.
            if self.rename_table.lookup(**symbol).is_some() {
                // Helper to lookup a symbol and create an argument for the phi function.
                let create_phi_argument = |table: &RenameTable, symbol: Symbol| {
                    let name = *table
                        .lookup(symbol)
                        .unwrap_or_else(|| panic!("Symbol {symbol} should exist in the program."));
                    Box::new(Expression::Identifier(Identifier {
                        name,
                        span: Default::default(),
                    }))
                };

                // Create a new name for the variable written to in the `ConditionalStatement`.
                let new_name = self.assigner.unique_symbol(symbol);

                let (value, stmts) = self.consume_ternary(TernaryExpression {
                    condition: Box::new(condition.clone()),
                    if_true: create_phi_argument(&if_table, **symbol),
                    if_false: create_phi_argument(&else_table, **symbol),
                    span: Default::default(),
                });

                statements.extend(stmts);

                // Create a new `AssignStatement` for the phi function.
                let assignment = self.assigner.simple_assign_statement(
                    Identifier {
                        name: new_name,
                        span: Default::default(),
                    },
                    value,
                );

                // Update the `RenameTable` with the new name of the variable.
                self.rename_table.update(*(*symbol), new_name);

                // Store the generated phi function.
                statements.push(assignment);
            }
        }

        statements
    }

    /// Consumes the expressions in a `ConsoleStatement`, returning the list of simplified statements.
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
                let (right, right_statements) = self.consume_expression(right);
                // Accumulate any statements produced.
                statements.extend(right_statements);

                (ConsoleFunction::AssertEq(left, right), statements)
            }
            ConsoleFunction::AssertNeq(left, right) => {
                // Reconstruct the lhs of the binary expression.
                let (left, mut statements) = self.consume_expression(left);
                // Reconstruct the rhs of the binary expression.
                let (right, right_statements) = self.consume_expression(right);
                // Accumulate any statements produced.
                statements.extend(right_statements);

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

    /// Consumes the expressions associated with the `DecrementStatement`, returning the simplified `DecrementStatement`.
    fn consume_decrement(&mut self, input: DecrementStatement) -> Self::Output {
        // First consume the expression associated with the amount.
        let (amount, mut statements) = self.consume_expression(input.amount);

        // Then, consume the expression associated with the index.
        let (index, index_statements) = self.consume_expression(input.index);
        statements.extend(index_statements);

        statements.push(Statement::Decrement(DecrementStatement {
            mapping: input.mapping,
            index,
            amount,
            span: input.span,
        }));

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

        statements.push(self.assigner.simple_assign_statement(identifier, value));

        statements
    }

    /// Consumes the expressions associated with the `FinalizeStatement`, returning the simplified `FinalizeStatement`.
    fn consume_finalize(&mut self, input: FinalizeStatement) -> Self::Output {
        let mut statements = Vec::new();

        // Process the arguments, accumulating any statements produced.
        let arguments = input
            .arguments
            .into_iter()
            .map(|argument| {
                let (argument, stmts) = self.consume_expression(argument);
                statements.extend(stmts);
                argument
            })
            .collect();

        // Construct and accumulate a simplified finalize statement.
        statements.push(Statement::Finalize(FinalizeStatement {
            arguments,
            span: input.span,
        }));

        statements
    }

    /// Consumes the expressions associated with the `IncrementStatement`, returning a simplified `IncrementStatement`.
    fn consume_increment(&mut self, input: IncrementStatement) -> Self::Output {
        // First consume the expression associated with the amount.
        let (amount, mut statements) = self.consume_expression(input.amount);

        // Then, consume the expression associated with the index.
        let (index, index_statements) = self.consume_expression(input.index);
        statements.extend(index_statements);

        statements.push(Statement::Increment(IncrementStatement {
            mapping: input.mapping,
            index,
            amount,
            span: input.span,
        }));

        statements
    }

    // TODO: Error message
    fn consume_iteration(&mut self, _input: IterationStatement) -> Self::Output {
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    /// Reconstructs the expression associated with the return statement, returning a simplified `ReturnStatement`.
    /// Note that type checking guarantees that there is at most one `ReturnStatement` in a block.
    fn consume_return(&mut self, input: ReturnStatement) -> Self::Output {
        // Consume the return expression.
        let (expression, mut statements) = self.consume_expression(input.expression);

        // Add the simplified return statement to the list of produced statements.
        statements.push(Statement::Return(ReturnStatement {
            expression,
            span: input.span,
        }));

        statements
    }
}
