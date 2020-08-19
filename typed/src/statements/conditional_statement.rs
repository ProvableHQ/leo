// Copyright (C) 2019-2020 Aleo Systems Inc.
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

use crate::{ConditionalNestedOrEndStatement, Expression, Statement};
use leo_ast::statements::ConditionalStatement as AstConditionalStatement;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConditionalStatement {
    pub condition: Expression,
    pub statements: Vec<Statement>,
    pub next: Option<ConditionalNestedOrEndStatement>,
}

impl<'ast> From<AstConditionalStatement<'ast>> for ConditionalStatement {
    fn from(statement: AstConditionalStatement<'ast>) -> Self {
        ConditionalStatement {
            condition: Expression::from(statement.condition),
            statements: statement
                .statements
                .into_iter()
                .map(|statement| Statement::from(statement))
                .collect(),
            next: statement
                .next
                .map(|n_or_e| Some(ConditionalNestedOrEndStatement::from(n_or_e)))
                .unwrap_or(None),
        }
    }
}

impl fmt::Display for ConditionalStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "if ({}) {{\n", self.condition)?;
        for statement in self.statements.iter() {
            write!(f, "\t\t{}\n", statement)?;
        }
        match self.next.clone() {
            Some(n_or_e) => write!(f, "\t}} {}", n_or_e),
            None => write!(f, "\t}}"),
        }
    }
}
