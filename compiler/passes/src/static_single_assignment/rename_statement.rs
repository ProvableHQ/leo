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
    DefinitionStatement, Expression, ExpressionReconstructor, Identifier, Node, Statement, StatementReconstructor,
    TernaryExpression,
};
use leo_span::{Span, Symbol};

use indexmap::IndexSet;

impl<'a> StatementReconstructor for StaticSingleAssigner<'a> {
    /// Reconstructs the `DefinitionStatement` into an `AssignStatement`, renaming the left-hand-side as appropriate.
    fn reconstruct_definition(&mut self, definition: DefinitionStatement) -> Statement {
        self.is_lhs = true;
        let identifier = match self.reconstruct_identifier(definition.variable_name.identifier).0 {
            Expression::Identifier(identifier) => identifier,
            _ => unreachable!("`reconstruct_identifier` will always return an `Identifier`."),
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
    // TODO: Verify that these are expected semantics.
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

    fn reconstruct_conditional(&mut self, conditional: ConditionalStatement) -> Statement {
        let condition = self.reconstruct_expression(conditional.condition).0;

        // Instantiate a `RenameTable` for the if-block.
        self.push();
        let block = self.reconstruct_block(conditional.block);
        let if_table = self.pop();

        // Instantiate a `RenameTable` for the else-block.
        self.push();
        let next = conditional
            .next
            .map(|statement| Box::new(match *statement {
                // The `ConditionalStatement` must be reconstructed as a `Block` statement to ensure that appropriate statements are produced.
                Statement::Conditional(stmt) => { self.reconstruct_statement(Statement::Block(Block {
                    statements: vec![Statement::Conditional(stmt)],
                    span: Default::default()
                }))}
                Statement::Block(stmt) => { self.reconstruct_statement(Statement::Block(stmt)) }
                _ => unreachable!("`ConditionalStatement`s next statement must be a `ConditionalStatement` or a `Block`."),
            }));

        let else_table = self.pop();

        // Instantiate phi functions for the nodes written in the `ConditionalStatement`.
        let if_write_set: IndexSet<&Symbol> = IndexSet::from_iter(if_table.get_local_names().into_iter());
        let else_write_set: IndexSet<&Symbol> = IndexSet::from_iter(else_table.get_local_names().into_iter());
        let write_set = if_write_set.union(&else_write_set);

        // TODO: Better error handling.
        for symbol in write_set {
            match self.rename_table.lookup(symbol) {
                Some(..) => {
                    let if_name = if_table
                        .lookup(symbol)
                        .unwrap_or_else(|| panic!("Symbol {} should exist in the program.", symbol));
                    let else_name = else_table
                        .lookup(symbol)
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
                    let new_name = Symbol::intern(&format!("{}${}", symbol, self.get_unique_id()));
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
                    self.phi_functions.push(assignment);
                }
                None => (),
            }
        }

        Statement::Conditional(ConditionalStatement {
            condition,
            block,
            next,
            span: conditional.span,
        })
    }

    /// This function:
    ///   - Converts all `DefinitionStatement`s to `AssignStatement`s.
    ///   - Introduces a new `AssignStatement` for non-trivial expressions in the condition of `ConditionalStatement`s.
    ///     For example,
    ///       - `if x > 0 { x = x + 1 }` becomes `let cond$0 = x > 0; if cond$0 { x = x + 1; }`
    ///       - `if true { x = x + 1 }` remains the same.
    ///       - `if b { x = x + 1 }` remains the same.
    ///   - Flattens the resulting `ConditionalStatement`s.
    fn reconstruct_block(&mut self, block: Block) -> Block {
        let mut statements = Vec::with_capacity(block.statements.len());
        for statement in block.statements.into_iter() {
            match statement {
                Statement::Conditional(conditional_statement) => {
                    let reconstructed_statement = match conditional_statement.condition {
                        // TODO: Do we have a better way of handling unreachable errors?
                        Expression::Call(..) => {
                            unreachable!("Call expressions should not exist in the AST at this stage of compilation.")
                        }
                        Expression::Err(_) => {
                            unreachable!("Err expressions should not exist in the AST at this stage of compilation.")
                        }
                        Expression::Identifier(..) | Expression::Literal(..) => {
                            self.reconstruct_conditional(conditional_statement)
                        }
                        Expression::Binary(..) | Expression::Unary(..) | Expression::Ternary(..) => {
                            // Create a fresh variable name for the condition.
                            let symbol = Symbol::intern(&format!("cond${}", self.get_unique_id()));
                            self.rename_table.update(symbol, symbol);

                            // Initialize a new `AssignStatement` for the condition.
                            let place = Expression::Identifier(Identifier::new(symbol));
                            let assign_statement = Statement::Assign(Box::new(AssignStatement {
                                operation: AssignOperation::Assign,
                                place: place.clone(),
                                value: self.reconstruct_expression(conditional_statement.condition).0,
                                span: Span::default(),
                            }));
                            let rewritten_conditional_statement = ConditionalStatement {
                                condition: place,
                                block: conditional_statement.block,
                                next: conditional_statement.next,
                                span: conditional_statement.span,
                            };
                            statements.push(assign_statement);
                            self.reconstruct_conditional(rewritten_conditional_statement)
                        }
                    };

                    // Flatten the reconstructed `ConditionalStatement` by lifting the statements in the "if" and "else" block
                    // into the current `BlockStatement`.
                    let mut conditional_statement = match reconstructed_statement {
                        Statement::Conditional(conditional_statement) => conditional_statement,
                        _ => unreachable!("`reconstruct_conditional` will always produce a `ConditionalStatement`"),
                    };
                    statements.append(&mut conditional_statement.block.statements);
                    if let Some(statement) = conditional_statement.next {
                        match *statement {
                            // If we encounter a `BlockStatement` we need to lift its constituent statements into the current `BlockStatement`.
                            Statement::Block(mut block) => statements.append(&mut block.statements),
                            _ => statements.push(*statement),
                        }
                    }

                    statements.append(&mut self.clear_phi_functions());
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
