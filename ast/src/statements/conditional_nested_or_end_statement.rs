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

use crate::{ConditionalStatement, Statement};
use leo_grammar::statements::ConditionalNestedOrEndStatement as AstConditionalNestedOrEndStatement;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionalNestedOrEndStatement {
    Nested(Box<ConditionalStatement>),
    End(Vec<Statement>),
}

impl<'ast> From<AstConditionalNestedOrEndStatement<'ast>> for ConditionalNestedOrEndStatement {
    fn from(statement: AstConditionalNestedOrEndStatement<'ast>) -> Self {
        match statement {
            AstConditionalNestedOrEndStatement::Nested(nested) => {
                ConditionalNestedOrEndStatement::Nested(Box::new(ConditionalStatement::from(*nested)))
            }
            AstConditionalNestedOrEndStatement::End(statements) => {
                ConditionalNestedOrEndStatement::End(statements.into_iter().map(Statement::from).collect())
            }
        }
    }
}

impl fmt::Display for ConditionalNestedOrEndStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConditionalNestedOrEndStatement::Nested(ref nested) => write!(f, "else {}", nested),
            ConditionalNestedOrEndStatement::End(ref statements) => {
                writeln!(f, "else {{")?;
                for statement in statements.iter() {
                    writeln!(f, "\t\t{}", statement)?;
                }
                write!(f, "\t}}")
            }
        }
    }
}
