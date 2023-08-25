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

use crate::FunctionInliner;

use leo_ast::{
    AssignStatement,
    Block,
    ConditionalStatement,
    ConsoleStatement,
    DefinitionStatement,
    Expression,
    ExpressionReconstructor,
    ExpressionStatement,
    IterationStatement,
    Statement,
    StatementReconstructor,
};

impl StatementReconstructor for FunctionInliner<'_> {
    /// Reconstruct an assignment statement by inlining any function calls.
    /// This function also segments tuple assignment statements into multiple assignment statements.
    fn reconstruct_assign(&mut self, input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let (value, mut statements) = self.reconstruct_expression(input.value.clone());
        match (input.place, value) {
            // If the function call produces a tuple, we need to segment the tuple into multiple assignment statements.
            (Expression::Tuple(left), Expression::Tuple(right)) if left.elements.len() == right.elements.len() => {
                statements.extend(left.elements.into_iter().zip(right.elements).map(|(lhs, rhs)| {
                    Statement::Assign(Box::new(AssignStatement {
                        place: lhs,
                        value: rhs,
                        span: Default::default(),
                        id: self.node_builder.next_id(),
                    }))
                }));
                (Statement::dummy(Default::default(), self.node_builder.next_id()), statements)
            }

            (place, value) => (
                Statement::Assign(Box::new(AssignStatement { place, value, span: input.span, id: input.id })),
                statements,
            ),
        }
    }

    /// Reconstructs the statements inside a basic block, accumulating any statements produced by function inlining.
    fn reconstruct_block(&mut self, block: Block) -> (Block, Self::AdditionalOutput) {
        let mut statements = Vec::with_capacity(block.statements.len());

        for statement in block.statements {
            let (reconstructed_statement, additional_statements) = self.reconstruct_statement(statement);
            statements.extend(additional_statements);
            statements.push(reconstructed_statement);
        }

        (Block { span: block.span, statements, id: block.id }, Default::default())
    }

    /// Flattening removes conditional statements from the program.
    fn reconstruct_conditional(&mut self, _: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`ConditionalStatement`s should not be in the AST at this phase of compilation.")
    }

    /// Parsing guarantees that console statements are not present in the program.
    fn reconstruct_console(&mut self, _: ConsoleStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`ConsoleStatement`s should not be in the AST at this phase of compilation.")
    }

    /// Static single assignment replaces definition statements with assignment statements.
    fn reconstruct_definition(&mut self, _: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`DefinitionStatement`s should not exist in the AST at this phase of compilation.")
    }

    /// Reconstructs expression statements by inlining any function calls.
    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Self::AdditionalOutput) {
        // Reconstruct the expression.
        // Note that type checking guarantees that the expression is a function call.
        let (expression, additional_statements) = self.reconstruct_expression(input.expression);

        // If the resulting expression is a unit expression, return a dummy statement.
        let statement = match expression {
            Expression::Unit(_) => Statement::dummy(Default::default(), self.node_builder.next_id()),
            _ => Statement::Expression(ExpressionStatement { expression, span: input.span, id: input.id }),
        };

        (statement, additional_statements)
    }

    /// Loop unrolling unrolls and removes iteration statements from the program.
    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }
}
