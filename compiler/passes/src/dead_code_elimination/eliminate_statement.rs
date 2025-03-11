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

use crate::DeadCodeEliminator;

use leo_ast::{
    AssertStatement,
    AssertVariant,
    AssignStatement,
    Block,
    ConditionalStatement,
    ConsoleStatement,
    DefinitionPlace,
    DefinitionStatement,
    Expression,
    ExpressionReconstructor,
    ExpressionStatement,
    IterationStatement,
    ReturnStatement,
    Statement,
    StatementReconstructor,
};

impl StatementReconstructor for DeadCodeEliminator {
    fn reconstruct_assert(&mut self, input: AssertStatement) -> (Statement, Self::AdditionalOutput) {
        // Set the `is_necessary` flag.
        self.is_necessary = true;

        // Visit the statement.
        let statement = Statement::Assert(AssertStatement {
            variant: match input.variant {
                AssertVariant::Assert(expr) => AssertVariant::Assert(self.reconstruct_expression(expr).0),
                AssertVariant::AssertEq(left, right) => {
                    AssertVariant::AssertEq(self.reconstruct_expression(left).0, self.reconstruct_expression(right).0)
                }
                AssertVariant::AssertNeq(left, right) => {
                    AssertVariant::AssertNeq(self.reconstruct_expression(left).0, self.reconstruct_expression(right).0)
                }
            },
            span: input.span,
            id: input.id,
        });

        // Unset the `is_necessary` flag.
        self.is_necessary = false;

        (statement, Default::default())
    }

    /// Static single assignment removed all assignments.
    fn reconstruct_assign(&mut self, _input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`AssignStatement`s should not exist in the AST at this phase of compilation.")
    }

    /// Reconstructs the statements inside a basic block, eliminating any dead code.
    fn reconstruct_block(&mut self, block: Block) -> (Block, Self::AdditionalOutput) {
        // Reconstruct each of the statements in reverse.
        let mut statements: Vec<Statement> =
            block.statements.into_iter().rev().map(|statement| self.reconstruct_statement(statement).0).collect();

        // Reverse the direction of `statements`.
        statements.reverse();

        (Block { statements, span: block.span, id: block.id }, Default::default())
    }

    /// Flattening removes conditional statements from the program.
    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        if !self.is_async {
            panic!("`ConditionalStatement`s should not be in the AST at this phase of compilation.")
        } else {
            (
                Statement::Conditional(ConditionalStatement {
                    then: self.reconstruct_block(input.then).0,
                    otherwise: input.otherwise.map(|n| Box::new(self.reconstruct_statement(*n).0)),
                    condition: {
                        // Set the `is_necessary` flag.
                        self.is_necessary = true;
                        let condition = self.reconstruct_expression(input.condition).0;
                        // Unset the `is_necessary` flag.
                        self.is_necessary = false;

                        condition
                    },
                    span: input.span,
                    id: input.id,
                }),
                Default::default(),
            )
        }
    }

    /// Parsing guarantees that console statements are not present in the program.
    fn reconstruct_console(&mut self, _: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`ConsoleStatement`s should not be in the AST at this phase of compilation.")
    }

    /// Static single assignment replaces definition statements with assignment statements.
    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        // Check the lhs of the definition to see any of variables are used.
        let lhs_is_used = match &input.place {
            DefinitionPlace::Single(identifier) => self.used_variables.contains(&identifier.name),
            DefinitionPlace::Multiple(identifiers) => {
                identifiers.iter().any(|identifier| self.used_variables.contains(&identifier.name))
            }
        };

        if lhs_is_used {
            // Set the `is_necessary` flag.
            self.is_necessary = true;

            input.value = self.reconstruct_expression(input.value).0;

            // Unset the `is_necessary` flag.
            self.is_necessary = false;

            (Statement::Definition(input), Default::default())
        } else {
            // Eliminate it.
            (Statement::dummy(), Default::default())
        }
    }

    /// Reconstructs expression statements by eliminating any dead code.
    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Self::AdditionalOutput) {
        match input.expression {
            // If the expression is a function call, then we reconstruct it.
            // Note that we preserve function calls because they may have side effects.
            Expression::Call(expression) => {
                // Set the `is_necessary` flag.
                self.is_necessary = true;

                // Visit the expression.
                let statement = Statement::Expression(ExpressionStatement {
                    expression: self.reconstruct_call(expression).0,
                    span: input.span,
                    id: input.id,
                });

                // Unset the `is_necessary` flag.
                self.is_necessary = false;

                (statement, Default::default())
            }
            Expression::AssociatedFunction(associated_function) => {
                // Visit the expression.
                (
                    Statement::Expression(ExpressionStatement {
                        expression: self.reconstruct_associated_function(associated_function).0,
                        span: input.span,
                        id: input.id,
                    }),
                    Default::default(),
                )
            }
            // Any other expression is dead code, since they do not have side effects.
            // Note: array access expressions will have side effects and need to be handled here.
            _ => (Statement::dummy(), Default::default()),
        }
    }

    /// Loop unrolling unrolls and removes iteration statements from the program.
    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
        // Set the `is_necessary` flag.
        self.is_necessary = true;

        // Visit the statement.
        let statement = Statement::Return(ReturnStatement {
            expression: self.reconstruct_expression(input.expression).0,
            span: input.span,
            id: input.id,
        });

        // Unset the `is_necessary` flag.
        self.is_necessary = false;

        (statement, Default::default())
    }
}
