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

use leo_ast::{AssignStatement, Expression, Identifier, Statement};
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
    /// A stack of condition `Expression`s visited up to the current point in the AST.
    pub(crate) condition_stack: Vec<Expression>,
    /// A list containing tuples of guards and expressions associated with early `ReturnStatement`s.
    /// Note that early returns are inserted in the order they are encountered during a pre-order traversal of the AST.
    pub(crate) early_returns: Vec<(Option<Expression>, Expression)>,
}

impl<'a> StaticSingleAssigner<'a> {
    pub(crate) fn new(handler: &'a Handler) -> Self {
        Self {
            rename_table: RenameTable::new(None),
            _handler: handler,
            counter: 0,
            is_lhs: false,
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
            place,
            value,
            span: Default::default(),
        }))
    }

    /// Constructs a simple assign statement for `expr` with a unique name.
    /// For example, `expr` is transformed into `$var$0 = expr;`.
    pub(crate) fn unique_simple_assign_statement(&mut self, expr: Expression) -> (Expression, Statement) {
        // Create a new variable for the expression.
        let name = self.unique_symbol("$var");
        let place = Expression::Identifier(Identifier {
            name,
            span: Default::default(),
        });
        // Update the rename table.
        self.rename_table.update(name, name);

        (place.clone(), Self::simple_assign_statement(place, expr))
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
}
