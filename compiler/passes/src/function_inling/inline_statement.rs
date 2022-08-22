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

use crate::Inliner;

use leo_ast::{AssignStatement, Block, ConditionalStatement, ConsoleStatement, DefinitionStatement, Expression, ExpressionReconstructor, IterationStatement, ReturnStatement, Statement, StatementReconstructor};

impl StatementReconstructor for Inliner<'_> {
    fn reconstruct_definition(&mut self, _input: DefinitionStatement) -> Statement {
        unreachable!("Definition statements cannot exist in the SSA form.")
    }

    fn reconstruct_conditional(&mut self, _input: ConditionalStatement) -> Statement {
        unreachable!("Conditional statements cannot exist in the SSA form.")
    }

    fn reconstruct_iteration(&mut self, _input: IterationStatement) -> Statement {
        unreachable!("Iteration statements cannot exist in the SSA form.")
    }

    fn reconstruct_block(&mut self, input: Block) -> Block {
        let mut statements = Vec::with_capacity(input.statements.len());

        // SSA form guarantees that complex expressions have been flattened,
        // ensuring that all complex expression only have identifiers and literals as arguments.
        // For example, an expression of the form `foo(a + b, bar(c))` will be
        // flattened to `let $expr$0 = a + b; $expr$1 = bar(c); foo($expr$0, $expr$1)`.
        // Therefore, we only need to check if the rhs of an assign statement is a call expression to determine if we need to inline it.
        for statement in statements {
            match statement {
                Statement::Assign(assign_statement) if matches!(assign_statement.value, Expression::Call(_)) => {
                    // Reconstruct the call expression, getting the new expression and additional statements.
                    let (value, additional_statements) = self.reconstruct_expression(assign_statement.value);
                    // Add the additional statements to the list of statements.
                    statements.extend(additional_statements);
                    // Add the inlined assign statement to the list of statements.
                    statements.push(Statement::Assign(Box::new(AssignStatement {
                        place: assign_statement.place,
                        value,
                        span: assign_statement.span
                    })))
                }
                _ => statements.push(self.reconstruct_statement(statement)),
            }

        }

        Block {
            statements,
            span: input.span,
        }
    }
}
