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
    AssignOperation, AssignStatement, BinaryExpression, BinaryOperation, Block, ConditionalStatement,
    DefinitionStatement, Expression, ExpressionReconstructor, Identifier, Node, ReturnStatement, Statement,
    StatementReconstructor, TernaryExpression, UnaryExpression, UnaryOperation,
};
use leo_span::{Span, Symbol};

use indexmap::IndexSet;

impl StatementReconstructor for StaticSingleAssigner<'_> {
    /// Transforms a `ReturnStatement` into an `AssignStatement`, storing the variable and the associated guard in `self.early_returns`.
    /// Note that this pass assumes that there is at most one `ReturnStatement` in a block.
    fn reconstruct_return(&mut self, input: ReturnStatement) -> Statement {
        // Create a fresh name for the expression in the return statement.
        let symbol = Symbol::intern(&format!("$return${}", self.unique_id()));
        self.rename_table.update(symbol, symbol);

        // Initialize a new `AssignStatement` for the return expression.
        let place = Expression::Identifier(Identifier::new(symbol));

        // Add the variable and associated guard.
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
        self.early_returns.push((guard, place.clone()));

        Statement::Assign(Box::new(AssignStatement {
            operation: AssignOperation::Assign,
            place,
            value: self.reconstruct_expression(input.expression).0,
            span: Span::default(),
        }))
    }

    /// Reconstructs the `DefinitionStatement` into an `AssignStatement`, renaming the left-hand-side as appropriate.
    fn reconstruct_definition(&mut self, definition: DefinitionStatement) -> Statement {
        self.is_lhs = true;
        let identifier = match self.reconstruct_identifier(definition.variable_name).0 {
            Expression::Identifier(identifier) => identifier,
            _ => unreachable!("`self.reconstruct_identifier` will always return an `Identifier`."),
        };
        self.is_lhs = false;

        Statement::Assign(Box::new(AssignStatement {
            operation: AssignOperation::Assign,
            place: Expression::Identifier(identifier),
            value: self.reconstruct_expression(definition.value).0,
            span: Default::default(),
        }))
    }

    /// Transform all `AssignStatement`s to simple `AssignStatement`s.
    /// For example,
    ///   `x += y * 3` becomes `x = x + (y * 3)`
    ///   `x &= y | 1` becomes `x = x & (y | 1)`
    ///   `x = y + 3` remains `x = y + 3`
    fn reconstruct_assign(&mut self, assign: AssignStatement) -> Statement {
        self.is_lhs = true;
        let place = self.reconstruct_expression(assign.place).0;
        self.is_lhs = false;

        // Helper function to construct a binary expression using `assignee` and `value` as operands.
        let mut reconstruct_to_binary_operation =
            |binary_operation: BinaryOperation, value: Expression| -> Expression {
                let expression_span = value.span();
                Expression::Binary(BinaryExpression {
                    left: Box::new(place.clone()),
                    right: Box::new(self.reconstruct_expression(value).0),
                    op: binary_operation,
                    span: expression_span,
                })
            };

        let value = match assign.operation {
            AssignOperation::Assign => self.reconstruct_expression(assign.value).0,
            AssignOperation::Add => reconstruct_to_binary_operation(BinaryOperation::Add, assign.value),
            AssignOperation::Sub => reconstruct_to_binary_operation(BinaryOperation::Sub, assign.value),
            AssignOperation::Mul => reconstruct_to_binary_operation(BinaryOperation::Mul, assign.value),
            AssignOperation::Div => reconstruct_to_binary_operation(BinaryOperation::Div, assign.value),
            AssignOperation::Pow => reconstruct_to_binary_operation(BinaryOperation::Pow, assign.value),
            AssignOperation::Or => reconstruct_to_binary_operation(BinaryOperation::Or, assign.value),
            AssignOperation::And => reconstruct_to_binary_operation(BinaryOperation::And, assign.value),
            AssignOperation::BitOr => reconstruct_to_binary_operation(BinaryOperation::BitwiseOr, assign.value),
            AssignOperation::BitAnd => reconstruct_to_binary_operation(BinaryOperation::BitwiseAnd, assign.value),
            AssignOperation::BitXor => reconstruct_to_binary_operation(BinaryOperation::Xor, assign.value),
            AssignOperation::Shr => reconstruct_to_binary_operation(BinaryOperation::Shr, assign.value),
            AssignOperation::Shl => reconstruct_to_binary_operation(BinaryOperation::Shl, assign.value),
        };

        Statement::Assign(Box::new(AssignStatement {
            operation: AssignOperation::Assign,
            place,
            value,
            span: Default::default(),
        }))
    }

    /// Reconstructs a `ConditionalStatement`, producing phi functions for variables written in the if and else-blocks.
    fn reconstruct_conditional(&mut self, conditional: ConditionalStatement) -> Statement {
        let condition = self.reconstruct_expression(conditional.condition).0;

        // Instantiate a `RenameTable` for the if-block.
        self.push();

        // Add condition to the condition stack.
        self.condition_stack.push(condition.clone());

        // Reconstruct the if-block.
        let block = self.reconstruct_block(conditional.block);

        // Remove condition from the condition stack.
        self.condition_stack.pop();

        // Remove the `RenameTable` for the if-block.
        let if_table = self.pop();

        // Instantiate a `RenameTable` for the else-block.
        self.push();

        // Reconstruct the else-block.
        let next = conditional.next.map(|statement| {
            // Add the negated condition to the condition stack.
            self.condition_stack.push(Expression::Unary(UnaryExpression {
                op: UnaryOperation::Not,
                receiver: Box::new(condition.clone()),
                span: Default::default(),
            }));

            let reconstructed_block = Box::new(match *statement {
                // The `ConditionalStatement` must be reconstructed as a `Block` statement to ensure that appropriate statements are produced.
                Statement::Conditional(stmt) => self.reconstruct_statement(Statement::Block(Block {
                    statements: vec![Statement::Conditional(stmt)],
                    span: Default::default(),
                })),
                Statement::Block(stmt) => self.reconstruct_statement(Statement::Block(stmt)),
                _ => unreachable!(
                    "`ConditionalStatement`s next statement must be a `ConditionalStatement` or a `Block`."
                ),
            });

            // Remove the negated condition from the condition stack.
            self.condition_stack.pop();

            reconstructed_block
        });

        // Remove the `RenameTable` for the else-block.
        let else_table = self.pop();

        // Compute the write set for the variables written in the if-block or else-block.
        let if_write_set: IndexSet<&Symbol> = IndexSet::from_iter(if_table.local_names().into_iter());
        let else_write_set: IndexSet<&Symbol> = IndexSet::from_iter(else_table.local_names().into_iter());
        let write_set = if_write_set.union(&else_write_set);

        // For each variable in the write set, instantiate a phi function.
        for symbol in write_set {
            // Note that phi functions only need to be instantiated if the variable exists before the `ConditionalStatement`.
            if self.rename_table.lookup(**symbol).is_some() {
                let if_name = if_table
                    .lookup(**symbol)
                    .unwrap_or_else(|| panic!("Symbol {} should exist in the program.", symbol));
                let else_name = else_table
                    .lookup(**symbol)
                    .unwrap_or_else(|| panic!("Symbol {} should exist in the program.", symbol));

                let ternary = Expression::Ternary(TernaryExpression {
                    condition: Box::new(condition.clone()),
                    if_true: Box::new(Expression::Identifier(Identifier {
                        name: *if_name,
                        span: Default::default(),
                    })),
                    if_false: Box::new(Expression::Identifier(Identifier {
                        name: *else_name,
                        span: Default::default(),
                    })),
                    span: Default::default(),
                });

                // Create a new name for the variable written to in the `ConditionalStatement`.
                let new_name = Symbol::intern(&format!("{}${}", symbol, self.unique_id()));
                self.rename_table.update(*(*symbol), new_name);

                // Create a new `AssignStatement` for the phi function.
                let assignment = Statement::Assign(Box::from(AssignStatement {
                    operation: AssignOperation::Assign,
                    place: Expression::Identifier(Identifier {
                        name: new_name,
                        span: Default::default(),
                    }),
                    value: ternary,
                    span: Default::default(),
                }));

                // Store the generate phi functions.
                self.phi_functions.push(assignment);
            }
        }

        // Note that we only produce
        Statement::Conditional(ConditionalStatement {
            condition,
            block,
            next,
            span: conditional.span,
        })
    }

    /// Reconstructs a `Block`, flattening its constituent `ConditionalStatement`s.
    fn reconstruct_block(&mut self, block: Block) -> Block {
        let mut statements = Vec::with_capacity(block.statements.len());

        // Reconstruct each statement in the block.
        for statement in block.statements.into_iter() {
            match statement {
                Statement::Conditional(conditional_statement) => {
                    statements.extend(self.flatten_conditional_statement(conditional_statement))
                }
                _ => statements.push(self.reconstruct_statement(statement)),
            }
        }

        Block {
            statements,
            span: block.span,
        }
    }
}
