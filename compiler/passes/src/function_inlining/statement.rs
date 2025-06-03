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

use super::FunctionInliningVisitor;

use leo_ast::{
    AssignStatement,
    Block,
    ConditionalStatement,
    DefinitionPlace,
    DefinitionStatement,
    Expression,
    ExpressionReconstructor,
    ExpressionStatement,
    IterationStatement,
    Statement,
    StatementReconstructor,
};

impl StatementReconstructor for FunctionInliningVisitor<'_> {
    fn reconstruct_assign(&mut self, _input: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`AssignStatement`s should not exist in the AST at this phase of compilation.")
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
    fn reconstruct_conditional(&mut self, input: ConditionalStatement) -> (Statement, Self::AdditionalOutput) {
        if !self.is_async {
            panic!("`ConditionalStatement`s should not be in the AST at this phase of compilation.")
        } else {
            (
                ConditionalStatement {
                    condition: self.reconstruct_expression(input.condition).0,
                    then: self.reconstruct_block(input.then).0,
                    otherwise: input.otherwise.map(|n| Box::new(self.reconstruct_statement(*n).0)),
                    span: input.span,
                    id: input.id,
                }
                .into(),
                Default::default(),
            )
        }
    }

    /// Reconstruct a definition statement by inlining any function calls.
    /// This function also segments tuple assignment statements into multiple assignment statements.
    fn reconstruct_definition(&mut self, mut input: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        let (value, mut statements) = self.reconstruct_expression(input.value);
        match (input.place, value) {
            // If we just inlined the production of a tuple literal, we need multiple definition statements.
            (DefinitionPlace::Multiple(left), Expression::Tuple(right)) => {
                assert_eq!(left.len(), right.elements.len());
                for (identifier, rhs_value) in left.into_iter().zip(right.elements) {
                    let stmt = DefinitionStatement {
                        place: DefinitionPlace::Single(identifier),
                        type_: None,
                        value: rhs_value,
                        span: Default::default(),
                        id: self.state.node_builder.next_id(),
                    }
                    .into();

                    statements.push(stmt);
                }
                (Statement::dummy(), statements)
            }

            (place, value) => {
                input.value = value;
                input.place = place;
                (input.into(), statements)
            }
        }
    }

    /// Reconstructs expression statements by inlining any function calls.
    fn reconstruct_expression_statement(&mut self, input: ExpressionStatement) -> (Statement, Self::AdditionalOutput) {
        // Reconstruct the expression.
        // Note that type checking guarantees that the expression is a function call.
        let (expression, additional_statements) = self.reconstruct_expression(input.expression);

        // If the resulting expression is a unit expression, return a dummy statement.
        let statement = match expression {
            Expression::Unit(_) => Statement::dummy(),
            _ => ExpressionStatement { expression, ..input }.into(),
        };

        (statement, additional_statements)
    }

    /// Loop unrolling unrolls and removes iteration statements from the program.
    fn reconstruct_iteration(&mut self, _: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        panic!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }
}
