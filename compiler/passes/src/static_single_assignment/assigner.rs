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

use leo_ast::{AssignStatement, Expression, Identifier, Statement};
use leo_span::Symbol;
use std::fmt::Display;

/// A struct used to create assignment statements.
#[derive(Default)]
pub struct Assigner {
    /// A strictly increasing counter, used to ensure that new variable names are unique.
    pub(crate) counter: usize,
}

impl Assigner {
    /// Return a new unique `Symbol` from a `&str`.
    pub(crate) fn unique_symbol(&mut self, arg: impl Display) -> Symbol {
        self.counter += 1;
        Symbol::intern(&format!("{}${}", arg, self.counter - 1))
    }

    /// Constructs the assignment statement `place = expr;`.
    /// This function should be the only place where `AssignStatement`s are constructed.
    pub(crate) fn simple_assign_statement(&mut self, identifier: Identifier, value: Expression) -> Statement {
        Statement::Assign(Box::new(AssignStatement {
            place: Expression::Identifier(identifier),
            value,
            span: Default::default(),
        }))
    }

    /// Constructs a simple assign statement for `expr` with a unique name.
    /// For example, `expr` is transformed into `$var$0 = expr;`.
    pub(crate) fn unique_simple_assign_statement(&mut self, expr: Expression) -> (Expression, Statement) {
        // Create a new variable for the expression.
        let name = self.unique_symbol("$var");

        let place = Identifier {
            name,
            span: Default::default(),
        };

        (Expression::Identifier(place), self.simple_assign_statement(place, expr))
    }
}
