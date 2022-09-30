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

use crate::Flattener;

use leo_ast::{
    AssignStatement, BinaryExpression, BinaryOperation, Block, ConditionalStatement, DefinitionStatement, Expression,
    ExpressionReconstructor, FinalizeStatement, IterationStatement, Node, ReturnStatement, Statement,
    StatementReconstructor, UnaryExpression, UnaryOperation,
};

impl StatementReconstructor for Flattener<'_> {
    /// Flattens an assign statement, if necessary.
    /// Marks variables as structs as necessary.
    /// Note that new statements are only produced if the right hand side is a ternary expression over structs.
    /// Otherwise, the statement is returned as is.
    fn reconstruct_assign(&mut self, assign: AssignStatement) -> (Statement, Self::AdditionalOutput) {
        let lhs = match assign.place {
            Expression::Identifier(identifier) => identifier,
            _ => unreachable!("`AssignStatement`s can only have `Identifier`s on the left hand side."),
        };

        let (value, statements) = match assign.value {
            // If the rhs of the assignment is ternary expression, reconstruct it.
            Expression::Ternary(ternary) => self.reconstruct_ternary(ternary),
            // Otherwise return the original statement.
            value => (value, Default::default()),
        };

        // Update the `self.structs` if the rhs is a struct.
        self.update_structs(&lhs, &value);

        (
            Statement::Assign(Box::new(AssignStatement {
                place: Expression::Identifier(lhs),
                value,
                span: assign.span,
            })),
            statements,
        )
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

        (
            Block {
                span: block.span,
                statements,
            },
            Default::default(),
        )
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
            }));

            // Reconstruct the otherwise-block and accumulate it constituent statements.
            match *statement {
                Statement::Block(block) => statements.extend(self.reconstruct_block(block).0.statements),
                _ => unreachable!("SSA guarantees that the `otherwise` is always a `Block`"),
            }

            // Remove the negated condition from the condition stack.
            self.condition_stack.pop();
        };

        (Statement::dummy(Default::default()), statements)
    }

    /// Static single assignment converts definition statements into assignment statements.
    fn reconstruct_definition(&mut self, _definition: DefinitionStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`DefinitionStatement`s should not exist in the AST at this phase of compilation.")
    }

    /// Replaces a finalize statement with an empty block statement.
    /// Stores the arguments to the finalize statement, which are later folded into a single finalize statement at the end of the function.
    fn reconstruct_finalize(&mut self, input: FinalizeStatement) -> (Statement, Self::AdditionalOutput) {
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

        // For each finalize argument, add it and its associated guard to the appropriate list of finalize arguments.
        // Note that type checking guarantees that the number of arguments in a finalize statement is equal to the number of arguments in to the finalize block.
        for (i, argument) in input.arguments.into_iter().enumerate() {
            // Note that this unwrap is safe since we initialize `self.finalizes` with a number of vectors equal to the number of finalize arguments.
            self.finalizes.get_mut(i).unwrap().push((guard.clone(), argument));
        }

        (Statement::dummy(Default::default()), Default::default())
    }

    // TODO: Error message requesting the user to enable loop-unrolling.
    fn reconstruct_iteration(&mut self, _input: IterationStatement) -> (Statement, Self::AdditionalOutput) {
        unreachable!("`IterationStatement`s should not be in the AST at this phase of compilation.");
    }

    /// Transforms a return statement into an empty block statement.
    /// Stores the arguments to the return statement, which are later folded into a single return statement at the end of the function.
    fn reconstruct_return(&mut self, input: ReturnStatement) -> (Statement, Self::AdditionalOutput) {
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

        // Add it to the list of return statements.
        self.returns.push((guard, input.expression));

        (Statement::dummy(Default::default()), Default::default())
    }
}
