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

use crate::RenameTable;
use std::fmt::Display;

use leo_ast::{
    AssignOperation, AssignStatement, ConditionalStatement, Expression, ExpressionReconstructor, Identifier, Statement,
    StatementReconstructor,
};
use leo_errors::emitter::Handler;
use leo_span::Symbol;

pub struct StaticSingleAssigner<'a> {
    /// The `RenameTable` for the current basic block in the AST
    pub(crate) rename_table: RenameTable,
    /// An error handler used for any errors found during unrolling.
    pub(crate) _handler: &'a Handler,
    /// A strictly increasing counter, used to ensure that new variable names are unique.
    pub(crate) counter: usize,
    /// A flag to determine whether or not the traversal is on the left-hand side of a definition or an assignment.
    pub(crate) is_lhs: bool,
    /// Phi functions produced by static single assignment.
    pub(crate) phi_functions: Vec<Statement>,
    /// A stack of condition `Expression`s visited up to the current point in the AST.
    pub(crate) condition_stack: Vec<Expression>,
    /// A list containing tuples of guards and expressions associated with early `ReturnStatement`s.
    pub(crate) early_returns: Vec<(Option<Expression>, Expression)>,
}

impl<'a> StaticSingleAssigner<'a> {
    pub(crate) fn new(handler: &'a Handler) -> Self {
        Self {
            rename_table: RenameTable::new(None),
            _handler: handler,
            counter: 0,
            is_lhs: false,
            phi_functions: Vec::new(),
            condition_stack: Vec::new(),
            early_returns: Vec::new(),
        }
    }

    /// Return a new unique `Symbol` from a `&str`.
    pub(crate) fn unique_symbol(&mut self, arg: impl Display) -> Symbol {
        self.counter += 1;
        Symbol::intern(&format!("{}${}", arg, self.counter - 1))
    }

    /// Constructs the assignment statement `place = expr;`.
    pub(crate) fn simple_assign_statement(place: Expression, value: Expression) -> Statement {
        Statement::Assign(Box::new(AssignStatement {
            operation: AssignOperation::Assign,
            place,
            value,
            span: Default::default(),
        }))
    }

    /// Clears the `self.phi_functions`, returning the ones that were previously produced.
    pub(crate) fn clear_phi_functions(&mut self) -> Vec<Statement> {
        core::mem::take(&mut self.phi_functions)
    }

    /// Clears the state associated with `ReturnStatements`, returning the ones that were previously produced.
    pub(crate) fn clear_early_returns(&mut self) -> Vec<(Option<Expression>, Expression)> {
        core::mem::take(&mut self.early_returns)
    }

    /// Pushes a new scope, setting the current scope as the new scope's parent.
    pub(crate) fn push(&mut self) {
        let parent_table = core::mem::take(&mut self.rename_table);
        self.rename_table = RenameTable::new(Some(Box::from(parent_table)));
    }

    /// If the RenameTable has a parent, then `self.rename_table` is set to the parent, otherwise it is set to a default `RenameTable`.
    pub(crate) fn pop(&mut self) -> RenameTable {
        let parent = self.rename_table.parent.clone().unwrap_or_default();
        core::mem::replace(&mut self.rename_table, *parent)
    }

    /// Introduces a new `AssignStatement` for non-trivial expressions in the condition of `ConditionalStatement`s.
    /// For example,
    ///   - `if x > 0 { x = x + 1 }` becomes `let $cond$0 = x > 0; if $cond$0 { x = x + 1; }`
    ///   - `if true { x = x + 1 }` remains the same.
    ///   - `if b { x = x + 1 }` remains the same.
    /// And then reconstructs and flattens `ConditionalStatement`.
    pub(crate) fn flatten_conditional_statement(
        &mut self,
        conditional_statement: ConditionalStatement,
    ) -> Vec<Statement> {
        let mut statements = Vec::new();

        // Reconstruct the `ConditionalStatement`.
        let reconstructed_statement = match conditional_statement.condition {
            Expression::Err(_) => {
                unreachable!("Err expressions should not exist in the AST at this stage of compilation.")
            }
            Expression::Identifier(..) | Expression::Literal(..) => self.reconstruct_conditional(conditional_statement),
            // If the condition is a complex expression, introduce a new `AssignStatement` for it.
            Expression::Access(..)
            | Expression::Call(..)
            | Expression::Circuit(..)
            | Expression::Tuple(..)
            | Expression::Binary(..)
            | Expression::Unary(..)
            | Expression::Ternary(..) => {
                // Create a fresh variable name for the condition.
                let symbol = self.unique_symbol("$cond$");
                self.rename_table.update(symbol, symbol);

                // Initialize a new `AssignStatement` for the condition.
                let place = Expression::Identifier(Identifier::new(symbol));
                let assign_statement = Self::simple_assign_statement(
                    place.clone(),
                    self.reconstruct_expression(conditional_statement.condition).0,
                );
                let rewritten_conditional_statement = ConditionalStatement {
                    condition: place,
                    then: conditional_statement.then,
                    otherwise: conditional_statement.otherwise,
                    span: conditional_statement.span,
                };
                statements.push(assign_statement);
                self.reconstruct_conditional(rewritten_conditional_statement)
            }
        };

        // Flatten the reconstructed `ConditionalStatement`
        // by lifting the statements in the "if" and "else" block into their parent block.
        let mut conditional_statement = match reconstructed_statement {
            Statement::Conditional(conditional_statement) => conditional_statement,
            _ => unreachable!("`reconstruct_conditional` will always produce a `ConditionalStatement`"),
        };
        statements.append(&mut conditional_statement.then.statements);
        if let Some(statement) = conditional_statement.otherwise {
            match *statement {
                // If we encounter a `BlockStatement`,
                // we need to lift its constituent statements into the current `BlockStatement`.
                Statement::Block(mut block) => statements.append(&mut block.statements),
                _ => unreachable!(
                    "`self.reconstruct_conditional` will always produce a `BlockStatement` in the next block."
                ),
            }
        }

        // Add all phi functions to the current block.
        statements.append(&mut self.clear_phi_functions());

        statements
    }
}
